//! Oscillator peripheral.
//!
//! \[low_frequency\] features should not be used if the flc is also in use
//! because the FLC_CLK needs to be 1MHz.

use max78000::gcr::CLKCTRL;
use max78000::trimsir::INRO;

/// Acceptable Internal Primary Oscillator frequency. Can be converted into a
/// u32 integer representing a value in hertz.
#[derive(Clone, Copy, Default)]
pub enum IpoFrequency {
    /// 100 megahertz
    #[default]
    _100MHz,
}

impl From<IpoFrequency> for u32 {
    fn from(val: IpoFrequency) -> Self {
        match val {
            IpoFrequency::_100MHz => 100_000_000,
        }
    }
}

/// Acceptable Internal Secondary Oscillator frequency. Can be converted into a
/// u32 integer representing a value in hertz.
#[derive(Clone, Copy, Default)]
pub enum IsoFrequency {
    /// 60 megahertz
    #[default]
    _60MHz,
}

impl From<IsoFrequency> for u32 {
    fn from(val: IsoFrequency) -> Self {
        match val {
            IsoFrequency::_60MHz => 60_000_000,
        }
    }
}

/// Acceptable Internal Nano-Ring Oscillator frequencies. Can be converted into
/// a u32 integer representing a value in hertz.
#[cfg(feature = "low_frequency")]
#[derive(Clone, Copy)]
pub enum InroFrequency {
    /// 8 kilohertz
    _8kHz,
    /// 16 kilohertz
    _16kHz,
    /// 30 kilohertz
    _30kHz,
}

#[cfg(feature = "low_frequency")]
impl From<InroFrequency> for u32 {
    fn from(val: InroFrequency) -> Self {
        match val {
            InroFrequency::_8kHz => 8_000,
            InroFrequency::_16kHz => 16_000,
            InroFrequency::_30kHz => 30_000,
        }
    }
}

/// Acceptable Internal Baud Rate Oscillator frequency. Can be converted into
/// a u32 integer representing a value in hertz.
#[derive(Clone, Copy, Default)]
pub enum IbroFrequency {
    /// 7.3728 megahertz
    #[default]
    _7_3728MHz,
}

impl From<IbroFrequency> for u32 {
    fn from(val: IbroFrequency) -> Self {
        match val {
            IbroFrequency::_7_3728MHz => 7_372_800,
        }
    }
}

/// Acceptable External Real-Time Clock Oscillator frequency. Can be converted into
/// a u32 integer representing a value in hertz.
#[cfg(feature = "low_frequency")]
#[derive(Clone, Copy, Default)]
pub enum ErtcoFrequency {
    /// 32.768 kilohertz
    #[default]
    _32_768kHz,
}

#[cfg(feature = "low_frequency")]
impl From<ErtcoFrequency> for u32 {
    fn from(val: ErtcoFrequency) -> Self {
        match val {
            ErtcoFrequency::_32_768kHz => 32_768,
        }
    }
}

#[derive(Clone, Copy)]
#[allow(missing_docs)]
#[non_exhaustive]
/// Acceptable Internal Primary Oscillator dividers.
/// Can be converted into a u8 integer.
pub enum IpoDivider {
    _1 = 1,
    _2 = 2,
    _4 = 4,
    _8 = 8,
    _16 = 16,
    _32 = 32,
    _64 = 64,
    #[cfg(feature = "low_frequency")]
    _128 = 128,
}

impl From<IpoDivider> for u8 {
    fn from(val: IpoDivider) -> Self {
        match val {
            IpoDivider::_1 => 1,
            IpoDivider::_2 => 2,
            IpoDivider::_4 => 4,
            IpoDivider::_8 => 8,
            IpoDivider::_16 => 16,
            IpoDivider::_32 => 32,
            IpoDivider::_64 => 64,
            #[cfg(feature = "low_frequency")]
            IpoDivider::_128 => 128,
        }
    }
}

#[derive(Clone, Copy)]
#[allow(missing_docs)]
/// Acceptable Internal Secondary Oscillator dividers.
/// Can be converted into a u8 integer.
pub enum IsoDivider {
    _1 = 1,
    _2 = 2,
    _4 = 4,
    _8 = 8,
    _16 = 16,
    _32 = 32,
    #[cfg(feature = "low_frequency")]
    _64 = 64,
    #[cfg(feature = "low_frequency")]
    _128 = 128,
}

