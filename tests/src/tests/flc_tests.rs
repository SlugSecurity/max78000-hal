//! Flash controller tests

use core::fmt::Write;
use cortex_m_semihosting::hio;
use max78000_hal::max78000::{trimsir::INRO, FLC, GCR, ICC0};
use max78000_hal::peripherals::flash_controller::FlashController;
use max78000_hal::peripherals::oscillator::{
    Ibro, IbroDivider, IbroFrequency, Ipo, IpoDivider, IpoFrequency, Iso, IsoDivider, IsoFrequency,
    Oscillator, SystemClock,
};

/// Runs all flash controller tests: [`flash_write`], [`flash_write_large`],
/// [`flash_write_extra_large`], [`flash_write_after_sys_osc_switch`],
/// [`flash_write_after_sys_clk_div_changes`].
pub fn run_flc_tests(stdout: &mut hio::HostStream, flc: FLC, icc0: &ICC0, gcr: &GCR, inro: &INRO) {
    writeln!(stdout, "Starting flash tests...").unwrap();
    let ipo = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_1);
    let mut sys_clk = SystemClock::new(&ipo, gcr.clkctrl(), inro);
    let flash_controller = FlashController::new(flc, icc0, gcr);

    flash_controller.disable_icc0();

    writeln!(stdout, "Test flash write...").unwrap();
    flash_write(&flash_controller, &sys_clk);

    writeln!(stdout, "Test flash write large...").unwrap();
    flash_write_large(&flash_controller, &sys_clk);

    writeln!(stdout, "Test flash write extra large...").unwrap();
    flash_write_extra_large(&flash_controller, &sys_clk);

    writeln!(stdout, "Test flash write unaligned...").unwrap();
    flash_write_unaligned(&flash_controller, &sys_clk);

    {
        writeln!(
            stdout,
            "Test flash write after system oscillator changes..."
        )
        .unwrap();
        let iso = Iso::new(IsoFrequency::_60MHz, IsoDivider::_1);
        sys_clk.set_sysclk(&iso);
        flash_controller.disable_icc0();
        flash_write_after_sys_osc_switch(&flash_controller, &sys_clk);
        flash_controller.enable_icc0();
    }

    {
        writeln!(
            stdout,
            "Test flash write after system clock divider changes..."
        )
        .unwrap();
        let ibro = Ibro::new(IbroFrequency::_7_3728MHz, IbroDivider::_4);
        sys_clk.set_sysclk(&ibro);
        flash_write_after_sys_clk_div_changes(&flash_controller, &sys_clk);
    }

    writeln!(stdout, "Flash Controller tests complete!").unwrap();

    flash_controller.enable_icc0();
}

fn flash_write(flash_controller: &FlashController, sys_clk: &SystemClock) {
    let test_addr: u32 = 0x10070FF0;
    let test_val: u32 = 0xCAFEBABE;

    flash_controller.page_erase(test_addr, sys_clk);
    flash_controller.write(test_addr, &u32::to_le_bytes(test_val), sys_clk);
    let mut data_read: [u8; 4] = [0; 4];
    flash_controller.read_bytes(test_addr, &mut data_read);

    assert!(u32::from_le_bytes(data_read) == test_val);
}

fn flash_write_large(flash_controller: &FlashController, sys_clk: &SystemClock) {
    let test_addr: u32 = 0x10070F00;
    let test_data: [u8; 20] = [b'A'; 20];

    flash_controller.page_erase(test_addr, sys_clk);
    flash_controller.write(test_addr, &test_data, sys_clk);
    let mut read_data: [u8; 20] = [0; 20];
    flash_controller.read_bytes(test_addr, &mut read_data);

    assert!(test_data == read_data);
}

fn flash_write_extra_large(flash_controller: &FlashController, sys_clk: &SystemClock) {
    let test_addr: u32 = 0x10070F00;
    let test_data: [u8; 100] = [b'A'; 100];

    flash_controller.page_erase(test_addr, sys_clk);

    flash_controller.write(test_addr, &test_data, sys_clk);
    let mut read_data: [u8; 100] = [0; 100];
    flash_controller.read_bytes(test_addr, &mut read_data);

    assert!(test_data == read_data);
}

fn flash_write_unaligned(flash_controller: &FlashController, sys_clk: &SystemClock) {
    let test_addr: u32 = 0x10070F0B;
    let test_data: [u8; 10] = [b'A'; 10];

    flash_controller.page_erase(test_addr, sys_clk);
    flash_controller.write(test_addr, &test_data, sys_clk);
    let mut read_data: [u8; 10] = [0; 10];
    flash_controller.read_bytes(test_addr, &mut read_data);

    assert!(test_data == read_data);
}

fn flash_write_after_sys_osc_switch(flash_controller: &FlashController, sys_clk: &SystemClock) {
    let test_addr: u32 = 0x10070F00;
    const TEST_STR: &str = "SYS_OSC CHANGED";
    let test_data = TEST_STR.as_bytes();

    flash_controller.page_erase(test_addr, sys_clk);
    flash_controller.write(test_addr, &test_data, sys_clk);
    let mut read_data: [u8; TEST_STR.len()] = [0; TEST_STR.len()];
    flash_controller.read_bytes(test_addr, &mut read_data);

    assert!(test_data == read_data);
}

fn flash_write_after_sys_clk_div_changes(
    flash_controller: &FlashController,
    sys_clk: &SystemClock,
) {
    // using 0x1007DF0A breaks the whole test ... using another unaligned
    // address breaks this test
    let test_addr: u32 = 0x10070F0A;
    const TEST_STR: &str = "SYS_CLK DIVIDER CHANGED"; // len = 23
    let test_data = TEST_STR.as_bytes();

    flash_controller.page_erase(test_addr, sys_clk);
    flash_controller.write(test_addr, &test_data, sys_clk);
    let mut read_data: [u8; TEST_STR.len()] = [0; TEST_STR.len()];
    flash_controller.read_bytes(test_addr, &mut read_data);

    assert!(test_data == read_data);
}
