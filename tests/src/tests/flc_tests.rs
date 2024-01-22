//! Flash controller tests

use core::fmt::Write;
use cortex_m_semihosting::hio;
use max78000_hal::max78000::{trimsir::INRO, FLC, GCR, ICC0};
use max78000_hal::peripherals::flash_controller::{FlashController, FlashErr};
use max78000_hal::peripherals::oscillator::{
    Ibro, IbroDivider, IbroFrequency, Ipo, IpoDivider, IpoFrequency, Iso, IsoDivider, IsoFrequency,
    Oscillator, SystemClock,
};

/// Runs all flash controller tests: [`flash_write`], [`flash_write_large`],
/// [`flash_write_extra_large`], [`flash_write_after_sys_osc_switch`],
/// [`flash_write_after_sys_clk_div_changes`], [`flash_write_invalid_clk_div`],
/// [`flash_write_full_outbounds`],
/// [`flash_write_paritially_outbound_beginning`],
/// [`flash_write_full_paritially_outbound_end`].
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
        writeln!(stdout, "Test flash write after invalid clock divider...").unwrap();
        let ibro = Ibro::new(IbroFrequency::_7_3728MHz, IbroDivider::_2);
        sys_clk.set_sysclk(&ibro);
        flash_write_invalid_clk_div(&flash_controller, &sys_clk);
    }

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

    writeln!(stdout, "Test flash write address fully out of bounds").unwrap();
    flash_write_full_outbounds(&flash_controller, &sys_clk);

    writeln!(
        stdout,
        "Test flash write address paritally out of bounds beginning"
    )
    .unwrap();
    flash_write_paritially_outbound_beginning(&flash_controller, &sys_clk);

    writeln!(
        stdout,
        "Test flash write address paritally out of bounds end"
    )
    .unwrap();
    flash_write_full_paritially_outbound_end(&flash_controller, &sys_clk);

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

/// Invalid flash clock divider is detect by the page_erase function
fn flash_write_invalid_clk_div(flash_controller: &FlashController, sys_clk: &SystemClock) {
    let test_addr: u32 = 0x10070F00;

    unsafe {
        if let Err(why) = flash_controller.page_erase(test_addr, sys_clk) {
            match why {
                FlashErr::FlcClkErr => (),
                _ => panic!("Not FlcClk Err"),
            }
        }
    }
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

/// Fully out of bound addresses are caught by the page_erase function which
/// checks if the start address page is in bounds
fn flash_write_full_outbounds(flash_controller: &FlashController, sys_clk: &SystemClock) {
    let test_addr: u32 = 0x0FFF_FFFF;

    unsafe {
        if let Err(why) = flash_controller.page_erase(test_addr, sys_clk) {
            match why {
                FlashErr::PtrBoundsErr => (),
                _ => panic!("Not Bounds Err"),
            }
        }
    }
}

/// Flash writes which have the start address bellow the start of a valid flash
/// address range are caught by the page_erase function which checks if the
/// start address page is in bounds
fn flash_write_paritially_outbound_beginning(
    flash_controller: &FlashController,
    sys_clk: &SystemClock,
) {
    let test_addr: u32 = 0x0FFF_FF00;

    unsafe {
        if let Err(why) = flash_controller.page_erase(test_addr, sys_clk) {
            match why {
                FlashErr::PtrBoundsErr => (),
                _ => panic!("Not Bounds Err"),
            }
        }
    }
}

/// Flash writes which have the end address above the end of a valid flash
/// address range are caught by the write function which checks if the
/// end address is in bounds
fn flash_write_full_paritially_outbound_end(
    flash_controller: &FlashController,
    sys_clk: &SystemClock,
) {
    let test_addr: u32 = 0x1007_FFFA;
    let test_data: [u8; 10] = [b'A'; 10];

    unsafe {
        flash_controller.page_erase(test_addr, sys_clk).unwrap();
        if let Err(why) = flash_controller.write(test_addr, &test_data, sys_clk) {
            match why {
                FlashErr::PtrBoundsErr => (),
                _ => panic!("Not Bounds Err"),
            }
        }
    }
}
