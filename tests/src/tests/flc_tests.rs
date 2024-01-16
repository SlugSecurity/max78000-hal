use core::fmt::Write;
use cortex_m_semihosting::hio;
use max78000_hal::max78000::{FLC, GCR, ICC0};
use max78000_hal::peripherals::flash_controller::*;

pub fn run_flc_tests(stdout: &mut hio::HostStream, flc: FLC, icc0: &ICC0, gcr: &GCR) {
    writeln!(stdout, "Starting flash tests...").unwrap();
    writeln!(stdout, "Test flash write...").unwrap();
    test_flash_write(flc, icc0, gcr);
    writeln!(stdout, "Flash Controller tests complete!").unwrap();
}

fn test_flash_write(flc: FLC, icc0: &ICC0, gcr: &GCR) {
    let flc = FlashController::new(flc, icc0, gcr);
    const MAGIC: u32 = 0xFEEDBEEF;
    const TEST_VALUE: u32 = 0xDEADBEEF;
    flc.page_erase(MAGIC);
    flc.write(MAGIC, &u32::to_le_bytes(TEST_VALUE));
    let mut data_read: [u8; 4] = [0; 4];
    flc.read_bytes(MAGIC, &mut data_read);

    assert_eq!(u32::from_le_bytes(data_read) == TEST_VALUE, true);
}
