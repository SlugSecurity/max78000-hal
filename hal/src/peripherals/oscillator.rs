use crate::peripherals::bit_banding as bb;
use max78000::FLC;
use max78000::GCR;

#[cfg(feature = "low_frequency")]
use max78000::TRIMSIR;

#[derive(Clone, Copy)]
/// All acceptable oscillators configurations
pub enum Oscillator {
    /// 100 mHz
    Primary(IpoFrequency),
    /// 60 mHz
    Secondary(IsoFrequency),
    /// Warning: if you decide to use the INRO, you need to set the daplink
    /// adapter speed to the lowest frequency you are testing
    /// 8kHz, 16kHz, or 30kHz
    #[cfg(feature = "low_frequency")]
    NanoRing(InroFrequency),
    /// 7.3728 mHz
    BaudRate(IbroFrequency),
    /// Warning: if you decide to use the ERTCO, you need to set the daplink
    /// adapter speed to the lowest frequency you are testing
    /// 32.768 kHz
    #[cfg(feature = "low_frequency")]
    RealTimeClock(ErtcoFrequency),
}

/// Acceptable Internal Primary Oscillator frequency
#[derive(Clone, Copy)]
pub enum IpoFrequency {
    /// 100 megahertz
    _100MHz,
}

impl Into<u32> for IpoFrequency {
    fn into(self) -> u32 {
        match self {
            Self::_100MHz => 100,
        }
    }
}

/// Acceptable Internal Secondary Oscillator frequency
#[derive(Clone, Copy)]
pub enum IsoFrequency {
    /// 60 megahertz
    _60MHz,
}

impl Into<u32> for IsoFrequency {
    fn into(self) -> u32 {
        match self {
            Self::_60MHz => 60,
        }
    }
}

/// Acceptable Internal Nano-Ring Oscillator frequencies
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

/// Acceptable Internal Baud Rate Oscillator frequency
#[derive(Clone, Copy)]
pub enum IbroFrequency {
    /// 7.3728 megahertz
    _7_3728MHz,
}

impl Into<u32> for IbroFrequency {
    fn into(self) -> u32 {
        match self {
            Self::_7_3728MHz => 7,
        }
    }
}

/// Acceptable External Real-Time Clock Oscillator frequency
#[cfg(feature = "low_frequency")]
#[derive(Clone, Copy)]
pub enum ErtcoFrequency {
    /// 32.768 kilohertz
    _32_768kHz,
}

#[derive(Clone, Copy)]
#[allow(missing_docs)]
/// Warning: You can't set the divider to be larger than the oscillator
/// frequency because the flash controllers frequency needs to be 1MHz
/// All acceptable oscillator dividors
pub enum Divider {
    _1 = 1,
    _2 = 2,
    _4 = 4,
    _8 = 8,
    _16 = 16,
    _32 = 32,
    _64 = 64,
    _128 = 128,
}

impl Into<u32> for Divider {
    fn into(self) -> u32 {
        match self {
            Self::_1 => 1,
            Self::_2 => 2,
            Self::_4 => 4,
            Self::_8 => 8,
            Self::_16 => 16,
            Self::_32 => 32,
            Self::_64 => 64,
            Self::_128 => 128,
        }
    }
}

/// All acceptable flash clock dividers
#[derive(Clone, Copy)]
#[allow(missing_docs)]
enum FlashClkDivider {
    IpoDiv = 100,
    IsoDiv = 60,
    IbroDiv = 7,
    None = 0,
}

impl Into<u32> for FlashClkDivider {
    fn into(self) -> u32 {
        match self {
            Self::IpoDiv => 100,
            Self::IsoDiv => 60,
            Self::IbroDiv => 7,
            Self::None => {
                unreachable!("The oscillator choosen is incompatible with flash controller")
            }
        }
    }
}

/// SystemClock struct, owns gcr::CLKCTRL
pub struct SystemClock {
    osc: Oscillator,
    divider: Divider,
    flash_divider: Option<u32>,
}

