//! Peripheral drivers for the MAX78000.
//!
//! This module contains the peripheral manager to interact with the various MAX78000
//! peripherals. See the below example for how to set it up and use it.
//!
//! # Example
//! ```
//! use max78000_hal::{
//!     max78000::Peripherals,
//!     peripherals::{
//!         oscillator::{Ipo, IpoDivider, IpoFrequency},
//!         timer::{Oscillator, Prescaler},
//!         PeripheralManagerBuilder, SplittablePeripheral,
//!     },
//! };
//!
//! let (to_consume, to_borrow, rem) = Peripherals::take().unwrap().split();
//!
//! // Builds a peripheral manager using Ipo as the system clock oscillator with
//! // the specified frequency and divider. Also configures the timers' oscillators
//! // and prescalers according to the values put below.
//! let manager = PeripheralManagerBuilder::<Ipo>::new(
//!     &to_borrow,
//!     to_consume,
//!     IpoFrequency::_100MHz,
//!     IpoDivider::_1,
//! )
//! .configure_timer_0(Oscillator::ERTCO, Prescaler::_1)
//! .configure_timer_1(Oscillator::IBRO, Prescaler::_512)
//! .configure_timer_2(Oscillator::ISO, Prescaler::_4096)
//! .build();
//!
//! // Gets the peripheral wrapper for timer 0.
//! let clock = manager.timer_0().unwrap();
//! let mut timer = clock.new_timer(Time::Milliseconds(3000));
//!
//! while !timer.poll() {}
//!
//! // 3 seconds has passed.
//! ```

use core::cell::{BorrowMutError, RefCell, RefMut};
use core::ops::{Deref, DerefMut};

use crate::peripherals::flash_controller::FlashController;
use crate::peripherals::oscillator::SystemClock;
use max78000::*;

use self::gpio::{new_gpio0, new_gpio1, new_gpio2, Gpio0, Gpio1, Gpio2};
use self::oscillator::{private, Oscillator};
use self::power::{PowerControl, ToggleableModule};
use self::timer::{Clock, Prescaler};
use self::trng::Trng;

// Embedded HAL peripherals.
pub mod adc;
pub mod delay;
pub mod gpio;
pub mod serial;
pub mod timer;
pub mod trng;
pub mod watchdog;

// Non embedded HAL peripherals.
pub mod aes;
pub mod bit_banding;
pub mod bootloader;
pub mod crc;
pub mod ecc;
pub mod flash_controller;
pub mod i2c;
pub mod i2c_bitbang;
pub mod oscillator;
pub mod power;
pub mod raw;
pub mod rtc;
pub mod synchronization;

/// The peripherals that are completely unused by the [`PeripheralManager].
pub struct RemainingPeripherals {
    /// ADC
    pub adc: ADC,
    /// AES
    pub aes: AES,
    /// AESKEYS
    pub aeskeys: AESKEYS,
    /// CAMERAIF
    pub cameraif: CAMERAIF,
    /// CRC
    pub crc: CRC,
    /// DMA
    pub dma: DMA,
    /// DVS
    pub dvs: DVS,
    /// FCR
    pub fcr: FCR,
    /// GCFR
    pub gcfr: GCFR,
    /// I2C0
    pub i2c0: I2C0,
    /// I2C1
    pub i2c1: I2C1,
    /// I2C2
    pub i2c2: I2C2,
    /// I2S
    pub i2s: I2S,
    /// LPCMP
    pub lpcmp: LPCMP,
    /// OWM
    pub owm: OWM,
    /// PT
    pub pt: PT,
    /// PT1
    pub pt1: PT1,
    /// PT2
    pub pt2: PT2,
    /// PT3
    pub pt3: PT3,
    /// PTG
    pub ptg: PTG,
    /// PWRSEQ
    pub pwrseq: PWRSEQ,
    /// RTC
    pub rtc: RTC,
    /// SEMA
    pub sema: SEMA,
    /// SIMO
    pub simo: SIMO,
    /// SIR
    pub sir: SIR,
    /// SPI0
    pub spi0: SPI0,
    /// SPI1
    pub spi1: SPI1,
    /// TMR4
    pub tmr4: TMR4,
    /// TMR5
    pub tmr5: TMR5,
    /// UART
    pub uart: UART,
    /// UART1
    pub uart1: UART1,
    /// UART2
    pub uart2: UART2,
    /// UART3
    pub uart3: UART3,
    /// WDT
    pub wdt: WDT,
    /// WDT1
    pub wdt1: WDT1,
    /// WUT
    pub wut: WUT,
}

