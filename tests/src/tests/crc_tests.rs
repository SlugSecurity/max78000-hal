use core::fmt::Write;

use bitvec::prelude::*;
use cortex_m_semihosting::hio; // not needed?
use max78000_hal::{
    max78000::CRC,
    peripherals::crc::Crc,
};

pub unsafe fn run_crc_tests(crc_regs : CRC, stdout : &mut hio::HostStream) {
    writeln!(stdout, "Starting CRC peripheral tests...").unwrap();

    let crc_n = Crc::new(crc_regs); // TODO test on plantmachine

    writeln!(stdout, "CRC peripheral tests complete!\n").unwrap();
}