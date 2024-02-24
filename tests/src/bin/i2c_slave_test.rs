//! Test runner for the MAX78000 HAL I2C Master Mode.

#![warn(missing_docs)]
#![no_main]
#![no_std]

use core::fmt::Write;

use cortex_m_rt::entry;
use cortex_m_semihosting::hio;
use max78000_hal::{max78000::Peripherals, peripherals::i2c, peripherals::timer};
use max78000_hal::peripherals::timer::{Clock, Oscillator, Prescaler, Time};
use embedded_hal::i2c::I2c;
use max78000_hal::peripherals::i2c::SlavePollResult;

extern crate panic_semihosting;


/// Entry point for tests.
#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();

    writeln!(stdout, "Starting i2c slave tests...\n").unwrap();

    let peripherals = Peripherals::take().unwrap();
    let mut i2c_slave = i2c::I2CSlave::new(&peripherals.GCR, peripherals.I2C1, 69);
    let clock = Clock::new(peripherals.TMR, &peripherals.GCR, Oscillator::IBRO, Prescaler::_64);

    let mut timer = clock.new_timer(Time::Milliseconds(1000));

    writeln!(stdout, "Waiting for 3 seconds to come online\n").unwrap();

    while !timer.poll() {}

    writeln!(stdout, "Ok we poll now\n").unwrap();

    let mut buf = [0u8; 256];

    if let SlavePollResult::Received(num, overflow) = i2c_slave.slave_poll(&mut buf).unwrap() {
        writeln!(stdout, stringify!(buf)).unwrap();
    }
    writeln!(stdout, "Finished i2c slave tests!\n").unwrap();

    #[allow(clippy::empty_loop)]
    loop {}
}
