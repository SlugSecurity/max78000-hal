//! Peripheral drivers for the MAX78000.

use core::cell::{BorrowMutError, RefCell, RefMut};
use core::ops::Deref;

use crate::peripherals::flash_controller::FlashController;
use crate::peripherals::oscillator::SystemClock;
use max78000::*;

use self::oscillator::{private, Oscillator};
use self::power::{PowerControl, ToggleableModule};
use self::timer::{Clock, Prescaler};
use self::trng::Trng;

// Embedded HAL peripherals.
pub mod adc;
pub mod delay;
pub mod digital;
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
pub mod oscillator;
pub mod power;
pub mod raw;
pub mod rtc;
pub mod synchronization;

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

pub trait SplittablePeripheral {
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

macro_rules! timer_fn {
    ($fn_name:ident, $field:ident) => {
        /// Configures the timer with the specified oscillator and prescaler.
        pub fn $fn_name(&mut self, osc: timer::Oscillator, prescaler: Prescaler) -> &mut Self {
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
    /// appropriate [`configure_*`] methods.
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

    pub fn build(self) -> PeripheralManager<'a> {
        PeripheralManager {
            power_ctrl: PowerControl::new(&self.borrowed_periphs.gcr, &self.borrowed_periphs.lpgcr),
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

macro_rules! enable_rst_periph_fn {
    ($fn_name:ident, $p_type:ty, $field_name:ident, $variant:expr) => {
        /// Gets the specified peripheral if not already taken elsewhere, enabling and
        /// resetting it. Otherwise, returns [`BorrowMutError`].
        pub fn $fn_name(&'a self) -> Result<PeripheralHandle<$p_type>, BorrowMutError> {
            self.power_ctrl.enable_peripheral($variant);
            self.power_ctrl.reset_toggleable($variant);

            PeripheralHandle::new(&self.$field_name)
        }
    };
}

/// The peripheral manager containing all the peripheral abstractions provided by the HAL.
/// Use [`PeripheralManagerBuilder`] to construct an instance of [`PeripheralManager`].
pub struct PeripheralManager<'a> {
    power_ctrl: PowerControl<'a, 'a>,
    flash_controller: RefCell<FlashController<'a, 'a>>,
    system_clock: RefCell<SystemClock<'a, 'a>>,
    timer_0: RefCell<Clock<TMR>>,
    timer_1: RefCell<Clock<TMR1>>,
    timer_2: RefCell<Clock<TMR2>>,
    timer_3: RefCell<Clock<TMR3>>,
    trng: RefCell<Trng>,
}

impl<'a> PeripheralManager<'a> {
    no_enable_rst_periph_fn!(flash_controller, FlashController<'_, '_>, flash_controller);
    no_enable_rst_periph_fn!(system_clock, SystemClock<'_, '_>, system_clock);
    enable_rst_periph_fn!(timer_0, Clock<TMR>, timer_0, ToggleableModule::TMR0);
    enable_rst_periph_fn!(timer_1, Clock<TMR1>, timer_1, ToggleableModule::TMR1);
    enable_rst_periph_fn!(timer_2, Clock<TMR2>, timer_2, ToggleableModule::TMR2);
    enable_rst_periph_fn!(timer_3, Clock<TMR3>, timer_3, ToggleableModule::TMR3);
    enable_rst_periph_fn!(trng, Trng, trng, ToggleableModule::TRNG);
}
