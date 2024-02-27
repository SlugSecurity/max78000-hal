//! Oscillator tests

use core::fmt::Write;
use cortex_m_semihosting::hio;
use max78000_hal::max78000::gcr::CLKCTRL;
use max78000_hal::peripherals::{oscillator::*, PeripheralHandle};

#[cfg(feature = "low_frequency_test")]
use max78000_hal::max78000::trimsir::INRO;

/// Goes through all the oscillators and sets each one to be the system clk
pub fn run_oscillator_tests(
    gcr_clkctrl_reg: &CLKCTRL,
    mut sysclk: PeripheralHandle<'_, SystemClock<'_, '_>>,
    stdout: &mut hio::HostStream,
    #[cfg(feature = "low_frequency_test")] trimsir_inro_reg: &INRO,
) {
    writeln!(stdout, "Starting oscillator tests...").unwrap();

    writeln!(stdout, "Testing setting IPO to SYS_CLK input...").unwrap();
    test_ipo(gcr_clkctrl_reg, &mut sysclk);

    writeln!(stdout, "Testing setting ISO to SYS_CLK input...").unwrap();
    test_iso(gcr_clkctrl_reg, &mut sysclk);

    writeln!(stdout, "Testing setting IBRO to SYS_CLK input...").unwrap();
    test_ibro(gcr_clkctrl_reg, &mut sysclk);

    #[cfg(feature = "low_frequency_test")]
    {
        writeln!(stdout, "Testing setting INRO to frequency to 8kHz...").unwrap();
        test_inro_freq_8kHz(gcr_clkctrl_reg, trimsir_inro_reg, &mut sysclk);

        writeln!(stdout, "Testing setting INRO to frequency to 16kHz...").unwrap();
        test_inro_freq_16kHz(gcr_clkctrl_reg, trimsir_inro_reg, &mut sysclk);

        writeln!(stdout, "Testing setting INRO to frequency to 30kHz...").unwrap();
        test_inro_freq_30kHz(gcr_clkctrl_reg, trimsir_inro_reg, &mut sysclk);

        writeln!(stdout, "Testing setting ERTCO to SYS_CLK input...").unwrap();
        test_ertco(gcr_clkctrl_reg, &mut sysclk);
    }

    writeln!(stdout, "Testing setting system oscillator divider to 1...").unwrap();
    test_divider_1(gcr_clkctrl_reg, &mut sysclk);

    writeln!(stdout, "Testing setting system oscillator divider to 2...").unwrap();
    test_divider_2(gcr_clkctrl_reg, &mut sysclk);

    writeln!(stdout, "Testing setting system oscillator divider to 4...").unwrap();
    test_divider_4(gcr_clkctrl_reg, &mut sysclk);

    writeln!(stdout, "Testing setting system oscillator divider to 8...").unwrap();
    test_divider_8(gcr_clkctrl_reg, &mut sysclk);

    writeln!(stdout, "Testing setting system oscillator divider to 16...").unwrap();
    test_divider_16(gcr_clkctrl_reg, &mut sysclk);

    writeln!(stdout, "Testing setting system oscillator divider to 32...").unwrap();
    test_divider_32(gcr_clkctrl_reg, &mut sysclk);

    writeln!(stdout, "Testing setting system oscillator divider to 64...").unwrap();
    test_divider_64(gcr_clkctrl_reg, &mut sysclk);

    writeln!(
        stdout,
        "Testing get system oscillator frequency before setting to iso..."
    )
    .unwrap();
    test_frequency_before(&mut sysclk);
    writeln!(
        stdout,
        "Testing get system oscillator divider before setting to iso..."
    )
    .unwrap();
    test_divider_before(&mut sysclk);
    writeln!(
        stdout,
        "Testing get system oscillator frequency after setting to iso..."
    )
    .unwrap();
    test_frequency_after(&mut sysclk);
    writeln!(
        stdout,
        "Testing get system oscillator divider after setting to iso..."
    )
    .unwrap();
    test_divider_after(&mut sysclk);

    writeln!(stdout, "Oscillator tests complete!").unwrap();
}

fn test_ipo(gcr_clkctrl_reg: &CLKCTRL, sys_clk: &mut SystemClock) {
    let new_osc = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_1);
    sys_clk.set_sysclk(&new_osc);
    assert!(gcr_clkctrl_reg.read().sysclk_sel().is_ipo());
}

fn test_iso(gcr_clkctrl_reg: &CLKCTRL, sys_clk: &mut SystemClock) {
    let new_osc = Iso::new(IsoFrequency::_60MHz, IsoDivider::_1);
    sys_clk.set_sysclk(&new_osc);
    assert!(gcr_clkctrl_reg.read().sysclk_sel().is_iso());
}

#[cfg(feature = "low_frequency_test")]
#[allow(non_snake_case)]
fn test_inro_freq_8kHz(
    gcr_clkctrl_reg: &CLKCTRL,
    trimsir_inro_reg: &INRO,
    sys_clk: &mut SystemClock,
) {
    let new_osc = Inro::new(InroFrequency::_8kHz, InroDivider::_1);
    sys_clk.set_sysclk(&new_osc);
    assert!(gcr_clkctrl_reg.read().sysclk_sel().is_inro());
    assert!(trimsir_inro_reg.read().lpclksel().is_8khz());
}

