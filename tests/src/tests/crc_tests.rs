//! Tests for the CRC peripheral.
// #[cfg(test)]

use core::fmt::Write;
// use bitvec::prelude::*;
use cortex_m_semihosting::hio; // not needed?
use max78000_hal::{
    max78000::CRC,
    peripherals::crc::{CrcCalculator, CrcDataU32},
};

/// run crc tests
pub fn run_crc_tests(crc_regs: CRC, stdout: &mut hio::HostStream) {
    writeln!(stdout, "Starting CRC peripheral tests...").unwrap();

    run_crc_test_u8(crc_regs, stdout);

    writeln!(stdout, "CRC peripheral tests complete!\n").unwrap();
}

fn run_crc_test_u8(crc_regs: CRC, stdout: &mut hio::HostStream) {
    let data = [0x00, 0x00, 0x00, 0xf4];
    let crc_data = CrcDataU32::new(&data);

    let mut crc_calc: CrcCalculator<CrcDataU32> = CrcCalculator::new(crc_regs);

    crc_calc
        .byte_swap_in(false)
        .byte_swap_out(false)
        .msb(true)
        .poly(0x07)
        .xorout(0);

    let _value = crc_calc.calc(crc_data);

    writeln!(stdout, "{:#?}", crc_calc).unwrap();

    writeln!(
        stdout,
        "input: {:#?}, value: {:#x}",
        data,
        crc_calc.val_reg_bits()
    )
    .unwrap();
}
