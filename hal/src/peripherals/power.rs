//! Power control API.

use max78000::{GCR, LPGCR};

/// Enable/disable peripheral clocks; reset peripherals.
pub struct PowerControl<'r, 'l> {
    gcr: &'r GCR,
    lpgcr: &'l LPGCR,
}

/// Indicate a module to enable, disable, or reset through power control registers
pub enum ToggleableModule {
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
    /// Watchdog timer 0
    WDT0,
    /// I2C2
    I2C2,
    /// Audio interface
    I2S,
    /// SPI 0
    SPI0,
    /// AES block
    AES,
    /// CPU1 (RISC-V core)
    CPU1,
}

/// Indicate a module that can be reset but not enabled or disabled
pub enum NonToggleableModule {
    /// Entire system
    SYS,
    /// Peripherals
    PERIPH,
    /// Soft reset
    SOFT,
    /// RTC reset
    RTC,
    /// Single Inductor Multiple Output
    SIMO,
    /// Dynamic Voltage Scaling
    DVS,
}

impl<'r, 'l> PowerControl<'r, 'l> {
    // TODO: Make pub(crate)
    /// Creates a new PowerControl instance that holds references to the GCR and LPGCR registers.
    pub fn new(gcr: &'r GCR, lpgcr: &'l LPGCR) -> Self {
        Self { gcr, lpgcr }
    }

    /// Enables the module from the Module enum
    pub fn enable_peripheral(&self, module_input: ToggleableModule) {
        match module_input {
            ToggleableModule::LPCOMP => self.lpgcr.pclkdis().write(|w| w.lpcomp().en()),
            ToggleableModule::UART3 => self.lpgcr.pclkdis().write(|w| w.uart3().en()),
            ToggleableModule::TMR5 => self.lpgcr.pclkdis().write(|w| w.tmr5().en()),
            ToggleableModule::TMR4 => self.lpgcr.pclkdis().write(|w| w.tmr4().en()),
            ToggleableModule::WDT1 => self.lpgcr.pclkdis().write(|w| w.wdt1().en()),
            ToggleableModule::GPIO2 => self.lpgcr.pclkdis().write(|w| w.gpio2().en()),

            ToggleableModule::PT => self.gcr.pclkdis0().write(|w| w.pt().en()),
            ToggleableModule::I2C1 => self.gcr.pclkdis0().write(|w| w.i2c1().en()),
            ToggleableModule::CNN => self.gcr.pclkdis0().write(|w| w.cnn().en()),
            ToggleableModule::ADC => self.gcr.pclkdis0().write(|w| w.adc().en()),
            ToggleableModule::TMR3 => self.gcr.pclkdis0().write(|w| w.tmr3().en()),
            ToggleableModule::TMR2 => self.gcr.pclkdis0().write(|w| w.tmr2().en()),
            ToggleableModule::TMR1 => self.gcr.pclkdis0().write(|w| w.tmr1().en()),
            ToggleableModule::TMR0 => self.gcr.pclkdis0().write(|w| w.tmr0().en()),
            ToggleableModule::I2C0 => self.gcr.pclkdis0().write(|w| w.i2c0().en()),
            ToggleableModule::UART1 => self.gcr.pclkdis0().write(|w| w.uart1().en()),
            ToggleableModule::UART0 => self.gcr.pclkdis0().write(|w| w.uart0().en()),
            ToggleableModule::SPI1 => self.gcr.pclkdis0().write(|w| w.spi1().en()),
            ToggleableModule::DMA => self.gcr.pclkdis0().write(|w| w.dma().en()),
            ToggleableModule::GPIO1 => self.gcr.pclkdis0().write(|w| w.gpio1().en()),
            ToggleableModule::GPIO0 => self.gcr.pclkdis0().write(|w| w.gpio0().en()),

            ToggleableModule::CRC => self.gcr.pclkdis1().write(|w| w.crc().en()),
            ToggleableModule::OWM => self.gcr.pclkdis1().write(|w| w.owm().en()),
            ToggleableModule::SMPHR => self.gcr.pclkdis1().write(|w| w.smphr().en()),
            ToggleableModule::TRNG => self.gcr.pclkdis1().write(|w| w.trng().en()),
            ToggleableModule::UART2 => self.gcr.pclkdis1().write(|w| w.uart2().en()),
            ToggleableModule::WDT0 => self.gcr.pclkdis1().write(|w| w.wdt0().en()),
            ToggleableModule::I2C2 => self.gcr.pclkdis1().write(|w| w.i2c2().en()),
            ToggleableModule::I2S => self.gcr.pclkdis1().write(|w| w.i2s().en()),
            ToggleableModule::SPI0 => self.gcr.pclkdis1().write(|w| w.spi0().en()),
            ToggleableModule::AES => self.gcr.pclkdis1().write(|w| w.aes().en()),
            ToggleableModule::CPU1 => self.gcr.pclkdis1().write(|w| w.cpu1().en()),
        }
    }

