//! CSPRNG (cryptographically-secure pseudorandom number generator) abstraction API.

mod entropy;

use max78000::TMR;
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};

use crate::communication::lower_layers::crypto::RandomSource;

use self::entropy::{ClockDrift, EntropyHasher, Secret, TrngEntropy};

use super::{timer::Clock, trng::Trng};

/// The size of the static secret in bytes.
pub const SECRET_SIZE: usize = 32;

/// CSPRNG initialization arguments.
pub(crate) struct CsprngInitArgs<'a, 'b, 'c, F: FnMut(&mut [u8])> {
    pub trng: &'a Trng,
    pub csprng_timer: &'b Clock<'c, TMR>,
    pub get_rng_static_secret: F,
}

/// Entropy gatherer.
pub(crate) struct EntropyGatherer {}

impl EntropyGatherer {
    /// Creates a new entropy gatherer.
    pub(crate) fn init_csprng<F: FnMut(&mut [u8])>(
        csprng_init_args: CsprngInitArgs<F>,
    ) -> ChaCha20Rng {
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
