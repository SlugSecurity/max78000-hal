use crate::peripherals::bit_banding as bb;
use max78000::gcr::CLKCTRL;
use max78000::trimsir::INRO;

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
/// All acceptable oscillators
pub enum Oscillator {
    /// 100 mHz
    Primary,
    /// 60 mHz
    Secondary,
    /// 8kHz, 16kHz, or 30kHz
    NanoRing(INRO_Frequency),
    /// 7.3728 mHz
    BaudRate,
    /// 32.768 kHz
    RealTimeClock,
}

#[derive(Clone, Copy)]
/// All acceptable frequencies for the internal Nano-Ring Oscillator
pub enum INRO_Frequency {
    _8kHz,
    _16kHz,
    _30kHz,
}

impl Into<Hertz> for INRO_Frequency {
    fn into(self) -> Hertz {
        Hertz(match self {
            INRO_Frequency::_8kHz => 8_000,
            INRO_Frequency::_16kHz => 16_000,
            INRO_Frequency::_30kHz => 30_000,
        })
    }
}

#[derive(Clone, Copy)]
/// All acceptable dividors
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

/// SystemClock struct, owns gcr::CLKCTRL
pub struct SystemClock<'a> {
    osc: Oscillator,
    divider: Divider,
    gcr_clkctrl: &'a CLKCTRL,
    trimsir_inro: &'a INRO,
}

impl<'a> SystemClock<'a> {
    /// takes owner ship of gcr::CLKCTRL registers
    pub fn new(
        osc: Oscillator,
        divider: Divider,
        gcr_peripheral: &'a mut CLKCTRL,
        trimsir_inro: &'a mut INRO,
    ) -> Self {
        Self {
            osc,
            divider,
            gcr_clkctrl: &*gcr_peripheral,
            trimsir_inro: &*trimsir_inro,
        }
    }

    /// Configures the system clock
    pub fn set(&self) {
        let gcr_ptr = self.gcr_clkctrl;
        gcr_ptr.write(|w| {
            match self.osc {
                Oscillator::Primary => {
                    w.ipo_en().set_bit();
                    // bb::spin_bit(&gcr_ptr.clkctrl, 27);
                    w.sysclk_sel().ipo();
                    // bb::spin_bit(&gcr_ptr.clkctrl, 13);
                }

                Oscillator::Secondary => {
                    w.iso_en().set_bit();
                    // bb::spin_bit(&gcr_ptr.clkctrl, 26);
                    w.sysclk_sel().iso();
                    // bb::spin_bit(&gcr_ptr.clkctrl, 13);
                }

                Oscillator::NanoRing(freq) => {
                    // bb::spin_bit(&gcr_ptr.clkctrl, 29);
                    w.sysclk_sel().inro();
                    // bb::spin_bit(&gcr_ptr.clkctrl, 13);

                    self.trimsir_inro.write(|w| {
                        match freq {
                            INRO_Frequency::_8kHz => {
                                w.lpclksel()._8khz();
                            }
                            INRO_Frequency::_16kHz => {
                                w.lpclksel()._16khz();
                            }
                            INRO_Frequency::_30kHz => {
                                w.lpclksel()._30khz();
                            }
                        }

                        w
                    });
                }

                Oscillator::BaudRate => {
                    w.ibro_en().set_bit();
                    // bb::spin_bit(&gcr_ptr.clkctrl, 28);
                    w.sysclk_sel().ibro();
                    // bb::spin_bit(&gcr_ptr.clkctrl, 13);
                }

                Oscillator::RealTimeClock => {
                    w.ertco_en().set_bit();
                    // bb::spin_bit(&gcr_ptr.clkctrl, 25);
                    w.sysclk_sel().ertco();
                    // bb::spin_bit(&gcr_ptr.clkctrl, 13);
                }
            }

            match self.divider {
                Divider::_1 => {
                    w.sysclk_div().div1();
                }

                Divider::_2 => {
                    w.sysclk_div().div2();
                }

                Divider::_4 => {
                    w.sysclk_div().div4();
                }

                Divider::_8 => {
                    w.sysclk_div().div8();
                }

                Divider::_16 => {
                    w.sysclk_div().div16();
                }

                Divider::_32 => {
                    w.sysclk_div().div16();
                }

                Divider::_64 => {
                    w.sysclk_div().div16();
                }

                Divider::_128 => {
                    w.sysclk_div().div16();
                }
            }

            w
        });
    }
}
