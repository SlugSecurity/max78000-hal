//! Power control API.

use max78000::{GCR, LPGCR, lpgcr::{PCLKDIS, pclkdis::PCLKDIS_SPEC}, TMR3, I2C0, SPI1, DMA, GPIO1, GPIO0, TRNG};

/// Enable/disable peripheral clocks; reset peripherals.
pub struct PowerControl<'r> {
    gcr: &'r GCR,
    lpgcr: &'r LPGCR,
}

enum Module {
    LPCOMP,
    UART0,
    UART1,
    UART2,
    UART3,
    TMR0,
    TMR1,
    TMR2,
    TMR3,
    TMR4,
    TMR5,
    WDT1,
    GPIO0,
    GPIO1,
    GPIO2,
    PT,
    I2C0,
    I2C1,
    CNN,
    ADC,
    SPI1,
    DMA,
    CRC,
    OWM,
    SMPHR,
    TRNG,

}

impl<'r> PowerControl<'r> {
    // TODO: Make pub(crate)
    /// Creates a new PowerControl instance that holds references to the GCR and LPGCR registers.
    pub fn new(gcr: &'r GCR, lpgcr: &'r LPGCR) -> Self {
        Self { gcr, lpgcr }
    }

    //Enables the module from the Module enum
    pub fn enable_peripheral(&self, module_input : Module){
        match module_input {
            LPCOMP => self.lpgcr.pclkdis.write(|w| w.lpcomp().en()),
            UART3=> self.lpgcr.pclkdis.write(|w| w.uart3().en()),
            TMR5=>self.lpgcr.pclkdis.write(|w| w.tmr5().en()),
            TMR4=>self.lpgcr.pclkdis.write(|w| w.tmr4().en()),
            WDT1 => self.lpgcr.pclkdis.write(|w| w.wdt1().en()),
            GPIO2 => self.lpgcr.pclkdis.write(|w| w.gpio2().en()),
            
            PT=> self.gcr.pclkdis0.write(|w| w.pt().en()),
            IC21=> self.gcr.pclkdis0.write(|w| w.i2c0().en()),
            CNN => self.gcr.pclkdis0.write(|w| w.cnn().en()),
            ADC => self.gcr.pclkdis0.write(|w| w.adc().en()),
            TMR3 => self.gcr.pclkdis0.write(|w| w.adc().en()),
            TMR2 => self.gcr.pclkdis0.write(|w| w.tmr2().en()),
            TMR1 => self.gcr.pclkdis0.write(|w| w.tmr1().en()),
            TMR0 => self.gcr.pclkdis0.write(|w| w.tmr0().en()),
            I2C0 => self.gcr.pclkdis0.write(|w| w.i2c0().en()),
            UART1 => self.gcr.pclkdis0.write(|w| w.uart1().en()),
            UART0 => self.gcr.pclkdis0.write(|w| w.uart0().en()),
            SPI1 => self.gcr.pclkdis0.write(|w| w.spi1().en()),
            DMA => self.gcr.pclkdis0.write(|w| w.dma().en()),
            GPIO1 => self.gcr.pclkdis0.write(|w| w.gpio1().en()),
            GPIO0 => self.gcr.pclkdis0.write(|w| w.gpio0().en()),

            CRC => self.gcr.pclkdis1.write(|w| w.crc().en()),
            OWM => self.gcr.pclkdis1.write(|w| w.owm().en()),
            SMPHR => self.gcr.pclkdis1.write(|w| w.smphr().en()),
            TRNG => self.gcr.pclkdis1.write(|w| w.trng().en()),
            UART2 => self.gcr.pclkdis1.write(|w| w.uart2().en()),
        }
    }

