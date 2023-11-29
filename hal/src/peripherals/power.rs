//! Power control API.

use max78000::{GCR, LPGCR};

/// Enable/disable peripheral clocks; reset peripherals.
pub struct PowerControl<'r> {
    gcr: &'r GCR,
    lpgcr: &'r LPGCR,
}

impl<'r> PowerControl<'r> {
    /// Creates a new PowerControl instance that holds references to the GCR and LPGCR registers.
    pub(crate) fn new(gcr: &'r GCR, lpgcr: &'r LPGCR) -> Self {
        Self { gcr, lpgcr }
    }

    // LPGCR_PCLKDIS

    /// Enable Low Power Comparators Clock
    pub fn enable_lpcomp(&self) {
        self.lpgcr.pclkdis.write(|w| w.lpcomp().en());
    }

    /// Disable Low Power Comparators Clock
    pub fn disable_lpcomp(&self) {
        self.lpgcr.pclkdis.write(|w| w.lpcomp().dis());
    }

    /// Enable UART3 (LPUART0) Clock
    pub fn enable_uart3(&self) {
        self.lpgcr.pclkdis.write(|w| w.uart3().en());
    }

    /// Disable UART3 (LPUART0) Clock
    pub fn disable_uart3(&self) {
        self.lpgcr.pclkdis.write(|w| w.uart3().dis());
    }

    /// Enable TMR5 (LPTMR1) Clock
    pub fn enable_tmr5(&self) {
        self.lpgcr.pclkdis.write(|w| w.tmr5().en());
    }

    /// Disable TMR5 (LPTMR1) Clock
    pub fn disable_tmr5(&self) {
        self.lpgcr.pclkdis.write(|w| w.tmr5().dis());
    }

    /// Enable TMR4 (LPTMR0) Clock
    pub fn enable_tmr4(&self) {
        self.lpgcr.pclkdis.write(|w| w.tmr4().en());
    }

    /// Disable TMR4 (LPTMR0) Clock
    pub fn disable_tmr4(&self) {
        self.lpgcr.pclkdis.write(|w| w.tmr4().dis());
    }

    /// Enable WDT1 (LPWDT0) Clock
    pub fn enable_wdt1(&self) {
        self.lpgcr.pclkdis.write(|w| w.wdt1().en());
    }

    /// Disable WDT1 (LPWDT0) Clock
    pub fn disable_wdt1(&self) {
        self.lpgcr.pclkdis.write(|w| w.wdt1().dis());
    }

    /// Enable GPIO2 Clock
    pub fn enable_gpio2(&self) {
        self.lpgcr.pclkdis.write(|w| w.gpio2().en());
    }

    /// Disable GPIO2 Clock
    pub fn disable_gpio2(&self) {
        self.lpgcr.pclkdis.write(|w| w.gpio2().dis());
    }

    // GCR_PCLKDIS0

    /// Enable Pulse Train Clock
    pub fn enable_pt(&self) {
        self.gcr.pclkdis0.write(|w| w.pt().en());
    }

    /// Disable Pulse Train Clock
    pub fn disable_pt(&self) {
        self.gcr.pclkdis0.write(|w| w.pt().dis());
    }

    /// Enable I2C1 Clock
    pub fn enable_i2c1(&self) {
        self.gcr.pclkdis0.write(|w| w.i2c1().en());
    }

    /// Disable I2C1 Clock
    pub fn disable_i2c1(&self) {
        self.gcr.pclkdis0.write(|w| w.i2c1().dis());
    }

    /// Enable CNN Clock
    pub fn enable_cnn(&self) {
        self.gcr.pclkdis0.write(|w| w.cnn().en());
    }

    /// Disable CNN Clock
    pub fn disable_cnn(&self) {
        self.gcr.pclkdis0.write(|w| w.cnn().dis());
    }

    /// Enable ADC Clock
    pub fn enable_adc(&self) {
        self.gcr.pclkdis0.write(|w| w.adc().en());
    }

    /// Disable ADC Clock
    pub fn disable_adc(&self) {
        self.gcr.pclkdis0.write(|w| w.adc().dis());
    }

    /// Enable TMR3 Clock
    pub fn enable_tmr3(&self) {
        self.gcr.pclkdis0.write(|w| w.tmr3().en());
    }

    /// Disable TMR3 Clock
    pub fn disable_tmr3(&self) {
        self.gcr.pclkdis0.write(|w| w.tmr3().dis());
    }

    /// Enable TMR2 Clock
    pub fn enable_tmr2(&self) {
        self.gcr.pclkdis0.write(|w| w.tmr2().en());
    }

    /// Disable TMR2 Clock
    pub fn disable_tmr2(&self) {
        self.gcr.pclkdis0.write(|w| w.tmr2().dis());
    }

    /// Enable TMR1 Clock
    pub fn enable_tmr1(&self) {
        self.gcr.pclkdis0.write(|w| w.tmr1().en());
    }

    /// Disable TMR1 Clock
    pub fn disable_tmr1(&self) {
        self.gcr.pclkdis0.write(|w| w.tmr1().dis());
    }

