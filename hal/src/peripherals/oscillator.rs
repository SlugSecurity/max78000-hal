use crate::peripherals::bit_banding as bb;
use core::cell::RefCell;
use max78000::{gcr::CLKCTRL, GCR, TRIMSIR};

/// Hertz
#[derive(Clone, Copy)]
pub struct Hertz(pub u32);

impl From<u32> for Hertz {
    fn from(val: u32) -> Self {
        Hertz(val)
    }
}

/// KiloHertz
#[derive(Clone, Copy)]
pub struct KiloHertz(pub u32);

impl From<Hertz> for KiloHertz {
    fn from(h: Hertz) -> Self {
        KiloHertz(h.0 / 1_000)
    }
}

/// MegaHertz
#[derive(Clone, Copy)]
pub struct MegaHertz(pub u32);

impl From<Hertz> for MegaHertz {
    fn from(h: Hertz) -> Self {
        MegaHertz(h.0 / 1_000_000)
    }
}

impl From<KiloHertz> for MegaHertz {
    fn from(kil: KiloHertz) -> Self {
        MegaHertz(kil.0 / 1_000)
    }
}

#[derive(Clone, Copy)]
pub enum Oscillator {
    /// 1000 mHz
    Primary(ClockType, CrystalFrequency),
    /// 60 mHz
    Secondary(ClockType, CrystalFrequency),
    /// 8kHz, 16kHz, or 30kHz
    NanoRing(ClockType, CrystalFrequency),
    /// 7.3728 mHz
    BaudRate(ClockType, CrystalFrequency),
    /// 32.768 kHz, 8kH
    RealTimeClock(ClockType, CrystalFrequency),
}

#[derive(Clone, Copy)]
pub enum CrystalFrequency {
    _8kHz,
    _16kHz,
    _30kHz,
    _32_768kHz,
    _7_3728mHz,
    _60mHz,
    _1000mHz,
}

impl Into<Hertz> for CrystalFrequency {
    fn into(self) -> Hertz {
        Hertz(match self {
            CrystalFrequency::_8kHz => 8_000,
            CrystalFrequency::_16kHz => 16_000,
            CrystalFrequency::_30kHz => 30_000,
            CrystalFrequency::_32_768kHz => 32_768,
            CrystalFrequency::_7_3728mHz => 7_372_800,
            CrystalFrequency::_60mHz => 60_000_000,
            CrystalFrequency::_1000mHz => 1_000_000_000,
        })
    }
}

#[derive(Clone, Copy)]
pub enum ClockType {
    SystemOscillator,
    RealTimeClock,
    UartClock,
}

#[derive(Clone, Copy)]
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

pub struct SystemClock {
    osc: Oscillator,
    divider: Divider,
    gcr: Gcr,
}

/// GCR peripheral.
pub struct Gcr {
    gcr: RefCell<GCR>,
}

impl Gcr {
    pub fn new(gcr: GCR) -> Self {
        Self {
            gcr: RefCell::new(gcr),
        }
    }
}

impl SystemClock {
    pub fn new(osc: Oscillator, divider: Divider, gcr: GCR) -> Self {
        Self {
            osc,
            divider,
            gcr: Gcr::new(gcr),
        }
    }

    pub fn set(&self) {
        let gcr_ptr = self.gcr.gcr.borrow();
        gcr_ptr.clkctrl.write(|w| {
            match self.osc {
                Oscillator::Primary(ClockType::SystemOscillator, _) => {
                    w.ipo_en().set_bit();
                    // bb::spin_bit(&gcr_ptr.clkctrl, 27);
                    w.sysclk_sel().ipo();
                    // bb::spin_bit(&gcr_ptr.clkctrl, 13);
                }

                Oscillator::Secondary(ClockType::SystemOscillator, _) => {
                    w.iso_en().set_bit();
                    // bb::spin_bit(&gcr_ptr.clkctrl, 26);
                    w.sysclk_sel().iso();
                    // bb::spin_bit(&gcr_ptr.clkctrl, 13);
                }

                Oscillator::NanoRing(ClockType::SystemOscillator, _) => {
                    // bb::spin_bit(&gcr_ptr.clkctrl, 29);
                    w.sysclk_sel().inro();
                    // bb::spin_bit(&gcr_ptr.clkctrl, 13);
                }

                Oscillator::BaudRate(ClockType::SystemOscillator, _) => {
                    w.ibro_en().set_bit();
                    // bb::spin_bit(&gcr_ptr.clkctrl, 28);
                    w.sysclk_sel().ibro();
                    // bb::spin_bit(&gcr_ptr.clkctrl, 13);
                }

                Oscillator::RealTimeClock(ClockType::SystemOscillator, _) => {
                    w.ertco_en().set_bit();
                    // bb::spin_bit(&gcr_ptr.clkctrl, 25);
                    w.sysclk_sel().ertco();
                    // bb::spin_bit(&gcr_ptr.clkctrl, 13);
                }

                _ => {
                    // TODO:
                    // need to handle cases of ibro being used as UART rate
                    // clock, ertco used as RTC input clock, and inro used as
                    // RTC input clock
                }
            }

            // TODO:
            // set frequency later

            // TODO:
            // handle oscillator dividor later

            w
        });
    }
}