/// The peripherals that are immutably borrowed by the [`PeripheralManager].
pub struct PeripheralsToBorrow {
    /// GCR
    pub gcr: GCR,
    /// LPGCR
    pub lpgcr: LPGCR,
    /// MCR
    pub mcr: MCR,
    /// ICC0
    pub icc0: ICC0,
    /// TRIMSIR
    pub trimsir: TRIMSIR,
}

/// The peripherals that are completely consumed and moved by the [`PeripheralManager].
pub struct PeripheralsToConsume {
    flc: FLC,
    gpio0: GPIO0,
    gpio1: GPIO1,
    gpio2: GPIO2,
    trng: TRNG,
    tmr0: TMR,
    tmr1: TMR1,
    tmr2: TMR2,
    tmr3: TMR3,
}

/// Extension trait for splitting peripherals for the [`PeripheralManager`].
pub trait SplittablePeripheral {
    /// Splits the peripherals into three parts:
    /// - the peripherals that are consumed by the [`PeripheralManager`]
    /// - the peripherals that are borrowed by the [`PeripheralManager`]
    /// - the remaining peripherals not borrowed or consumed
    fn split(
        self,
    ) -> (
        PeripheralsToConsume,
        PeripheralsToBorrow,
        RemainingPeripherals,
    );
}

impl SplittablePeripheral for Peripherals {
    fn split(
        self,
    ) -> (
        PeripheralsToConsume,
        PeripheralsToBorrow,
        RemainingPeripherals,
    ) {
        let to_consume = PeripheralsToConsume {
            flc: self.FLC,
            gpio0: self.GPIO0,
            gpio1: self.GPIO1,
            gpio2: self.GPIO2,
            trng: self.TRNG,
            tmr0: self.TMR,
            tmr1: self.TMR1,
            tmr2: self.TMR2,
            tmr3: self.TMR3,
        };

        let to_borrow = PeripheralsToBorrow {
            gcr: self.GCR,
            lpgcr: self.LPGCR,
            mcr: self.MCR,
            icc0: self.ICC0,
            trimsir: self.TRIMSIR,
        };

        let remaining = RemainingPeripherals {
            adc: self.ADC,
            aes: self.AES,
            aeskeys: self.AESKEYS,
            cameraif: self.CAMERAIF,
            crc: self.CRC,
            dma: self.DMA,
            dvs: self.DVS,
            fcr: self.FCR,
            gcfr: self.GCFR,
            i2c0: self.I2C0,
            i2c1: self.I2C1,
            i2c2: self.I2C2,
            i2s: self.I2S,
            lpcmp: self.LPCMP,
            owm: self.OWM,
            pt: self.PT,
            pt1: self.PT1,
            pt2: self.PT2,
            pt3: self.PT3,
            ptg: self.PTG,
            pwrseq: self.PWRSEQ,
            rtc: self.RTC,
            sema: self.SEMA,
            simo: self.SIMO,
            sir: self.SIR,
            spi0: self.SPI0,
            spi1: self.SPI1,
            tmr4: self.TMR4,
            tmr5: self.TMR5,
            uart: self.UART,
            uart1: self.UART1,
            uart2: self.UART2,
            uart3: self.UART3,
            wdt: self.WDT,
            wdt1: self.WDT1,
            wut: self.WUT,
        };

        (to_consume, to_borrow, remaining)
    }
}

/// A handle to a peripheral wrapper. Only one handle can be taken out
/// at a time for a given peripheral wrapper from the HAL. Once it's
/// dropped, it can be taken out again.
pub struct PeripheralHandle<'a, T>(RefMut<'a, T>);

impl<'a, T> PeripheralHandle<'a, T> {
    pub(crate) fn new(cell: &'a RefCell<T>) -> Result<Self, BorrowMutError> {
        Ok(Self(cell.try_borrow_mut()?))
    }
}

impl<'a, T> Deref for PeripheralHandle<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T> DerefMut for PeripheralHandle<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A builder for the [`PeripheralManager]. This builder can be used to configure
/// the system clock frequency and divider along with timer oscillators and prescalers.
pub struct PeripheralManagerBuilder<'a, T: Oscillator + private::Oscillator> {
    borrowed_periphs: &'a PeripheralsToBorrow,
    consumed_periphs: PeripheralsToConsume,
    sysclk_osc_freq: <T as oscillator::Oscillator>::Frequency,
    sysclk_osc_div: <T as oscillator::Oscillator>::Divider,
    timer_0_cfg: (timer::Oscillator, Prescaler),
    timer_1_cfg: (timer::Oscillator, Prescaler),
    timer_2_cfg: (timer::Oscillator, Prescaler),
    timer_3_cfg: (timer::Oscillator, Prescaler),
}

