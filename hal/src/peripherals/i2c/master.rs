use crate::communication::Timeout;
use crate::peripherals::gpio::GpioError;
use crate::peripherals::i2c::{BusSpeed, I2CMaster, GCRI2C};
use crate::peripherals::oscillator::SystemClock;
use core::cell::{Ref, RefMut};
use core::time::Duration;
use embedded_hal::i2c::{ErrorKind, ErrorType, Operation, SevenBitAddress};

impl<'a, T: GCRI2C> I2CMaster<'a, T> {
    pub(crate) fn new(
        bus_speed: BusSpeed,
        system_clock: Ref<SystemClock>,
        i2c_regs: RefMut<'a, T>,
    ) -> Result<Self, GpioError> {
        i2c_regs.ctrl().modify(|_, w| w.en().bit(true));

        // TODO: configure
        i2c_regs.ctrl().modify(|_, w| {
            w.mst_mode()
                .bit(true)
                .gc_addr_en()
                .bit(false)
                .irxm_en()
                .bit(false)
                .clkstr_dis()
                .bit(false)
                .hs_en()
                .bit(false)
                .en()
                .bit(true)
                .bb_mode()
                .bit(false)
        });

        /*i2c_regs
        .timeout()
        .modify(|_, w| w.scl_to_val().variant(0xffff));*/
        // i2c_regs.ctrl().modify(|_, w| w.scl_out().bit(false));

        let target_speed = match bus_speed {
            BusSpeed::Standard100kbps => 100_000,
            BusSpeed::Fast400kbps => 400_000,
            BusSpeed::FastPlus1mbps => 1_000_000,
        };

        let pclk_speed = system_clock.get_freq() / (system_clock.get_div() as u32) / 2;

        let multiplier = pclk_speed / target_speed;
        let val = multiplier / 2 - 1;

        unsafe {
            i2c_regs.clkhi().modify(|_, w| w.bits(val));

            i2c_regs.clklo().modify(|_, w| w.bits(val));
        }

        i2c_regs.ctrl().modify(|_, w| w.en().bit(true));

        Ok(Self {
            i2c_regs,
            target_addr: 255,
        })
    }

    /// Configure the address of the target for comm stack traits
    pub fn set_target_addr(&mut self, addr: SevenBitAddress) {
        self.target_addr = addr;
    }

    /// Get the address of the target for comm stack traits
    pub fn get_target_addr(&self) -> SevenBitAddress {
        self.target_addr
    }

    /// Reads up to 256 bytes to read slice, in single i2c transaction
    pub fn master_recv<TMT: Timeout>(
        &mut self,
        address: SevenBitAddress,
        read: &mut [u8],
        tmt: &mut TMT,
        rst_on_byte: bool,
    ) -> Result<(), ErrorKind> {
        // Let's flush the FIFO buffers
        self.i2c_regs.clear_interrupt_flags();
        self.i2c_regs.flush_fifo();

        let bytes_to_read = if read.len() >= 256 { 256 } else { read.len() };

        // Write the number of data bytes to receive to the I2C receive count field (I2Cn_RXCTRL1.cnt).
        self.i2c_regs
            .rxctrl1()
            .modify(|_, w| w.cnt().variant(bytes_to_read as u8));
        // Write the I2C slave address byte to the I2Cn_FIFO register with the R/W bit set to 1
        self.i2c_regs
            .fifo()
            .write(|w| w.data().variant((address << 1) | 1));
        // Send a START condition by setting I2Cn_MSTCTRL.start = 1
        self.i2c_regs
            .mstctrl()
            .modify(|_, w| w.start().variant(true));
        // The slave address is transmitted by the controller from the I2Cn_FIFO register.
        // The I2C controller receives an ACK from the slave, and the controller sets the address ACK interrupt flag
        // (I2Cn_INTFL0.addr_ack = 1).
        while !self.i2c_regs.intfl0().read().addr_ack().bit()
            && !self.i2c_regs.bus_error()
            && !tmt.poll()
        {}

        if self.i2c_regs.bus_error() || tmt.poll() {
            self.i2c_regs.mstctrl().modify(|_, w| w.stop().bit(true));
            return Err(ErrorKind::Bus);
        }
        // The I2C controller receives data from the slave and automatically ACKs each byte. The software must retrieve this
        // data by reading the I2Cn_FIFO register.
        for cell in read.iter_mut().take(bytes_to_read) {
            while self.i2c_regs.is_rx_fifo_empty() && !self.i2c_regs.bus_error() && !tmt.poll() {}
            if self.i2c_regs.bus_error() || tmt.poll() {
                self.i2c_regs.mstctrl().modify(|_, w| w.stop().bit(true));
                return Err(ErrorKind::Bus);
            }
            *cell = self.i2c_regs.fifo().read().data().bits();
            if rst_on_byte {
                tmt.reset()
            }
        }

        self.i2c_regs.mstctrl().modify(|_, w| w.stop().bit(true));

        Ok(())
    }

