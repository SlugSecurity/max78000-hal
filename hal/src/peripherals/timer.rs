//! Peripheral API for Timers

use core::cell::Cell;
use core::ops::Deref;
use core::time::Duration;
use max78000::gcr::clkctrl::ERTCO_EN_A;
use max78000::gcr::pclkdis0::GPIO0_A;
use max78000::gcr::rst0::RESET_A;
use max78000::tmr;
use max78000::tmr1::ctrl0::{CLKDIV_A_A, MODE_A_A};
use max78000::GCR;
use max78000::{TMR, TMR1, TMR2, TMR3};

use crate::communication::Timeout;

/// Auxiliary trait that only the TMR, TMR1, TMR2, and TMR3 registers can implement;
/// Allows peripheral toggle and reset functionality to said peripherals if GCR regs
/// are provided.
pub trait TimerPeripheralGCR: Deref<Target = tmr::RegisterBlock> {
    /// Disable peripheral
    fn peripheral_clock_disable(gcr_reg: &GCR);
    /// Enable peripheral
    fn peripheral_clock_enable(gcr_reg: &GCR);
    /// Reset the peripheral
    fn reset_peripheral(gcr_reg: &GCR);
}

macro_rules! gen_impl_tpgcr {
    ($register:ty, $lowercaseName:ident) => {
        impl TimerPeripheralGCR for $register {
            fn peripheral_clock_disable(gcr_reg: &GCR) {
                gcr_reg
                    .pclkdis0()
                    .modify(|_, w| w.$lowercaseName().variant(GPIO0_A::DIS))
            }
            fn peripheral_clock_enable(gcr_reg: &GCR) {
                gcr_reg
                    .pclkdis0()
                    .modify(|_, w| w.$lowercaseName().variant(GPIO0_A::EN))
            }
            fn reset_peripheral(gcr_reg: &GCR) {
                gcr_reg
                    .rst0()
                    .modify(|_, w| w.$lowercaseName().variant(RESET_A::BUSY));
                while gcr_reg.rst0().read().$lowercaseName() == RESET_A::BUSY {}
            }
        }
    };
}

gen_impl_tpgcr!(TMR, tmr0);
gen_impl_tpgcr!(TMR1, tmr1);
gen_impl_tpgcr!(TMR2, tmr2);
gen_impl_tpgcr!(TMR3, tmr3);

/// `Clock` struct. This will take ownership of the timer peripheral registers and is generic to
/// `TMR`, `TMR1`, `TMR2`, and `TMR3`. With it you can start timers using [`Clock::new_timer`]
///
/// # Example
///
/// ```rust
/// use max78000_hal::peripherals::timer::{Clock, Oscillator, Prescaler, Time};
/// use max78000_hal::max78000::{Peripherals};
///
/// let peripherals = Peripherals::take().unwrap();
/// // create the clock
/// let mut clock = Clock::new(peripherals.TMR, &peripherals.GCR, Oscillator::IBRO, Prescaler::_1024);
///
/// // spawn a new timer
/// let mut timer = clock.new_timer(Time::Milliseconds(5000));
///
/// // will stall for 5 seconds
/// while !timer.poll() {};
/// ```
pub struct Clock<'a, T: TimerPeripheralGCR> {
    gcr: &'a GCR,
    tmr_registers: T,
    ticks_per_ms: Cell<f64>,
    active_timers: Cell<usize>,
}

/// Oscillator options for the Clock struct
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Oscillator {
    /// IBRO - 7.3728MHz Internal Baud Rate Oscillator
    IBRO,
    /// ISO - 60MHz Internal Secondary Oscillator
    ISO,
    /// 32.768kHz External Real-Time Clock Oscillator
    ERTCO,
}

/// Instance of a timer. Internally keeps clock start and end values, and a reference to the clock
/// that created it.
pub struct Timer<'clock, 'gcr, T: TimerPeripheralGCR> {
    /// Start timestamp
    pub start: u32,
    /// End timestamp
    pub end: u32,
    clock: &'clock Clock<'gcr, T>,
    finished: bool,
}

impl<T: TimerPeripheralGCR> Drop for Timer<'_, '_, T> {
    fn drop(&mut self) {
        // Decrease the active timer counter for the reconfigure function.
        self.clock
            .active_timers
            .set(self.clock.active_timers.get() - 1);
    }
}

impl<T: TimerPeripheralGCR> Timeout for Timer<'_, '_, T> {
    fn poll(&mut self) -> bool {
        self.poll()
    }

    fn reset(&mut self) {
        self.reset()
    }

    fn duration(&self) -> Duration {
        Duration::from_millis(self.duration_ms() as u64)
    }
}