macro_rules! timer_field {
    ($self:ident, $tmr_field:ident, $cfg_field:ident) => {
        RefCell::new(Clock::new(
            $self.consumed_periphs.$tmr_field,
            &$self.borrowed_periphs.gcr,
            $self.$cfg_field.0,
            $self.$cfg_field.1,
        ))
    };
}

/// Creates a method to configure a timer with the given function
/// name and field name. Must be put inside an impl block.
macro_rules! timer_fn {
    ($fn_name:ident, $field:ident) => {
        /// Configures the timer with the specified oscillator and prescaler.
        pub fn $fn_name(mut self, osc: timer::Oscillator, prescaler: Prescaler) -> Self {
            self.$field = (osc, prescaler);

            self
        }
    };
}

impl<'a, T: Oscillator + private::Oscillator> PeripheralManagerBuilder<'a, T> {
    /// Creates a new [`PeripheralManagerBuilder`] with the system clock configured
    /// to the desired oscillator, frequency, and divider. All oscillators used
    /// for timers are configured to [`timer::Oscillator::IBRO`] with a prescaler of
    /// [`Prescaler::_1`]. These timers can be configured individually through the
    /// appropriate `configure_*` methods.
    pub fn new(
        borrowed_periphs: &'a PeripheralsToBorrow,
        consumed_periphs: PeripheralsToConsume,
        freq: <T as oscillator::Oscillator>::Frequency,
        div: <T as oscillator::Oscillator>::Divider,
    ) -> PeripheralManagerBuilder<T> {
        PeripheralManagerBuilder {
            borrowed_periphs,
            consumed_periphs,
            sysclk_osc_freq: freq,
            sysclk_osc_div: div,
            timer_0_cfg: (timer::Oscillator::IBRO, Prescaler::_1),
            timer_1_cfg: (timer::Oscillator::IBRO, Prescaler::_1),
            timer_2_cfg: (timer::Oscillator::IBRO, Prescaler::_1),
            timer_3_cfg: (timer::Oscillator::IBRO, Prescaler::_1),
        }
    }

    /// Changes the system clock oscillator along with its frequency
    /// and divider.
    pub fn configure_sysclk<O: Oscillator + private::Oscillator>(
        self,
        sysclk_osc_freq: <O as oscillator::Oscillator>::Frequency,
        sysclk_osc_div: <O as oscillator::Oscillator>::Divider,
    ) -> PeripheralManagerBuilder<'a, O> {
        PeripheralManagerBuilder {
            borrowed_periphs: self.borrowed_periphs,
            consumed_periphs: self.consumed_periphs,
            sysclk_osc_freq,
            sysclk_osc_div,
            timer_0_cfg: self.timer_0_cfg,
            timer_1_cfg: self.timer_1_cfg,
            timer_2_cfg: self.timer_2_cfg,
            timer_3_cfg: self.timer_3_cfg,
        }
    }

    timer_fn!(configure_timer_0, timer_0_cfg);
    timer_fn!(configure_timer_1, timer_1_cfg);
    timer_fn!(configure_timer_2, timer_2_cfg);
    timer_fn!(configure_timer_3, timer_3_cfg);

    /// Builds the [`PeripheralManager`] given the system clock
    /// oscillator settings along with the timer settings.
    pub fn build(self) -> PeripheralManager<'a> {
        // TODO: Lazily initialize timers
        //       For now, they're eagerly intialized.
        let power_ctrl =
            PowerControl::new(&self.borrowed_periphs.gcr, &self.borrowed_periphs.lpgcr);

        // Timers are eagerly initialized because they are configured upon creation of a Clock.
        power_ctrl.enable_peripheral(ToggleableModule::TMR0);
        power_ctrl.enable_peripheral(ToggleableModule::TMR1);
        power_ctrl.enable_peripheral(ToggleableModule::TMR2);
        power_ctrl.enable_peripheral(ToggleableModule::TMR3);

        power_ctrl.reset_toggleable(ToggleableModule::TMR0);
        power_ctrl.reset_toggleable(ToggleableModule::TMR1);
        power_ctrl.reset_toggleable(ToggleableModule::TMR2);
        power_ctrl.reset_toggleable(ToggleableModule::TMR3);

        // GPIO ports are eagerly initialized because they do not use `PeripheralHandle`s.
        power_ctrl.enable_peripheral(ToggleableModule::GPIO0);
        power_ctrl.enable_peripheral(ToggleableModule::GPIO1);
        power_ctrl.enable_peripheral(ToggleableModule::GPIO2);

        power_ctrl.reset_toggleable(ToggleableModule::GPIO0);
        power_ctrl.reset_toggleable(ToggleableModule::GPIO1);
        power_ctrl.reset_toggleable(ToggleableModule::GPIO2);

        PeripheralManager {
            power_ctrl,
            flash_controller: RefCell::new(FlashController::new(
                self.consumed_periphs.flc,
                &self.borrowed_periphs.icc0,
                &self.borrowed_periphs.gcr,
            )),
            system_clock: RefCell::new(SystemClock::new(
                &T::new(self.sysclk_osc_freq, self.sysclk_osc_div),
                self.borrowed_periphs.gcr.clkctrl(),
                self.borrowed_periphs.trimsir.inro(),
            )),
            timer_0: timer_field!(self, tmr0, timer_0_cfg),
            timer_1: timer_field!(self, tmr1, timer_1_cfg),
            timer_2: timer_field!(self, tmr2, timer_2_cfg),
            timer_3: timer_field!(self, tmr3, timer_3_cfg),
            gpio0: new_gpio0(self.consumed_periphs.gpio0),
            gpio1: new_gpio1(self.consumed_periphs.gpio1),
            gpio2: new_gpio2(self.consumed_periphs.gpio2),
            trng: RefCell::new(Trng::new(self.consumed_periphs.trng)),
        }
    }
}

