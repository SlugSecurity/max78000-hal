//! Tests for the watchdog timer peripheral

use core::fmt::Write;
use cortex_m::asm::nop;
use cortex_m::interrupt::free;
use cortex_m_semihosting::hio;
use max78000_hal::interrupt;
use max78000_hal::max78000::{Interrupt, GCR, WDT};
use max78000_hal::peripherals::watchdog::{ClockSource, Configuration, Threshold, WatchdogTimer};

static mut INTERRUPT_COUNT: u32 = 0;

#[interrupt]
fn WWDT() {
    free(|_| {
        // SAFETY: safe;
        let mut watchdog_timer: WatchdogTimer = WatchdogTimer::new(unsafe { WDT::steal() });
        let mut stdout = hio::hstdout().unwrap();

        writeln!(stdout, "WWDT INT: Inside WWDT interrupt!").unwrap();

        // SAFETY: safe; this is the only place that modifies INTERRUPT_COUNT and it's inside an
        // interrupt context in a single-threaded environment, no races will occur
        unsafe { INTERRUPT_COUNT += 1 };

        if watchdog_timer.interrupt_late_event() {
            writeln!(stdout, "WWDT INT: Clearing late interrupt flag.").unwrap();
            watchdog_timer.clear_interrupt_late_flag();
        }
        if watchdog_timer.interrupt_early_event() {
            writeln!(stdout, "WWDT INT: Clearing late interrupt flag...").unwrap();
            watchdog_timer.clear_interrupt_early_flag();
        }
        if watchdog_timer.reset_early_event() {
            writeln!(stdout, "WWDT INT: Clearing early reset flag.").unwrap();
            watchdog_timer.clear_reset_early_flag();
        }
        if watchdog_timer.reset_late_event() {
            writeln!(stdout, "WWDT INT: Clearing late reset flag.").unwrap();
            watchdog_timer.clear_reset_late_flag();
        }
    });
}

/// Run the tests for the watchdog timer peripheral module
pub fn run_watchdog_tests(stdout: &mut hio::HostStream, wdt_regs: WDT, gcr_regs: &GCR) {
    let mut watchdog_timer = WatchdogTimer::new(wdt_regs);
    WatchdogTimer::enable_peripheral_clock(gcr_regs);
    WatchdogTimer::reset_peripheral(gcr_regs);

    writeln!(stdout, "WDT: Starting watchdog timer tests!").unwrap();

    writeln!(stdout, "WDT: Testing basic non-windowed interrupt.").unwrap();

    // SAFETY: Safe, as this function is not being executed in a critical section
    unsafe { cortex_m::peripheral::NVIC::unmask(Interrupt::WWDT) };

    watchdog_timer.disable();
    watchdog_timer.kick();

    writeln!(stdout, "WDT: Disabled watchdog timer.").unwrap();

    watchdog_timer.configure(Configuration {
        clock_source: ClockSource::PCLK,
        windowed_mode: None,
        reset_late_val: Threshold::_2POW31,
        interrupt_late_val: Threshold::_2POW16,
        watchdog_reset_interrupt_enable: true,
        watchdog_interrupt_enable: true,
    });

    writeln!(stdout, "WDT: Configured watchdog timer.").unwrap();

    watchdog_timer.enable();

    writeln!(
        stdout,
        "WDT: Enabled watchdog timer. Stalling a bit to let the late interrupt trigger..."
    )
    .unwrap();

    for _ in 0..65536 {
        nop();
    }

    if unsafe { INTERRUPT_COUNT } == 0 {
        writeln!(
            stdout,
            "WDT: Basic test failed, expected a watchdog interrupt to occur,"
        )
        .unwrap();
    } else {
        writeln!(stdout, "WDT: Basic test passed! Watchdog interrupt fired.").unwrap();
    }

    writeln!(stdout, "WDT: Tests complete, disabling watchdog timer...").unwrap();
    watchdog_timer.disable();
}
