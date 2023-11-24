//! Test runner for the MAX78000 HAL.

#![warn(missing_docs)]
#![no_main]
#![no_std]

use core::fmt::{self, Write};

use cortex_m_rt::entry;
use cortex_m_semihosting::hio;
use max78000_hal::max78000::Peripherals;
use tests::trng_tests;

extern crate panic_halt;

pub mod tests;

/// Entry point for tests.
#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().map_err(|_| fmt::Error).unwrap();
    writeln!(stdout, "Starting MAX78000 HAL tests...\n").unwrap();

    // TODO: Use peripheral API when available.
    let peripherals = Peripherals::take().unwrap();
    trng_tests::run_trng_tests(peripherals.TRNG, &peripherals.GCR, &mut stdout);

    #[allow(clippy::empty_loop)]
    loop {}
}
