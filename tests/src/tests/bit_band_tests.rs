//! Tests for the bit-banding API

use core::fmt::Write;
use cortex_m_semihosting::hio;
use max78000_hal::peripherals::bit_banding::{change_bit, read_bit, toggle_bit};

/// Runs all bit band tests
///
/// # Safety
///
/// Bit band methods are unsafe, thus the function testing them must be unsafe
pub unsafe fn run_bit_band_tests(stdout: &mut hio::HostStream) {
    writeln!(stdout, "Starting bit band tests...").unwrap();

    test_change_bit();
    test_toggle_bit();
    test_read_bit();

    writeln!(stdout, "Bit band tests complete!").unwrap();
}

/// Tests the [`change_bit`] function
unsafe fn test_change_bit() {
    let mut test: u32 = 0;
    let mut control: u32 = 0;

    for i in 0u8..32 {
        change_bit(&test, i, true);
        control = control | (1 << i);
        assert_eq!(test, control);
    }

    test = 0;

    for i in 0u8..32 {
        change_bit(&test, i, true);
        assert_eq!(test, 1 << i);
        change_bit(&test, i, false);
    }
}

/// Tests the [`toggle_bit`] function
unsafe fn test_toggle_bit() {
    let test: u32 = 0;
    let mut control: u32 = 0;

    for i in 0u8..32 {
        toggle_bit(&test, i);
        control = control ^ (1 << i);
        assert_eq!(test, control);
        toggle_bit(&test, i);
        control = control ^ (1 << i);
        assert_eq!(test, control);
    }
}

/// Tests the [`read_bit`] function
fn test_read_bit() {
    let mut control = 0;
    for i in 0u8..32 {
        let mut bit = read_bit(&control, i);
        assert_eq!(bit, (control & (1 << i)) != 0);
        control = control | (1 << i);
        bit = read_bit(&control, i);
        assert_eq!(bit, (control & (1 << i)) != 0);
    }
}

// TODO: test [`bit_banding::spin_bit`]
