//! Power control API.

use max78000::{GCR, LPGCR};

/// Enable/disable peripheral clocks; reset peripherals.
pub struct PowerControl<'r, 'l> {
    gcr: &'r GCR,
    lpgcr: &'l LPGCR,
}

/// Indicate a module to enable, disable, or reset through power control registers
pub enum Module {
    /// Low-power comparators
    LPCOMP,
    /// UART 0
    UART0,
    /// UART 1
    UART1,
    /// UART 2
    UART2,
    /// UART 3
    UART3,
    /// Timer 0
    TMR0,
    /// Timer 1
    TMR1,
    /// Timer 2
    TMR2,
    /// Timer 3
    TMR3,
    /// Timer 4 (low-power timer 0)
    TMR4,
    /// Timer 5 (low-power timer 1)
    TMR5,
    /// Watchdog timer 1
    WDT1,
    /// GPIO 0
    GPIO0,
    /// GPIO 1
    GPIO1,
    /// GPIO 2
    GPIO2,
    /// Pulse train engine
    PT,
    /// I2C 0
    I2C0,
    /// I2C 1
    I2C1,
    /// Convolutional neural network
    CNN,
    /// Analog-to-digital converter
    ADC,
    /// SPI 1
    SPI1,
    /// Direct memory access
    DMA,
    /// Cyclic redundancy check
    CRC,
    /// 1-Wire Master
    OWM,
    /// Semaphore
    SMPHR,
    /// True random number generator
    TRNG,
}

impl<'r, 'l> PowerControl<'r, 'l> {
    // TODO: Make pub(crate)
    /// Creates a new PowerControl instance that holds references to the GCR and LPGCR registers.
    pub fn new(gcr: &'r GCR, lpgcr: &'l LPGCR) -> Self {
        Self { gcr, lpgcr }
    }

    /// Enables the module from the Module enum
    pub fn enable_peripheral(&self, module_input: Module) {
        match module_input {
            Module::LPCOMP => self.lpgcr.pclkdis().write(|w| w.lpcomp().en()),
            Module::UART3 => self.lpgcr.pclkdis().write(|w| w.uart3().en()),
            Module::TMR5 => self.lpgcr.pclkdis().write(|w| w.tmr5().en()),
            Module::TMR4 => self.lpgcr.pclkdis().write(|w| w.tmr4().en()),
            Module::WDT1 => self.lpgcr.pclkdis().write(|w| w.wdt1().en()),
            Module::GPIO2 => self.lpgcr.pclkdis().write(|w| w.gpio2().en()),

            Module::PT => self.gcr.pclkdis0().write(|w| w.pt().en()),
            Module::I2C1 => self.gcr.pclkdis0().write(|w| w.i2c0().en()),
            Module::CNN => self.gcr.pclkdis0().write(|w| w.cnn().en()),
            Module::ADC => self.gcr.pclkdis0().write(|w| w.adc().en()),
            Module::TMR3 => self.gcr.pclkdis0().write(|w| w.adc().en()),
            Module::TMR2 => self.gcr.pclkdis0().write(|w| w.tmr2().en()),
            Module::TMR1 => self.gcr.pclkdis0().write(|w| w.tmr1().en()),
            Module::TMR0 => self.gcr.pclkdis0().write(|w| w.tmr0().en()),
            Module::I2C0 => self.gcr.pclkdis0().write(|w| w.i2c0().en()),
            Module::UART1 => self.gcr.pclkdis0().write(|w| w.uart1().en()),
            Module::UART0 => self.gcr.pclkdis0().write(|w| w.uart0().en()),
            Module::SPI1 => self.gcr.pclkdis0().write(|w| w.spi1().en()),
            Module::DMA => self.gcr.pclkdis0().write(|w| w.dma().en()),
            Module::GPIO1 => self.gcr.pclkdis0().write(|w| w.gpio1().en()),
            Module::GPIO0 => self.gcr.pclkdis0().write(|w| w.gpio0().en()),

            Module::CRC => self.gcr.pclkdis1().write(|w| w.crc().en()),
            Module::OWM => self.gcr.pclkdis1().write(|w| w.owm().en()),
            Module::SMPHR => self.gcr.pclkdis1().write(|w| w.smphr().en()),
            Module::TRNG => self.gcr.pclkdis1().write(|w| w.trng().en()),
            Module::UART2 => self.gcr.pclkdis1().write(|w| w.uart2().en()),
        }
    }

    /// Disables the module from the module enum
    pub fn disable_peripheral(&self, module_input: Module) {
        match module_input {
            Module::LPCOMP => self.lpgcr.pclkdis().write(|w| w.lpcomp().dis()),
            Module::UART3 => self.lpgcr.pclkdis().write(|w| w.uart3().dis()),
            Module::TMR5 => self.lpgcr.pclkdis().write(|w| w.tmr5().dis()),
            Module::TMR4 => self.lpgcr.pclkdis().write(|w| w.tmr4().dis()),
            Module::WDT1 => self.lpgcr.pclkdis().write(|w| w.wdt1().dis()),
            Module::GPIO2 => self.lpgcr.pclkdis().write(|w| w.gpio2().dis()),

            Module::PT => self.gcr.pclkdis0().write(|w| w.pt().dis()),
            Module::I2C1 => self.gcr.pclkdis0().write(|w| w.i2c0().dis()),
            Module::CNN => self.gcr.pclkdis0().write(|w| w.cnn().dis()),
            Module::ADC => self.gcr.pclkdis0().write(|w| w.adc().dis()),
            Module::TMR3 => self.gcr.pclkdis0().write(|w| w.adc().dis()),
            Module::TMR2 => self.gcr.pclkdis0().write(|w| w.tmr2().dis()),
            Module::TMR1 => self.gcr.pclkdis0().write(|w| w.tmr1().dis()),
            Module::TMR0 => self.gcr.pclkdis0().write(|w| w.tmr0().dis()),
            Module::I2C0 => self.gcr.pclkdis0().write(|w| w.i2c0().dis()),
            Module::UART1 => self.gcr.pclkdis0().write(|w| w.uart1().dis()),
            Module::UART0 => self.gcr.pclkdis0().write(|w| w.uart0().dis()),
            Module::SPI1 => self.gcr.pclkdis0().write(|w| w.spi1().dis()),
            Module::DMA => self.gcr.pclkdis0().write(|w| w.dma().dis()),
            Module::GPIO1 => self.gcr.pclkdis0().write(|w| w.gpio1().dis()),
            Module::GPIO0 => self.gcr.pclkdis0().write(|w| w.gpio0().dis()),

            Module::CRC => self.gcr.pclkdis1().write(|w| w.crc().dis()),
            Module::OWM => self.gcr.pclkdis1().write(|w| w.owm().dis()),
            Module::SMPHR => self.gcr.pclkdis1().write(|w| w.smphr().dis()),
            Module::TRNG => self.gcr.pclkdis1().write(|w| w.trng().dis()),
            Module::UART2 => self.gcr.pclkdis1().write(|w| w.uart2().dis()),
        }
    }