impl<'clock, 'gcr, T: TimerPeripheralGCR> Timer<'clock, 'gcr, T> {
    fn new(start: u32, end: u32, clock: &'clock Clock<'gcr, T>) -> Self {
        Self {
            start,
            end,
            clock,
            finished: false,
        }
    }

    /// Poll if the timer has finished yet or not. Will return `true` upon timer finish.
    ///
    /// Caveat: only reliable if called within `2^31` clock ticks of timer creation
    pub fn poll(&mut self) -> bool {
        if self.finished {
            return true;
        };
        let res = if self.start > self.end {
            // funni overflow!
            let cnt = self.clock.get_count();
            cnt >= self.end && cnt < self.start
        } else {
            self.clock.get_count() >= self.end
        };
        if res {
            self.finished = true;
        }
        res
    }

    /// Reset the timer back to 0. Will also restart a finished timer.
    pub fn reset(&mut self) {
        let duration = self.end.wrapping_sub(self.start);
        let cnt = self.clock.get_count();
        self.start = cnt;
        self.end = self.start.wrapping_add(duration);
        self.finished = false;
    }

    /// Get total duration, in clock ticks
    pub fn duration_ticks(&self) -> u32 {
        self.end.wrapping_sub(self.start)
    }

    /// Get total duration, in milliseconds
    pub fn duration_ms(&self) -> u32 {
        self.clock.ticks_to_ms(self.duration_ticks())
    }
}

/// Error type that represents that an operation cannot be performed
/// because the timer is in use.
#[derive(Debug, Copy, Clone)]
pub struct TimerInUseError;

/// Represents a time value for starting a timer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Time {
    /// Raw clock ticks, i.e, number of oscillations / prescaler
    Ticks(u32),
    /// Milliseconds
    Milliseconds(u32),
}

/// Prescaler values to divide oscillator by. Available in powers of two from `0` to `12`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Prescaler {
    /// Divide by 1
    _1,
    /// Divide by 2
    _2,
    /// Divide by 4
    _4,
    /// Divide by 8
    _8,
    /// Divide by 16
    _16,
    /// Divide by 32
    _32,
    /// Divide by 64
    _64,
    /// Divide by 128
    _128,
    /// Divide by 256
    _256,
    /// Divide by 512
    _512,
    /// Divide by 1024
    _1024,
    /// Divide by 2048
    _2048,
    /// Divide by 4096
    _4096,
}

impl<'gcr, T: TimerPeripheralGCR> Clock<'gcr, T> {
    /// Creates a new `Clock`, taking ownership of the timer peripheral register block,
    /// a reference to the GCR registers for initial configuration, as
    /// well as config values for the oscillator source and the prescaler value, which will divide
    /// the oscillator source to only increment count once per `prescaler` ticks.
    pub(crate) fn new(
        tmr_registers: T,
        gcr_registers: &'gcr GCR,
        oscillator: Oscillator,
        prescaler: Prescaler,
    ) -> Self {
        let clock = Clock {
            active_timers: Cell::new(0),
            gcr: gcr_registers,
            tmr_registers,
            ticks_per_ms: Cell::new(0f64),
        };

        clock.configure(gcr_registers, oscillator, prescaler);
        clock
    }

