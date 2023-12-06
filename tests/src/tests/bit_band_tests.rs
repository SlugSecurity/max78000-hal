//! Tests for the bit-banding API

use core::fmt::Write;
use cortex_m_semihosting::hio;
use max78000_hal::max78000::rtc::ctrl::{EN_A, RDY_A, WR_EN_A};
use max78000_hal::max78000::RTC;
use max78000_hal::peripherals::bit_banding::{change_bit, read_bit, spin_bit};

/// Runs all bit band tests: [`change_bit`], [`read_bit`], and [`spin_bit`].
/// Requires RTC registers to test [`spin_bit`] on the RTC_CTRL.rdy register bit.
pub fn run_bit_band_tests(stdout: &mut hio::HostStream, rtc_regs: &RTC) {
    writeln!(stdout, "Starting bit band tests...").unwrap();

    writeln!(stdout, "Testing change_bit...").unwrap();
    test_change_bit();
    writeln!(stdout, "change_bit complete").unwrap();

    writeln!(stdout, "Testing read_bit...").unwrap();
    test_read_bit();
    writeln!(stdout, "read_bit complete").unwrap();

    writeln!(stdout, "Testing spin_bit...").unwrap();
    test_spin_bit(stdout, rtc_regs);
    writeln!(stdout, "spin_bit complete").unwrap();

    writeln!(stdout, "Bit band tests complete!").unwrap();
}

/// Tests the [`change_bit`] function
fn test_change_bit() {
    let mut test: u32 = 0;
    let mut control: u32 = 0;

    for i in 0u8..32 {
        // SAFETY: Safe as we are passing in a valid memory address in the bit-banded SRAM space, initialized in the `test` variable.
        unsafe {
            change_bit(&test as *const _, i, true);
        }
        control |= 1 << i;
        assert_eq!(test, control);
    }

    test = 0;

    for i in 0u8..32 {
        // Safety: safe as we are passing in an SRAM address which is initialized (on stack).
        unsafe {
            change_bit(&test as *const _, i, true);
        }
        assert_eq!(test, 1 << i);
        // Safety: safe as we are passing in an SRAM address which is initialized (on stack).
        unsafe {
            change_bit(&test as *const _, i, false);
        }
    }
}

/// Tests the [`read_bit`] function
fn test_read_bit() {
    let mut control = 0;
    for i in 0u8..32 {
        // Safety: safe as we are passing in an SRAM address which is initialized (on stack).
        let mut bit = unsafe { read_bit(&control as *const _, i) };
        assert_eq!(bit, (control & (1 << i)) != 0);
        control |= 1 << i;
        // Safety: safe as we are passing in an SRAM address which is initialized (on stack).
        bit = unsafe { read_bit(&control as *const _, i) };
        assert_eq!(bit, (control & (1 << i)) != 0);
    }
}

fn test_spin_bit(stdout: &mut hio::HostStream, clock: &RTC) {
    writeln!(
        stdout,
        "Running sanity check on spin_bit. The program should NOT stall."
    )
    .unwrap();
    // Sanity check tests:
    let mut control = 0;
    for i in 0u8..32 {
        // SAFETY: Safe as we are passing in a valid memory address in the bit-banded SRAM space, initialized in the `control` variable.
        unsafe {
            spin_bit(&control as *const _, i, false);
        }

            control |= 1 << i;

        // SAFETY: Safe as we are passing in a valid memory address in the bit-banded SRAM space, initialized in the `control` variable.
        unsafe {
            spin_bit(&control as *const _, i, true);
        }
    }

    writeln!(
        stdout,
        "Testing spin_bit on RTC peripheral to test if clock is ready to read."
    )
    .unwrap();
    clock.ctrl.write(|w| w.wr_en().variant(WR_EN_A::PENDING));
    // SAFETY: Safe as we are passing in a peripheral address in bit-banding space
    // (0x4000_6010), bit 3 (RTC_CTRL.busy) is a writable bit of a valid register
    // (page 288 of user guide).
    unsafe {
        spin_bit(clock.ctrl.as_ptr(), 3, false);
    }
    clock.ctrl.write(|w| w.en().variant(EN_A::EN));
    clock.ctrl.write(|w| w.rdy().variant(RDY_A::BUSY));
    writeln!(
        stdout,
        "set clock ready bit to BUSY, waiting for it to become ready again..."
    )
    .unwrap();
    // Safety: safe as we are passing in a peripheral address in bit-banding space
    // (0x4000_6010), said bit RTC_CTRL.rdy (4) will be made 1 automatically by the peripheral
    // when it is no longer busy writing to registers. (Page 287 of user guide)
    unsafe {
        spin_bit(clock.ctrl.as_ptr(), 4, true);
    }

    writeln!(stdout, "Caught the clock ready bit!").unwrap();
    writeln!(stdout, "Disabling RTC write enable").unwrap();
    clock.ctrl.write(|w| w.wr_en().variant(WR_EN_A::INACTIVE));
}
