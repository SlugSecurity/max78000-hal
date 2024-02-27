//! CSPRNG (cryptographically-secure pseudorandom number generator) abstraction API.

mod entropy;

use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};

use crate::communication::lower_layers::crypto::RandomSource;

use self::entropy::{ClockDrift, EntropyHasher, Secret, TrngEntropy};

use super::trng::Trng;

/// The size of the static secret in bytes.
pub const SECRET_SIZE: usize = 32;

/// CSPRNG initialization arguments.
pub(crate) struct CsprngInitArgs<'a> {
    pub trng: &'a Trng,
    pub get_rng_static_secret: fn(&mut [u8]),
}

/// Entropy gatherer.
pub(crate) struct EntropyGatherer {}

impl EntropyGatherer {
    /// Creates a new entropy gatherer.
    pub(crate) fn init_csprng(csprng_init_args: CsprngInitArgs) -> ChaCha20Rng {
        ChaCha20Rng::from_seed(
            EntropyHasher::<Secret<ClockDrift<TrngEntropy<()>>>>::new(csprng_init_args).hash(),
        )
    }
}

impl RandomSource for ChaCha20Rng {
    fn fill_rand_slice<T: AsMut<[u8]>>(&mut self, mut slice_ref: T) {
        self.fill_bytes(slice_ref.as_mut());
    }
}
