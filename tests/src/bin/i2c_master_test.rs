//! Test runner for the MAX78000 HAL I2C Master Mode.

#![warn(missing_docs)]
#![no_main]
#![no_std]

use core::fmt::Write;

use cortex_m_rt::entry;
use cortex_m_semihosting::hio;

use embedded_hal::i2c::I2c;
use max78000_hal::peripherals::gpio::pin_traits::IoPin;
use max78000_hal::peripherals::gpio::PinOperatingMode;
use max78000_hal::peripherals::oscillator::{Iso, IsoDivider, IsoFrequency};
use max78000_hal::peripherals::timer::Time;
use max78000_hal::peripherals::{PeripheralManagerBuilder, SplittablePeripheral};
use max78000_hal::{max78000::Peripherals, peripherals::i2c};

extern crate panic_semihosting;

/// Entry point for tests.
#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();

    writeln!(stdout, "Starting i2c master tests...\n").unwrap();

    let (to_consume, to_borrow, rem) = Peripherals::take().unwrap().split();
    let manager = PeripheralManagerBuilder::<Iso>::new(
        &to_borrow,
        to_consume,
        IsoFrequency::_60MHz,
        IsoDivider::_1,
    )
    .build();

    let gpio0 = manager.gpio0();
    let mut scl_handle = gpio0.get_pin_handle(16).unwrap();
    let mut sda_handle = gpio0.get_pin_handle(17).unwrap();

    sda_handle
        .set_operating_mode(PinOperatingMode::AltFunction1)
        .unwrap();
    scl_handle
        .set_operating_mode(PinOperatingMode::AltFunction1)
        .unwrap();

    let clock = manager.timer_0().unwrap();

    // whether or not the test should use the manual rewrite of i2c protocol using pins or not
    let mut i2c_master = i2c::I2CMaster::new(rem.i2c1);

    let mut timer = clock.new_timer(Time::Milliseconds(1000));

    writeln!(stdout, "Waiting for 1 seconds to let slave come online\n").unwrap();

    while !timer.poll() {}

    writeln!(stdout, "Writing to slave...\n").unwrap();

    let mut stuff = [0u8; 16];

    /*loop {
        timer.reset();
        while !timer.poll() {}
        writeln!(stdout, "Toggling logic...").unwrap();
        if scl_state {scl_output.set_high().unwrap()} else {scl_output.set_low().unwrap()}
        scl_state = !scl_state;
    }*/

    i2c_master.write(69, "ping".as_bytes()).unwrap();

    writeln!(stdout, "Reading from slave...").unwrap();

    i2c_master.read(69, &mut stuff).unwrap();

    writeln!(stdout, "Read {:?}", stuff).unwrap();

    writeln!(stdout, "Finished i2c master tests!\n").unwrap();

    #[allow(clippy::empty_loop)]
    loop {}
}
