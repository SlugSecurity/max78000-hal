//! Test runner for the MAX78000 HAL I2C Master Mode.

#![warn(missing_docs)]
#![no_main]
#![no_std]

use core::fmt::Write;

use cortex_m_rt::entry;
use cortex_m_semihosting::hio;
use max78000_hal::max78000::Peripherals;
use max78000_hal::peripherals::i2c::{BusSpeed, SlavePollResult};
use max78000_hal::peripherals::oscillator::{Iso, IsoDivider, IsoFrequency};
use max78000_hal::peripherals::timer::{Oscillator, Prescaler, Time};
use max78000_hal::peripherals::{PeripheralManagerBuilder, SplittablePeripheral};

extern crate panic_semihosting;

/// Entry point for tests.
#[entry]
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();

    writeln!(stdout, "Starting i2c slave tests...\n").unwrap();

    let (to_consume, to_borrow, _rem) = Peripherals::take().unwrap().split();
    let manager = PeripheralManagerBuilder::<Iso>::new(
        &to_borrow,
        to_consume,
        IsoFrequency::_60MHz,
        IsoDivider::_1,
    )
    .configure_timer_0(Oscillator::ERTCO, Prescaler::_1)
    .build();

    let mut i2c_slave = manager.i2c_slave(BusSpeed::Standard100kbps, 69).unwrap();
    let clock = manager.timer_0().unwrap();

    let mut timer = clock.new_timer(Time::Milliseconds(1000));

    writeln!(stdout, "Waiting for 1 second to come online\n").unwrap();

    while !timer.poll() {}

    writeln!(stdout, "Ok we poll now\n").unwrap();

    let mut buf = [0u8; 16];
    let res = i2c_slave.slave_poll(&mut buf).unwrap();

    match res {
        SlavePollResult::Received(_, _) => {
            writeln!(stdout, "received: {:?}", buf).unwrap();
        }
        SlavePollResult::TransmitNeeded => {
            writeln!(stdout, "transmit needed").unwrap();
        }
    }

    writeln!(stdout, "Polling again...").unwrap();

    let res = i2c_slave.slave_poll(&mut buf).unwrap();

    match res {
        SlavePollResult::Received(num, overflow) => {
            writeln!(stdout, "received: {:?} {} {}", buf, num, overflow).unwrap();
        }
        SlavePollResult::TransmitNeeded => {
            writeln!(stdout, "transmit needed").unwrap();
            let n = i2c_slave.slave_send("pong".as_bytes());
            writeln!(stdout, "sent {} bytes", n).unwrap();
        }
    }

    writeln!(stdout, "Finished i2c slave tests!\n").unwrap();

    #[allow(clippy::empty_loop)]
    loop {}
}
