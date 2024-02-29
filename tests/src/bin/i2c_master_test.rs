//! Test runner for the MAX78000 HAL I2C Master Mode.

#![warn(missing_docs)]
#![no_main]
#![no_std]

use core::fmt::Write;
use cortex_m::asm::delay;

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

    i2c_master.set_target_addr(69);

    let mut funny = [1u8, 2u8, 3u8, 4u8];

    i2c_master.send(&mut funny).unwrap();

    delay(100000);

    writeln!(stdout, "ok we need funny back").unwrap();

    i2c_master
        .recv_with_data_timeout(&mut funny, &mut InfTimeout::new())
        .unwrap();

    delay(100000);

    writeln!(stdout, "time for big funny").unwrap();

    let mut big_funny = [0u8; 765];

    let mut i: u8 = 1;

    for byte in big_funny.as_mut() {
        *byte = i;
        i = i.wrapping_add(1);
    }

    i2c_master.send(&mut big_funny).unwrap();

    writeln!(stdout, "big funny was sent, waiting for big funny back...").unwrap();

    delay(100000);

    let mut recv_big_funny = [0u8; 765];

    i2c_master
        .recv_with_timeout(&mut recv_big_funny, &mut InfTimeout::new())
        .unwrap();

    for i in 0..765 {
        assert_eq!(recv_big_funny[i], big_funny[i]);
    }

    writeln!(stdout, "Verified that the big funny was indeed correct").unwrap();

    delay(100000);

    writeln!(stdout, "Funny: {:?}", funny).unwrap();

    //i2c_master.write(69, "ping".as_bytes()).unwrap();

    /*let mut timer2 = clock.new_timer(Time::Milliseconds(5000));
    while !timer2.poll() {}

    writeln!(stdout, "Reading from slave...").unwrap();

    i2c_master.read(69, &mut stuff).unwrap();

    writeln!(stdout, "Read {:?}", stuff).unwrap();*/

    writeln!(stdout, "Finished i2c master tests!\n").unwrap();

    #[allow(clippy::empty_loop)]
    loop {}
}