    /// Reset the given module
    pub fn reset(&self, module_input: Module) {
        match module_input {
            Module::LPCOMP => self.lpgcr.rst().write(|w| w.lpcomp().bit(true)),
            Module::UART3 => self.lpgcr.rst().write(|w| w.uart3().bit(true)),
            Module::TMR5 => self.lpgcr.rst().write(|w| w.tmr5().bit(true)),
            Module::TMR4 => self.lpgcr.rst().write(|w| w.tmr4().bit(true)),
            Module::WDT1 => self.lpgcr.rst().write(|w| w.wdt1().bit(true)),
            Module::GPIO2 => self.lpgcr.rst().write(|w| w.gpio2().bit(true)),

            Module::PT => self.gcr.rst1().write(|w| w.pt().bit(true)),
            Module::I2C1 => self.gcr.rst1().write(|w| w.i2c1().bit(true)),
            Module::CNN => self.gcr.rst0().write(|w| w.cnn().bit(true)),
            Module::ADC => self.gcr.rst0().write(|w| w.adc().bit(true)),
            Module::TMR3 => self.gcr.rst0().write(|w| w.adc().bit(true)),
            Module::TMR2 => self.gcr.rst0().write(|w| w.tmr2().bit(true)),
            Module::TMR1 => self.gcr.rst0().write(|w| w.tmr1().bit(true)),
            Module::TMR0 => self.gcr.rst0().write(|w| w.tmr0().bit(true)),
            Module::I2C0 => self.gcr.rst0().write(|w| w.i2c0().bit(true)),
            Module::UART1 => self.gcr.rst0().write(|w| w.uart1().bit(true)),
            Module::UART0 => self.gcr.rst0().write(|w| w.uart0().bit(true)),
            Module::SPI1 => self.gcr.rst0().write(|w| w.spi1().bit(true)),
            Module::DMA => self.gcr.rst0().write(|w| w.dma().bit(true)),
            Module::GPIO1 => self.gcr.rst0().write(|w| w.gpio1().bit(true)),
            Module::GPIO0 => self.gcr.rst0().write(|w| w.gpio0().bit(true)),

            Module::CRC => self.gcr.rst1().write(|w| w.crc().bit(true)),
            Module::OWM => self.gcr.rst1().write(|w| w.owm().bit(true)),
            Module::SMPHR => self.gcr.rst1().write(|w| w.smphr().bit(true)),
            Module::TRNG => self.gcr.rst0().write(|w| w.trng().bit(true)),
            Module::UART2 => self.gcr.rst0().write(|w| w.uart2().bit(true)),
        }
    }
    /// System Reset
    pub fn reset_sys(&self) {
        self.gcr.rst0().write(|w| w.sys().bit(true));
    }

    /// Peripheral Reset
    pub fn reset_periph(&self) {
        self.gcr.rst0().write(|w| w.periph().bit(true));
    }

    /// Soft Reset
    pub fn reset_soft(&self) {
        self.gcr.rst0().write(|w| w.soft().bit(true));
    }

    /// RTC Reset
    pub fn reset_rtc(&self) {
        self.gcr.rst0().write(|w| w.rtc().bit(true));
    }

    /// Watchdog Timer 0 Reset
    pub fn reset_wdt0(&self) {
        self.gcr.rst0().write(|w| w.wdt0().bit(true));
    }

    /// Single Inductor Multiple Output Block Reset
    pub fn reset_simo(&self) {
        self.gcr.rst1().write(|w| w.simo().bit(true));
    }

    /// Dynamic Voltage Scaling Controller Reset
    pub fn reset_dvs(&self) {
        self.gcr.rst1().write(|w| w.dvs().bit(true));
    }

    /// I2C2 Reset
    pub fn reset_i2c2(&self) {
        self.gcr.rst1().write(|w| w.i2c2().bit(true));
    }

    /// Audio Interface Reset
    pub fn reset_i2s(&self) {
        self.gcr.rst1().write(|w| w.i2s().bit(true));
    }

    /// Semaphore Block Reset
    pub fn reset_smphr(&self) {
        self.gcr.rst1().write(|w| w.smphr().bit(true));
    }

    /// SPI0 Reset
    pub fn reset_spi0(&self) {
        self.gcr.rst1().write(|w| w.spi0().bit(true));
    }

    /// AES Block Reset
    pub fn reset_aes(&self) {
        self.gcr.rst1().write(|w| w.aes().bit(true));
    }
}