#[cfg(feature = "low_frequency_test")]
#[allow(non_snake_case)]
fn test_inro_freq_16kHz(
    gcr_clkctrl_reg: &CLKCTRL,
    trimsir_inro_reg: &INRO,
    sys_clk: &mut SystemClock,
) {
    let new_osc = Inro::new(InroFrequency::_16kHz, InroDivider::_1);
    sys_clk.set_sysclk(&new_osc);
    assert!(gcr_clkctrl_reg.read().sysclk_sel().is_inro());
    assert!(trimsir_inro_reg.read().lpclksel().is_16khz());
}

#[cfg(feature = "low_frequency_test")]
#[allow(non_snake_case)]
fn test_inro_freq_30kHz(
    gcr_clkctrl_reg: &CLKCTRL,
    trimsir_inro_reg: &INRO,
    sys_clk: &mut SystemClock,
) {
    let new_osc = Inro::new(InroFrequency::_30kHz, InroDivider::_1);
    sys_clk.set_sysclk(&new_osc);
    assert!(gcr_clkctrl_reg.read().sysclk_sel().is_inro());
    assert!(trimsir_inro_reg.read().lpclksel().is_30khz());
}

fn test_ibro(gcr_clkctrl_reg: &CLKCTRL, sys_clk: &mut SystemClock) {
    let new_osc = Ibro::new(IbroFrequency::_7_3728MHz, IbroDivider::_1);
    sys_clk.set_sysclk(&new_osc);
    assert!(gcr_clkctrl_reg.read().sysclk_sel().is_ibro());
}

#[cfg(feature = "low_frequency_test")]
fn test_ertco(gcr_clkctrl_reg: &CLKCTRL, sys_clk: &mut SystemClock) {
    let new_osc = Ertco::new(ErtcoFrequency::_32_768kHz, ErtcoDivider::_1);
    sys_clk.set_sysclk(&new_osc);
    assert!(gcr_clkctrl_reg.read().sysclk_sel().is_ertco());
}

fn test_divider_1(gcr_clkctrl_reg: &CLKCTRL, sys_clk: &mut SystemClock) {
    let new_osc = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_1);
    sys_clk.set_sysclk(&new_osc);
    assert!(gcr_clkctrl_reg.read().sysclk_div().is_div1());
}

fn test_divider_2(gcr_clkctrl_reg: &CLKCTRL, sys_clk: &mut SystemClock) {
    let new_osc = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_2);
    sys_clk.set_sysclk(&new_osc);
    assert!(gcr_clkctrl_reg.read().sysclk_div().is_div2());
}

fn test_divider_4(gcr_clkctrl_reg: &CLKCTRL, sys_clk: &mut SystemClock) {
    let new_osc = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_4);
    sys_clk.set_sysclk(&new_osc);
    assert!(gcr_clkctrl_reg.read().sysclk_div().is_div4());
}

fn test_divider_8(gcr_clkctrl_reg: &CLKCTRL, sys_clk: &mut SystemClock) {
    let new_osc = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_8);
    sys_clk.set_sysclk(&new_osc);
    assert!(gcr_clkctrl_reg.read().sysclk_div().is_div8());
}

fn test_divider_16(gcr_clkctrl_reg: &CLKCTRL, sys_clk: &mut SystemClock) {
    let new_osc = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_16);
    sys_clk.set_sysclk(&new_osc);
    assert!(gcr_clkctrl_reg.read().sysclk_div().is_div16());
}

fn test_divider_32(gcr_clkctrl_reg: &CLKCTRL, sys_clk: &mut SystemClock) {
    let new_osc = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_32);
    sys_clk.set_sysclk(&new_osc);
    assert!(gcr_clkctrl_reg.read().sysclk_div().is_div32());
}

fn test_divider_64(gcr_clkctrl_reg: &CLKCTRL, sys_clk: &mut SystemClock) {
    let new_osc = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_64);
    sys_clk.set_sysclk(&new_osc);
    assert!(gcr_clkctrl_reg.read().sysclk_div().is_div64());
}

fn test_frequency_before(sys_clk: &mut SystemClock) {
    let new_osc = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_1);
    sys_clk.set_sysclk(&new_osc);
    assert!(sys_clk.get_freq() == 100_000_000);
}

fn test_divider_before(sys_clk: &mut SystemClock) {
    let new_osc = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_1);
    sys_clk.set_sysclk(&new_osc);
    assert!(sys_clk.get_div() == 1);
}

fn test_frequency_after(sys_clk: &mut SystemClock) {
    let new_osc = Iso::new(IsoFrequency::_60MHz, IsoDivider::_8);
    sys_clk.set_sysclk(&new_osc);
    assert!(sys_clk.get_freq() == 60_000_000);
}

fn test_divider_after(sys_clk: &mut SystemClock) {
    let new_osc = Iso::new(IsoFrequency::_60MHz, IsoDivider::_8);
    sys_clk.set_sysclk(&new_osc);
    assert!(sys_clk.get_div() == 8);
}
