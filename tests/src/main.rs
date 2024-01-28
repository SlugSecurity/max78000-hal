//! Test runner for the MAX78000 HAL.

#![warn(missing_docs)]
#![no_main]
#![no_std]

use core::fmt::Write;

use cortex_m_rt::entry;
use cortex_m_semihosting::hio;
use max78000_hal::{max78000::Peripherals, peripherals::power::PowerControl};
use tests::{bit_band_tests, oscillator_tests, timer_tests, trng_tests};

extern crate panic_halt;

pub mod tests;

/// Entry point for tests.
#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Starting MAX78000 HAL tests...\n").unwrap();

    // TODO: Use peripheral API when available.
    let peripherals = Peripherals::take().unwrap();

    bit_band_tests::run_bit_band_tests(&mut stdout, &peripherals.RTC);

    oscillator_tests::run_oscillator_tests(
        peripherals.GCR.clkctrl(),
        peripherals.TRIMSIR.inro(),
        &mut stdout,
    );

    // TODO: Assuming peripheral API will initialize this later.
    let power = PowerControl::new(&peripherals.GCR, &peripherals.LPGCR);

    timer_tests::run_timer_tests(
        &mut stdout,
        peripherals.TMR,
        peripherals.TMR1,
        &peripherals.GCR,
    );

    trng_tests::run_trng_tests(peripherals.TRNG, &power, &mut stdout);

    writeln!(stdout, "Finished MAX78000 HAL tests!\n").unwrap();

    #[allow(clippy::empty_loop)]
    loop {}
}