    /// Sends bytes from slice to slave specified by address.
    #[allow(clippy::while_let_on_iterator)]
    pub fn master_send<I: Iterator<Item = u8>>(
        &mut self,
        address: SevenBitAddress,
        write: &mut I,
    ) -> Result<(), ErrorKind> {
        // Let's flush the FIFO buffers
        self.i2c_regs.clear_interrupt_flags();
        self.i2c_regs.flush_fifo();

        self.i2c_regs
            .intfl0()
            .modify(|_, w| w.tx_lockout().bit(true));

        // Write the I2C slave address byte to the I2Cn_FIFO register with the R/W bit set to 0
        self.i2c_regs
            .fifo()
            .write(|w| w.data().variant(address << 1));

        // Write the desired data bytes to the I2Cn_FIFO register, up to the size of the transmit FIFO. (e.g., If the transmit
        // FIFO size is 8 bytes, the software may write one address byte and seven data bytes before starting the transaction.)
        // let mut num_written = 0;

        while !self.i2c_regs.status().read().tx_full().bit() {
            if let Some(byte) = write.next() {
                self.i2c_regs.fifo().write(|w| w.data().variant(byte));
                // num_written += 1;
            } else {
                break;
            }
        }

        // Send a START condition by setting I2Cn_MSTCTRL.start = 1
        self.i2c_regs
            .mstctrl()
            .modify(|_, w| w.start().variant(true));

        // The controller transmits the slave address byte written to the I2Cn_FIFO register

        // The I2C controller receives an ACK from the slave, and the controller sets the address ACK interrupt flag
        // (I2Cn_INTFL0.addr_ack = 1).
        // TODO: add operation timeouts using timer module

        // poll addr_ack
        while !self.i2c_regs.intfl0().read().addr_ack().bit() && !self.i2c_regs.bus_error() {}

        if self.i2c_regs.bus_error() {
            self.i2c_regs.mstctrl().modify(|_, w| w.stop().bit(true));
            return Err(ErrorKind::Bus);
        }

        while let Some(byte) = write.next() {
            while self.i2c_regs.status().read().tx_full().bit() && !self.i2c_regs.bus_error() {}
            if self.i2c_regs.bus_error() {
                return Err(ErrorKind::Bus);
            }
            self.i2c_regs.fifo().write(|w| w.data().variant(byte));
            // num_written += 1;
        }

        // Once the software writes all the desired bytes to the I2Cn_FIFO register; the software should set either
        // I2Cn_MSTCTRL.restart or I2Cn_MSTCTRL.stop.

        self.i2c_regs.mstctrl().modify(|_, w| w.stop().bit(true));

        // Once the controller sends all the remaining bytes and empties the transmit FIFO, the hardware sets
        // I2Cn_INTFL0.done and proceeds to send out either a RESTART condition if I2Cn_MSTCTRL.restart was set, or a
        // STOP condition if I2Cn_MSTCTRL.stop was set.

        while !self.i2c_regs.intfl0().read().done().bit() {}

        Ok(())
    }
}

impl<'a, T: GCRI2C> ErrorType for I2CMaster<'a, T> {
    type Error = ErrorKind;
}

/// Dummy infinite timeout struct.
pub struct InfTimeout {}

impl InfTimeout {
    /// Create a new instance of an infinite timeout
    pub fn new() -> Self {
        InfTimeout {}
    }
}

impl Default for InfTimeout {
    fn default() -> Self {
        Self::new()
    }
}

impl Timeout for InfTimeout {
    fn poll(&mut self) -> bool {
        false
    }

    fn reset(&mut self) {}

    fn duration(&self) -> Duration {
        Duration::new(0, 0)
    }
}

impl<'a, T: GCRI2C> embedded_hal::i2c::I2c for I2CMaster<'a, T> {
    fn read(&mut self, address: SevenBitAddress, read: &mut [u8]) -> Result<(), Self::Error> {
        let bytes_to_read = read.len();
        for i in 0..bytes_to_read / 256 {
            self.master_recv(address, &mut read[i * 256..], &mut InfTimeout::new(), false)?;
        }
        let leftover = read.len() - (read.len() % 256);
        self.master_recv(
            address,
            &mut read[leftover..],
            &mut InfTimeout::new(),
            false,
        )?;
        Ok(())
    }

    fn write(&mut self, address: SevenBitAddress, write: &[u8]) -> Result<(), Self::Error> {
        self.master_send(address, &mut write.iter().copied())
    }

    fn write_read(
        &mut self,
        address: SevenBitAddress,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        //free(|_| -> Result<(), Self::Error> {
        self.write(address, write)?;
        self.read(address, read)?;
        Ok(())
        //})
    }

    fn transaction(
        &mut self,
        address: SevenBitAddress,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        for operation in operations.iter_mut() {
            match operation {
                Operation::Read(read) => {
                    self.read(address, read)?;
                }
                Operation::Write(write) => {
                    self.write(address, write)?;
                }
            }
        }
        Ok(())
    }
}
