use core::fmt::Write;
use cortex_m_semihosting::hio;
use max78000_hal::max78000::FLC;
use max78000_hal::max78000::GCR;
use max78000_hal::max78000::TRIMSIR;
use max78000_hal::peripherals::oscillator::*;

/// Goes through all the oscillators and sets each one to be the system clk
pub fn run_oscillator_tests(
    gcr_reg: &GCR,
    flc_reg: &FLC,
    trsimsir_reg: &TRIMSIR,
    stdout: &mut hio::HostStream,
) {
    writeln!(stdout, "Starting oscillator tests...").unwrap();

    writeln!(stdout, "Testing setting IPO to SYS_CLK input...").unwrap();
    test_ipo(gcr_reg, flc_reg);

    writeln!(stdout, "Testing setting ISO to SYS_CLK input...").unwrap();
    test_iso(gcr_reg, flc_reg);

    writeln!(stdout, "Testing setting IBRO to SYS_CLK input...").unwrap();
    test_ibro(gcr_reg, flc_reg);

    #[cfg(feature = "low_frequency_test")]
    {
        writeln!(stdout, "Testing setting INRO to SYS_CLK input...").unwrap();
        test_inro(gcr_reg, flc_reg, trsimsir_reg);

        writeln!(stdout, "Testing setting INRO to frequency to 8kHz...").unwrap();
        test_inro_freq_8kHz(gcr_reg, flc_reg, trsimsir_reg);

        writeln!(stdout, "Testing setting INRO to frequency to 16kHz...").unwrap();
        test_inro_freq_16kHz(gcr_reg, flc_reg, trsimsir_reg);

        writeln!(stdout, "Testing setting INRO to frequency to 30kHz...").unwrap();
        test_inro_freq_30kHz(gcr_reg, flc_reg, trsimsir_reg);

        writeln!(stdout, "Testing setting ERTCO to SYS_CLK input...").unwrap();
        test_ertco(gcr_reg, flc_reg);
    }

    writeln!(stdout, "Testing setting system oscillator divider to 1...").unwrap();
    test_divider_1(gcr_reg, flc_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 2...").unwrap();
    test_divider_2(gcr_reg, flc_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 4...").unwrap();
    test_divider_4(gcr_reg, flc_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 8...").unwrap();
    test_divider_8(gcr_reg, flc_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 16...").unwrap();
    test_divider_16(gcr_reg, flc_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 32...").unwrap();
    test_divider_32(gcr_reg, flc_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 64...").unwrap();
    test_divider_64(gcr_reg, flc_reg);

    writeln!(stdout, "Oscillator tests complete!").unwrap();
}

fn test_ipo(gcr_reg: &GCR, flc_reg: &FLC) {
    let div = Divider::_1;
    let sys_clk = SystemClock::configure_ipo(div, gcr_reg, flc_reg);
    assert_eq!(gcr_reg.clkctrl().read().sysclk_sel().is_ipo(), true);
}

fn test_iso(gcr_reg: &GCR, flc_reg: &FLC) {
    let div = Divider::_1;
    let sys_clk = SystemClock::configure_iso(div, gcr_reg, flc_reg);
    assert_eq!(gcr_reg.clkctrl().read().sysclk_sel().is_iso(), true);
}

#[cfg(feature = "low_frequency_test")]
fn test_inro(gcr_reg: &GCR, flc_reg: &FLC, trsimsir_reg: &TRIMSIR) {
    let div = Divider::_1;
    let freq = InroFrequency::_8kHz;
    let sys_clk = SystemClock::configure_inro(freq, div, gcr_reg, flc_reg, trsimsir_reg);
    assert_eq!(gcr_reg.clkctrl().read().sysclk_sel().is_inro(), true);
}

#[cfg(feature = "low_frequency_test")]
#[allow(non_snake_case)]
fn test_inro_freq_8kHz(gcr_reg: &GCR, flc_reg: &FLC, trsimsir_reg: &TRIMSIR) {
    let div = Divider::_1;
    let freq = InroFrequency::_8kHz;
    let sys_clk = SystemClock::configure_inro(freq, div, gcr_reg, flc_reg, trsimsir_reg);
    assert_eq!(gcr_reg.clkctrl().read().sysclk_sel().is_inro(), true);
    assert_eq!(trsimsir_reg.inro().read().lpclksel().is_8khz(), true);
}

#[cfg(feature = "low_frequency_test")]
#[allow(non_snake_case)]
fn test_inro_freq_16kHz(gcr_reg: &GCR, flc_reg: &FLC, trsimsir_reg: &TRIMSIR) {
    let div = Divider::_1;
    let freq = InroFrequency::_16kHz;
    let sys_clk = SystemClock::configure_inro(freq, div, gcr_reg, flc_reg, trsimsir_reg);
    assert_eq!(gcr_reg.clkctrl().read().sysclk_sel().is_inro(), true);
    assert_eq!(trsimsir_reg.inro().read().lpclksel().is_16khz(), true);
}

#[cfg(feature = "low_frequency_test")]
#[allow(non_snake_case)]
fn test_inro_freq_30kHz(gcr_reg: &GCR, flc_reg: &FLC, trsimsir_reg: &TRIMSIR) {
    let div = Divider::_1;
    let freq = InroFrequency::_30kHz;
    let sys_clk = SystemClock::configure_inro(freq, div, gcr_reg, flc_reg, trsimsir_reg);
    assert_eq!(gcr_reg.clkctrl().read().sysclk_sel().is_inro(), true);
    assert_eq!(trsimsir_reg.inro().read().lpclksel().is_30khz(), true);
}

fn test_ibro(gcr_reg: &GCR, flc_reg: &FLC) {
    let div = Divider::_1;
    let sys_clk = SystemClock::configure_ibro(div, gcr_reg, flc_reg);
    assert_eq!(gcr_reg.clkctrl().read().sysclk_sel().is_ibro(), true);
}

#[cfg(feature = "low_frequency_test")]
fn test_ertco(gcr_reg: &GCR, flc_reg: &FLC) {
    let div = Divider::_1;
    let sys_clk = SystemClock::configure_ertco(div, gcr_reg, flc_reg);
    assert_eq!(gcr_reg.clkctrl().read().sysclk_sel().is_ertco(), true);
}

fn test_divider_1(gcr_reg: &GCR, flc_reg: &FLC) {
    let div = Divider::_1;
    let sys_clk = SystemClock::configure_ipo(div, gcr_reg, flc_reg);
    assert_eq!(gcr_reg.clkctrl().read().sysclk_div().is_div1(), true);
}

fn test_divider_2(gcr_reg: &GCR, flc_reg: &FLC) {
    let div = Divider::_2;
    let sys_clk = SystemClock::configure_ipo(div, gcr_reg, flc_reg);
    assert_eq!(gcr_reg.clkctrl().read().sysclk_div().is_div2(), true);
}

fn test_divider_4(gcr_reg: &GCR, flc_reg: &FLC) {
    let div = Divider::_4;
    let sys_clk = SystemClock::configure_ipo(div, gcr_reg, flc_reg);
    assert_eq!(gcr_reg.clkctrl().read().sysclk_div().is_div4(), true);
}

fn test_divider_8(gcr_reg: &GCR, flc_reg: &FLC) {
    let div = Divider::_8;
    let sys_clk = SystemClock::configure_ipo(div, gcr_reg, flc_reg);
    assert_eq!(gcr_reg.clkctrl().read().sysclk_div().is_div8(), true);
}

fn test_divider_16(gcr_reg: &GCR, flc_reg: &FLC) {
    let div = Divider::_16;
    let sys_clk = SystemClock::configure_ipo(div, gcr_reg, flc_reg);
    assert_eq!(gcr_reg.clkctrl().read().sysclk_div().is_div16(), true);
}

fn test_divider_32(gcr_reg: &GCR, flc_reg: &FLC) {
    let div = Divider::_32;
    let sys_clk = SystemClock::configure_ipo(div, gcr_reg, flc_reg);
    assert_eq!(gcr_reg.clkctrl().read().sysclk_div().is_div32(), true);
}

fn test_divider_64(gcr_reg: &GCR, flc_reg: &FLC) {
    let div = Divider::_64;
    let sys_clk = SystemClock::configure_ipo(div, gcr_reg, flc_reg);
    assert_eq!(gcr_reg.clkctrl().read().sysclk_div().is_div64(), true);
}

fn test_divider_128(gcr_reg: &GCR, flc_reg: &FLC) {
    let div = Divider::_128;
    let sys_clk = SystemClock::configure_ipo(div, gcr_reg, flc_reg);
    assert_eq!(gcr_reg.clkctrl().read().sysclk_div().is_div128(), true);
}