    /// Enable TMR0 Clock
    pub fn enable_tmr0(&self) {
        self.gcr.pclkdis0.write(|w| w.tmr0().en());
    }

    /// Disable TMR0 Clock
    pub fn disable_tmr0(&self) {
        self.gcr.pclkdis0.write(|w| w.tmr0().dis());
    }

    /// Enable I2C0 Clock
    pub fn enable_i2c0(&self) {
        self.gcr.pclkdis0.write(|w| w.i2c0().en());
    }

    /// Disable I2C0 Clock
    pub fn disable_i2c0(&self) {
        self.gcr.pclkdis0.write(|w| w.i2c0().dis());
    }

    /// Enable UART1 Clock
    pub fn enable_uart1(&self) {
        self.gcr.pclkdis0.write(|w| w.uart1().en());
    }

    /// Disable UART1 Clock
    pub fn disable_uart1(&self) {
        self.gcr.pclkdis0.write(|w| w.uart1().dis());
    }

    /// Enable UART0 Clock
    pub fn enable_uart0(&self) {
        self.gcr.pclkdis0.write(|w| w.uart0().en());
    }

    /// Disable UART0 Clock
    pub fn disable_uart0(&self) {
        self.gcr.pclkdis0.write(|w| w.uart0().dis());
    }

    /// Enable SPI1 Clock
    pub fn enable_spi1(&self) {
        self.gcr.pclkdis0.write(|w| w.spi1().en());
    }

    /// Disable SPI1 Clock
    pub fn disable_spi1(&self) {
        self.gcr.pclkdis0.write(|w| w.spi1().dis());
    }

    /// Enable DMA Clock
    pub fn enable_dma(&self) {
        self.gcr.pclkdis0.write(|w| w.dma().en());
    }

    /// Disable DMA Clock
    pub fn disable_dma(&self) {
        self.gcr.pclkdis0.write(|w| w.dma().dis());
    }

    /// Enable GPIO1 Port and Pad Logic Clock
    pub fn enable_gpio1(&self) {
        self.gcr.pclkdis0.write(|w| w.gpio1().en());
    }

    /// Disable GPIO1 Port and Pad Logic Clock
    pub fn disable_gpio1(&self) {
        self.gcr.pclkdis0.write(|w| w.gpio1().dis());
    }

    /// Enable GPIO0 Port and Pad Logic Clock
    pub fn enable_gpio0(&self) {
        self.gcr.pclkdis0.write(|w| w.gpio0().en());
    }

    /// Disable GPIO0 Port and Pad Logic Clock
    pub fn disable_gpio0(&self) {
        self.gcr.pclkdis0.write(|w| w.gpio0().dis());
    }

    // GCR_PCLKDIS1

    /// Enable CRC Clock
    pub fn enable_crc(&self) {
        self.gcr.pclkdis1.write(|w| w.crc().en());
    }

    /// Disable CRC Clock
    pub fn disable_crc(&self) {
        self.gcr.pclkdis1.write(|w| w.crc().dis());
    }

    /// Enable 1-Wire Clock
    pub fn enable_owm(&self) {
        self.gcr.pclkdis1.write(|w| w.owm().en());
    }

    /// Disable 1-Wire Clock
    pub fn disable_owm(&self) {
        self.gcr.pclkdis1.write(|w| w.owm().dis());
    }

    /// Enable Semaphore Block Clock
    pub fn enable_smphr(&self) {
        self.gcr.pclkdis1.write(|w| w.smphr().en());
    }

    /// Disable Semaphore Block Clock
    pub fn disable_smphr(&self) {
        self.gcr.pclkdis1.write(|w| w.smphr().dis());
    }

    /// Enable TRNG Clock
    pub fn enable_trng(&self) {
        self.gcr.pclkdis1.write(|w| w.trng().en());
    }

    /// Disable TRNG Clock
    pub fn disable_trng(&self) {
        self.gcr.pclkdis1.write(|w| w.trng().dis());
    }

    /// Enable UART2 Clock
    pub fn enable_uart2(&self) {
        self.gcr.pclkdis1.write(|w| w.uart2().en());
    }

    /// Disable UART2 Clock
    pub fn disable_uart2(&self) {
        self.gcr.pclkdis1.write(|w| w.uart2().dis());
    }

    // LPGCR_RST

    /// Low Power Comparators Reset
    pub fn reset_lpcomp(&self) {
        self.lpgcr.rst.write(|w| w.lpcomp().bit(true));
    }

    /// UART3 (LPUART0) Reset
    pub fn reset_uart3(&self) {
        self.lpgcr.rst.write(|w| w.uart3().bit(true));
    }

    /// TMR5 (LPTMR1) Reset
    pub fn reset_tmr5(&self) {
        self.lpgcr.rst.write(|w| w.tmr5().bit(true));
    }

    /// TMR4 (LPTMR0) Reset
    pub fn reset_tmr4(&self) {
        self.lpgcr.rst.write(|w| w.tmr4().bit(true));
    }

    /// WDT1 (LPWDT0) Reset
    pub fn reset_wdt1(&self) {
        self.lpgcr.rst.write(|w| w.wdt1().bit(true));
    }

