//! Oscillator tests

use core::fmt::Write;
use cortex_m_semihosting::hio;
use max78000_hal::max78000::gcr::CLKCTRL;
use max78000_hal::max78000::trimsir::INRO;
use max78000_hal::peripherals::oscillator::*;

/// Goes through all the oscillators and sets each one to be the system clk
pub fn run_oscillator_tests(
    gcr_clkctrl_reg: &CLKCTRL,
    trimsir_inro_reg: &INRO,
    stdout: &mut hio::HostStream,
) {
    writeln!(stdout, "Starting oscillator tests...").unwrap();

    writeln!(stdout, "Testing setting IPO to SYS_CLK input...").unwrap();
    test_ipo(gcr_clkctrl_reg, trimsir_inro_reg);

    writeln!(stdout, "Testing setting ISO to SYS_CLK input...").unwrap();
    test_iso(gcr_clkctrl_reg, trimsir_inro_reg);

    writeln!(stdout, "Testing setting IBRO to SYS_CLK input...").unwrap();
    test_ibro(gcr_clkctrl_reg, trimsir_inro_reg);

    #[cfg(feature = "low_frequency_test")]
    {
        writeln!(stdout, "Testing setting INRO to frequency to 8kHz...").unwrap();
        test_inro_freq_8kHz(gcr_clkctrl_reg, trimsir_inro_reg);

        writeln!(stdout, "Testing setting INRO to frequency to 16kHz...").unwrap();
        test_inro_freq_16kHz(gcr_clkctrl_reg, trimsir_inro_reg);

        writeln!(stdout, "Testing setting INRO to frequency to 30kHz...").unwrap();
        test_inro_freq_30kHz(gcr_clkctrl_reg, trimsir_inro_reg);

        writeln!(stdout, "Testing setting ERTCO to SYS_CLK input...").unwrap();
        test_ertco(gcr_clkctrl_reg, trimsir_inro_reg);
    }

    writeln!(stdout, "Testing setting system oscillator divider to 1...").unwrap();
    test_divider_1(gcr_clkctrl_reg, trimsir_inro_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 2...").unwrap();
    test_divider_2(gcr_clkctrl_reg, trimsir_inro_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 4...").unwrap();
    test_divider_4(gcr_clkctrl_reg, trimsir_inro_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 8...").unwrap();
    test_divider_8(gcr_clkctrl_reg, trimsir_inro_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 16...").unwrap();
    test_divider_16(gcr_clkctrl_reg, trimsir_inro_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 32...").unwrap();
    test_divider_32(gcr_clkctrl_reg, trimsir_inro_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 64...").unwrap();
    test_divider_64(gcr_clkctrl_reg, trimsir_inro_reg);

    writeln!(
        stdout,
        "Testing get system oscillator frequency before setting to iso..."
    )
    .unwrap();
    test_frequency_before(gcr_clkctrl_reg, trimsir_inro_reg);
    writeln!(
        stdout,
        "Testing get system oscillator divider before setting to iso..."
    )
    .unwrap();
    test_divider_before(gcr_clkctrl_reg, trimsir_inro_reg);
    writeln!(
        stdout,
        "Testing get system oscillator frequency after setting to iso..."
    )
    .unwrap();
    test_frequency_after(gcr_clkctrl_reg, trimsir_inro_reg);
    writeln!(
        stdout,
        "Testing get system oscillator divider after setting to iso..."
    )
    .unwrap();
    test_divider_after(gcr_clkctrl_reg, trimsir_inro_reg);

    writeln!(stdout, "Oscillator tests complete!").unwrap();
}

fn test_ipo(gcr_clkctrl_reg: &CLKCTRL, trimsir_inro_reg: &INRO) {
    let ipo = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_1);
    let _sys_clk = SystemClock::new(&ipo, gcr_clkctrl_reg, trimsir_inro_reg);
    assert!(gcr_clkctrl_reg.read().sysclk_sel().is_ipo());
}

fn test_iso(gcr_clkctrl_reg: &CLKCTRL, trimsir_inro_reg: &INRO) {
    let iso = Iso::new(IsoFrequency::_60MHz, IsoDivider::_1);
    let _sys_clk = SystemClock::new(&iso, gcr_clkctrl_reg, trimsir_inro_reg);
    assert!(gcr_clkctrl_reg.read().sysclk_sel().is_iso());
}

#[cfg(feature = "low_frequency_test")]
#[allow(non_snake_case)]
fn test_inro_freq_8kHz(gcr_clkctrl_reg: &CLKCTRL, trimsir_inro_reg: &INRO) {
    let inro = Inro::new(InroFrequency::_8kHz, InroDivider::_1);
    let _sys_clk = SystemClock::new(&inro, gcr_clkctrl_reg, trimsir_inro_reg);
    assert!(gcr_clkctrl_reg.read().sysclk_sel().is_inro());
    assert!(trimsir_inro_reg.read().lpclksel().is_8khz());
}

#[cfg(feature = "low_frequency_test")]
#[allow(non_snake_case)]
fn test_inro_freq_16kHz(gcr_clkctrl_reg: &CLKCTRL, trimsir_inro_reg: &INRO) {
    let inro = Inro::new(InroFrequency::_16kHz, InroDivider::_1);
    let _sys_clk = SystemClock::new(&inro, gcr_clkctrl_reg, trimsir_inro_reg);
    assert!(gcr_clkctrl_reg.read().sysclk_sel().is_inro());
    assert!(trimsir_inro_reg.read().lpclksel().is_16khz());
}

#[cfg(feature = "low_frequency_test")]
#[allow(non_snake_case)]
fn test_inro_freq_30kHz(gcr_clkctrl_reg: &CLKCTRL, trimsir_inro_reg: &INRO) {
    let inro = Inro::new(InroFrequency::_30kHz, InroDivider::_1);
    let _sys_clk = SystemClock::new(&inro, gcr_clkctrl_reg, trimsir_inro_reg);
    assert!(gcr_clkctrl_reg.read().sysclk_sel().is_inro());
    assert!(trimsir_inro_reg.read().lpclksel().is_30khz());
}

fn test_ibro(gcr_clkctrl_reg: &CLKCTRL, trimsir_inro_reg: &INRO) {
    let ibro = Ibro::new(IbroFrequency::_7_3728MHz, IbroDivider::_1);
    let _sys_clk = SystemClock::new(&ibro, gcr_clkctrl_reg, trimsir_inro_reg);
    assert!(gcr_clkctrl_reg.read().sysclk_sel().is_ibro());
}

#[cfg(feature = "low_frequency_test")]
fn test_ertco(gcr_clkctrl_reg: &CLKCTRL, trimsir_inro_reg: &INRO) {
    let ertco = Ertco::new(ErtcoFrequency::_32_768kHz, ErtcoDivider::_1);
    let _sys_clk = SystemClock::new(&ertco, gcr_clkctrl_reg, trimsir_inro_reg);
    assert!(gcr_clkctrl_reg.read().sysclk_sel().is_ertco());
}

fn test_divider_1(gcr_clkctrl_reg: &CLKCTRL, trimsir_inro_reg: &INRO) {
    let ipo = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_1);
    let _sys_clk = SystemClock::new(&ipo, gcr_clkctrl_reg, trimsir_inro_reg);
    assert!(gcr_clkctrl_reg.read().sysclk_div().is_div1());
}

fn test_divider_2(gcr_clkctrl_reg: &CLKCTRL, trimsir_inro_reg: &INRO) {
    let ipo = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_2);
    let _sys_clk = SystemClock::new(&ipo, gcr_clkctrl_reg, trimsir_inro_reg);
    assert!(gcr_clkctrl_reg.read().sysclk_div().is_div2());
}

fn test_divider_4(gcr_clkctrl_reg: &CLKCTRL, trimsir_inro_reg: &INRO) {
    let ipo = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_4);
    let _sys_clk = SystemClock::new(&ipo, gcr_clkctrl_reg, trimsir_inro_reg);
    assert!(gcr_clkctrl_reg.read().sysclk_div().is_div4());
}

fn test_divider_8(gcr_clkctrl_reg: &CLKCTRL, trimsir_inro_reg: &INRO) {
    let ipo = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_8);
    let _sys_clk = SystemClock::new(&ipo, gcr_clkctrl_reg, trimsir_inro_reg);
    assert!(gcr_clkctrl_reg.read().sysclk_div().is_div8());
}

fn test_divider_16(gcr_clkctrl_reg: &CLKCTRL, trimsir_inro_reg: &INRO) {
    let ipo = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_16);
    let _sys_clk = SystemClock::new(&ipo, gcr_clkctrl_reg, trimsir_inro_reg);
    assert!(gcr_clkctrl_reg.read().sysclk_div().is_div16());
}

