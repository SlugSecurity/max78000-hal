//! Tests for the bit-banding API

use core::fmt::Write;
use cortex_m_semihosting::hio;
use max78000_hal::max78000::rtc::ctrl::{EN_A, RDY_A, WR_EN_A};
use max78000_hal::max78000::RTC;
use max78000_hal::peripherals::bit_banding::{change_bit, read_bit, spin_bit};

/// Runs all bit band tests
///
/// # Safety
///
/// Bit band methods are unsafe, thus the function testing them must be unsafe
pub unsafe fn run_bit_band_tests(stdout: &mut hio::HostStream) {
    writeln!(stdout, "Starting bit band tests...").unwrap();

    // sanity check
    let test: u32 = 0;
    change_bit(&test, 0, true);
    writeln!(stdout, "{test}").unwrap();

    writeln!(stdout, "Testing change_bit...").unwrap();
    test_change_bit();
    writeln!(stdout, "change_bit complete").unwrap();

    writeln!(stdout, "Testing read_bit...").unwrap();
    test_read_bit();
    writeln!(stdout, "read_bit complete").unwrap();

    writeln!(stdout, "Testing spin_bit...").unwrap();
    test_spin_bit(stdout);
    writeln!(stdout, "spin_bit complete").unwrap();

    writeln!(stdout, "Bit band tests complete!").unwrap();
}

/// Tests the [`change_bit`] function
unsafe fn test_change_bit() {
    let mut test: u32 = 0;
    let mut control: u32 = 0;

    for i in 0u8..32 {
        change_bit(&test as *const _, i, true);
        control |= 1 << i;
        assert_eq!(test, control);
    }

    test = 0;

    for i in 0u8..32 {
        change_bit(&test as *const _, i, true);
        assert_eq!(test, 1 << i);
        change_bit(&test as *const _, i, false);
    }
}

/*
/// Tests the [`toggle_bit`] function
unsafe fn test_toggle_bit(stdout: &mut hio::HostStream) {
    writeln!(stdout, "toggle_bit tests unimplemented").unwrap();

    /*let test: u32 = 0;
    let mut control: u32 = 0;

    for i in 0u8..32 {
        toggle_bit(&test as *const _, i);
        control = control ^ (1 << i);
        assert_eq!(test, control);
        toggle_bit(&test as *const _, i);
        control = control ^ (1 << i);
        assert_eq!(test, control);
    }*/
}*/

/// Tests the [`read_bit`] function
fn test_read_bit() {
    let mut control = 0;
    for i in 0u8..32 {
        let mut bit = read_bit(&control as *const _, i);
        assert_eq!(bit, (control & (1 << i)) != 0);
        control |= 1 << i;
        bit = read_bit(&control as *const _, i);
        assert_eq!(bit, (control & (1 << i)) != 0);
    }
}

unsafe fn test_spin_bit(stdout: &mut hio::HostStream) {
    // TODO: use timer peripheral API once implemented

    writeln!(
        stdout,
        "Running sanity check on spin_bit. The program should NOT stall."
    )
    .unwrap();
    // Sanity check tests:
    let mut control = 0;
    for i in 0u8..32 {
        spin_bit(&control as *const _, i, false);
        control |= 1 << i;
        spin_bit(&control as *const _, i, true);
    }

    let clock = RTC::steal();

    writeln!(
        stdout,
        "Testing spin_bit on RTC peripheral to test if clock is ready to read."
    )
    .unwrap();
    clock.ctrl.write(|w| w.wr_en().variant(WR_EN_A::PENDING));
    spin_bit(clock.ctrl.as_ptr(), 3, false);
    clock.ctrl.write(|w| w.en().variant(EN_A::EN));

    clock.ctrl.write(|w| w.rdy().variant(RDY_A::BUSY));

    writeln!(
        stdout,
        "set clock ready bit to BUSY, waiting for it to become ready again..."
    )
    .unwrap();

    spin_bit(clock.ctrl.as_ptr(), 4, true);

    writeln!(stdout, "Caught the clock ready bit!").unwrap();

    writeln!(stdout, "Disabling RTC write enable").unwrap();

    clock.ctrl.write(|w| w.wr_en().variant(WR_EN_A::INACTIVE));
}