impl SystemClock {
    /// constructor for ipo type oscillator struct
    /// Sets the gcr_clkctrl regester values according to the Analog Devices
    /// user guide
    /// Additionly sets the flash controller clock divider to the correct value
    /// so that its frequency is 1Mhz
    /// Can not set the clock divider above 64
    pub fn configure_ipo(divider: Divider, gcr_peripheral: &GCR, flc_peripheral: &FLC) -> Self {
        gcr_peripheral.clkctrl().modify(|_, w| w.ipo_en().en());
        unsafe {
            bb::spin_bit(gcr_peripheral.clkctrl().as_ptr(), 27, true);
        }
        gcr_peripheral.clkctrl().modify(|_, w| w.sysclk_sel().ipo());
        unsafe {
            bb::spin_bit(gcr_peripheral.clkctrl().as_ptr(), 13, true);
        }

        match divider {
            Divider::_128 => unreachable!("Divider too large"),
            _ => Self::set_divider(gcr_peripheral, divider),
        }

        let flc_clk_div = (divider as u32) / (FlashClkDivider::IpoDiv as u32);
        unsafe {
            flc_peripheral.clkdiv().modify(|_, w| w.bits(flc_clk_div));
        }

        Self {
            osc: Oscillator::Primary(IpoFrequency::_100MHz),
            divider,
            flash_divider: Some(flc_clk_div),
        }
    }

    /// constructor for iso type oscillator struct
    /// Sets the gcr_clkctrl regester values according to the Analog Devices
    /// user guide
    /// Additionly sets the flash controller clock divider to the correct value
    /// so that its frequency is 1Mhz
    /// Can not set the clock divider above 32
    pub fn configure_iso(divider: Divider, gcr_peripheral: &GCR, flc_peripheral: &FLC) -> Self {
        gcr_peripheral.clkctrl().modify(|_, w| w.iso_en().en());
        unsafe {
            bb::spin_bit(gcr_peripheral.clkctrl().as_ptr(), 26, true);
        }
        gcr_peripheral.clkctrl().modify(|_, w| w.sysclk_sel().iso());
        unsafe {
            bb::spin_bit(gcr_peripheral.clkctrl().as_ptr(), 13, true);
        }

        match divider {
            Divider::_128 => unreachable!("Divider too large"),
            Divider::_64 => unreachable!("Divider too large"),
            _ => Self::set_divider(gcr_peripheral, divider),
        }

        let flc_clk_div = (divider as u32) / (FlashClkDivider::IsoDiv as u32);
        unsafe {
            flc_peripheral.clkdiv().modify(|_, w| w.bits(flc_clk_div));
        }

        Self {
            osc: Oscillator::Secondary(IsoFrequency::_60MHz),
            divider,
            flash_divider: Some(flc_clk_div),
        }
    }

    /// constructor for ibro type oscillator struct
    /// Sets the gcr_clkctrl regester values according to the Analog Devices
    /// user guide
    /// Additionly sets the flash controller clock divider to the correct value
    /// so that its frequency is 1Mhz
    /// Can not set the clock divider above 4
    pub fn configure_ibro(divider: Divider, gcr_peripheral: &GCR, flc_peripheral: &FLC) -> Self {
        gcr_peripheral.clkctrl().modify(|_, w| w.ibro_en().en());
        unsafe {
            bb::spin_bit(gcr_peripheral.clkctrl().as_ptr(), 28, true);
        }
        gcr_peripheral
            .clkctrl()
            .modify(|_, w| w.sysclk_sel().ibro());
        unsafe {
            bb::spin_bit(gcr_peripheral.clkctrl().as_ptr(), 13, true);
        }

        match divider {
            Divider::_128 => unreachable!("Divider too large"),
            Divider::_64 => unreachable!("Divider too large"),
            Divider::_32 => unreachable!("Divider too large"),
            Divider::_16 => unreachable!("Divider too large"),
            Divider::_8 => unreachable!("Divider too large"),
            _ => Self::set_divider(gcr_peripheral, divider),
        }

        let flc_clk_div = (divider as u32) / (FlashClkDivider::IbroDiv as u32);
        unsafe {
            flc_peripheral.clkdiv().modify(|_, w| w.bits(flc_clk_div));
        }

        Self {
            osc: Oscillator::BaudRate(IbroFrequency::_7_3728MHz),
            divider,
            flash_divider: Some(flc_clk_div),
        }
    }

