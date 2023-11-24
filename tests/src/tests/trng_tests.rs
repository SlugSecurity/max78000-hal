//! Tests for the TRNG peripheral.

use max78000_hal::{max78000::TRNG, peripherals::trng};

/// Runs all TRNG tests.
pub fn run_trng_tests(trng: TRNG) {
    trng::Trng::new(trng);
}
