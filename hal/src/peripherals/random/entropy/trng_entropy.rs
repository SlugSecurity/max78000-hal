use sha3::{digest::Update, Sha3_256};
use zeroize::Zeroize;

use crate::peripherals::random::CsprngInitArgs;

use super::EntropySource;

/// The number of bytes to get from the TRNG.
const TRNG_ENTROPY_SIZE: usize = 64;

/// TRNG entropy source.
///
/// IMPORTANT: This struct should not be moved to ensure the entropy gets zeroed out on drop.
pub(crate) struct TrngEntropy<T: EntropySource> {
    next: T,
    entropy: [u8; TRNG_ENTROPY_SIZE],
}

impl<T: EntropySource> EntropySource for TrngEntropy<T> {
    fn init(csprng_init_args: CsprngInitArgs) -> Self {
        let mut trng_entropy = [0; TRNG_ENTROPY_SIZE];
        csprng_init_args.trng.fill_buffer(&mut trng_entropy);

        TrngEntropy {
            next: T::init(csprng_init_args),
            entropy: trng_entropy,
        }
    }

    fn add_to_hasher(&self, hasher: &mut Sha3_256) {
        hasher.update(&self.entropy);
        self.next.add_to_hasher(hasher);
    }
}

impl<T: EntropySource> Drop for TrngEntropy<T> {
    fn drop(&mut self) {
        self.entropy.zeroize();
    }
}
