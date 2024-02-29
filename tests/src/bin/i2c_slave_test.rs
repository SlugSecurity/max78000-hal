//! Test runner for the MAX78000 HAL I2C Master Mode.

#![warn(missing_docs)]
#![no_main]
#![no_std]

use core::fmt::Write;

use cortex_m_rt::entry;
use cortex_m_semihosting::hio;
use max78000_hal::communication::{RxChannel, TxChannel};
use max78000_hal::max78000::Peripherals;
use max78000_hal::peripherals::i2c::master::InfTimeout;
use max78000_hal::peripherals::i2c::BusSpeed;
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

    let mut buf = [0u8; 4];
    let res = i2c_slave
        .recv_with_data_timeout(&mut buf, &mut InfTimeout::new())
        .unwrap();

    let mut buf2 = [5, 6, 7, 8];
    i2c_slave.send(&mut buf2).unwrap();

    let mut big_funny = [0u8; 765];

    let mut i: u8 = 1;

    for byte in big_funny.as_mut() {
        *byte = i;
        i = i.wrapping_add(1);
    }

    let mut recv_big_funny = [0u8; 765];

    i2c_slave
        .recv_with_timeout(&mut recv_big_funny, &mut InfTimeout::new())
        .unwrap();

    for i in 0..765 {
        if recv_big_funny[i] != big_funny[i] {
            writeln!(
                stdout,
                "index {}, {} != {}",
                i, recv_big_funny[i], big_funny[i]
            )
            .unwrap();
        }
        // assert!(recv_big_funny[i] == big_funny[i]);
    }

    i2c_slave.send(&mut big_funny).unwrap();

    writeln!(stdout, "erm\n").unwrap();

    writeln!(stdout, "received: {:?} {} {}", buf, res, res).unwrap();

    /*match res {
        SlavePollResult::IncomingTransmission => {
            let rcv = i2c_slave
                .slave_recv(&mut buf, &mut InfTimeout::new(), false)
                .unwrap();
            writeln!(stdout, "received: {:?} {} {}", buf, rcv.0, rcv.1).unwrap();
        }
        SlavePollResult::TransmitNeeded => {
            writeln!(stdout, "transmit needed").unwrap();
        }
    }

    writeln!(stdout, "Polling again...").unwrap();

    let res = i2c_slave.slave_poll(&mut InfTimeout::new()).unwrap();

    match res {
        SlavePollResult::IncomingTransmission => {
            let rcv = i2c_slave
                .slave_recv(&mut buf, &mut InfTimeout::new(), false)
                .unwrap();
            writeln!(stdout, "received: {:?} {} {}", buf, rcv.0, rcv.1).unwrap();
        }
        SlavePollResult::TransmitNeeded => {
            writeln!(stdout, "transmit needed").unwrap();
            let n = i2c_slave.slave_send("pong".as_bytes()).unwrap();
            writeln!(stdout, "sent {} bytes", n).unwrap();
        }
    }*/

    writeln!(stdout, "Finished i2c slave tests!\n").unwrap();

    #[allow(clippy::empty_loop)]
    loop {}
}
