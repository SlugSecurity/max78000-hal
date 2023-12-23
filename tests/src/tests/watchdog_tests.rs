//! Tests for the watchdog timer peripheral

use core::fmt::Write;
use cortex_m::asm::nop;
use cortex_m_semihosting::hio;
use max78000_hal::interrupt;
use max78000_hal::max78000::{GCR, WDT};
use max78000_hal::peripherals::watchdog::{ClockSource, Configuration, Threshold, WatchdogTimer};

#[interrupt]
fn WWDT() {
    // SAFETY: probably safe lol
    let mut watchdog_timer: WatchdogTimer =
        WatchdogTimer::new(unsafe { WDT::steal() }, unsafe { &GCR::steal() });
    let mut stdout = hio::hstdout().unwrap();

    writeln!(stdout, "Inside WWDT interrupt!").unwrap();

    watchdog_timer.disable();

    if watchdog_timer.interrupt_late_event() {
        writeln!(stdout, "Clearing late interrupt flag...").unwrap();
        watchdog_timer.clear_interrupt_late_flag();
    }
    if watchdog_timer.interrupt_early_event() {
        writeln!(stdout, "Clearing late interrupt flag...").unwrap();
        watchdog_timer.clear_interrupt_early_flag();
    }
}

/// Run the tests for the watchdog timer peripheral module
pub fn run_watchdog_tests(stdout: &mut hio::HostStream, wdt_regs: WDT, gcr_regs: &GCR) {
    let mut watchdog_timer = WatchdogTimer::new(wdt_regs, gcr_regs);
    writeln!(stdout, "Starting watchdog timer tests...").unwrap();

    watchdog_timer.disable();

    writeln!(stdout, "Disabled watchdog timer!").unwrap();

    watchdog_timer.configure(Configuration {
        clock_source: ClockSource::PCLK,
        windowed_mode: None,
        reset_late_val: Threshold::_2POW31,
        interrupt_late_val: Threshold::_2POW16,
    });

    writeln!(stdout, "Configured watchdog timer!").unwrap();

    watchdog_timer.enable();

    writeln!(
        stdout,
        "Enabled watchdog timer! Stalling a bit before kicking it..."
    )
    .unwrap();

    for _ in 0..65536 {
        nop();
    }

    writeln!(stdout, "About to kick watchdog...").unwrap();

    watchdog_timer.kick();

    writeln!(stdout, "this should be after the interrupt").unwrap();
}
