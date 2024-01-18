//! Flash controller tests

use core::fmt::Write;
use cortex_m_semihosting::hio;
use max78000_hal::max78000::{FLC, GCR, ICC0};
use max78000_hal::peripherals::flash_controller::FlashController;

/// Runs all flash controller tests: [`flash_write`], [`flash_write_large`],
/// [`flash_write_extra_large`].
pub fn run_flc_tests(stdout: &mut hio::HostStream, flc: FLC, icc0: &ICC0, gcr: &GCR) {
    writeln!(stdout, "Starting flash tests...").unwrap();
    let flash_controller = FlashController::new(flc, icc0, gcr);

    writeln!(stdout, "Test flash write...").unwrap();
    flash_write(&flash_controller);

    writeln!(stdout, "Test flash write large...").unwrap();
    flash_write_large(&flash_controller);

    writeln!(stdout, "Test flash write extra large...").unwrap();
    flash_write_extra_large(&flash_controller);

    writeln!(stdout, "Test flash write unaligned...").unwrap();
    flash_write_unaligned(&flash_controller);

    writeln!(stdout, "Flash Controller tests complete!").unwrap();
}

fn flash_write(flash_controller: &FlashController) {
    let test_addr: u32 = 0x1007DFF0;
    let test_val: u32 = 0xCAFEBABE;

    flash_controller.disable_icc0();

    flash_controller.page_erase(test_addr);
    flash_controller.write(test_addr, &u32::to_le_bytes(test_val));
    let mut data_read: [u8; 4] = [0; 4];
    flash_controller.read_bytes(test_addr, &mut data_read);

    assert!(u32::from_le_bytes(data_read) == test_val);

    flash_controller.enable_icc0();
}

fn flash_write_large(flash_controller: &FlashController) {
    let test_addr: u32 = 0x1007DF00;
    let test_data: [u8; 20] = [b'A'; 20];

    flash_controller.disable_icc0();

    flash_controller.page_erase(test_addr);
    flash_controller.write(test_addr, &test_data);
    let mut read_data: [u8; 20] = [0; 20];
    flash_controller.read_bytes(test_addr, &mut read_data);

    assert!(test_data == read_data);

    flash_controller.enable_icc0();
}

fn flash_write_extra_large(flash_controller: &FlashController) {
    let test_addr: u32 = 0x1007DF00;
    let test_data: [u8; 100] = [b'A'; 100];

    flash_controller.disable_icc0();

    flash_controller.page_erase(test_addr);
    flash_controller.write(test_addr, &test_data);
    let mut read_data: [u8; 100] = [0; 100];
    flash_controller.read_bytes(test_addr, &mut read_data);

    assert!(test_data == read_data);

    flash_controller.enable_icc0();
}

fn flash_write_unaligned(flash_controller: &FlashController) {
    let test_addr: u32 = 0x1007DF0A;
    let test_data: [u8; 10] = [b'A'; 10];

    flash_controller.disable_icc0();

    flash_controller.page_erase(test_addr);
    flash_controller.write(test_addr, &test_data);
    let mut read_data: [u8; 10] = [0; 10];
    flash_controller.read_bytes(test_addr, &mut read_data);

    assert!(test_data == read_data);

    flash_controller.enable_icc0();
}
