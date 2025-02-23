//! Test runner for the MAX78000 HAL.

#![warn(missing_docs)]
#![no_main]
#![no_std]

use core::fmt::Write;

use cortex_m_rt::entry;
use cortex_m_semihosting::hio;
use max78000_hal::{
    max78000::Peripherals,
    peripherals::{
        oscillator::{Ipo, IpoDivider, IpoFrequency},
        timer::{Oscillator, Prescaler},
        PeripheralManagerBuilder, SplittablePeripheral,
    },
};
use tests::{
    bit_band_tests, csprng_tests, flc_tests, gpio_tests, oscillator_tests, timer_tests, trng_tests,
    uart_tests,
};

extern crate panic_semihosting;

pub mod tests;

/// Oscillator to use for TMR0 during tests
pub const TIMER_0_OSC: Oscillator = Oscillator::ERTCO;
/// Prescaler to use for TMR0 during tests
pub const TIMER_0_PRESCALER: Prescaler = Prescaler::_1;

/// Entry point for tests.
#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Starting MAX78000 HAL tests...\n").unwrap();

    let (to_consume, to_borrow, rem) = Peripherals::take().unwrap().split();

    // WARNING: This is for testing purposes only. Do NOT copy this into production code. Production code MUST have a secure static secret.
    let static_secret = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
        0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D,
        0x1E, 0x1F,
    ];

    let manager = PeripheralManagerBuilder::<Ipo, _>::new(
        &to_borrow,
        to_consume,
        IpoFrequency::_100MHz,
        IpoDivider::_1,
        // WARNING: This is for testing purposes only. Do NOT copy this into production code. Production code MUST have a secure static secret.
        |buf| buf.copy_from_slice(&static_secret),
    )
    .configure_timer_0(TIMER_0_OSC, TIMER_0_PRESCALER)
    .configure_timer_1(Oscillator::IBRO, Prescaler::_512)
    .configure_timer_2(Oscillator::ISO, Prescaler::_4096)
    .build();

    // run FLC tests with semi-hosting
    flc_tests::run_flc_tests(
        &mut stdout,
        manager.flash_controller().unwrap(),
        manager.system_clock().unwrap(),
    );

    {
        let mut uart = manager.build_uart().unwrap().build(115200);

        // run FLC tests with UART
        flc_tests::run_flc_tests(
            &mut uart,
            manager.flash_controller().unwrap(),
            manager.system_clock().unwrap(),
        );

        // UART instance is tossed here
    }

    flc_tests::run_flc_tests(
        &mut stdout,
        manager.flash_controller().unwrap(),
        manager.system_clock().unwrap(),
    );

    bit_band_tests::run_bit_band_tests(&mut stdout, &rem.rtc);

    oscillator_tests::run_oscillator_tests(
        to_borrow.gcr.clkctrl(),
        manager.system_clock().unwrap(),
        &mut stdout,
        #[cfg(feature = "low_frequency_test")]
        to_borrow.trimsir.inro(),
    );

    timer_tests::run_timer_tests(
        &mut stdout,
        manager.timer_0().unwrap(),
        manager.timer_1().unwrap(),
        manager.timer_2().unwrap(),
    );

    trng_tests::run_trng_tests(manager.trng().unwrap(), &mut stdout);
    csprng_tests::run_csprng_tests(manager.csprng().unwrap(), &mut stdout);

    gpio_tests::run_gpio_tests(
        manager.gpio0(),
        manager.gpio1(),
        manager.gpio2(),
        &mut stdout,
    );

    uart_tests::run_uart_test(
        &mut stdout,
        manager.build_uart().unwrap(),
        manager.timer_0().unwrap(),
    );

    writeln!(stdout, "Finished MAX78000 HAL tests!\n").unwrap();

    #[allow(clippy::empty_loop)]
    loop {}
}
