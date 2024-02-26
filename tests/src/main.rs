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
use tests::{bit_band_tests, flc_tests, gpio_tests, oscillator_tests, timer_tests, trng_tests};

extern crate panic_semihosting;

pub mod tests;

/// Entry point for tests.
#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Starting MAX78000 HAL tests...\n").unwrap();

    // TODO: Use peripheral API when available.
    let (to_consume, to_borrow, rem) = Peripherals::take().unwrap().split();
    let manager = PeripheralManagerBuilder::<Ipo>::new(
        &to_borrow,
        to_consume,
        IpoFrequency::_100MHz,
        IpoDivider::_1,
    )
    .configure_timer_0(Oscillator::ERTCO, Prescaler::_1)
    .configure_timer_1(Oscillator::IBRO, Prescaler::_512)
    .configure_timer_2(Oscillator::ISO, Prescaler::_4096)
    .build();

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
    gpio_tests::run_gpio_tests(
        manager.gpio0().unwrap(),
        manager.gpio1().unwrap(),
        manager.gpio2().unwrap(),
        &mut stdout,
    );

    writeln!(stdout, "Finished MAX78000 HAL tests!\n").unwrap();

    #[allow(clippy::empty_loop)]
    loop {}
}
