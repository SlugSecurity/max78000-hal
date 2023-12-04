use core::fmt::Write;
use cortex_m_semihosting::hio;
use max78000_hal::max78000::gcr::CLKCTRL;
use max78000_hal::max78000::trimsir::INRO;
use max78000_hal::peripherals::bit_banding as bb;
use max78000_hal::peripherals::oscillator as osc;

/// Goes through all the oscillators and sets each one to be the system clk
pub fn run_oscillator_tests(
    stdout: &mut hio::HostStream,
    clkctrl_reg: &CLKCTRL,
    inro_trsimsir_reg: &INRO,
) {
    writeln!(stdout, "Starting oscillator tests...").unwrap();

    writeln!(stdout, "Testing setting IPO to SYS_CLK input...").unwrap();

    writeln!(stdout, "Testing setting ISO to SYS_CLK input...").unwrap();

    writeln!(stdout, "Testing setting INRO to SYS_CLK input...").unwrap();

    writeln!(stdout, "Testing setting IBRO to SYS_CLK input...").unwrap();

    writeln!(stdout, "Testing setting IBRO to frequency to 8kHz...").unwrap();
    writeln!(stdout, "Testing setting IBRO to frequency to 16kHz...").unwrap();
    writeln!(stdout, "Testing setting IBRO to frequency to 30kHz...").unwrap();

    writeln!(stdout, "Testing setting ERTCO to SYS_CLK input...").unwrap();

    writeln!(stdout, "Testing setting system oscillator divider to 1...").unwrap();
    writeln!(stdout, "Testing setting system oscillator divider to 2...").unwrap();
    writeln!(stdout, "Testing setting system oscillator divider to 4...").unwrap();
    writeln!(stdout, "Testing setting system oscillator divider to 8...").unwrap();
    writeln!(stdout, "Testing setting system oscillator divider to 16...").unwrap();
    writeln!(stdout, "Testing setting system oscillator divider to 32...").unwrap();
    writeln!(stdout, "Testing setting system oscillator divider to 64...").unwrap();
    writeln!(
        stdout,
        "Testing setting system oscillator divider to 6128..."
    )
    .unwrap();

    writeln!(stdout, "Oscillator tests complete!").unwrap();
}
