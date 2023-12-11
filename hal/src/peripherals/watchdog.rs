//! Watchdog timer peripheral API.
//! TODO: documentation

use cortex_m::interrupt::free;
use max78000::wdt::ctrl::EN_A;
use max78000::wdt::ctrl::{INT_EARLY_A, INT_LATE_A, RST_EARLY_A, RST_LATE_A, WIN_EN_A};
use max78000::wdt::ctrl::{INT_EARLY_VAL_A, INT_LATE_VAL_A, RST_EARLY_VAL_A, RST_LATE_VAL_A};
use max78000::wdt::rst::RESET_AW;
use max78000::WDT;

use crate::peripherals::bit_banding::spin_bit;

/// The Watchdog Timer peripheral struct. Obtain an instance of one with `WatchDogTimer::new`
pub struct WatchdogTimer {
    wdt_regs: WDT,
}

/// Clock source for the watchdog timer
pub enum ClockSource {
    /// PCLK
    PCLK,
    /// IBRO
    IBRO,
}

/// Denotes a threshold for one of the watchdog timer parameters,
/// which represents a value in clock cycles for one of the watchdog timer events:
/// late interrupt, late reset, early interrupt, and early reset. Values are in powers of two,
/// ranging from `2^16` to `2^31`
pub enum Threshold {
    /// `2^16` cycles
    _2POW16,
    /// `2^17` cycles
    _2POW17,
    /// `2^18` cycles
    _2POW18,
    /// `2^19` cycles
    _2POW19,
    /// `2^20` cycles
    _2POW20,
    /// `2^21` cycles
    _2POW21,
    /// `2^22` cycles
    _2POW22,
    /// `2^23` cycles
    _2POW23,
    /// `2^24` cycles
    _2POW24,
    /// `2^25` cycles
    _2POW25,
    /// `2^26` cycles
    _2POW26,
    /// `2^27` cycles
    _2POW27,
    /// `2^28` cycles
    _2POW28,
    /// `2^29` cycles
    _2POW29,
    /// `2^30` cycles
    _2POW30,
    /// `2^31` cycles
    _2POW31,
}

/// Declarative macro to avoid duplicated code when converting from generic
/// [`Threshold`] into the various enums like [`INT_EARLY_VAL_A`]
macro_rules! into_threshold {
    ($thresholdVar:expr,$thresholdName:ty) => {
        match $thresholdVar {
            Threshold::_2POW16 => <$thresholdName>::WDT2POW16,
            Threshold::_2POW17 => <$thresholdName>::WDT2POW17,
            Threshold::_2POW18 => <$thresholdName>::WDT2POW18,
            Threshold::_2POW19 => <$thresholdName>::WDT2POW19,
            Threshold::_2POW20 => <$thresholdName>::WDT2POW20,
            Threshold::_2POW21 => <$thresholdName>::WDT2POW21,
            Threshold::_2POW22 => <$thresholdName>::WDT2POW22,
            Threshold::_2POW23 => <$thresholdName>::WDT2POW23,
            Threshold::_2POW24 => <$thresholdName>::WDT2POW24,
            Threshold::_2POW25 => <$thresholdName>::WDT2POW25,
            Threshold::_2POW26 => <$thresholdName>::WDT2POW26,
            Threshold::_2POW27 => <$thresholdName>::WDT2POW27,
            Threshold::_2POW28 => <$thresholdName>::WDT2POW28,
            Threshold::_2POW29 => <$thresholdName>::WDT2POW29,
            Threshold::_2POW30 => <$thresholdName>::WDT2POW30,
            Threshold::_2POW31 => <$thresholdName>::WDT2POW31,
        }
    };
}

/// Windowed timer mode configuration - allows the timer to trigger an interrupt or reset
/// if the watchdog is kicked too early, as well as too late
pub struct WindowedConfiguration {
    interrupt_early_val: Threshold,
    reset_early_val: Threshold,
}

/// Configuration for the watchdog timer.
pub struct Configuration {
    clock_source: ClockSource,
    interrupt_late_val: Threshold,
    reset_late_val: Threshold,
    windowed_mode: Option<WindowedConfiguration>,
}

enum FeedSequenceOperation {
    Disable,
    Enable,
    Kick,
}

impl WatchdogTimer {
    /// Creates a new instance of the Watchdog Timer peripheral.
    pub fn new(wdt_regs: WDT) -> Self {
        Self { wdt_regs }
    }

    /// Kicks the watchdog
    pub fn kick(&mut self) {
        self.feed_sequence(FeedSequenceOperation::Kick);
    }

    /// Configure the watchdog timer
    ///
    /// # Danger
    ///
    /// Provide reasonable values, otherwise you risk bricking the system
    pub fn configure(&mut self, options: Configuration) {
        if self.is_enabled() {
            self.disable();
        }

        self.wdt_regs.clksel().write(|w| {
            w.source().variant(match options.clock_source {
                ClockSource::PCLK => 1,
                ClockSource::IBRO => 2,
            })
        });

        self.wdt_regs.ctrl().write(|w| {
            w.int_late_val()
                .variant(into_threshold!(options.interrupt_late_val, INT_LATE_VAL_A))
                .rst_late_val()
                .variant(into_threshold!(options.reset_late_val, RST_LATE_VAL_A))
        });

        match options.windowed_mode {
            None => {
                self.wdt_regs
                    .ctrl()
                    .write(|w| w.win_en().variant(WIN_EN_A::DIS));
            }
            Some(windowed_config) => self.wdt_regs.ctrl().write(|w| {
                w.win_en()
                    .variant(WIN_EN_A::EN)
                    .rst_early_val()
                    .variant(into_threshold!(
                        windowed_config.reset_early_val,
                        RST_EARLY_VAL_A
                    ))
                    .int_early_val()
                    .variant(into_threshold!(
                        windowed_config.interrupt_early_val,
                        INT_EARLY_VAL_A
                    ))
            }),
        }
    }

