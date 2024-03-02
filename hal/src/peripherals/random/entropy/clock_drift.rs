use bitvec::prelude::*;
use sha3::{digest::Update, Sha3_256};
use zeroize::Zeroize;

use crate::peripherals::{random::CsprngInitArgs, timer::Time};

use super::EntropySource;

/// Number of milliseconds to count for clock drift.
const MS_TO_COUNT: u32 = 4;

/// The number of bytes to get from clock drift.
const CLOCK_DRIFT_ENTROPY_SIZE: usize = 24;

/// Clock drift entropy source.
///
/// IMPORTANT: This struct should not be moved to ensure the entropy gets zeroed out on drop.
pub(crate) struct ClockDrift<T: EntropySource> {
    next: T,
    entropy: [u8; CLOCK_DRIFT_ENTROPY_SIZE],
}

impl<T: EntropySource> EntropySource for ClockDrift<T> {
    fn init<F: FnMut(&mut [u8])>(csprng_init_args: CsprngInitArgs<F>) -> Self {
        let mut entropy_pool = [0; CLOCK_DRIFT_ENTROPY_SIZE];

        for mut bit in entropy_pool.as_mut_bits::<Lsb0>() {
            // Initialize timer.
            let mut clock_drift_timer = csprng_init_args
                .csprng_timer
                .new_timer(Time::Milliseconds(MS_TO_COUNT));

            // Wait for timer to reach MS_TO_COUNT ms and count.
            let mut counter: u32 = 0;

            while !clock_drift_timer.poll() {
                counter += 1;
            }

            // Set bit to 1 if counter LSB is 1.
            bit.set((counter & 1) == 1);
        }

        ClockDrift {
            next: T::init(csprng_init_args),
            entropy: entropy_pool,
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
