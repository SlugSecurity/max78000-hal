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
    bit_band_tests, flc_tests, gpio_tests, hello_uart, oscillator_tests, timer_tests, trng_tests,
};

extern crate panic_semihosting;

pub mod tests;

const TIMER_0_OSC: Oscillator = Oscillator::ERTCO;
const TIMER_0_PRESCALER: Prescaler = Prescaler::_1;

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
    .configure_timer_0(TIMER_0_OSC, TIMER_0_PRESCALER)
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
        manager.gpio0(),
        manager.gpio1(),
        manager.gpio2(),
        &mut stdout,
    );

    // do not run! these 2 lines break the board you run them on
    // peripherals.GCR.clkctrl().modify(|_r, w| w.ibro_en().en());
    peripherals.GCR.pclkdis0().modify(|_r, w| w.uart0().en());

    // function mode alt 1
    peripherals.GPIO0.en1_clr().write(|w| w.all().variant(0b11));
    peripherals.GPIO0.en0_clr().write(|w| w.all().variant(0b11));

    // pad mode none
    peripherals
        .GPIO0
        .padctrl0()
        .modify(|r, w| w.gpio_padctrl0().variant(r.bits() & !0b11));
    peripherals
        .GPIO0
        .padctrl1()
        .modify(|r, w| w.gpio_padctrl1().variant(r.bits() & !0b11));

    // voltage vddio
    peripherals
        .GPIO0
        .vssel()
        .modify(|r, w| w.all().variant(r.bits() & !0b11));

    // drive strength 0
    peripherals
        .GPIO0
        .ds0()
        .modify(|r, w| w.gpio_ds0().variant(r.bits() & !0b11));
    peripherals
        .GPIO0
        .ds1()
        .modify(|r, w| w.gpio_ds1().variant(r.bits() & !0b11));

    hello_uart::run_uart_test(&peripherals.UART, &mut stdout);

    writeln!(stdout, "Finished MAX78000 HAL tests!\n").unwrap();

    #[allow(clippy::empty_loop)]
    loop {}
}
