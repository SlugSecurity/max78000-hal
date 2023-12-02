//! Tests for the bit-banding API

use core::fmt::Write;
use cortex_m_semihosting::hio;
use max78000_hal::max78000::tmr::ctrl0::MODE_A_A::ONE_SHOT;
use max78000_hal::max78000::tmr1::ctrl0::CLKDIV_A_A::DIV_BY_4096;
use max78000_hal::max78000::TMR1;
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
    // TODO: implement test for spin_bit
    // TODO: use timer peripheral API once implemented
    let tmr = TMR1::ptr();

    writeln!(stdout, "test_spin_bit: disabling TMR1 timerA").unwrap();
    // Disable the timer peripheral
    (*tmr).ctrl0.write(|w| w.en_a().variant(false));
    // wait for timer to be disabled
    spin_bit((*tmr).ctrl1.as_ptr(), 2, false);
    writeln!(stdout, "test_spin_bit: TMR1 timerA has been disabled").unwrap();

    // Select PCLK as the clock source
    (*tmr).ctrl1.write(|w| w.clksel_a().variant(0));
    // Select one-shot mode and set prescaler to 4096
    (*tmr)
        .ctrl0
        .write(|w| w.mode_a().variant(ONE_SHOT).clkdiv_a().variant(DIV_BY_4096));

    // Set the compare value
    (*tmr).cmp.write(|w| w.compare().variant(48828));

    writeln!(
        stdout,
        "test_spin_bit: enabling TMR1 timerA for about 2 seconds"
    )
    .unwrap();

    (*tmr).ctrl0.write(|w| w.en_a().variant(true));

    spin_bit((*tmr).ctrl1.as_ptr(), 2, true);

    writeln!(stdout, "test_spin_bit: timer has been enabled. the next output should display after roughly 2 seconds").unwrap();

    spin_bit((*tmr).intfl.as_ptr(), 0, true);

    writeln!(
        stdout,
        "test_spin_bit: timer has finished. spin_bit tests complete!"
    )
    .unwrap();
}
