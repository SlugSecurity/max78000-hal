//! Test runner for the MAX78000 HAL I2C Master Mode.

#![warn(missing_docs)]
#![no_main]
#![no_std]

use core::fmt::Write;

use cortex_m_rt::entry;
use cortex_m_semihosting::hio;
use embedded_hal::digital::InputPin;
use max78000_hal::{max78000::Peripherals, peripherals::i2c, peripherals::timer};
use max78000_hal::peripherals::timer::{Clock, Oscillator, Prescaler, Time};
use embedded_hal::i2c::I2c;
use max78000_hal::peripherals::i2c::SlavePollResult;
use max78000_hal::peripherals::gpio;
use max78000_hal::peripherals::gpio::pin_traits::{GeneralIoPin, IoPin};
use max78000_hal::peripherals::gpio::PinOperatingMode;

extern crate panic_semihosting;


/// Entry point for tests.
#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();

    writeln!(stdout, "Starting i2c slave tests...\n").unwrap();

    let peripherals = Peripherals::take().unwrap();

    let gpio0 = gpio::new_gpio0(peripherals.GPIO0);

    /*peripherals.GPIO0.en0().modify(|r, w| w.gpio_en().variant(
        r.gpio_en().bits() | (((1 << 16) | (1 << 17)))
    ));

    peripherals.GPIO0.en1().modify(|r, w| w.gpio_en1().variant(
        r.gpio_en1().bits() & (!((1 << 16) | (1 << 17)))
    ));

    peripherals.GPIO0.en2().modify(|r, w| w.gpio_en2().variant(
        r.gpio_en2().bits() & (!((1 << 16) | (1 << 17)))
    ));

    peripherals.GPIO0.en0().modify(|r, w| w.gpio_en().variant(
        r.gpio_en().bits() & (!((1 << 16) | (1 << 17)))
    ));*/

    let mut scl_handle = gpio0.get_pin_handle(16).unwrap().into_input_pin().unwrap();
    let mut sda_handle = gpio0.get_pin_handle(17).unwrap();

    sda_handle.set_operating_mode(PinOperatingMode::AltFunction1);
    scl_handle.set_operating_mode(PinOperatingMode::AltFunction1);

    let mut i2c_slave = i2c::I2CSlave::new(&peripherals.GCR, peripherals.I2C1, 69);
    let clock = Clock::new(peripherals.TMR, &peripherals.GCR, Oscillator::IBRO, Prescaler::_64);

    let mut timer = clock.new_timer(Time::Milliseconds(1000));

    writeln!(stdout, "Waiting for 3 seconds to come online\n").unwrap();

    while !timer.poll() {}

    writeln!(stdout, "Ok we poll now\n").unwrap();

    let mut buf = [0u8; 256];

    let mut scl = scl_handle.is_high(); //peripherals.GPIO0.in_().read().bits() & (1 << 16);
    // let mut sda = peripherals.GPIO0.in_().read().bits() & (1 << 17);

    // let read_sda = || peripherals.GPIO0.in_().read().bits() & (1 << 17);
    //let mut read_scl = || scl_handle.is_high(); //peripherals.GPIO0.in_().read().bits() & (1 << 16);

    loop {
        while scl_handle.is_high() == scl {};
        scl = scl_handle.is_high();
        writeln!(stdout, "SDA changed!!\n").unwrap();
    }

    if let SlavePollResult::Received(num, overflow) = i2c_slave.slave_poll(&mut buf).unwrap() {
        writeln!(stdout, stringify!(buf)).unwrap();
    }
    writeln!(stdout, "Finished i2c slave tests!\n").unwrap();

    #[allow(clippy::empty_loop)]
    loop {}
}
