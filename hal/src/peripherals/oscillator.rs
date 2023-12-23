use crate::peripherals::bit_banding as bb;
use core::fmt::Write;
use cortex_m_semihosting::hio;
use max78000::GCR;
use max78000::TRIMSIR;

#[derive(Clone, Copy)]
/// All acceptable oscillators configurations
pub enum Oscillator {
    /// 100 mHz
    Primary(IpoFrequency),
    /// 60 mHz
    Secondary(IsoFrequency),
    /// 8kHz, 16kHz, or 30kHz
    NanoRing(InroFrequency),
    /// 7.3728 mHz
    BaudRate(IbroFrequency),
    /// 32.768 kHz
    RealTimeClock(ErtcoFrequency),
}

/// Acceptable Internal Primary Oscillator frequency
#[derive(Clone, Copy)]
pub enum IpoFrequency {
    /// 100 megahertz
    _100MHz,
}

/// Acceptable Internal Secondary Oscillator frequency
#[derive(Clone, Copy)]
pub enum IsoFrequency {
    /// 60 megahertz
    _60MHz,
}

/// Acceptable Internal Nano-Ring Oscillator frequencies
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

/// Acceptable External Real-Time Clock Oscillator frequency
#[derive(Clone, Copy)]
pub enum ErtcoFrequency {
    /// 32.768 kilohertz
    _32_768kHz,
}

#[derive(Clone, Copy)]
#[allow(missing_docs)]
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

#[derive(Clone, Copy)]
/// Frequency sum type
pub enum FrequencyPeripheral {
    /// For oscillators that can not change their frequency
    None,
    /// For the internal nano-ring oscillator that can change its frequency to
    /// 8kHz, 16kHz, or 30kHz
    TrimsirInro,
}

/// SystemClock struct, owns gcr::CLKCTRL
pub struct SystemClock {
    osc: Oscillator,
    divider: Divider,
    freq_perf: FrequencyPeripheral,
}

impl SystemClock {
    /// constructor for ipo type oscillator struct
    pub fn configure_ipo(divider: Divider, gcr_peripheral: &GCR) -> Self {
        gcr_peripheral.clkctrl().modify(|_, w| w.ipo_en().en());
        unsafe {
            bb::spin_bit(gcr_peripheral.clkctrl().as_ptr(), 27, true);
        }
        gcr_peripheral.clkctrl().modify(|_, w| w.sysclk_sel().ipo());
        unsafe {
            bb::spin_bit(gcr_peripheral.clkctrl().as_ptr(), 13, true);
        }

        Self::set_divider(gcr_peripheral, divider);

        Self {
            osc: Oscillator::Primary(IpoFrequency::_100MHz),
            divider,
            freq_perf: FrequencyPeripheral::None,
        }
    }

    /// constructor for iso type oscillator struct
    pub fn configure_iso(divider: Divider, gcr_peripheral: &GCR) -> Self {
        gcr_peripheral.clkctrl().modify(|_, w| w.iso_en().en());
        unsafe {
            bb::spin_bit(gcr_peripheral.clkctrl().as_ptr(), 26, true);
        }
        gcr_peripheral.clkctrl().modify(|_, w| w.sysclk_sel().iso());
        unsafe {
            bb::spin_bit(gcr_peripheral.clkctrl().as_ptr(), 13, true);
        }

        Self::set_divider(gcr_peripheral, divider);

        Self {
            osc: Oscillator::Secondary(IsoFrequency::_60MHz),
            divider,
            freq_perf: FrequencyPeripheral::None,
        }
    }

    /// constructor for inro type oscillator struct
    pub fn configure_inro(
        freq: InroFrequency,
        divider: Divider,
        gcr_peripheral: &GCR,
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

        Self::set_divider(gcr_peripheral, divider);

        Self {
            osc: Oscillator::NanoRing(freq),
            divider,
            freq_perf: FrequencyPeripheral::TrimsirInro,
        }
    }

    /// constructor for ibro type oscillator struct
    pub fn configure_ibro(divider: Divider, gcr_peripheral: &GCR) -> Self {
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

        Self::set_divider(gcr_peripheral, divider);

        Self {
            osc: Oscillator::BaudRate(IbroFrequency::_7_3728MHz),
            divider,
            freq_perf: FrequencyPeripheral::None,
        }
    }

    /// constructor for ertco type oscillator struct
    pub fn configure_ertco(divider: Divider, gcr_peripheral: &GCR) -> Self {
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

        Self::set_divider(gcr_peripheral, divider);

        Self {
            osc: Oscillator::RealTimeClock(ErtcoFrequency::_32_768kHz),
            divider,
            freq_perf: FrequencyPeripheral::None,
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