    /// Disables the module from the module enum
    pub fn disable_peripheral(&self, module_input: ToggleableModule) {
        match module_input {
            ToggleableModule::LPCOMP => self.lpgcr.pclkdis().write(|w| w.lpcomp().dis()),
            ToggleableModule::UART3 => self.lpgcr.pclkdis().write(|w| w.uart3().dis()),
            ToggleableModule::TMR5 => self.lpgcr.pclkdis().write(|w| w.tmr5().dis()),
            ToggleableModule::TMR4 => self.lpgcr.pclkdis().write(|w| w.tmr4().dis()),
            ToggleableModule::WDT1 => self.lpgcr.pclkdis().write(|w| w.wdt1().dis()),
            ToggleableModule::GPIO2 => self.lpgcr.pclkdis().write(|w| w.gpio2().dis()),

            ToggleableModule::PT => self.gcr.pclkdis0().write(|w| w.pt().dis()),
            ToggleableModule::I2C1 => self.gcr.pclkdis0().write(|w| w.i2c1().dis()),
            ToggleableModule::CNN => self.gcr.pclkdis0().write(|w| w.cnn().dis()),
            ToggleableModule::ADC => self.gcr.pclkdis0().write(|w| w.adc().dis()),
            ToggleableModule::TMR3 => self.gcr.pclkdis0().write(|w| w.tmr3().dis()),
            ToggleableModule::TMR2 => self.gcr.pclkdis0().write(|w| w.tmr2().dis()),
            ToggleableModule::TMR1 => self.gcr.pclkdis0().write(|w| w.tmr1().dis()),
            ToggleableModule::TMR0 => self.gcr.pclkdis0().write(|w| w.tmr0().dis()),
            ToggleableModule::I2C0 => self.gcr.pclkdis0().write(|w| w.i2c0().dis()),
            ToggleableModule::UART1 => self.gcr.pclkdis0().write(|w| w.uart1().dis()),
            ToggleableModule::UART0 => self.gcr.pclkdis0().write(|w| w.uart0().dis()),
            ToggleableModule::SPI1 => self.gcr.pclkdis0().write(|w| w.spi1().dis()),
            ToggleableModule::DMA => self.gcr.pclkdis0().write(|w| w.dma().dis()),
            ToggleableModule::GPIO1 => self.gcr.pclkdis0().write(|w| w.gpio1().dis()),
            ToggleableModule::GPIO0 => self.gcr.pclkdis0().write(|w| w.gpio0().dis()),

            ToggleableModule::CRC => self.gcr.pclkdis1().write(|w| w.crc().dis()),
            ToggleableModule::OWM => self.gcr.pclkdis1().write(|w| w.owm().dis()),
            ToggleableModule::SMPHR => self.gcr.pclkdis1().write(|w| w.smphr().dis()),
            ToggleableModule::TRNG => self.gcr.pclkdis1().write(|w| w.trng().dis()),
            ToggleableModule::UART2 => self.gcr.pclkdis1().write(|w| w.uart2().dis()),
            ToggleableModule::WDT0 => self.gcr.pclkdis1().write(|w| w.wdt0().dis()),
            ToggleableModule::I2C2 => self.gcr.pclkdis1().write(|w| w.i2c2().dis()),
            ToggleableModule::I2S => self.gcr.pclkdis1().write(|w| w.i2s().dis()),
            ToggleableModule::SPI0 => self.gcr.pclkdis1().write(|w| w.spi0().dis()),
            ToggleableModule::AES => self.gcr.pclkdis1().write(|w| w.aes().dis()),
            ToggleableModule::CPU1 => self.gcr.pclkdis1().write(|w| w.cpu1().dis()),
        }
    }