    fn configure(&self, gcr_registers: &GCR, oscillator: Oscillator, prescaler: Prescaler) {
        // Disable timer
        self.tmr_registers
            .ctrl0()
            .modify(|_, w| w.en_a().variant(false));
        self.tmr_registers
            .ctrl0()
            .modify(|_, w| w.en_b().variant(false));
        while self.tmr_registers.ctrl1().read().clken_a().bit() {}
        while self.tmr_registers.ctrl1().read().clken_b().bit() {}

        // Configure for continuous mode
        self.tmr_registers.ctrl0().modify(|_, w| {
            w.mode_a()
                .variant(MODE_A_A::CONTINUOUS)
                .clkdiv_a()
                .variant(match prescaler {
                    Prescaler::_1 => CLKDIV_A_A::DIV_BY_1,
                    Prescaler::_2 => CLKDIV_A_A::DIV_BY_2,
                    Prescaler::_4 => CLKDIV_A_A::DIV_BY_4,
                    Prescaler::_8 => CLKDIV_A_A::DIV_BY_8,
                    Prescaler::_16 => CLKDIV_A_A::DIV_BY_16,
                    Prescaler::_32 => CLKDIV_A_A::DIV_BY_32,
                    Prescaler::_64 => CLKDIV_A_A::DIV_BY_64,
                    Prescaler::_128 => CLKDIV_A_A::DIV_BY_128,
                    Prescaler::_256 => CLKDIV_A_A::DIV_BY_256,
                    Prescaler::_512 => CLKDIV_A_A::DIV_BY_512,
                    Prescaler::_1024 => CLKDIV_A_A::DIV_BY_1024,
                    Prescaler::_2048 => CLKDIV_A_A::DIV_BY_2048,
                    Prescaler::_4096 => CLKDIV_A_A::DIV_BY_4096,
                })
        });
        // Configure oscillator and set the timer to be cascading 32 bit
        self.tmr_registers.ctrl1().modify(|_, w| {
            w.clksel_a()
                .variant(match oscillator {
                    //Oscillator::PCLK => 0,
                    Oscillator::ISO => 1,
                    Oscillator::IBRO => 2,
                    Oscillator::ERTCO => 3,
                })
                .cascade()
                .variant(true)
        });

        // enable underlying oscillator

        match oscillator {
            Oscillator::ERTCO => gcr_registers
                .clkctrl()
                .modify(|_, w| w.ertco_en().variant(ERTCO_EN_A::EN)),
            Oscillator::ISO => gcr_registers
                .clkctrl()
                .modify(|_, w| w.iso_en().variant(ERTCO_EN_A::EN)),
            Oscillator::IBRO => gcr_registers
                .clkctrl()
                .modify(|_, w| w.ibro_en().variant(ERTCO_EN_A::EN)),
        }

        // Figure out conversion factor between ticks and milliseconds
        let clkdiv = match prescaler {
            Prescaler::_1 => 1f64,
            Prescaler::_2 => 2f64,
            Prescaler::_4 => 4f64,
            Prescaler::_8 => 8f64,
            Prescaler::_16 => 16f64,
            Prescaler::_32 => 32f64,
            Prescaler::_64 => 64f64,
            Prescaler::_128 => 128f64,
            Prescaler::_256 => 256f64,
            Prescaler::_512 => 512f64,
            Prescaler::_1024 => 1024f64,
            Prescaler::_2048 => 2048f64,
            Prescaler::_4096 => 4096f64,
        };

        // TODO: add PCLK support if ever needed
        let clks_per_ms = match oscillator {
            Oscillator::ISO => 60000f64, // 60 Mhz
            Oscillator::IBRO => 7372.8,  // 7.3728 Mhz
            Oscillator::ERTCO => 32.768, // 32.768 Khz
        };

        self.ticks_per_ms.set(clks_per_ms / clkdiv);

        // Set time to repeat every 2^32 ticks (basically highest period possible)
        self.tmr_registers
            .cmp()
            .write(|w| w.compare().variant(0xffffffff));
        self.tmr_registers.cnt().write(|w| w.count().variant(1));

        // enable the timer clock
        self.tmr_registers
            .ctrl0()
            .modify(|_, w| w.clken_a().variant(true));
        while !self.tmr_registers.ctrl1().read().clkrdy_a().bit() {}
        self.tmr_registers
            .ctrl0()
            .modify(|_, w| w.en_a().variant(true));
        while !self.tmr_registers.ctrl0().read().clken_a().bit() {}
    }

    /// Modify the prescaler and oscillator for this timer. This operation
    /// only works if no timers are actively in use. In other words, any
    /// [`Timer`] linked to this Clock must be dropped prior to calling
    /// this. Otherwise, the operation fails with a [`TimerInUseError`].
    pub fn reconfigure(
        &self,
        oscillator: Oscillator,
        prescaler: Prescaler,
    ) -> Result<(), TimerInUseError> {
        if self.active_timers.get() == 0 {
            self.configure(self.gcr, oscillator, prescaler);

            Ok(())
        } else {
            Err(TimerInUseError)
        }
    }

    /// Return raw clk count val
    pub fn get_count(&self) -> u32 {
        self.tmr_registers.cnt().read().count().bits()
    }

    /// Convert milliseconds to ticks
    pub fn ms_to_ticks(&self, ms: u32) -> u32 {
        ((ms as f64) * self.ticks_per_ms.get()) as u32
    }

    /// Convert ticks to milliseconds
    pub fn ticks_to_ms(&self, ticks: u32) -> u32 {
        (ticks as f64 / self.ticks_per_ms.get()) as u32
    }

    /// Start a new timer with given `Time`, which can be expressed with either raw `Ticks`
    /// or `Milliseconds`, which will be converted into ticks internally.
    ///
    /// Caveat: Will only work reliably for durations of less than `2^31` ticks.
    pub fn new_timer(&self, duration: Time) -> Timer<T> {
        self.active_timers.set(self.active_timers.get() + 1);
        let current = self.get_count();
        match duration {
            Time::Ticks(ticks) => {
                // Ticks are straightforward
                Timer::new(current, current.wrapping_add(ticks), self)
            }
            Time::Milliseconds(ms) => {
                Timer::new(current, current.wrapping_add(self.ms_to_ticks(ms)), self)
            }
        }
    }
}
