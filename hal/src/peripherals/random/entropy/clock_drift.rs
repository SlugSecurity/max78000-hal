use sha3::{digest::Update, Sha3_256};
use zeroize::Zeroize;

use crate::peripherals::random::CsprngInitArgs;

use super::EntropySource;

/// The number of bytes to get from the TRNG.
const CLOCK_DRIFT_ENTROPY_SIZE: usize = 64;

/// TRNG entropy source.
///
/// This struct should not be moved to ensure the entropy gets zeroed out on drop.
pub(crate) struct ClockDrift<T: EntropySource> {
    next: T,
    entropy: [u8; CLOCK_DRIFT_ENTROPY_SIZE],
}

impl<T: EntropySource> EntropySource for ClockDrift<T> {
    fn init(csprng_init_args: CsprngInitArgs) -> Self {
        let mut clock_drift_entropy = [0; CLOCK_DRIFT_ENTROPY_SIZE];
        // TODO: Implement.

        ClockDrift {
            next: T::init(csprng_init_args),
            entropy: clock_drift_entropy,
        }
    }

    fn add_to_hasher(&self, hasher: &mut Sha3_256) {
        hasher.update(&self.entropy);
        self.next.add_to_hasher(hasher);
    }
}

impl<T: EntropySource> Drop for ClockDrift<T> {
    fn drop(&mut self) {
        self.entropy.zeroize();
    }
}