    /// Reset the given module
    pub fn reset_toggleable(&self, module_input: ToggleableModule) {
        match module_input {
            ToggleableModule::LPCOMP => self.lpgcr.rst().write(|w| w.lpcomp().bit(true)),
            ToggleableModule::UART3 => self.lpgcr.rst().write(|w| w.uart3().bit(true)),
            ToggleableModule::TMR5 => self.lpgcr.rst().write(|w| w.tmr5().bit(true)),
            ToggleableModule::TMR4 => self.lpgcr.rst().write(|w| w.tmr4().bit(true)),
            ToggleableModule::WDT1 => self.lpgcr.rst().write(|w| w.wdt1().bit(true)),
            ToggleableModule::GPIO2 => self.lpgcr.rst().write(|w| w.gpio2().bit(true)),

            ToggleableModule::PT => self.gcr.rst1().write(|w| w.pt().bit(true)),
            ToggleableModule::I2C1 => self.gcr.rst1().write(|w| w.i2c1().bit(true)),
            ToggleableModule::CNN => self.gcr.rst0().write(|w| w.cnn().bit(true)),
            ToggleableModule::ADC => self.gcr.rst0().write(|w| w.adc().bit(true)),
            ToggleableModule::TMR3 => self.gcr.rst0().write(|w| w.tmr3().bit(true)),
            ToggleableModule::TMR2 => self.gcr.rst0().write(|w| w.tmr2().bit(true)),
            ToggleableModule::TMR1 => self.gcr.rst0().write(|w| w.tmr1().bit(true)),
            ToggleableModule::TMR0 => self.gcr.rst0().write(|w| w.tmr0().bit(true)),
            ToggleableModule::I2C0 => self.gcr.rst0().write(|w| w.i2c0().bit(true)),
            ToggleableModule::UART1 => self.gcr.rst0().write(|w| w.uart1().bit(true)),
            ToggleableModule::UART0 => self.gcr.rst0().write(|w| w.uart0().bit(true)),
            ToggleableModule::SPI1 => self.gcr.rst0().write(|w| w.spi1().bit(true)),
            ToggleableModule::DMA => self.gcr.rst0().write(|w| w.dma().bit(true)),
            ToggleableModule::GPIO1 => self.gcr.rst0().write(|w| w.gpio1().bit(true)),
            ToggleableModule::GPIO0 => self.gcr.rst0().write(|w| w.gpio0().bit(true)),

            ToggleableModule::CRC => self.gcr.rst1().write(|w| w.crc().bit(true)),
            ToggleableModule::OWM => self.gcr.rst1().write(|w| w.owm().bit(true)),
            ToggleableModule::SMPHR => self.gcr.rst1().write(|w| w.smphr().bit(true)),
            ToggleableModule::TRNG => self.gcr.rst0().write(|w| w.trng().bit(true)),
            ToggleableModule::UART2 => self.gcr.rst0().write(|w| w.uart2().bit(true)),
            ToggleableModule::WDT0 => self.gcr.rst0().write(|w| w.wdt0().bit(true)),
            ToggleableModule::I2C2 => self.gcr.rst1().write(|w| w.i2c2().bit(true)),
            ToggleableModule::I2S => self.gcr.rst1().write(|w| w.i2s().bit(true)),
            ToggleableModule::SPI0 => self.gcr.rst1().write(|w| w.spi0().bit(true)),
            ToggleableModule::AES => self.gcr.rst1().write(|w| w.aes().bit(true)),
            ToggleableModule::CPU1 => todo!("CPU1 reset not implemented due to inconsistent documentation, see slugSecurity/max78000#11"),
        }
    }

    /// Reset a module that cannot be enabled or disabled
    pub fn reset_non_toggleable(&self, module_input: NonToggleableModule) {
        match module_input {
            NonToggleableModule::SYS => self.gcr.rst0().write(|w| w.sys().bit(true)),
            NonToggleableModule::PERIPH => self.gcr.rst0().write(|w| w.periph().bit(true)),
            NonToggleableModule::SOFT => self.gcr.rst0().write(|w| w.soft().bit(true)),
            NonToggleableModule::RTC => self.gcr.rst0().write(|w| w.rtc().bit(true)),
            NonToggleableModule::SIMO => self.gcr.rst1().write(|w| w.simo().bit(true)),
            NonToggleableModule::DVS => self.gcr.rst1().write(|w| w.dvs().bit(true)),
        }
    }
}
