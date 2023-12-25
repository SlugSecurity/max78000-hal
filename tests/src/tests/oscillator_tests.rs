use core::fmt::Write;
use cortex_m_semihosting::hio;
use max78000_hal::max78000::GCR;
use max78000_hal::max78000::TRIMSIR;
use max78000_hal::peripherals::oscillator::*;

/// Goes through all the oscillators and sets each one to be the system clk
pub fn run_oscillator_tests(gcr_reg: &GCR, trsimsir_reg: &TRIMSIR, stdout: &mut hio::HostStream) {
    writeln!(stdout, "Starting oscillator tests...").unwrap();

    writeln!(stdout, "Testing setting IPO to SYS_CLK input...").unwrap();
    test_ipo(gcr_reg);

    writeln!(stdout, "Testing setting ISO to SYS_CLK input...").unwrap();
    test_iso(gcr_reg);

    writeln!(stdout, "Testing setting IBRO to SYS_CLK input...").unwrap();
    test_ibro(gcr_reg);

    #[cfg(feature = "low_frequency")]
    {
        writeln!(stdout, "Testing setting INRO to SYS_CLK input...").unwrap();
        test_inro(gcr_reg, trsimsir_reg);

        writeln!(stdout, "Testing setting INRO to frequency to 8kHz...").unwrap();
        test_inro_freq_8kHz(gcr_reg, trsimsir_reg);

        writeln!(stdout, "Testing setting INRO to frequency to 16kHz...").unwrap();
        test_inro_freq_16kHz(gcr_reg, trsimsir_reg);

        writeln!(stdout, "Testing setting INRO to frequency to 30kHz...").unwrap();
        test_inro_freq_30kHz(gcr_reg, trsimsir_reg);

        writeln!(stdout, "Testing setting ERTCO to SYS_CLK input...").unwrap();
        test_ertco(gcr_reg);
    }

    writeln!(stdout, "Testing setting system oscillator divider to 1...").unwrap();
    test_divider_1(gcr_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 2...").unwrap();
    test_divider_2(gcr_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 4...").unwrap();
    test_divider_4(gcr_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 8...").unwrap();
    test_divider_8(gcr_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 16...").unwrap();
    test_divider_16(gcr_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 32...").unwrap();
    test_divider_32(gcr_reg);

    writeln!(stdout, "Testing setting system oscillator divider to 64...").unwrap();
    test_divider_64(gcr_reg);

    writeln!(
        stdout,
        "Testing setting system oscillator divider to 128..."
    )
    .unwrap();
    test_divider_128(gcr_reg);

    writeln!(stdout, "Oscillator tests complete!").unwrap();
}

fn test_ipo(gcr_reg: &GCR) {
    let sys_clk = SystemClock::new(
        Oscillator::Primary(IpoFrequency::_100MHz),
        Divider::_1,
        FrequencyPeripheral::None,
        gcr_reg,
    );
    sys_clk.set();
    assert_eq!(gcr_reg.clkctrl().read().sysclk_sel().is_ipo(), true);
    gcr_reg.clkctrl().reset();
}

fn test_iso(gcr_reg: &GCR) {
    let sys_clk = SystemClock::new(
        Oscillator::Secondary(IsoFrequency::_60MHz),
        Divider::_1,
        FrequencyPeripheral::None,
        gcr_reg,
    );
    sys_clk.set();
    assert_eq!(gcr_reg.clkctrl().read().sysclk_sel().is_iso(), true);
    gcr_reg.clkctrl().reset();
}

#[cfg(feature = "low_frequency")]
fn test_inro(gcr_reg: &GCR, trsimsir_reg: &TRIMSIR) {
    let sys_clk = SystemClock::new(
        Oscillator::NanoRing(InroFrequency::_8kHz),
        Divider::_1,
        FrequencyPeripheral::TrimsirInro(trsimsir_reg),
        gcr_reg,
    );
    sys_clk.set();
    assert_eq!(gcr_reg.clkctrl().read().sysclk_sel().is_inro(), true);
    gcr_reg.clkctrl().reset();
}

#[cfg(feature = "low_frequency")]
#[allow(non_snake_case)]
fn test_inro_freq_8kHz(gcr_reg: &GCR, trsimsir_reg: &TRIMSIR) {
    let sys_clk = SystemClock::new(
        Oscillator::NanoRing(InroFrequency::_8kHz),
        Divider::_1,
        FrequencyPeripheral::TrimsirInro(trsimsir_reg),
        gcr_reg,
    );
    sys_clk.set();
    assert_eq!(gcr_reg.clkctrl().read().sysclk_sel().is_inro(), true);
    assert_eq!(trsimsir_reg.inro().read().lpclksel().is_8khz(), true);
    gcr_reg.clkctrl().reset();
}

#[cfg(feature = "low_frequency")]
#[allow(non_snake_case)]
fn test_inro_freq_16kHz(gcr_reg: &GCR, trsimsir_reg: &TRIMSIR) {
    let sys_clk = SystemClock::new(
        Oscillator::NanoRing(InroFrequency::_16kHz),
        Divider::_1,
        FrequencyPeripheral::TrimsirInro(trsimsir_reg),
        gcr_reg,
    );
    sys_clk.set();
    assert_eq!(gcr_reg.clkctrl().read().sysclk_sel().is_inro(), true);
    assert_eq!(trsimsir_reg.inro().read().lpclksel().is_16khz(), true);
    gcr_reg.clkctrl().reset();
}

#[cfg(feature = "low_frequency")]
#[allow(non_snake_case)]
fn test_inro_freq_30kHz(gcr_reg: &GCR, trsimsir_reg: &TRIMSIR) {
    let sys_clk = SystemClock::new(
        Oscillator::NanoRing(InroFrequency::_30kHz),
        Divider::_1,
        FrequencyPeripheral::TrimsirInro(trsimsir_reg),
        gcr_reg,
    );
    sys_clk.set();
    assert_eq!(gcr_reg.clkctrl().read().sysclk_sel().is_inro(), true);
    assert_eq!(trsimsir_reg.inro().read().lpclksel().is_30khz(), true);
    gcr_reg.clkctrl().reset();
}

fn test_ibro(gcr_reg: &GCR) {
    let sys_clk = SystemClock::new(
        Oscillator::BaudRate(IbroFrequency::_7_3728MHz),
        Divider::_1,
        FrequencyPeripheral::None,
        gcr_reg,
    );
    sys_clk.set();
    assert_eq!(gcr_reg.clkctrl().read().sysclk_sel().is_ibro(), true);
    gcr_reg.clkctrl().reset();
}

#[cfg(feature = "low_frequency")]
fn test_ertco(gcr_reg: &GCR) {
    let sys_clk = SystemClock::new(
        Oscillator::RealTimeClock(ErtcoFrequency::_32_768kHz),
        Divider::_1,
        FrequencyPeripheral::None,
        gcr_reg,
    );
    sys_clk.set();
    assert_eq!(gcr_reg.clkctrl().read().sysclk_sel().is_ertco(), true);
    gcr_reg.clkctrl().reset();
}

fn test_divider_1(gcr_reg: &GCR) {
    let sys_clk = SystemClock::new(
        Oscillator::Primary(IpoFrequency::_100MHz),
        Divider::_1,
        FrequencyPeripheral::None,
        gcr_reg,
    );
    sys_clk.set();
    assert_eq!(gcr_reg.clkctrl().read().sysclk_div().is_div1(), true);
    gcr_reg.clkctrl().reset();
}

fn test_divider_2(gcr_reg: &GCR) {
    let sys_clk = SystemClock::new(
        Oscillator::Primary(IpoFrequency::_100MHz),
        Divider::_2,
        FrequencyPeripheral::None,
        gcr_reg,
    );
    sys_clk.set();
    assert_eq!(gcr_reg.clkctrl().read().sysclk_div().is_div2(), true);
    gcr_reg.clkctrl().reset();
}

fn test_divider_4(gcr_reg: &GCR) {
    let sys_clk = SystemClock::new(
        Oscillator::Primary(IpoFrequency::_100MHz),
        Divider::_4,
        FrequencyPeripheral::None,
        gcr_reg,
    );
    sys_clk.set();
    assert_eq!(gcr_reg.clkctrl().read().sysclk_div().is_div4(), true);
    gcr_reg.clkctrl().reset();
}

fn test_divider_8(gcr_reg: &GCR) {
    let sys_clk = SystemClock::new(
        Oscillator::Primary(IpoFrequency::_100MHz),
        Divider::_8,
        FrequencyPeripheral::None,
        gcr_reg,
    );
    sys_clk.set();
    assert_eq!(gcr_reg.clkctrl().read().sysclk_div().is_div8(), true);
    gcr_reg.clkctrl().reset();
}

fn test_divider_16(gcr_reg: &GCR) {
    let sys_clk = SystemClock::new(
        Oscillator::Primary(IpoFrequency::_100MHz),
        Divider::_16,
        FrequencyPeripheral::None,
        gcr_reg,
    );
    sys_clk.set();
    assert_eq!(gcr_reg.clkctrl().read().sysclk_div().is_div16(), true);
    gcr_reg.clkctrl().reset();
}

fn test_divider_32(gcr_reg: &GCR) {
    let sys_clk = SystemClock::new(
        Oscillator::Primary(IpoFrequency::_100MHz),
        Divider::_32,
        FrequencyPeripheral::None,
        gcr_reg,
    );
    sys_clk.set();
    assert_eq!(gcr_reg.clkctrl().read().sysclk_div().is_div32(), true);
    gcr_reg.clkctrl().reset();
}

fn test_divider_64(gcr_reg: &GCR) {
    let sys_clk = SystemClock::new(
        Oscillator::Primary(IpoFrequency::_100MHz),
        Divider::_64,
        FrequencyPeripheral::None,
        gcr_reg,
    );
    sys_clk.set();
    assert_eq!(gcr_reg.clkctrl().read().sysclk_div().is_div64(), true);
    gcr_reg.clkctrl().reset();
}

fn test_divider_128(gcr_reg: &GCR) {
    let sys_clk = SystemClock::new(
        Oscillator::Primary(IpoFrequency::_100MHz),
        Divider::_128,
        FrequencyPeripheral::None,
        gcr_reg,
    );
    sys_clk.set();
    assert_eq!(gcr_reg.clkctrl().read().sysclk_div().is_div128(), true);
    gcr_reg.clkctrl().reset();
}
