//! Tests for the timer API

use core::fmt::Write;
use cortex_m_semihosting::hio;
use max78000_hal::max78000::{GCR, TMR, TMR1};
use max78000_hal::peripherals::timer::{Clock, Oscillator, Prescaler, Time};

/// Run the tests for the timer peripheral module
pub fn run_timer_tests(stdout: &mut hio::HostStream, tmr: TMR, tmr1: TMR1, gcr_regs: &GCR) {
    writeln!(stdout, "Starting Timer tests!").unwrap();
    let clock = Clock::new(tmr, gcr_regs, Oscillator::ERTCO, Prescaler::_1);

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

    while !timer.poll() {}

    writeln!(stdout, "Resetting timer, poll for another 3 seconds").unwrap();

    timer.reset();
    while !timer.poll() {}

    writeln!(stdout, "Creating new clock with ISO / 4096").unwrap();

    let clock = Clock::new(tmr1, gcr_regs, Oscillator::ISO, Prescaler::_4096);
    let mut timer = clock.new_timer(Time::Milliseconds(3000));

    writeln!(stdout, "New timer duration is {}", timer.duration_ticks()).unwrap();

    /*for _ in 0..10 {
        writeln!(stdout, "clock val is {}", clock.get_count()).unwrap();
    }*/

    writeln!(stdout, "Poll for 3 seconds...").unwrap();

    while !timer.poll() {}

    writeln!(stdout, "Creating new clock with IBRO / 512").unwrap();

    let clock = Clock::new(clock.consume(), gcr_regs, Oscillator::IBRO, Prescaler::_512);
    let mut timer = clock.new_timer(Time::Milliseconds(5000));

    writeln!(
        stdout,
        "New timer duration is {}ms, {} ticks",
        timer.duration_ms(),
        timer.duration_ticks()
    )
    .unwrap();

    writeln!(stdout, "Polling for ~5 seconds...").unwrap();

    while !timer.poll() {}

    writeln!(stdout, "Timer tests complete!").unwrap();
}