    #[cfg(feature = "low_frequency")]
    /// constructor for inro type oscillator struct
    /// Sets the gcr_clkctrl regester values according to the Analog Devices
    /// user guide
    /// Uses the Trimsir register to set the clock divider for low power mode
    /// Warning: Do not use this oscillator as the system oscillator because it
    /// breaks the flash controller in assumption that the system oscillator
    /// frequency is above 1Mhz
    pub fn configure_inro(
        freq: InroFrequency,
        divider: Divider,
        gcr_peripheral: &GCR,
        flc_peripheral: &FLC,
        trimsir_peripheral: &TRIMSIR,
    ) -> Self {
        unsafe {
            bb::spin_bit(gcr_peripheral.clkctrl().as_ptr(), 29, true);
        }
        gcr_peripheral
            .clkctrl()
            .modify(|_, w| w.sysclk_sel().inro());
        unsafe {
            bb::spin_bit(gcr_peripheral.clkctrl().as_ptr(), 13, true);
        }

        match freq {
            InroFrequency::_8kHz => {
                trimsir_peripheral
                    .inro()
                    .modify(|_, w| w.lpclksel()._8khz());
            }
            InroFrequency::_16kHz => {
                trimsir_peripheral
                    .inro()
                    .modify(|_, w| w.lpclksel()._16khz());
            }
            InroFrequency::_30kHz => {
                trimsir_peripheral
                    .inro()
                    .modify(|_, w| w.lpclksel()._30khz());
            }
        }

        Self {
            osc: Oscillator::NanoRing(freq),
            divider,
            flash_divider: None,
        }
    }

    #[cfg(feature = "low_frequency")]
    /// constructor for ertco type oscillator struct
    /// Warning: Do not use this oscillator as the system oscillator because it
    /// breaks the flash controller in assumption that the system oscillator
    /// frequency is above 1Mhz
    pub fn configure_ertco(divider: Divider, gcr_peripheral: &GCR, flc_peripheral: &FLC) -> Self {
        gcr_peripheral.clkctrl().modify(|_, w| w.ertco_en().en());
        unsafe {
            bb::spin_bit(gcr_peripheral.clkctrl().as_ptr(), 25, true);
        }
        gcr_peripheral
            .clkctrl()
            .modify(|_, w| w.sysclk_sel().ertco());
        unsafe {
            bb::spin_bit(gcr_peripheral.clkctrl().as_ptr(), 13, true);
        }

        Self {
            osc: Oscillator::RealTimeClock(ErtcoFrequency::_32_768kHz),
            divider,
            flash_divider: None,
        }
    }

    /// Configures the system clock hardware according to the oscillator struct
    /// configuration
    fn set_divider(gcr_peripheral: &GCR, div: Divider) {
        match div {
            Divider::_1 => {
                gcr_peripheral
                    .clkctrl()
                    .modify(|_, w| w.sysclk_div().div1());
            }

            Divider::_2 => {
                gcr_peripheral
                    .clkctrl()
                    .modify(|_, w| w.sysclk_div().div2());
            }

            Divider::_4 => {
                gcr_peripheral
                    .clkctrl()
                    .modify(|_, w| w.sysclk_div().div4());
            }

            Divider::_8 => {
                gcr_peripheral
                    .clkctrl()
                    .modify(|_, w| w.sysclk_div().div8());
            }

            Divider::_16 => {
                gcr_peripheral
                    .clkctrl()
                    .modify(|_, w| w.sysclk_div().div16());
            }

            Divider::_32 => {
                gcr_peripheral
                    .clkctrl()
                    .modify(|_, w| w.sysclk_div().div32());
            }

            Divider::_64 => {
                gcr_peripheral
                    .clkctrl()
                    .modify(|_, w| w.sysclk_div().div64());
            }

            Divider::_128 => {
                gcr_peripheral
                    .clkctrl()
                    .modify(|_, w| w.sysclk_div().div128());
            }
        }
    }
}
