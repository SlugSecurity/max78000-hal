mod clock_drift;
mod secret;
mod trng_entropy;

pub(crate) use clock_drift::ClockDrift;
pub(crate) use secret::Secret;
pub(crate) use trng_entropy::TrngEntropy;

use sha3::{Digest, Sha3_256};

use super::CsprngInitArgs;

/// The size of the hashed entropy. 256 bits = 32 bytes.
const ENTROPY_HASH_SIZE: usize = 32;

/// A trait for all entropy sources.
pub(crate) trait EntropySource {
    /// Initializes the internal state of the entropy source. May block to gather entropy.
    ///
    /// IMPORTANT NOTE: This function must call the next entropy source's `init()` function.
    fn init(csprng_init_args: CsprngInitArgs) -> Self;

    /// Adds entropy from the entropy source to a hasher.
    ///
    /// IMPORTANT NOTE: This function must call the next entropy source's `add_to_hasher()` function.
    fn add_to_hasher(&self, hasher: &mut Sha3_256);
}

// We implement this trait for () so that we can use it to end the list of entropy sources.
impl EntropySource for () {
    fn init(_csprng_init_args: CsprngInitArgs) {}
    fn add_to_hasher(&self, _hasher: &mut Sha3_256) {}
}

/// A hasher that concatenates entropy sources together and hashes the result.
pub(crate) struct EntropyHasher<T: EntropySource> {
    /// The sources of entropy to hash.
    entropy: T,
}

impl<T: EntropySource> EntropyHasher<T> {
    /// Initializes the entropy hasher, gathering entropy from all of the inputted sources.
    pub(crate) fn new(csprng_init_args: CsprngInitArgs) -> Self {
        EntropyHasher {
            entropy: T::init(csprng_init_args),
        }
    }

    /// Concatenates entropy sources together and hashes the result.
    pub(crate) fn hash(&self) -> [u8; ENTROPY_HASH_SIZE] {
        let mut hasher = Sha3_256::new();
        self.entropy.add_to_hasher(&mut hasher);
        hasher.finalize().into()
    }
}
