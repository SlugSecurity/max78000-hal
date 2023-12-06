use core::fmt::Write;
use cortex_m_semihosting::hio;
use max78000_hal::max78000::gcr::CLKCTRL;
use max78000_hal::max78000::trimsir::INRO;
use max78000_hal::peripherals::oscillator::Divider;
use max78000_hal::peripherals::oscillator::InroFrequency;
use max78000_hal::peripherals::oscillator::SystemClock;

/// Goes through all the oscillators and sets each one to be the system clk
pub fn run_oscillator_tests(
    clkctrl_reg: &CLKCTRL,
    inro_trsimsir_reg: &INRO,
    stdout: &mut hio::HostStream,
) {
    writeln!(stdout, "Starting oscillator tests...").unwrap();

    writeln!(stdout, "Testing setting IPO to SYS_CLK input...").unwrap();
    test_ipo(clkctrl_reg);

    writeln!(stdout, "Testing setting ISO to SYS_CLK input...").unwrap();
    test_iso(clkctrl_reg);

    writeln!(stdout, "Testing setting INRO to SYS_CLK input...").unwrap();
    test_inro(clkctrl_reg, inro_trsimsir_reg);

    writeln!(stdout, "Testing setting IBRO to SYS_CLK input...").unwrap();
    test_ibro(clkctrl_reg);

    writeln!(stdout, "Testing setting INRO to frequency to 8kHz...").unwrap();
    test_inro_freq_8kHz(clkctrl_reg, inro_trsimsir_reg);

    writeln!(stdout, "Testing setting INRO to frequency to 16kHz...").unwrap();
    test_inro_freq_16kHz(clkctrl_reg, inro_trsimsir_reg);

    writeln!(stdout, "Testing setting INRO to frequency to 30kHz...").unwrap();
    test_inro_freq_30kHz(clkctrl_reg, inro_trsimsir_reg);

    writeln!(stdout, "Testing setting ERTCO to SYS_CLK input...").unwrap();
    test_ertco(clkctrl_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 1...").unwrap();
    test_divider_1(clkctrl_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 2...").unwrap();
    test_divider_2(clkctrl_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 4...").unwrap();
    test_divider_4(clkctrl_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 8...").unwrap();
    test_divider_8(clkctrl_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 16...").unwrap();
    test_divider_16(clkctrl_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 32...").unwrap();
    test_divider_32(clkctrl_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 64...").unwrap();
    test_divider_64(clkctrl_reg);

    writeln!(stdout, "Oscillator tests complete!").unwrap();
}

fn test_ipo(clkctrl_reg: &CLKCTRL) {
    let div = Divider::_1;
    let sys_clk = SystemClock::configure_ipo(div, clkctrl_reg);
    sys_clk.set();
    assert_eq!(clkctrl_reg.read().sysclk_sel().is_ipo(), true);
}

fn test_iso(clkctrl_reg: &CLKCTRL) {
    let div = Divider::_1;
    let sys_clk = SystemClock::configure_iso(div, clkctrl_reg);
    sys_clk.set();
    assert_eq!(clkctrl_reg.read().sysclk_sel().is_iso(), true);
}

fn test_inro(clkctrl_reg: &CLKCTRL, inro_trsimsir_reg: &INRO) {
    let div = Divider::_1;
    let freq = InroFrequency::_8kHz;
    let sys_clk = SystemClock::configure_inro(freq, div, clkctrl_reg, inro_trsimsir_reg);
    sys_clk.set();
    assert_eq!(clkctrl_reg.read().sysclk_sel().is_inro(), true);
}

#[allow(non_snake_case)]
fn test_inro_freq_8kHz(clkctrl_reg: &CLKCTRL, inro_trsimsir_reg: &INRO) {
    let div = Divider::_1;
    let freq = InroFrequency::_8kHz;
    let sys_clk = SystemClock::configure_inro(freq, div, clkctrl_reg, inro_trsimsir_reg);
    sys_clk.set();
    assert_eq!(clkctrl_reg.read().sysclk_sel().is_inro(), true);
    assert_eq!(inro_trsimsir_reg.read().lpclksel().is_8khz(), true);
}

#[allow(non_snake_case)]
fn test_inro_freq_16kHz(clkctrl_reg: &CLKCTRL, inro_trsimsir_reg: &INRO) {
    let div = Divider::_1;
    let freq = InroFrequency::_16kHz;
    let sys_clk = SystemClock::configure_inro(freq, div, clkctrl_reg, inro_trsimsir_reg);
    sys_clk.set();
    assert_eq!(clkctrl_reg.read().sysclk_sel().is_inro(), true);
    assert_eq!(inro_trsimsir_reg.read().lpclksel().is_16khz(), true);
}

#[allow(non_snake_case)]
fn test_inro_freq_30kHz(clkctrl_reg: &CLKCTRL, inro_trsimsir_reg: &INRO) {
    let div = Divider::_1;
    let freq = InroFrequency::_30kHz;
    let sys_clk = SystemClock::configure_inro(freq, div, clkctrl_reg, inro_trsimsir_reg);
    sys_clk.set();
    assert_eq!(clkctrl_reg.read().sysclk_sel().is_inro(), true);
    assert_eq!(inro_trsimsir_reg.read().lpclksel().is_30khz(), true);
}

fn test_ibro(clkctrl_reg: &CLKCTRL) {
    let div = Divider::_1;
    let sys_clk = SystemClock::configure_ibro(div, clkctrl_reg);
    sys_clk.set();
    assert_eq!(clkctrl_reg.read().sysclk_sel().is_ibro(), true);
}

fn test_ertco(clkctrl_reg: &CLKCTRL) {
    let div = Divider::_1;
    let sys_clk = SystemClock::configure_ertco(div, clkctrl_reg);
    sys_clk.set();
    assert_eq!(clkctrl_reg.read().sysclk_sel().is_ertco(), true);
}

fn test_divider_1(clkctrl_reg: &CLKCTRL) {
    let div = Divider::_1;
    let sys_clk = SystemClock::configure_ipo(div, clkctrl_reg);
    sys_clk.set();
    assert_eq!(clkctrl_reg.read().sysclk_div().is_div1(), true);
}

fn test_divider_2(clkctrl_reg: &CLKCTRL) {
    let div = Divider::_2;
    let sys_clk = SystemClock::configure_ipo(div, clkctrl_reg);
    sys_clk.set();
    assert_eq!(clkctrl_reg.read().sysclk_div().is_div2(), true);
}

fn test_divider_4(clkctrl_reg: &CLKCTRL) {
    let div = Divider::_4;
    let sys_clk = SystemClock::configure_ipo(div, clkctrl_reg);
    sys_clk.set();
    assert_eq!(clkctrl_reg.read().sysclk_div().is_div4(), true);
}

fn test_divider_8(clkctrl_reg: &CLKCTRL) {
    let div = Divider::_8;
    let sys_clk = SystemClock::configure_ipo(div, clkctrl_reg);
    sys_clk.set();
    assert_eq!(clkctrl_reg.read().sysclk_div().is_div8(), true);
}

fn test_divider_16(clkctrl_reg: &CLKCTRL) {
    let div = Divider::_16;
    let sys_clk = SystemClock::configure_ipo(div, clkctrl_reg);
    sys_clk.set();
    assert_eq!(clkctrl_reg.read().sysclk_div().is_div16(), true);
}

fn test_divider_32(clkctrl_reg: &CLKCTRL) {
    let div = Divider::_32;
    let sys_clk = SystemClock::configure_ipo(div, clkctrl_reg);
    sys_clk.set();
    assert_eq!(clkctrl_reg.read().sysclk_div().is_div32(), true);
}

fn test_divider_64(clkctrl_reg: &CLKCTRL) {
    let div = Divider::_64;
    let sys_clk = SystemClock::configure_ipo(div, clkctrl_reg);
    sys_clk.set();
    assert_eq!(clkctrl_reg.read().sysclk_div().is_div64(), true);
}
