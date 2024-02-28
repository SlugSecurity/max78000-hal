//! Test runner for the MAX78000 HAL I2C Master Mode.

#![warn(missing_docs)]
#![no_main]
#![no_std]

use core::fmt::Write;

use cortex_m_rt::entry;
use cortex_m_semihosting::hio;
use embedded_hal::digital::{OutputPin, PinState};
use embedded_hal::i2c::I2c;
use max78000_hal::peripherals::gpio::PinOperatingMode;
use max78000_hal::peripherals::timer::{Clock, Oscillator, Prescaler, Time};
use max78000_hal::{max78000::Peripherals, peripherals::i2c};

use max78000_hal::peripherals::gpio::pin_traits::IoPin;
use max78000_hal::peripherals::oscillator::{Ipo, IpoDivider, IpoFrequency};
use max78000_hal::peripherals::{PeripheralManagerBuilder, SplittablePeripheral};

extern crate panic_semihosting;

/// Entry point for tests.
#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();

    writeln!(stdout, "Starting i2c master tests...\n").unwrap();

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

    let clock = manager.system_clock().unwrap();
    manager.system_clock().unwrap().get_freq();
    manager.system_clock().unwrap().get_div();



    let gpio0 = manager.gpio0();

    let mut scl_handle = gpio0.get_pin_handle(16).unwrap();
    let mut sda_handle = gpio0.get_pin_handle(17).unwrap();

    sda_handle
        .set_operating_mode(PinOperatingMode::AltFunction1)
        .unwrap();
    scl_handle
        .set_operating_mode(PinOperatingMode::AltFunction1)
        .unwrap();

    let timer = manager.timer_0().unwrap();
    let timer2 = manager.timer_1().unwrap();
    let tr = timer.new_timer(Time::Milliseconds(3000));

    let clock = Clock::new(
        peripherals.TMR,
        &peripherals.GCR,
        Oscillator::IBRO,
        Prescaler::_64,
    );

    let delay_timer = clock.new_timer(Time::Milliseconds(1));

    // whether or not the test should use the manual rewrite of i2c protocol using pins or not
    let mut i2c_master = i2c::I2CMaster::new(&peripherals.GCR, peripherals.I2C1);

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