impl From<IsoDivider> for u8 {
    fn from(val: IsoDivider) -> Self {
        match val {
            IsoDivider::_1 => 1,
            IsoDivider::_2 => 2,
            IsoDivider::_4 => 4,
            IsoDivider::_8 => 8,
            IsoDivider::_16 => 16,
            IsoDivider::_32 => 32,
            #[cfg(feature = "low_frequency")]
            IsoDivider::_64 => 64,
            #[cfg(feature = "low_frequency")]
            IsoDivider::_128 => 128,
        }
    }
}

#[derive(Clone, Copy)]
#[allow(missing_docs)]
/// Acceptable Internal Baud Rate Oscillator dividers.
/// Can be converted into a u8 integer.
pub enum IbroDivider {
    _1 = 1,
    _2 = 2,
    _4 = 4,
    #[cfg(feature = "low_frequency")]
    _8 = 8,
    #[cfg(feature = "low_frequency")]
    _16 = 16,
    #[cfg(feature = "low_frequency")]
    _32 = 32,
    #[cfg(feature = "low_frequency")]
    _64 = 64,
    #[cfg(feature = "low_frequency")]
    _128 = 128,
}

impl From<IbroDivider> for u8 {
    fn from(val: IbroDivider) -> Self {
        match val {
            IbroDivider::_1 => 1,
            IbroDivider::_2 => 2,
            IbroDivider::_4 => 4,
            #[cfg(feature = "low_frequency")]
            IbroDivider::_8 => 8,
            #[cfg(feature = "low_frequency")]
            IbroDivider::_16 => 16,
            #[cfg(feature = "low_frequency")]
            IbroDivider::_32 => 32,
            #[cfg(feature = "low_frequency")]
            IbroDivider::_64 => 64,
            #[cfg(feature = "low_frequency")]
            IbroDivider::_128 => 128,
        }
    }
}

#[cfg(feature = "low_frequency")]
#[derive(Clone, Copy)]
#[allow(missing_docs)]
/// Acceptable Internal Nano Ring Oscillator dividers.
/// Can be converted into a u8 integer.
pub enum InroDivider {
    _1 = 1,
    _2 = 2,
    _4 = 4,
    _8 = 8,
    _16 = 16,
    _32 = 32,
    _64 = 64,
    _128 = 128,
}

#[cfg(feature = "low_frequency")]
impl From<InroDivider> for u8 {
    fn from(val: InroDivider) -> Self {
        match val {
            InroDivider::_1 => 1,
            #[cfg(feature = "low_frequency")]
            InroDivider::_2 => 2,
            #[cfg(feature = "low_frequency")]
            InroDivider::_4 => 4,
            #[cfg(feature = "low_frequency")]
            InroDivider::_8 => 8,
            #[cfg(feature = "low_frequency")]
            InroDivider::_16 => 16,
            #[cfg(feature = "low_frequency")]
            InroDivider::_32 => 32,
            #[cfg(feature = "low_frequency")]
            InroDivider::_64 => 64,
            #[cfg(feature = "low_frequency")]
            InroDivider::_128 => 128,
        }
    }
}

#[cfg(feature = "low_frequency")]
/// Acceptable External Real Time Clock dividers.
/// Can be converted into a u8 integer.
pub type ErtcoDivider = InroDivider;

/// The SystemClock struct is the userfacing api to configure the onboard system
/// clock. It has access to the GCR's clkctrl registers and the TRIMSIR's inro
/// registers. The GCR's clkctrl register is used turn on certain oscillators and
/// select the system oscillator. The TRIMSIR's inro register is used to configure
/// the frequency of the inro.
pub struct SystemClock<'a, 'b> {
    /// Reference to the clkctrl register from the GCR
    gcr_clkctrl_register: &'a CLKCTRL,
    /// Reference to the inro register from the TRIMSIR
    trimsir_inro_register: &'b INRO,
    /// The current SYS_OSC frequency
    clock_frequency: u32,
    /// The current SYS_OSC divider
    clock_divider: u8,
}

