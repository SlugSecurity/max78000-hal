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
    // let test_addr: u32 = FLASH_MEM_BASE + FLASH_MEM_SIZE - (2 * FLASH_PAGE_SIZE);
    let test_addr: u32 = 0x1007bf00;
    let test_val: u32 = 0xCAFEBABE;
    flc.page_erase(test_addr);
    flc.write(test_addr, &u32::to_le_bytes(test_val));
    let mut data_read: [u8; 4] = [0; 4];
    flc.read_bytes(test_addr, &mut data_read);

    assert_eq!(u32::from_le_bytes(data_read) == test_val, true);
}