    //Disables the module from the module enum
    pub fn disable_peripheral(&self, module_input : Module){
        match module_input {
            LPCOMP => self.lpgcr.pclkdis.write(|w| w.lpcomp().dis()),
            UART3=> self.lpgcr.pclkdis.write(|w| w.uart3().dis()),
            TMR5=>self.lpgcr.pclkdis.write(|w| w.tmr5().dis()),
            TMR4=>self.lpgcr.pclkdis.write(|w| w.tmr4().dis()),
            WDT1 => self.lpgcr.pclkdis.write(|w| w.wdt1().dis()),
            GPIO2 => self.lpgcr.pclkdis.write(|w| w.gpio2().dis()),
            
            PT=> self.gcr.pclkdis0.write(|w| w.pt().dis()),
            IC21=> self.gcr.pclkdis0.write(|w| w.i2c0().dis()),
            CNN => self.gcr.pclkdis0.write(|w| w.cnn().dis()),
            ADC => self.gcr.pclkdis0.write(|w| w.adc().dis()),
            TMR3 => self.gcr.pclkdis0.write(|w| w.adc().dis()),
            TMR2 => self.gcr.pclkdis0.write(|w| w.tmr2().dis()),
            TMR1 => self.gcr.pclkdis0.write(|w| w.tmr1().dis()),
            TMR0 => self.gcr.pclkdis0.write(|w| w.tmr0().dis()),
            I2C0 => self.gcr.pclkdis0.write(|w| w.i2c0().dis()),
            UART1 => self.gcr.pclkdis0.write(|w| w.uart1().dis()),
            UART0 => self.gcr.pclkdis0.write(|w| w.uart0().dis()),
            SPI1 => self.gcr.pclkdis0.write(|w| w.spi1().dis()),
            DMA => self.gcr.pclkdis0.write(|w| w.dma().dis()),
            GPIO1 => self.gcr.pclkdis0.write(|w| w.gpio1().dis()),
            GPIO0 => self.gcr.pclkdis0.write(|w| w.gpio0().dis()),

            CRC => self.gcr.pclkdis1.write(|w| w.crc().dis()),
            OWM => self.gcr.pclkdis1.write(|w| w.owm().dis()),
            SMPHR => self.gcr.pclkdis1.write(|w| w.smphr().dis()),
            TRNG => self.gcr.pclkdis1.write(|w| w.trng().dis()),
            UART2 => self.gcr.pclkdis1.write(|w| w.uart2().dis()),
        }
    }


    pub fn reset(&self, module_input : Module){
        match module_input{
            LPCOMP => self.lpgcr.rst.write(|w| w.lpcomp().bit(true)),
            UART3=> self.lpgcr.rst.write(|w| w.uart3().bit(true)),
            TMR5=>self.lpgcr.rst.write(|w| w.tmr5().bit(true)),
            TMR4=>self.lpgcr.rst.write(|w| w.tmr4().bit(true)),
            WDT1 => self.lpgcr.rst.write(|w| w.wdt1().bit(true)),
            GPIO2 => self.lpgcr.rst.write(|w| w.gpio2().bit(true)),
            
            PT=> self.gcr.rst0.write(|w| w.pt().bit(true)),
            IC21=> self.gcr.rst0.write(|w| w.i2c0().bit(true)),
            -CNN => self.gcr.rst0.write(|w| w.cnn().bit(true)),
            -ADC => self.gcr.rst0.write(|w| w.adc().bit(true)),
            -TMR3 => self.gcr.rst0.write(|w| w.adc().bit(true)),
            -TMR2 => self.gcr.rst0.write(|w| w.tmr2().bit(true)),
            -TMR1 => self.gcr.rst0.write(|w| w.tmr1().bit(true)),
            -TMR0 => self.gcr.rst0.write(|w| w.tmr0().bit(true)),
            -I2C0 => self.gcr.rst0.write(|w| w.i2c0().bit(true)),
            -UART1 => self.gcr.rst0.write(|w| w.uart1().bit(true)),
            -UART0 => self.gcr.rst0.write(|w| w.uart0().bit(true)),
            -SPI1 => self.gcr.rst0.write(|w| w.spi1().bit(true)),
            -DMA => self.gcr.rst0.write(|w| w.dma().bit(true)),
            -GPIO1 => self.gcr.rst0.write(|w| w.gpio1().bit(true)),
            -GPIO0 => self.gcr.rst0.write(|w| w.gpio0().bit(true)),

            CRC => self.gcr.rst1.write(|w| w.crc().bit(true)),
            OWM => self.gcr.rst1.write(|w| w.owm().bit(true)),
            SMPHR => self.gcr.rst1.write(|w| w.smphr().bit(true)),
            //TRNG => self.gcr.rst1.write(|w| w.trng().bit(true)),
            //UART2 => self.gcr.rst1.write(|w| w.uart2().bit(true)),

        }
    }
    // LPGCR_RST


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


    /// TRNG Reset
    pub fn reset_trng(&self) {
        self.gcr.rst0.write(|w| w.trng().bit(true));
    }

    /// RTC Reset
    pub fn reset_rtc(&self) {
        self.gcr.rst0.write(|w| w.rtc().bit(true));
    }



    /// Watchdog Timer 0 Reset
    pub fn reset_wdt0(&self) {
        self.gcr.rst0.write(|w| w.wdt0().bit(true));
    }


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
