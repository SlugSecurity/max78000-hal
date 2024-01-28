//! Tests for the timer API

use core::fmt::Write;
use cortex_m::asm::nop;
use cortex_m_semihosting::hio;
use max78000_hal::max78000::{GCR, TMR};
use max78000_hal::peripherals::timer::{Clock, Oscillator, Prescaler, Time};

/// Run the tests for the timer peripheral module
pub fn run_timer_tests(stdout: &mut hio::HostStream, tmr_regs: TMR, gcr_regs: &GCR) {
    writeln!(stdout, "Starting Timer tests!").unwrap();
    let clock = Clock::new(tmr_regs, gcr_regs, Oscillator::ERTCO, Prescaler::_1);

    let mut timer = clock.new_timer(Time::Milliseconds(3000));
    writeln!(
        stdout,
        "Timer duration in ticks is {}",
        timer.duration_ticks()
    )
    .unwrap();
    writeln!(stdout, "Timer start, end is {} {}", timer.start, timer.end).unwrap();
    writeln!(stdout, "Clk count is {}", clock.get_count()).unwrap();
    writeln!(stdout, "Timer poll is {}", timer.poll()).unwrap();
    writeln!(stdout, "Poll for 3 seconds...").unwrap();

    while !timer.poll() {
        nop()
    }

    writeln!(stdout, "Timer tests complete!").unwrap();
}
