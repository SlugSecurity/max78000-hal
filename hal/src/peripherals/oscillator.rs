use crate::peripherals::bit_banding as bb;
use max78000::gcr::CLKCTRL;
use max78000::trimsir::INRO;

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

#[allow(missing_docs)]
#[derive(Clone, Copy)]
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
pub enum FrequencyPeripheral<'a> {
    /// For oscillators that can not change their frequency
    None,
    /// For the internal nano-ring oscillator that can change its frequency to
    /// 8kHz, 16kHz, or 30kHz
    TrimsirInro(&'a INRO),
}

/// SystemClock struct, owns gcr::CLKCTRL
pub struct SystemClock<'a> {
    osc: Oscillator,
    divider: Divider,
    gcr_clkctrl: &'a CLKCTRL,
    freq_perf: FrequencyPeripheral<'a>,
}

impl<'a> SystemClock<'a> {
    /// constructor for ipo type oscillator struct
    pub fn configure_ipo(divider: Divider, gcr_peripheral: &'a CLKCTRL) -> Self {
        Self {
            osc: Oscillator::Primary(IpoFrequency::_100MHz),
            divider: divider,
            gcr_clkctrl: gcr_peripheral,
            freq_perf: FrequencyPeripheral::None,
        }
    }

    /// constructor for iso type oscillator struct
    pub fn configure_iso(divider: Divider, gcr_peripheral: &'a CLKCTRL) -> Self {
        Self {
            osc: Oscillator::Secondary(IsoFrequency::_60MHz),
            divider: divider,
            gcr_clkctrl: gcr_peripheral,
            freq_perf: FrequencyPeripheral::None,
        }
    }

    /// constructor for inro type oscillator struct
    pub fn configure_inro(
        freq: InroFrequency,
        divider: Divider,
        gcr_peripheral: &'a CLKCTRL,
        inro_trsimsir_peripheral: &'a INRO,
    ) -> Self {
        Self {
            osc: Oscillator::NanoRing(freq),
            divider: divider,
            gcr_clkctrl: gcr_peripheral,
            freq_perf: FrequencyPeripheral::TrimsirInro(inro_trsimsir_peripheral),
        }
    }

    /// constructor for ibro type oscillator struct
    pub fn configure_ibro(divider: Divider, gcr_peripheral: &'a CLKCTRL) -> Self {
        Self {
            osc: Oscillator::BaudRate(IbroFrequency::_7_3728MHz),
            divider: divider,
            gcr_clkctrl: gcr_peripheral,
            freq_perf: FrequencyPeripheral::None,
        }
    }

    /// constructor for ertco type oscillator struct
    pub fn configure_ertco(divider: Divider, gcr_peripheral: &'a CLKCTRL) -> Self {
        Self {
            osc: Oscillator::RealTimeClock(ErtcoFrequency::_32_768kHz),
            divider: divider,
            gcr_clkctrl: gcr_peripheral,
            freq_perf: FrequencyPeripheral::None,
        }
    }

    /// Configures the system clock hardware according to the oscillator struct
    /// configuration
    pub fn set(&self) {
        let gcr_ptr = self.gcr_clkctrl;
        gcr_ptr.write(|w| {
            match self.osc {
                Oscillator::Primary(_) => {
                    w.ipo_en().set_bit();
                    // bb::spin_bit(&gcr_ptr.clkctrl, 27);
                    w.sysclk_sel().ipo();
                    // bb::spin_bit(&gcr_ptr.clkctrl, 13);
                }

                Oscillator::Secondary(_) => {
                    w.iso_en().set_bit();
                    // bb::spin_bit(&gcr_ptr.clkctrl, 26);
                    w.sysclk_sel().iso();
                    // bb::spin_bit(&gcr_ptr.clkctrl, 13);
                }

                Oscillator::NanoRing(freq) => {
                    // bb::spin_bit(&gcr_ptr.clkctrl, 29);
                    w.sysclk_sel().inro();
                    // bb::spin_bit(&gcr_ptr.clkctrl, 13);

                    match self.freq_perf {
                        FrequencyPeripheral::TrimsirInro(trimsir_perf) => {
                            trimsir_perf.write(|w| {
                                match freq {
                                    InroFrequency::_8kHz => {
                                        w.lpclksel()._8khz();
                                    }
                                    InroFrequency::_16kHz => {
                                        w.lpclksel()._16khz();
                                    }
                                    InroFrequency::_30kHz => {
                                        w.lpclksel()._30khz();
                                    }
                                }

                                w
                            });
                        }

                        FrequencyPeripheral::None => {
                            unreachable!("You need to give access to the trimsir peripheral")
                        }
                    }
                }

                Oscillator::BaudRate(_) => {
                    w.ibro_en().set_bit();
                    // bb::spin_bit(&gcr_ptr.clkctrl, 28);
                    w.sysclk_sel().ibro();
                    // bb::spin_bit(&gcr_ptr.clkctrl, 13);
                }

                Oscillator::RealTimeClock(_) => {
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
