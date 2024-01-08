use core::fmt::Write;
use cortex_m_semihosting::hio;
use max78000_hal::max78000::gcr::CLKCTRL;
use max78000_hal::max78000::trimsir::INRO;
use max78000_hal::peripherals::oscillator::*;

/// Goes through all the oscillators and sets each one to be the system clk
pub fn run_oscillator_tests(
    clkctrl_reg: &CLKCTRL,
    trimsir_reg: &INRO,
    stdout: &mut hio::HostStream,
) {
    writeln!(stdout, "Starting oscillator tests...").unwrap();

    writeln!(stdout, "Testing setting IPO to SYS_CLK input...").unwrap();
    test_ipo(clkctrl_reg, trimsir_reg);

    writeln!(stdout, "Testing setting ISO to SYS_CLK input...").unwrap();
    test_iso(clkctrl_reg, trimsir_reg);

    writeln!(stdout, "Testing setting IBRO to SYS_CLK input...").unwrap();
    test_ibro(clkctrl_reg, trimsir_reg);

    #[cfg(feature = "low_frequency_test")]
    {
        writeln!(stdout, "Testing setting INRO to SYS_CLK input...").unwrap();
        test_inro(clkctrl_reg, trimsir_reg);

        writeln!(stdout, "Testing setting INRO to frequency to 8kHz...").unwrap();
        test_inro_freq_8kHz(clkctrl_reg, trimsir_reg);

        writeln!(stdout, "Testing setting INRO to frequency to 16kHz...").unwrap();
        test_inro_freq_16kHz(clkctrl_reg, trimsir_reg);

        writeln!(stdout, "Testing setting INRO to frequency to 30kHz...").unwrap();
        test_inro_freq_30kHz(clkctrl_reg, trimsir_reg);

        writeln!(stdout, "Testing setting ERTCO to SYS_CLK input...").unwrap();
        test_ertco(clkctrl_reg, trimsir_reg);
    }

    writeln!(stdout, "Testing setting system oscillator divider to 1...").unwrap();
    test_divider_1(clkctrl_reg, trimsir_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 2...").unwrap();
    test_divider_2(clkctrl_reg, trimsir_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 4...").unwrap();
    test_divider_4(clkctrl_reg, trimsir_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 8...").unwrap();
    test_divider_8(clkctrl_reg, trimsir_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 16...").unwrap();
    test_divider_16(clkctrl_reg, trimsir_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 32...").unwrap();
    test_divider_32(clkctrl_reg, trimsir_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 64...").unwrap();
    test_divider_64(clkctrl_reg, trimsir_reg);

    writeln!(stdout, "Oscillator tests complete!").unwrap();
}

fn test_ipo(clkctrl_reg: &CLKCTRL, trimsir_reg: &INRO) {
    let ipo = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_1);
    let _sys_clk = SystemClock::new(&ipo, clkctrl_reg, trimsir_reg);
    assert_eq!(clkctrl_reg.read().sysclk_sel().is_ipo(), true);
}

fn test_iso(clkctrl_reg: &CLKCTRL, trimsir_reg: &INRO) {
    let iso = Iso::new(IsoFrequency::_60MHz, IsoDivider::_1);
    let _sys_clk = SystemClock::new(&iso, clkctrl_reg, trimsir_reg);
    assert_eq!(clkctrl_reg.read().sysclk_sel().is_iso(), true);
}

#[cfg(feature = "low_frequency_test")]
fn test_inro(clkctrl_reg: &CLKCTRL, trimsir_reg: &INRO) {
    let inro = Inro::new(InroFrequency::_8kHz, InroDivider::_1);
    let _sys_clk = SystemClock::new(&inro, clkctrl_reg, trimsir_reg);
    assert_eq!(clkctrl_reg.read().sysclk_sel().is_inro(), true);
}

#[cfg(feature = "low_frequency_test")]
#[allow(non_snake_case)]
fn test_inro_freq_8kHz(clkctrl_reg: &CLKCTRL, trimsir_reg: &INRO) {
    let inro = Inro::new(InroFrequency::_8kHz, InroDivider::_1);
    let _sys_clk = SystemClock::new(&inro, clkctrl_reg, trimsir_reg);
    assert_eq!(clkctrl_reg.read().sysclk_sel().is_inro(), true);
    assert_eq!(trimsir_reg.read().lpclksel().is_8khz(), true);
}

#[cfg(feature = "low_frequency_test")]
#[allow(non_snake_case)]
fn test_inro_freq_16kHz(clkctrl_reg: &CLKCTRL, trimsir_reg: &INRO) {
    let inro = Inro::new(InroFrequency::_16kHz, InroDivider::_1);
    let _sys_clk = SystemClock::new(&inro, clkctrl_reg, trimsir_reg);
    assert_eq!(clkctrl_reg.read().sysclk_sel().is_inro(), true);
    assert_eq!(trimsir_reg.read().lpclksel().is_16khz(), true);
}

#[cfg(feature = "low_frequency_test")]
#[allow(non_snake_case)]
fn test_inro_freq_30kHz(clkctrl_reg: &CLKCTRL, trimsir_reg: &INRO) {
    let inro = Inro::new(InroFrequency::_30kHz, InroDivider::_1);
    let _sys_clk = SystemClock::new(&inro, clkctrl_reg, trimsir_reg);
    assert_eq!(clkctrl_reg.read().sysclk_sel().is_inro(), true);
    assert_eq!(trimsir_reg.read().lpclksel().is_30khz(), true);
}

fn test_ibro(clkctrl_reg: &CLKCTRL, trimsir_reg: &INRO) {
    let ibro = Ibro::new(IbroFrequency::_7_3728MHz, IbroDivider::_1);
    let _sys_clk = SystemClock::new(&ibro, clkctrl_reg, trimsir_reg);
    assert_eq!(clkctrl_reg.read().sysclk_sel().is_ibro(), true);
}

#[cfg(feature = "low_frequency_test")]
fn test_ertco(clkctrl_reg: &CLKCTRL, trimsir_reg: &INRO) {
    let ertco = Ertco::new(ErtcoFrequency::_32_768kHz, ErtcoDivider::_1);
    let _sys_clk = SystemClock::new(&ertco, clkctrl_reg, trimsir_reg);
    assert_eq!(clkctrl_reg.read().sysclk_sel().is_ertco(), true);
}

fn test_divider_1(clkctrl_reg: &CLKCTRL, trimsir_reg: &INRO) {
    let ipo = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_1);
    let _sys_clk = SystemClock::new(&ipo, clkctrl_reg, trimsir_reg);
    assert_eq!(clkctrl_reg.read().sysclk_div().is_div1(), true);
}

fn test_divider_2(clkctrl_reg: &CLKCTRL, trimsir_reg: &INRO) {
    let ipo = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_2);
    let _sys_clk = SystemClock::new(&ipo, clkctrl_reg, trimsir_reg);
    assert_eq!(clkctrl_reg.read().sysclk_div().is_div2(), true);
}

fn test_divider_4(clkctrl_reg: &CLKCTRL, trimsir_reg: &INRO) {
    let ipo = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_4);
    let _sys_clk = SystemClock::new(&ipo, clkctrl_reg, trimsir_reg);
    assert_eq!(clkctrl_reg.read().sysclk_div().is_div4(), true);
}

fn test_divider_8(clkctrl_reg: &CLKCTRL, trimsir_reg: &INRO) {
    let ipo = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_8);
    let _sys_clk = SystemClock::new(&ipo, clkctrl_reg, trimsir_reg);
    assert_eq!(clkctrl_reg.read().sysclk_div().is_div8(), true);
}

fn test_divider_16(clkctrl_reg: &CLKCTRL, trimsir_reg: &INRO) {
    let ipo = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_16);
    let _sys_clk = SystemClock::new(&ipo, clkctrl_reg, trimsir_reg);
    assert_eq!(clkctrl_reg.read().sysclk_div().is_div16(), true);
}

fn test_divider_32(clkctrl_reg: &CLKCTRL, trimsir_reg: &INRO) {
    let ipo = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_32);
    let _sys_clk = SystemClock::new(&ipo, clkctrl_reg, trimsir_reg);
    assert_eq!(clkctrl_reg.read().sysclk_div().is_div32(), true);
}

fn test_divider_64(clkctrl_reg: &CLKCTRL, trimsir_reg: &INRO) {
    let ipo = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_64);
    let _sys_clk = SystemClock::new(&ipo, clkctrl_reg, trimsir_reg);
    assert_eq!(clkctrl_reg.read().sysclk_div().is_div64(), true);
}
