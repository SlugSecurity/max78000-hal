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

impl Into<u32> for IpoFrequency {
    fn into(self) -> u32 {
        match self {
            Self::_100MHz => 100_000_000,
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

impl Into<u32> for IsoFrequency {
    fn into(self) -> u32 {
        match self {
            Self::_60MHz => 60_000_000,
        }
    }
}

/// Acceptable Internal Nano-Ring Oscillator frequencies. Can be converted into
/// a u32 integer representing a value in hertz.
#[cfg(feature = "low_frequency")]
#[derive(Clone, Copy)]
pub enum InroFrequency {
    /// 9 kilohertz
    _8kHz,
    /// 16 kilohertz
    _16kHz,
    /// 30 kilohertz
    _30kHz,
}

#[cfg(feature = "low_frequency")]
impl Into<u32> for InroFrequency {
    fn into(self) -> u32 {
        match self {
            Self::_8kHz => 8_000,
            Self::_16kHz => 16_000,
            Self::_30kHz => 30_000,
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

impl Into<u32> for IbroFrequency {
    fn into(self) -> u32 {
        match self {
            Self::_7_3728MHz => 7_372_800,
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
impl Into<u32> for ErtcoFrequency {
    fn into(self) -> u32 {
        match self {
            Self::_32_768kHz => 32_768,
        }
    }
}

#[derive(Clone, Copy)]
#[allow(missing_docs)]
/// Acceptable Internal Primary Oscillator dividers. Can not set the divider
/// above 64 because that would make the clock signal to the flash controller
/// lower than 1Mhz.
/// Can be converted into a u8 integer.
pub enum IpoDivider {
    _1 = 1,
    _2 = 2,
    _4 = 4,
    _8 = 8,
    _16 = 16,
    _32 = 32,
    _64 = 64,
}

impl Into<u8> for IpoDivider {
    fn into(self) -> u8 {
        match self {
            Self::_1 => 1,
            Self::_2 => 2,
            Self::_4 => 4,
            Self::_8 => 8,
            Self::_16 => 16,
            Self::_32 => 32,
            Self::_64 => 64,
        }
    }
}

#[derive(Clone, Copy)]
#[allow(missing_docs)]
/// Acceptable Internal Secondary Oscillator dividers. Can not set the divider
/// above 32 because that would make the clock signal to the flash controller
/// lower than 1Mhz.
/// Can be converted into a u8 integer.
pub enum IsoDivider {
    _1 = 1,
    _2 = 2,
    _4 = 4,
    _8 = 8,
    _16 = 16,
    _32 = 32,
}

impl Into<u8> for IsoDivider {
    fn into(self) -> u8 {
        match self {
            Self::_1 => 1,
            Self::_2 => 2,
            Self::_4 => 4,
            Self::_8 => 8,
            Self::_16 => 16,
            Self::_32 => 32,
        }
    }
}

#[derive(Clone, Copy)]
#[allow(missing_docs)]
/// Acceptable Internal Baud Rate Oscillator dividers. Can not set the divider
/// above 4 because that would make the clock signal to the flash controller
/// lower than 1Mhz.
/// Can be converted into a u8 integer.
pub enum IbroDivider {
    _1 = 1,
    _2 = 2,
    _4 = 4,
}

impl Into<u8> for IbroDivider {
    fn into(self) -> u8 {
        match self {
            Self::_1 => 1,
            Self::_2 => 2,
            Self::_4 => 4,
        }
    }
}

#[cfg(feature = "low_frequency")]
#[derive(Clone, Copy, Default)]
#[allow(missing_docs)]
/// Acceptable Internal Nano Ring Oscillator dividers. The frequency of the INRO
/// is bellow 1MHz so it should never be used. The divider is set to 1, but this
/// is arbitrary.
/// Can be converted into a u8 integer.
pub enum InroDivider {
    #[default]
    _1 = 1,
}

#[cfg(feature = "low_frequency")]
impl Into<u8> for InroDivider {
    fn into(self) -> u8 {
        match self {
            Self::_1 => 1,
        }
    }
}

#[cfg(feature = "low_frequency")]
/// Acceptable External Real Time Clock dividers. The frequency of the INRO
/// is bellow 1MHz so it should never be used. The divider is set to 1, but this
/// is arbitrary.
/// Can be converted into a u8 integer.
pub type ErtcoDivider = InroDivider;

/// The SystemClock struct is the userfacing api to configure the onboard system
/// clock. It has access to the GCR's clkctrl registers and the TRIMSIR's inro
/// registers. The GCR's clkctrl register is used turn on certain oscillators and
/// select the system oscillator. The TRIMSIR's inro register is used to configure
/// the frequency of the inro.
pub struct SystemClock<'a> {
    clkctrl_peripheral: &'a CLKCTRL,
    trimsir_peripheral: &'a INRO,
    clock_frequency: u32,
    clock_divider: u8,
}

impl<'a> SystemClock<'a> {
    /// The SystemClock constructor takes in a user defined Oscillator object
    /// and references to the GCR's clkctrl register block and the TRIMSIR's
    /// inro register block. The constructor defines current system clock's
    /// frequency and divider. In addition it sets the system oscillator to the
    /// desired oscillator using the SystemClock's set_sysclk function.
    /// # Example
    /// ```
    /// let ipo = Ipo::new(IpoFrequency::_100Mhz, IpoDivider::_1);
    /// let sys_clk = SystemClock::new(&ipo, clkctrl_peripheral, trimsir_peripheral);
    /// ```
    pub fn new<T: Oscillator>(
        osc: &'a T,
        clkctrl_peripheral: &'a CLKCTRL,
        trimsir_peripheral: &'a INRO,
    ) -> Self {
        let mut new_sysclk = Self {
            clkctrl_peripheral,
            trimsir_peripheral,
            clock_frequency: osc.get_freq().into(),
            clock_divider: osc.get_div().into(),
        };

        new_sysclk.set_sysclk(osc);
        new_sysclk
    }

    /// Sets the desired oscillator as the system oscillator using the
    /// set_sysclk function of the oscillator type. In addition, it updates the
    /// clock_frequency and clock_divider fields of the SystemClock struct.
    pub fn set_sysclk<T: Oscillator>(&mut self, osc: &'a T) {
        osc.set_sysclk(self.clkctrl_peripheral);
        osc.set_divider(self.clkctrl_peripheral, self.trimsir_peripheral);
        self.clock_frequency = osc.get_freq().into();
        self.clock_divider = osc.get_div().into();
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
    /// Sets the bits in the GCR clkctrl register to select the oscillitor as
    /// the system oscillator used by the system clock
    fn set_sysclk(&self, clkctrl: &CLKCTRL);
    /// Sets the bits in the GCR clkctrl register to select the clock divider and frequency
    fn set_divider(&self, clkctrl: &CLKCTRL, trimsir: &INRO);
    /// Returns the frequency of the oscillator
    fn get_freq(&self) -> Self::Frequency;
    /// Returns the divider of the system clock
    fn get_div(&self) -> Self::Divider;
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

    fn set_sysclk(&self, clkctrl: &CLKCTRL) {
        clkctrl.modify(|r, w| {
            w.ipo_en().en();
            while r.ipo_rdy().bit_is_set() == false {}
            w
        });

        clkctrl.modify(|r, w| {
            w.sysclk_sel().ipo();
            while r.sysclk_rdy().bit_is_set() == false {}
            w
        });
    }

    fn set_divider(&self, clkctrl: &CLKCTRL, _trimsir: &INRO) {
        match self.divider {
            IpoDivider::_1 => {
                clkctrl.modify(|_, w| w.sysclk_div().div1());
            }
            IpoDivider::_2 => {
                clkctrl.modify(|_, w| w.sysclk_div().div2());
            }
            IpoDivider::_4 => {
                clkctrl.modify(|_, w| w.sysclk_div().div4());
            }
            IpoDivider::_8 => {
                clkctrl.modify(|_, w| w.sysclk_div().div8());
            }
            IpoDivider::_16 => {
                clkctrl.modify(|_, w| w.sysclk_div().div16());
            }
            IpoDivider::_32 => {
                clkctrl.modify(|_, w| w.sysclk_div().div32());
            }
            IpoDivider::_64 => {
                clkctrl.modify(|_, w| w.sysclk_div().div64());
            }
        }
    }

    fn get_div(&self) -> Self::Divider {
        self.divider
    }

    fn get_freq(&self) -> Self::Frequency {
        self.frequency
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

    fn set_sysclk(&self, clkctrl: &CLKCTRL) {
        clkctrl.modify(|r, w| {
            w.iso_en().en();
            while r.iso_rdy().bit_is_set() == false {}
            w
        });

        clkctrl.modify(|r, w| {
            w.sysclk_sel().iso();
            while r.sysclk_rdy().bit_is_set() == false {}
            w
        });
    }

    fn set_divider(&self, clkctrl: &CLKCTRL, _trimsir: &INRO) {
        match self.divider {
            IsoDivider::_1 => {
                clkctrl.modify(|_, w| w.sysclk_div().div1());
            }
            IsoDivider::_2 => {
                clkctrl.modify(|_, w| w.sysclk_div().div2());
            }
            IsoDivider::_4 => {
                clkctrl.modify(|_, w| w.sysclk_div().div4());
            }
            IsoDivider::_8 => {
                clkctrl.modify(|_, w| w.sysclk_div().div8());
            }
            IsoDivider::_16 => {
                clkctrl.modify(|_, w| w.sysclk_div().div16());
            }
            IsoDivider::_32 => {
                clkctrl.modify(|_, w| w.sysclk_div().div32());
            }
        }
    }

    fn get_div(&self) -> Self::Divider {
        self.divider
    }

    fn get_freq(&self) -> Self::Frequency {
        self.frequency
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

    fn set_sysclk(&self, clkctrl: &CLKCTRL) {
        clkctrl.modify(|r, w| {
            w.ibro_en().en();
            while r.ibro_rdy().bit_is_set() == false {}
            w
        });

        clkctrl.modify(|r, w| {
            w.sysclk_sel().ibro();
            while r.sysclk_rdy().bit_is_set() == false {}
            w
        });
    }

    fn set_divider(&self, clkctrl: &CLKCTRL, _trimsir: &INRO) {
        match self.divider {
            IbroDivider::_1 => {
                clkctrl.modify(|_, w| w.sysclk_div().div1());
            }
            IbroDivider::_2 => {
                clkctrl.modify(|_, w| w.sysclk_div().div2());
            }
            IbroDivider::_4 => {
                clkctrl.modify(|_, w| w.sysclk_div().div4());
            }
        }
    }

    fn get_div(&self) -> Self::Divider {
        self.divider
    }

    fn get_freq(&self) -> Self::Frequency {
        self.frequency
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

    fn set_sysclk(&self, clkctrl: &CLKCTRL) {
        clkctrl.modify(|r, w| {
            while r.inro_rdy().bit_is_set() == false {}
            w
        });

        clkctrl.modify(|r, w| {
            w.sysclk_sel().inro();
            while r.sysclk_rdy().bit_is_set() == false {}
            w
        });
    }

    fn set_divider(&self, clkctrl: &CLKCTRL, trimsir: &INRO) {
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
                clkctrl.modify(|_, w| w.sysclk_div().div1());
            }
        }
    }

    fn get_div(&self) -> Self::Divider {
        self.divider
    }

    fn get_freq(&self) -> Self::Frequency {
        self.frequency
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

    fn set_sysclk(&self, clkctrl: &CLKCTRL) {
        clkctrl.modify(|r, w| {
            while r.ertco_rdy().bit_is_set() == false {}
            w
        });

        clkctrl.modify(|r, w| {
            w.sysclk_sel().ertco();
            while r.sysclk_rdy().bit_is_set() == false {}
            w
        });
    }

    fn set_divider(&self, clkctrl: &CLKCTRL, _trimsir: &INRO) {
        match self.divider {
            ErtcoDivider::_1 => {
                clkctrl.modify(|_, w| w.sysclk_div().div1());
            }
        }
    }

    fn get_div(&self) -> Self::Divider {
        self.divider
    }

    fn get_freq(&self) -> Self::Frequency {
        self.frequency
    }
}