impl<'a, 'b> SystemClock<'a, 'b> {
    /// The SystemClock constructor takes in a user defined Oscillator object
    /// and references to the GCR's clkctrl register block and the TRIMSIR's
    /// inro register block. The constructor defines current system clock's
    /// frequency and divider. In addition it sets the system oscillator to the
    /// desired oscillator using the SystemClock's set_sysclk function.
    /// # Example
    /// ```
    /// let ipo = Ipo::new(IpoFrequency::_100MHz, IpoDivider::_1);
    /// let sys_clk = SystemClock::new(&ipo, clkctrl_peripheral, trimsir_peripheral);
    /// ```
    pub(crate) fn new<T: Oscillator + private::Oscillator>(
        osc: &T,
        gcr_clkctrl_peripheral: &'a CLKCTRL,
        trimsir_inro_peripheral: &'b INRO,
    ) -> Self {
        let mut new_sysclk = Self {
            gcr_clkctrl_register: gcr_clkctrl_peripheral,
            trimsir_inro_register: trimsir_inro_peripheral,
            clock_frequency: osc.get_freq().into(),
            clock_divider: osc.get_div().into(),
        };

        new_sysclk.set_sysclk(osc);
        new_sysclk
    }

    /// Sets the desired oscillator as the system oscillator using the
    /// set_sysclk function of the oscillator type. In addition, it updates the
    /// clock_frequency and clock_divider fields of the SystemClock struct.
    pub fn set_sysclk<T: Oscillator + private::Oscillator>(&mut self, osc: &T) {
        osc.set_sysclk(self.gcr_clkctrl_register);
        osc.set_divider(self.gcr_clkctrl_register, self.trimsir_inro_register);
        self.clock_frequency = osc.get_freq().into();
        self.clock_divider = osc.get_div().into();
    }

    /// Returns the clock divider of the SYS_OSC
    pub fn get_div(&self) -> u8 {
        self.clock_divider
    }

    /// Returns the frequency of the SYS_OSC in hertz
    pub fn get_freq(&self) -> u32 {
        self.clock_frequency
    }
}

/// Oscillator trait that describes the needed functionality of a oscillator type
pub trait Oscillator {
    /// Type representing acceptable frequency values of the oscillator
    type Frequency: Into<u32>;
    /// Type representing acceptable divider values of the oscillator
    type Divider: Into<u8>;

    /// Oscillator type constructor
    fn new(frequency: Self::Frequency, divider: Self::Divider) -> Self;
    /// Returns the frequency of the oscillator
    fn get_freq(&self) -> Self::Frequency;
    /// Returns the divider of the system clock
    fn get_div(&self) -> Self::Divider;
}

pub(crate) mod private {
    use max78000::gcr::CLKCTRL;
    use max78000::trimsir::INRO;

    pub trait Oscillator {
        /// Type representing acceptable frequency values of the oscillator
        type Frequency: Into<u32>;
        /// Type representing acceptable divider values of the oscillator
        type Divider: Into<u8>;

        /// Sets the bits in the GCR clkctrl register to enable the oscillitor
        fn enable(&self, gcr_clkctrl: &CLKCTRL);
        /// Sets the bits in the GCR clkctrl register to select the oscillitor as
        /// the system oscillator used by the system clock. If the oscillator is not
        /// enable, this function enables it
        fn set_sysclk(&self, gcr_clkctrl: &CLKCTRL);
        /// Sets the bits in the GCR clkctrl register to select the clock divider and frequency
        fn set_divider(&self, gcr_clkctrl: &CLKCTRL, trimsir_inro: &INRO);
    }
}
/// The Internal Primary Oscillator structure which conforms to the oscillator trait
pub struct Ipo {
    frequency: IpoFrequency,
    divider: IpoDivider,
}

impl Oscillator for Ipo {
    type Frequency = IpoFrequency;
    type Divider = IpoDivider;

    fn new(frequency: IpoFrequency, divider: IpoDivider) -> Self {
        Self { frequency, divider }
    }

    fn get_div(&self) -> Self::Divider {
        self.divider
    }

    fn get_freq(&self) -> Self::Frequency {
        self.frequency
    }
}

impl private::Oscillator for Ipo {
    type Frequency = IpoFrequency;
    type Divider = IpoDivider;

    fn enable(&self, gcr_clkctrl: &CLKCTRL) {
        gcr_clkctrl.modify(|_, w| w.ipo_en().en());
        while !gcr_clkctrl.read().ipo_rdy().bit_is_set() {}
    }

    fn set_sysclk(&self, gcr_clkctrl: &CLKCTRL) {
        self.enable(gcr_clkctrl);
        gcr_clkctrl.modify(|_, w| w.sysclk_sel().ipo());
        while !gcr_clkctrl.read().sysclk_rdy().bit_is_set() {}
    }

