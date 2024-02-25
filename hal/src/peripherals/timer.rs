//! Peripheral API for Timers

use core::ops::Deref;
use max78000::gcr::clkctrl::ERTCO_EN_A;
use max78000::gcr::pclkdis0::GPIO0_A;
use max78000::gcr::rst0::RESET_A;
use max78000::tmr;
use max78000::tmr1::ctrl0::{CLKDIV_A_A, MODE_A_A};
use max78000::GCR;
use max78000::{TMR, TMR1, TMR2, TMR3};

// TODO: use peripheral API when done
/// Auxiliary trait that only the TMR, TMR1, TMR2, and TMR3 registers can implement;
/// Allows peripheral toggle and reset functionality to said peripherals if GCR regs
/// are provided.
pub trait TimerPeripheralGCR {
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
pub struct Clock<T: Sized + Deref<Target = tmr::RegisterBlock> + TimerPeripheralGCR> {
    tmr_registers: T,
    ticks_per_ms: f64,
}

/// Oscillator options for the Clock struct
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
pub struct Timer<'a, T: Sized + Deref<Target = tmr::RegisterBlock> + TimerPeripheralGCR> {
    /// Start timestamp
    pub start: u32,
    /// End timestamp
    pub end: u32,
    clock: &'a Clock<T>,
    finished: bool,
}

impl<'a, T: Sized + Deref<Target = tmr::RegisterBlock> + TimerPeripheralGCR> Timer<'a, T> {
    fn new(start: u32, end: u32, clock: &'a Clock<T>) -> Self {
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
    pub fn duration_ticks(&mut self) -> u32 {
        self.end.wrapping_sub(self.start)
    }

    /// Get total duration, in milliseconds
    pub fn duration_ms(&mut self) -> u32 {
        self.clock.ticks_to_ms(self.duration_ticks())
    }
}

/// Represents a time value for starting a timer
pub enum Time {
    /// Raw clock ticks, i.e, number of oscillations / prescaler
    Ticks(u32),
    /// Milliseconds
    Milliseconds(u32),
}

/// Prescaler values to divide oscillator by. Available in powers of two from `0` to `12`.
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

impl<T: Sized + Deref<Target = tmr::RegisterBlock> + TimerPeripheralGCR> Clock<T> {
    /// Creates a new `Clock`, taking ownership of the timer peripheral register block,
    /// a temporary reference to the GCR registers for initial configuration, as
    /// well as config values for the oscillator source and the prescaler value, which will divide
    /// the oscillator source to only increment count once per `prescaler` ticks.
    pub(crate) fn new(
        tmr_registers: T,
        gcr_registers: &GCR,
        oscillator: Oscillator,
        prescaler: Prescaler,
    ) -> Self {
        let mut this = Clock {
            tmr_registers,
            ticks_per_ms: 0f64,
        };

        // Disable timer
        this.tmr_registers
            .ctrl0()
            .modify(|_, w| w.en_a().variant(false));
        this.tmr_registers
            .ctrl0()
            .modify(|_, w| w.en_b().variant(false));
        while this.tmr_registers.ctrl1().read().clken_a().bit() {}
        while this.tmr_registers.ctrl1().read().clken_b().bit() {}

        // Configure for continuous mode
        this.tmr_registers.ctrl0().modify(|_, w| {
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
        this.tmr_registers.ctrl1().modify(|_, w| {
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

        this.ticks_per_ms = clks_per_ms / clkdiv;

        // Set time to repeat every 2^32 ticks (basically highest period possible)
        this.tmr_registers
            .cmp()
            .write(|w| w.compare().variant(0xffffffff));
        this.tmr_registers.cnt().write(|w| w.count().variant(1));

        // enable the timer clock
        this.tmr_registers
            .ctrl0()
            .modify(|_, w| w.clken_a().variant(true));
        while !this.tmr_registers.ctrl1().read().clkrdy_a().bit() {}
        this.tmr_registers
            .ctrl0()
            .modify(|_, w| w.en_a().variant(true));
        while !this.tmr_registers.ctrl0().read().clken_a().bit() {}

        this
    }

    /// Consume `Clock`, returning the underlying timer registers
    pub fn consume(self) -> T {
        self.tmr_registers
    }

    /// Return raw clk count val
    pub fn get_count(&self) -> u32 {
        self.tmr_registers.cnt().read().count().bits()
    }

    /// Convert milliseconds to ticks
    pub fn ms_to_ticks(&self, ms: u32) -> u32 {
        ((ms as f64) * self.ticks_per_ms) as u32
    }

    /// Convert ticks to milliseconds
    pub fn ticks_to_ms(&self, ticks: u32) -> u32 {
        (ticks as f64 / self.ticks_per_ms) as u32
    }

    /// Start a new timer with given `Time`, which can be expressed with either raw `Ticks`
    /// or `Milliseconds`, which will be converted into ticks internally.
    ///
    /// Caveat: Will only work reliably for durations of less than `2^31` ticks.
    pub fn new_timer(&self, duration: Time) -> Timer<T> {
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