    /// Returns if the watchdog timer peripheral is enabled
    pub fn is_enabled(&self) -> bool {
        self.wdt_regs.ctrl().read().en().bit()
    }

    /// Disables the watchdog timer peripheral
    pub fn disable(&mut self) {
        self.feed_sequence(FeedSequenceOperation::Disable);
    }

    /// Enables the watchdog timer peripheral
    pub fn enable(&mut self) {
        self.feed_sequence(FeedSequenceOperation::Enable);
    }

    /// Returns whether or not the Reset Late event flag is active
    pub fn reset_late_event(&self) -> bool {
        match self.wdt_regs.ctrl().read().rst_late().variant() {
            RST_LATE_A::NO_EVENT => false,
            RST_LATE_A::OCCURRED => true,
        }
    }

    /// Returns whether or not the Reset Early event flag is active
    pub fn reset_early_event(&self) -> bool {
        match self.wdt_regs.ctrl().read().rst_early().variant() {
            RST_EARLY_A::NO_EVENT => false,
            RST_EARLY_A::OCCURRED => true,
        }
    }

    /// Returns whether or not the Interrupt Late event flag is active
    pub fn interrupt_late_event(&self) -> bool {
        match self.wdt_regs.ctrl().read().int_late().variant() {
            INT_LATE_A::INACTIVE => false,
            INT_LATE_A::PENDING => true,
        }
    }

    /// Returns whether or not the Interrupt Early event flag is active
    pub fn interrupt_early_event(&self) -> bool {
        match self.wdt_regs.ctrl().read().int_early().variant() {
            INT_EARLY_A::INACTIVE => false,
            INT_EARLY_A::PENDING => true,
        }
    }

    /// Clears the Reset Late flag (WDT_CTRL.rst_late)
    pub fn clear_reset_late_flag(&mut self) {
        self.wdt_regs
            .ctrl()
            .write(|w| w.rst_late().variant(RST_LATE_A::NO_EVENT));
    }

    /// Clears the Reset Early flag (WDT_CTRL.rst_early)
    pub fn clear_reset_early_flag(&mut self) {
        self.wdt_regs
            .ctrl()
            .write(|w| w.rst_early().variant(RST_EARLY_A::NO_EVENT));
    }

    /// Clears the Interrupt Late flag (WDT_CTRL.int_late)
    pub fn clear_interrupt_late_flag(&mut self) {
        self.wdt_regs
            .ctrl()
            .write(|w| w.int_late().variant(INT_LATE_A::INACTIVE));
    }

    /// Clears the Interrupt Early flag (WDT_CTRL.int_early)
    pub fn clear_interrupt_early_flag(&mut self) {
        self.wdt_regs
            .ctrl()
            .write(|w| w.int_early().variant(INT_EARLY_A::INACTIVE));
    }

    fn clear_all_flags(&mut self) {
        // page 332: Software must clear all event flags before enabling the timers
        self.wdt_regs.ctrl().write(|w| {
            w.rst_late()
                .variant(RST_LATE_A::NO_EVENT)
                .rst_early()
                .variant(RST_EARLY_A::NO_EVENT)
                .int_early()
                .variant(INT_EARLY_A::INACTIVE)
                .int_late()
                .variant(INT_LATE_A::INACTIVE)
        });
    }

    fn poll_clkrdy(&mut self) {
        // SAFETY: safe, as we are passing in a peripheral address in bit-banding space,
        // (0x4000_3000), bit 28 (WDT0_CTRL.clkrdy) is a readable bit of a valid register
        // (page 336 of the user guide)
        unsafe {
            spin_bit(self.wdt_regs.ctrl().as_ptr(), 28, true);
        }
    }

    fn feed_sequence(&mut self, feed_sequence_operation: FeedSequenceOperation) {
        // run in an interrupt-free context
        free(|_| {
            // First value to be written to enable WDT (0xa5)
            self.wdt_regs
                .rst()
                .write(|w| w.reset().variant(RESET_AW::SEQ0));
            // Second value to be written to enable WDT (0x5a)
            self.wdt_regs
                .rst()
                .write(|w| w.reset().variant(RESET_AW::SEQ1));

            match feed_sequence_operation {
                FeedSequenceOperation::Disable => {
                    self.wdt_regs.ctrl().write(|w| w.en().variant(EN_A::DIS));
                    self.poll_clkrdy();
                }
                FeedSequenceOperation::Enable => {
                    self.clear_all_flags();
                    self.wdt_regs.ctrl().write(|w| w.en().variant(EN_A::EN));
                    self.poll_clkrdy();
                }
                FeedSequenceOperation::Kick => (),
            }
        });
    }
}