    fn set_divider(&self, gcr_clkctrl: &CLKCTRL, _trimsir_inro: &INRO) {
        match self.divider {
            IpoDivider::_1 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div1());
            }
            IpoDivider::_2 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div2());
            }
            IpoDivider::_4 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div4());
            }
            IpoDivider::_8 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div8());
            }
            IpoDivider::_16 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div16());
            }
            IpoDivider::_32 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div32());
            }
            IpoDivider::_64 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div64());
            }
            #[cfg(feature = "low_frequency")]
            IpoDivider::_128 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div128());
            }
        }
    }
}

/// The Internal Secondary Oscillator structure which conforms to the oscillator trait
pub struct Iso {
    frequency: IsoFrequency,
    divider: IsoDivider,
}

impl Oscillator for Iso {
    type Frequency = IsoFrequency;
    type Divider = IsoDivider;

    fn new(frequency: IsoFrequency, divider: IsoDivider) -> Self {
        Self { frequency, divider }
    }

    fn get_div(&self) -> Self::Divider {
        self.divider
    }

    fn get_freq(&self) -> Self::Frequency {
        self.frequency
    }
}

impl private::Oscillator for Iso {
    type Frequency = IsoFrequency;
    type Divider = IsoDivider;

    fn enable(&self, gcr_clkctrl: &CLKCTRL) {
        gcr_clkctrl.modify(|_, w| w.iso_en().en());
        while !gcr_clkctrl.read().iso_rdy().bit_is_set() {}
    }

    fn set_sysclk(&self, gcr_clkctrl: &CLKCTRL) {
        self.enable(gcr_clkctrl);
        gcr_clkctrl.modify(|_, w| w.sysclk_sel().iso());
        while !gcr_clkctrl.read().sysclk_rdy().bit_is_set() {}
    }

    fn set_divider(&self, gcr_clkctrl: &CLKCTRL, _trimsir_inro: &INRO) {
        match self.divider {
            IsoDivider::_1 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div1());
            }
            IsoDivider::_2 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div2());
            }
            IsoDivider::_4 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div4());
            }
            IsoDivider::_8 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div8());
            }
            IsoDivider::_16 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div16());
            }
            IsoDivider::_32 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div32());
            }
            #[cfg(feature = "low_frequency")]
            IsoDivider::_64 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div64());
            }
            #[cfg(feature = "low_frequency")]
            IsoDivider::_128 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div128());
            }
        }
    }
}

/// The Internal Baud Rate Oscillator structure which conforms to the oscillator trait
pub struct Ibro {
    frequency: IbroFrequency,
    divider: IbroDivider,
}

impl Oscillator for Ibro {
    type Frequency = IbroFrequency;
    type Divider = IbroDivider;

    fn new(frequency: IbroFrequency, divider: IbroDivider) -> Self {
        Self { frequency, divider }
    }

    fn get_div(&self) -> Self::Divider {
        self.divider
    }

    fn get_freq(&self) -> Self::Frequency {
        self.frequency
    }
}

impl private::Oscillator for Ibro {
    type Frequency = IbroFrequency;
    type Divider = IbroDivider;

    fn enable(&self, gcr_clkctrl: &CLKCTRL) {
        while !gcr_clkctrl.read().ibro_rdy().bit_is_set() {}
    }

    fn set_sysclk(&self, gcr_clkctrl: &CLKCTRL) {
        self.enable(gcr_clkctrl);
        gcr_clkctrl.modify(|_, w| w.sysclk_sel().ibro());
        while !gcr_clkctrl.read().sysclk_rdy().bit_is_set() {}
    }

    fn set_divider(&self, gcr_clkctrl: &CLKCTRL, _trimsir_inro: &INRO) {
        match self.divider {
            IbroDivider::_1 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div1());
            }
            IbroDivider::_2 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div2());
            }
            IbroDivider::_4 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div4());
            }
            #[cfg(feature = "low_frequency")]
            IbroDivider::_8 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div8());
            }
            #[cfg(feature = "low_frequency")]
            IbroDivider::_16 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div16());
            }
            #[cfg(feature = "low_frequency")]
            IbroDivider::_32 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div32());
            }
            #[cfg(feature = "low_frequency")]
            IbroDivider::_64 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div64());
            }
            #[cfg(feature = "low_frequency")]
            IbroDivider::_128 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div128());
            }
        }
    }
}

