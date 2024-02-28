//! Test runner for the MAX78000 HAL I2C Master Mode.

#![warn(missing_docs)]
#![no_main]
#![no_std]

use core::fmt::Write;

use cortex_m_rt::entry;
use cortex_m_semihosting::hio;

use embedded_hal::i2c::I2c;
use max78000_hal::max78000::Peripherals;
use max78000_hal::peripherals::i2c::BusSpeed;
use max78000_hal::peripherals::oscillator::{Iso, IsoDivider, IsoFrequency};
use max78000_hal::peripherals::timer::{Oscillator, Prescaler, Time};
use max78000_hal::peripherals::{PeripheralManagerBuilder, SplittablePeripheral};

extern crate panic_semihosting;

/// Entry point for tests.
#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();

    writeln!(stdout, "Starting i2c master tests...\n").unwrap();

    let (to_consume, to_borrow, _rem) = Peripherals::take().unwrap().split();
    let manager = PeripheralManagerBuilder::<Iso>::new(
        &to_borrow,
        to_consume,
        IsoFrequency::_60MHz,
        IsoDivider::_1,
    )
    .configure_timer_0(Oscillator::ERTCO, Prescaler::_1)
    .build();

    let mut i2c_master = manager.i2c_master(BusSpeed::Standard100kbps).unwrap();

    let clock = manager.timer_0().unwrap();

    let mut timer = clock.new_timer(Time::Milliseconds(1000));

    writeln!(stdout, "Waiting for 1 seconds to let slave come online\n").unwrap();

    while !timer.poll() {}

    writeln!(stdout, "Writing to slave...\n").unwrap();

    let mut stuff = [0u8; 16];

    i2c_master.write(69, "ping".as_bytes()).unwrap();

    writeln!(stdout, "Reading from slave...").unwrap();

    i2c_master.read(69, &mut stuff).unwrap();

    writeln!(stdout, "Read {:?}", stuff).unwrap();

    writeln!(stdout, "Finished i2c master tests!\n").unwrap();

    #[allow(clippy::empty_loop)]
    loop {}
}