macro_rules! no_enable_rst_periph_fn {
    ($fn_name:ident, $p_type:ty, $field_name:ident) => {
        /// Gets the specified peripheral if not already taken elsewhere. Otherwise,
        /// returns [`BorrowMutError`].
        pub fn $fn_name(&'a self) -> Result<PeripheralHandle<$p_type>, BorrowMutError> {
            PeripheralHandle::new(&self.$field_name)
        }
    };
}

macro_rules! no_enable_rst_periph_fn_no_handle {
    ($fn_name:ident, $p_type:ty, $field_name:ident) => {
        /// Gets the specified peripheral, enabling and resetting it.
        pub fn $fn_name(&'a self) -> &$p_type {
            &self.$field_name
        }
    };
}

macro_rules! enable_rst_periph_fn {
    ($fn_name:ident, $p_type:ty, $field_name:ident, $variant:expr) => {
        /// Gets the specified peripheral if not already taken elsewhere, enabling and
        /// resetting it. Otherwise, returns [`BorrowMutError`].
        pub fn $fn_name(&self) -> Result<PeripheralHandle<$p_type>, BorrowMutError> {
            let handle = PeripheralHandle::new(&self.$field_name)?;
            self.power_ctrl.enable_peripheral($variant);
            self.power_ctrl.reset_toggleable($variant);
            Ok(handle)
        }
    };
}

/// The peripheral manager containing all the peripheral abstractions provided by the HAL.
/// Use [`PeripheralManagerBuilder`] to construct an instance of [`PeripheralManager`].
/// The methods inside here can be used to interact with the board peripherals.
pub struct PeripheralManager<'a> {
    power_ctrl: PowerControl<'a, 'a>,
    flash_controller: RefCell<FlashController<'a, 'a>>,
    system_clock: RefCell<SystemClock<'a, 'a>>,
    gpio0: Gpio0,
    gpio1: Gpio1,
    gpio2: Gpio2,
    timer_0: RefCell<Clock<TMR>>,
    timer_1: RefCell<Clock<TMR1>>,
    timer_2: RefCell<Clock<TMR2>>,
    timer_3: RefCell<Clock<TMR3>>,
    trng: RefCell<Trng>,
}

impl<'a> PeripheralManager<'a> {
    no_enable_rst_periph_fn!(flash_controller, FlashController<'_, '_>, flash_controller);
    no_enable_rst_periph_fn!(system_clock, SystemClock<'_, '_>, system_clock);

    // Timers CANNOT be enabled and reset again after creation because
    // Clock holds state for it
    no_enable_rst_periph_fn!(timer_0, Clock<TMR>, timer_0);
    no_enable_rst_periph_fn!(timer_1, Clock<TMR1>, timer_1);
    no_enable_rst_periph_fn!(timer_2, Clock<TMR2>, timer_2);
    no_enable_rst_periph_fn!(timer_3, Clock<TMR3>, timer_3);

    no_enable_rst_periph_fn_no_handle!(gpio0, Gpio0, gpio0);
    no_enable_rst_periph_fn_no_handle!(gpio1, Gpio1, gpio1);
    no_enable_rst_periph_fn_no_handle!(gpio2, Gpio2, gpio2);

    enable_rst_periph_fn!(trng, Trng, trng, ToggleableModule::TRNG);
}