#[cfg(feature = "low_frequency")]
/// The Internal Nano Ring Oscillator structure which conforms to the oscillator trait
pub struct Inro {
    frequency: InroFrequency,
    divider: InroDivider,
}

#[cfg(feature = "low_frequency")]
impl Oscillator for Inro {
    type Frequency = InroFrequency;
    type Divider = InroDivider;

    fn new(frequency: InroFrequency, divider: InroDivider) -> Self {
        Self { frequency, divider }
    }

    fn get_div(&self) -> Self::Divider {
        self.divider
    }

    fn get_freq(&self) -> Self::Frequency {
        self.frequency
    }
}

#[cfg(feature = "low_frequency")]
impl private::Oscillator for Inro {
    type Frequency = InroFrequency;
    type Divider = InroDivider;

    fn enable(&self, gcr_clkctrl: &CLKCTRL) {
        while !gcr_clkctrl.read().inro_rdy().bit_is_set() {}
    }

    fn set_sysclk(&self, gcr_clkctrl: &CLKCTRL) {
        self.enable(gcr_clkctrl);
        gcr_clkctrl.modify(|_, w| w.sysclk_sel().inro());
        while !gcr_clkctrl.read().sysclk_rdy().bit_is_set() {}
    }

    fn set_divider(&self, gcr_clkctrl: &CLKCTRL, trimsir: &INRO) {
        match self.frequency {
            InroFrequency::_8kHz => {
                trimsir.modify(|_, w| w.lpclksel()._8khz());
            }
            InroFrequency::_16kHz => {
                trimsir.modify(|_, w| w.lpclksel()._16khz());
            }
            InroFrequency::_30kHz => {
                trimsir.modify(|_, w| w.lpclksel()._30khz());
            }
        }

        match self.divider {
            InroDivider::_1 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div1());
            }
            InroDivider::_2 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div2());
            }
            InroDivider::_4 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div4());
            }
            InroDivider::_8 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div8());
            }
            InroDivider::_16 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div16());
            }
            InroDivider::_32 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div32());
            }
            InroDivider::_64 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div64());
            }
            InroDivider::_128 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div128());
            }
        }
    }
}

#[cfg(feature = "low_frequency")]
/// The External Real Time Clock Oscillator structure which conforms to the oscillator trait
pub struct Ertco {
    frequency: ErtcoFrequency,
    divider: ErtcoDivider,
}

#[cfg(feature = "low_frequency")]
impl Oscillator for Ertco {
    type Frequency = ErtcoFrequency;
    type Divider = ErtcoDivider;

    fn new(frequency: ErtcoFrequency, divider: ErtcoDivider) -> Self {
        Self { frequency, divider }
    }

    fn get_div(&self) -> Self::Divider {
        self.divider
    }

    fn get_freq(&self) -> Self::Frequency {
        self.frequency
    }
}

#[cfg(feature = "low_frequency")]
impl private::Oscillator for Ertco {
    type Frequency = ErtcoFrequency;
    type Divider = ErtcoDivider;

    fn enable(&self, gcr_clkctrl: &CLKCTRL) {
        gcr_clkctrl.modify(|_, w| w.ertco_en().en());
        while !gcr_clkctrl.read().ertco_rdy().bit_is_set() {}
    }

    fn set_sysclk(&self, gcr_clkctrl: &CLKCTRL) {
        self.enable(gcr_clkctrl);
        gcr_clkctrl.modify(|_, w| w.sysclk_sel().ertco());
        while !gcr_clkctrl.read().sysclk_rdy().bit_is_set() {}
    }

    fn set_divider(&self, gcr_clkctrl: &CLKCTRL, _trimsir_inro: &INRO) {
        match self.divider {
            ErtcoDivider::_1 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div1());
            }
            ErtcoDivider::_2 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div2());
            }
            ErtcoDivider::_4 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div4());
            }
            ErtcoDivider::_8 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div8());
            }
            ErtcoDivider::_16 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div16());
            }
            ErtcoDivider::_32 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div32());
            }
            ErtcoDivider::_64 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div64());
            }
            ErtcoDivider::_128 => {
                gcr_clkctrl.modify(|_, w| w.sysclk_div().div128());
            }
        }
    }
}
