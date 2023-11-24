//! Test runner for the MAX78000 HAL.

#![warn(missing_docs)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use max78000_hal::max78000::Peripherals;
use tests::trng_tests;

extern crate panic_halt;

pub mod tests;

/// Entry point for tests.
#[entry]
fn main() -> ! {
    // TODO: Use peripheral API when available.
    let peripherals = Peripherals::take().unwrap();
    trng_tests::run_trng_tests(peripherals.TRNG);

    #[allow(clippy::empty_loop)]
    loop {}
}
