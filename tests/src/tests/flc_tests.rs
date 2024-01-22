//! Flash controller tests

use core::fmt::Write;
use cortex_m_semihosting::hio;
use max78000_hal::max78000::{trimsir::INRO, FLC, GCR, ICC0};
use max78000_hal::peripherals::flash_controller::FlashController;
use max78000_hal::peripherals::oscillator::{
    Ipo, IpoDivider, IpoFrequency, Iso, IsoDivider, IsoFrequency, Oscillator, SystemClock,
};

/// Runs all flash controller tests: [`flash_write`], [`flash_write_large`],
/// [`flash_write_extra_large`], [`flash_write_after_sys_osc_switch`],
/// [`flash_write_after_sys_clk_div_changes`], [`flash_write_full_outbounds`],
/// [`flash_write_paritially_outbound_beginning`],
/// [`flash_write_full_paritially_outbound_end`],
/// [`flash_write_full_outbound_fully_before_and_after`]
pub fn run_flc_tests(stdout: &mut hio::HostStream, flc: FLC, icc0: &ICC0, gcr: &GCR, inro: &INRO) {
    writeln!(stdout, "Starting flash tests...").unwrap();
    let ipo = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_1);
    let mut sys_clk = SystemClock::new(&ipo, gcr.clkctrl(), inro);
    let flash_controller = FlashController::new(flc, icc0, gcr);

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
        flash_write_after_sys_osc_switch(&flash_controller, &sys_clk);
    }

    {
        writeln!(
            stdout,
            "Test flash write after system clock divider changes..."
        )
        .unwrap();
        let iso = Iso::new(IsoFrequency::_60MHz, IsoDivider::_4);
        sys_clk.set_sysclk(&iso);
        flash_write_after_sys_clk_div_changes(&flash_controller, &sys_clk);
    }

    writeln!(stdout, "Flash Controller tests complete!").unwrap();
}

fn flash_write(flash_controller: &FlashController, sys_clk: &SystemClock) {
    let test_addr: u32 = 0x10070FF0;
    let test_val: u32 = 0xCAFEBABE;
    let mut data_read: [u8; 4] = [0; 4];

    unsafe {
        flash_controller.page_erase(test_addr, sys_clk).unwrap();
        flash_controller
            .write(test_addr, &u32::to_le_bytes(test_val), sys_clk)
            .unwrap();
        flash_controller
            .read_bytes(test_addr, &mut data_read)
            .unwrap();
    }

    assert!(u32::from_le_bytes(data_read) == test_val);
}

fn flash_write_large(flash_controller: &FlashController, sys_clk: &SystemClock) {
    let test_addr: u32 = 0x10070F00;
    let test_data: [u8; 20] = [b'A'; 20];
    let mut read_data: [u8; 20] = [0; 20];

    unsafe {
        flash_controller.page_erase(test_addr, sys_clk).unwrap();
        flash_controller
            .write(test_addr, &test_data, sys_clk)
            .unwrap();
        flash_controller
            .read_bytes(test_addr, &mut read_data)
            .unwrap();
    }

    assert!(test_data == read_data);
}

fn flash_write_extra_large(flash_controller: &FlashController, sys_clk: &SystemClock) {
    let test_addr: u32 = 0x10070F00;
    let test_data: [u8; 100] = [b'A'; 100];
    let mut read_data: [u8; 100] = [0; 100];

    unsafe {
        flash_controller.page_erase(test_addr, sys_clk).unwrap();
        flash_controller
            .write(test_addr, &test_data, sys_clk)
            .unwrap();
        flash_controller
            .read_bytes(test_addr, &mut read_data)
            .unwrap();
    }

    assert!(test_data == read_data);
}

fn flash_write_unaligned(flash_controller: &FlashController, sys_clk: &SystemClock) {
    let test_addr: u32 = 0x10070F0A;
    let test_data: [u8; 13] = [b'B'; 13];
    let mut read_data: [u8; 13] = [0; 13];

    unsafe {
        flash_controller.page_erase(test_addr, sys_clk).unwrap();
        flash_controller
            .write(test_addr, &test_data, sys_clk)
            .unwrap();
        flash_controller
            .read_bytes(test_addr, &mut read_data)
            .unwrap();
    }

    assert!(test_data == read_data);
}

fn flash_write_after_sys_osc_switch(flash_controller: &FlashController, sys_clk: &SystemClock) {
    let test_addr: u32 = 0x10070F00;
    const TEST_STR: &str = "SYS_OSC CHANGED";
    let test_data = TEST_STR.as_bytes();
    let mut read_data: [u8; TEST_STR.len()] = [0; TEST_STR.len()];

    unsafe {
        flash_controller.page_erase(test_addr, sys_clk).unwrap();
        flash_controller
            .write(test_addr, test_data, sys_clk)
            .unwrap();
        flash_controller
            .read_bytes(test_addr, &mut read_data)
            .unwrap();
    }

    assert!(test_data == read_data);
}

fn flash_write_after_sys_clk_div_changes(
    flash_controller: &FlashController,
    sys_clk: &SystemClock,
) {
    let test_addr: u32 = 0x10070F0A;
    const TEST_STR: &str = "SYS_CLK DIVIDER CHANGED";
    let test_data = TEST_STR.as_bytes();
    let mut read_data: [u8; TEST_STR.len()] = [0; TEST_STR.len()];

    unsafe {
        flash_controller.page_erase(test_addr, sys_clk).unwrap();
        flash_controller
            .write(test_addr, test_data, sys_clk)
            .unwrap();
        flash_controller
            .read_bytes(test_addr, &mut read_data)
            .unwrap();
    }

    assert!(test_data == read_data);
}