    /// GPIO2 Reset
    pub fn reset_gpio2(&self) {
        self.lpgcr.rst.write(|w| w.gpio2().bit(true));
    }

    // GCR_RST0

    /// System Reset
    ///
    /// A system reset is the same as a soft reset, except it also resets all GCR, resetting the clocks to their
    /// POR default state. The CPU state is reset, as well as the watchdog timers. The AoD and RAM are unaffected.
    pub fn reset_sys(&self) {
        self.gcr.rst0.write(|w| w.sys().bit(true));
    }

    /// Peripheral Reset
    pub fn reset_periph(&self) {
        self.gcr.rst0.write(|w| w.periph().bit(true));
    }

    /// Soft Reset
    pub fn reset_soft(&self) {
        self.gcr.rst0.write(|w| w.soft().bit(true));
    }

    /// UART2 Reset
    pub fn reset_uart2(&self) {
        self.gcr.rst0.write(|w| w.uart2().bit(true));
    }

    /// ADC Reset
    pub fn reset_adc(&self) {
        self.gcr.rst0.write(|w| w.adc().bit(true));
    }

    /// CNN Reset
    pub fn reset_cnn(&self) {
        self.gcr.rst0.write(|w| w.cnn().bit(true));
    }

    /// TRNG Reset
    pub fn reset_trng(&self) {
        self.gcr.rst0.write(|w| w.trng().bit(true));
    }

    /// RTC Reset
    pub fn reset_rtc(&self) {
        self.gcr.rst0.write(|w| w.rtc().bit(true));
    }

    /// I2C0 Reset
    pub fn reset_i2c0(&self) {
        self.gcr.rst0.write(|w| w.i2c0().bit(true));
    }

    /// SPI1 Reset
    pub fn reset_spi1(&self) {
        self.gcr.rst0.write(|w| w.spi1().bit(true));
    }

    /// UART1 Reset
    pub fn reset_uart1(&self) {
        self.gcr.rst0.write(|w| w.uart1().bit(true));
    }

    /// UART0 Reset
    pub fn reset_uart0(&self) {
        self.gcr.rst0.write(|w| w.uart0().bit(true));
    }

    /// TMR3 Reset
    pub fn reset_tmr3(&self) {
        self.gcr.rst0.write(|w| w.tmr3().bit(true));
    }

    /// TMR2 Reset
    pub fn reset_tmr2(&self) {
        self.gcr.rst0.write(|w| w.tmr2().bit(true));
    }

    /// TMR1 Reset
    pub fn reset_tmr1(&self) {
        self.gcr.rst0.write(|w| w.tmr1().bit(true));
    }

    /// TMR0 Reset
    pub fn reset_tmr0(&self) {
        self.gcr.rst0.write(|w| w.tmr0().bit(true));
    }

    /// GPIO1 Reset
    pub fn reset_gpio1(&self) {
        self.gcr.rst0.write(|w| w.gpio1().bit(true));
    }

    /// GPIO0 Reset
    pub fn reset_gpio0(&self) {
        self.gcr.rst0.write(|w| w.gpio0().bit(true));
    }

    /// Watchdog Timer 0 Reset
    pub fn reset_wdt0(&self) {
        self.gcr.rst0.write(|w| w.wdt0().bit(true));
    }

    /// DMA Access Block Reset
    pub fn reset_dma(&self) {
        self.gcr.rst0.write(|w| w.dma().bit(true));
    }

    // GCR_RST1

    /// Single Inductor Multiple Output Block Reset
    pub fn reset_simo(&self) {
        self.gcr.rst1.write(|w| w.simo().bit(true));
    }

    /// Dynamic Voltage Scaling Controller Reset
    pub fn reset_dvs(&self) {
        self.gcr.rst1.write(|w| w.dvs().bit(true));
    }

    /// I2C2 Reset
    pub fn reset_i2c2(&self) {
        self.gcr.rst1.write(|w| w.i2c2().bit(true));
    }

    /// Audio Interface Reset
    pub fn reset_i2s(&self) {
        self.gcr.rst1.write(|w| w.i2s().bit(true));
    }

    /// Semaphore Block Reset
    pub fn reset_smphr(&self) {
        self.gcr.rst1.write(|w| w.smphr().bit(true));
    }

    /// SPI0 Reset
    pub fn reset_spi0(&self) {
        self.gcr.rst1.write(|w| w.spi0().bit(true));
    }

    /// AES Block Reset
    pub fn reset_aes(&self) {
        self.gcr.rst1.write(|w| w.aes().bit(true));
    }

    /// CRC Reset
    pub fn reset_crc(&self) {
        self.gcr.rst1.write(|w| w.crc().bit(true));
    }

    /// 1-Wire Reset
    pub fn reset_owm(&self) {
        self.gcr.rst1.write(|w| w.owm().bit(true));
    }

    /// Pulse Train Reset
    pub fn reset_pt(&self) {
        self.gcr.rst1.write(|w| w.pt().bit(true));
    }

    /// I2C1 Reset
    pub fn reset_i2c1(&self) {
        self.gcr.rst1.write(|w| w.i2c1().bit(true));
    }
}
