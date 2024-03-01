//! Tests for the timer API

use core::fmt::Write;
use cortex_m_semihosting::hio;
use max78000_hal::max78000::{TMR, TMR1, TMR2};
use max78000_hal::peripherals::timer::{Clock, Oscillator, Prescaler, Time};
use max78000_hal::peripherals::PeripheralHandle;

use crate::{TIMER_0_OSC, TIMER_0_PRESCALER};

/// Run the tests for the timer peripheral module
pub fn run_timer_tests(
    stdout: &mut hio::HostStream,
    clk0: PeripheralHandle<Clock<TMR>>,
    clk1: PeripheralHandle<Clock<TMR1>>,
    clk2: PeripheralHandle<Clock<TMR2>>,
) {
    writeln!(stdout, "Starting Timer tests!").unwrap();

    {
        let mut timer = clk0.new_timer(Time::Milliseconds(3000));
        writeln!(
            stdout,
            "Timer duration in ticks is {}",
            timer.duration_ticks()
        )
        .unwrap();
        writeln!(stdout, "Timer start, end is {} {}", timer.start, timer.end).unwrap();
        writeln!(stdout, "Clk count is {}", clk0.get_count()).unwrap();
        writeln!(stdout, "Timer poll is {}", timer.poll()).unwrap();
        writeln!(stdout, "Poll for 3 seconds...").unwrap();

        while !timer.poll() {}

        writeln!(stdout, "Resetting timer, poll for another 3 seconds").unwrap();

        timer.reset();
        while !timer.poll() {}

        writeln!(stdout, "Testing new clock with ISO / 4096").unwrap();

        let mut timer = clk1.new_timer(Time::Milliseconds(3000));

        writeln!(stdout, "New timer duration is {}", timer.duration_ticks()).unwrap();

        /*for _ in 0..10 {
            writeln!(stdout, "clock val is {}", clock.get_count()).unwrap();
        }*/

        writeln!(stdout, "Poll for 3 seconds...").unwrap();

        while !timer.poll() {}

        writeln!(stdout, "Testing new clock with IBRO / 512").unwrap();

        let mut timer = clk2.new_timer(Time::Milliseconds(5000));

        writeln!(
            stdout,
            "New timer duration is {}ms, {} ticks",
            timer.duration_ms(),
            timer.duration_ticks()
        )
        .unwrap();

        writeln!(stdout, "Polling for ~5 seconds...").unwrap();

        while !timer.poll() {}
    }

    reconfigure_tests(&clk0);

    writeln!(stdout, "Timer tests complete!").unwrap();
}

fn reconfigure_tests(clk: &Clock<TMR>) {
    {
        let _timer = clk.new_timer(Time::Milliseconds(5000));
        clk.reconfigure(Oscillator::ISO, Prescaler::_1024)
            .expect_err("Reconfigured timer when active timer taken out.");
    }

    clk.reconfigure(TIMER_0_OSC, TIMER_0_PRESCALER)
        .expect("Couldn't reconfigure timer back to original value");
}