fn test_divider_32(gcr_clkctrl_reg: &CLKCTRL, trimsir_inro_reg: &INRO) {
    let ipo = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_32);
    let _sys_clk = SystemClock::new(&ipo, gcr_clkctrl_reg, trimsir_inro_reg);
    assert!(gcr_clkctrl_reg.read().sysclk_div().is_div32());
}

fn test_divider_64(gcr_clkctrl_reg: &CLKCTRL, trimsir_inro_reg: &INRO) {
    let ipo = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_64);
    let _sys_clk = SystemClock::new(&ipo, gcr_clkctrl_reg, trimsir_inro_reg);
    assert!(gcr_clkctrl_reg.read().sysclk_div().is_div64());
}

fn test_frequency_before(gcr_clkctrl_reg: &CLKCTRL, trimsir_inro_reg: &INRO) {
    let ipo = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_1);
    let sys_clk = SystemClock::new(&ipo, gcr_clkctrl_reg, trimsir_inro_reg);
    assert!(sys_clk.clock_frequency == 100_000_000);
}

fn test_divider_before(gcr_clkctrl_reg: &CLKCTRL, trimsir_inro_reg: &INRO) {
    let ipo = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_1);
    let sys_clk = SystemClock::new(&ipo, gcr_clkctrl_reg, trimsir_inro_reg);
    assert!(sys_clk.clock_divider == 1);
}

fn test_frequency_after(gcr_clkctrl_reg: &CLKCTRL, trimsir_inro_reg: &INRO) {
    let iso = Iso::new(IsoFrequency::_60MHz, IsoDivider::_8);
    let sys_clk = SystemClock::new(&iso, gcr_clkctrl_reg, trimsir_inro_reg);
    assert!(sys_clk.clock_frequency == 60_000_000);
}

fn test_divider_after(gcr_clkctrl_reg: &CLKCTRL, trimsir_inro_reg: &INRO) {
    let iso = Iso::new(IsoFrequency::_60MHz, IsoDivider::_8);
    let sys_clk = SystemClock::new(&iso, gcr_clkctrl_reg, trimsir_inro_reg);
    assert!(sys_clk.clock_divider == 8);
}
