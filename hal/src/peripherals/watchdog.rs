//! Watchdog timer peripheral API.

use max78000::{WDT, WDT1};
use crate::peripherals::bit_banding::{read_bit, change_bit, spin_bit};
use cortex_m::interrupt::free;
use max78000::wdt::clksel::CLKSEL_SPEC;
use max78000::wdt::ctrl::EN_A;
use max78000::wdt::rst::RESET_AW;

pub struct WatchdogTimer {
    wdt_regs: WDT
}

pub struct WatchdogTimerConfiguration {
    clock_source: CLKSEL_SPEC
}

enum FeedSequenceOperation {
    DISABLE,
    ENABLE,
    KICK
}

impl WatchdogTimer {
    pub fn new(wdt_regs: WDT) -> Self {
        Self {wdt_regs}
    }

    pub fn kick(&mut self) {
        self.feed_sequence(FeedSequenceOperation::KICK);
    }

    /// Configure the watchdog timer
    ///
    /// # Danger
    ///
    /// Provide reasonable values, otherwise you risk bricking the system
    pub fn configure(&mut self) {
        if self.is_enabled() {
            self.disable();
        }

    }

    fn is_enabled(&self) -> bool {
        self.wdt_regs.ctrl().read().en().bit()
    }

    /// Disables the watchdog timer peripheral
    pub fn disable(&mut self) {
        self.feed_sequence(FeedSequenceOperation::DISABLE);
    }

    /// Enables the watchdog timer peripheral
    pub fn enable(&mut self) {
        self.feed_sequence(FeedSequenceOperation::ENABLE);
    }

    fn feed_sequence(&mut self, feed_sequence_operation: FeedSequenceOperation) {
        free(|_| {
            // First value to be written to enable WDT (0xa5)
            self.wdt_regs.rst().write(|w| w.reset().variant(RESET_AW::SEQ0));
            // Second value to be written to enable WDT (0x5a)
            self.wdt_regs.rst().write(|w| w.reset().variant(RESET_AW::SEQ1));

            match feed_sequence_operation {
                FeedSequenceOperation::DISABLE => {
                    self.wdt_regs.ctrl().write(|w| w.en().variant(EN_A::DIS));
                    // Verify that the watchdog peripheral is disabled
                    // SAFETY: safe, as we are passing in a peripheral address in bit-banding space,
                    // (0x4000_3000), bit 28 (WDT0_CTRL.clkrdy) is a readable bit of a valid register
                    // (page 336 of the user guide)
                    unsafe {
                        spin_bit(self.wdt_regs.ctrl().as_ptr(), 28, true);
                    }
                },
                FeedSequenceOperation::ENABLE => {
                    self.wdt_regs.ctrl().write(|w| w.en().variant(EN_A::EN));
                    // Verify that the watchdog peripheral is enabled
                    // SAFETY: safe, as we are passing in a peripheral address in bit-banding space,
                    // (0x4000_3000), bit 28 (WDT0_CTRL.clkrdy) is a readable bit of a valid register
                    // (page 336 of the user guide)
                    unsafe {
                        spin_bit(self.wdt_regs.ctrl().as_ptr(), 28, true);
                    }
                },
                FeedSequenceOperation::KICK => ()
            }
        });
    }
}