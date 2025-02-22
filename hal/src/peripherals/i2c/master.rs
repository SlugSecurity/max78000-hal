use crate::communication::{InfTimeout, Timeout};
use crate::peripherals::gpio::active::port_num_types::GpioZero;
use crate::peripherals::gpio::active::ActivePinHandle;
use crate::peripherals::gpio::pin_traits::IoPin;
use crate::peripherals::gpio::{GpioError, PinOperatingMode};
use crate::peripherals::i2c::{BusSpeed, I2CMaster, GCRI2C};
use crate::peripherals::oscillator::SystemClock;
use core::cell::{Ref, RefMut};
use embedded_hal::i2c::{ErrorKind, ErrorType, NoAcknowledgeSource, Operation, SevenBitAddress};

impl<'a, T: GCRI2C> I2CMaster<'a, T> {
    pub(crate) fn new(
        bus_speed: BusSpeed,
        system_clock: Ref<SystemClock>,
        i2c_regs: RefMut<'a, T>,
        target_addr: SevenBitAddress,
        mut scl_pin: ActivePinHandle<'a, GpioZero, 31>,
        mut sda_pin: ActivePinHandle<'a, GpioZero, 31>,
    ) -> Result<Self, GpioError> {
        scl_pin.set_operating_mode(PinOperatingMode::AltFunction1)?;
        sda_pin.set_operating_mode(PinOperatingMode::AltFunction1)?;
        i2c_regs.ctrl().modify(|_, w| w.en().bit(true));

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

        let target_speed = match bus_speed {
            BusSpeed::Standard100kbps => 100_000,
            BusSpeed::Fast400kbps => 400_000,
            BusSpeed::FastPlus1mbps => 1_000_000,
        };

        // calculations pulled from msdk
        let pclk_speed = system_clock.get_freq() / (system_clock.get_div() as u32) / 2;

        let multiplier = pclk_speed / target_speed;
        let val = multiplier / 2 - 1;

        i2c_regs.clkhi().write(|w| w.hi().variant(val as u16));
        i2c_regs.clklo().write(|w| w.lo().variant(val as u16));

        i2c_regs.ctrl().modify(|_, w| w.en().bit(true));

        Ok(Self {
            i2c_regs,
            target_addr,
            scl_pin,
            sda_pin,
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
    pub fn recv_raw<TMT: Timeout>(
        &mut self,
        read: &mut [u8],
        tmt: &mut TMT,
        rst_on_byte: bool,
        num_to_read: usize,
    ) -> Result<(), ErrorKind> {
        // Let's flush the FIFO buffers
        self.i2c_regs.clear_interrupt_flags();
        self.i2c_regs.flush_fifo();

        let bytes_to_read = if num_to_read >= 256 { 256 } else { num_to_read };
        let mut num_read = 0;

        // Write the number of data bytes to receive to the I2C receive count field (I2Cn_RXCTRL1.cnt).
        // overflowing bytes to read is safe as 256 will become 0,
        // and 0 is interpreted by the hardware as 256
        self.i2c_regs
            .rxctrl1()
            .modify(|_, w| w.cnt().variant(bytes_to_read as u8));
        // Write the I2C slave address byte to the I2Cn_FIFO register with the R/W bit set to 1
        self.i2c_regs
            .fifo()
            .write(|w| w.data().variant((self.target_addr << 1) | 1));
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

        if self.i2c_regs.intfl0().read().addr_nack_err().bit() {
            self.i2c_regs.mstctrl().modify(|_, w| w.stop().bit(true));
            return Err(ErrorKind::NoAcknowledge(NoAcknowledgeSource::Address));
        }

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
            num_read += 1;
            if rst_on_byte {
                tmt.reset()
            }
        }

        // void excess bytes
        while num_read < bytes_to_read {
            while self.i2c_regs.is_rx_fifo_empty() && !self.i2c_regs.bus_error() && !tmt.poll() {}
            if self.i2c_regs.bus_error() || tmt.poll() {
                self.i2c_regs.mstctrl().modify(|_, w| w.stop().bit(true));
                return Err(ErrorKind::Bus);
            }
            self.i2c_regs.fifo().read();
            num_read += 1;
            if rst_on_byte {
                tmt.reset()
            }
        }

        self.i2c_regs.mstctrl().modify(|_, w| w.stop().bit(true));

        Ok(())
    }

    /// Sends bytes from slice to slave specified by address.
    #[allow(clippy::while_let_on_iterator)]
    // while let is needed as this relies on only partially consuming an iterator
    // for .. in appears to consume the entire iterator
    pub fn send_raw<I: Iterator<Item = u8>>(&mut self, buffer: &mut I) -> Result<(), ErrorKind> {
        // Let's flush the FIFO buffers
        self.i2c_regs.clear_interrupt_flags();
        self.i2c_regs.flush_fifo();

        self.i2c_regs
            .intfl0()
            .modify(|_, w| w.tx_lockout().bit(true));

        // Write the I2C slave address byte to the I2Cn_FIFO register with the R/W bit set to 0
        self.i2c_regs
            .fifo()
            .write(|w| w.data().variant(self.target_addr << 1));

        // Write the desired data bytes to the I2Cn_FIFO register, up to the size of the transmit FIFO. (e.g., If the transmit
        // FIFO size is 8 bytes, the software may write one address byte and seven data bytes before starting the transaction.)
        // let mut num_written = 0;

        while !self.i2c_regs.status().read().tx_full().bit() {
            if let Some(byte) = buffer.next() {
                self.i2c_regs.fifo().write(|w| w.data().variant(byte));
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

        if self.i2c_regs.intfl0().read().addr_nack_err().bit() {
            self.i2c_regs.mstctrl().modify(|_, w| w.stop().bit(true));
            return Err(ErrorKind::NoAcknowledge(NoAcknowledgeSource::Address));
        }

        if self.i2c_regs.bus_error() {
            self.i2c_regs.mstctrl().modify(|_, w| w.stop().bit(true));
            return Err(ErrorKind::Bus);
        }

        while let Some(byte) = buffer.next() {
            while self.i2c_regs.status().read().tx_full().bit() && !self.i2c_regs.bus_error() {}
            if self.i2c_regs.bus_error() {
                self.i2c_regs.mstctrl().modify(|_, w| w.stop().bit(true));
                return Err(ErrorKind::Bus);
            }
            self.i2c_regs.fifo().write(|w| w.data().variant(byte));
        }

        // Once the software writes all the desired bytes to the I2Cn_FIFO register; the software should set either
        // I2Cn_MSTCTRL.restart or I2Cn_MSTCTRL.stop.

        self.i2c_regs.mstctrl().modify(|_, w| w.stop().bit(true));

        // Once the controller sends all the remaining bytes and empties the transmit FIFO, the hardware sets
        // I2Cn_INTFL0.done and proceeds to send out either a RESTART condition if I2Cn_MSTCTRL.restart was set, or a
        // STOP condition if I2Cn_MSTCTRL.stop was set.

        while !self.i2c_regs.intfl0().read().done().bit() && !self.i2c_regs.bus_error() {}

        if self.i2c_regs.bus_error() {
            return Err(ErrorKind::Bus);
        }

        Ok(())
    }
}

impl<T: GCRI2C> ErrorType for I2CMaster<'_, T> {
    type Error = ErrorKind;
}

impl<T: GCRI2C> embedded_hal::i2c::I2c for I2CMaster<'_, T> {
    fn read(&mut self, address: SevenBitAddress, read: &mut [u8]) -> Result<(), Self::Error> {
        let bytes_to_read = read.len();
        let old_addr = self.get_target_addr();
        self.set_target_addr(address);
        for i in 0..bytes_to_read / 256 {
            self.recv_raw(&mut read[i * 256..], &mut InfTimeout::new(), false, 256)?;
        }
        let leftover = read.len() - (read.len() % 256);
        let leftover_len = read.len() % 256;
        self.recv_raw(
            &mut read[leftover..],
            &mut InfTimeout::new(),
            false,
            leftover_len,
        )?;
        self.set_target_addr(old_addr);
        Ok(())
    }

    fn write(&mut self, address: SevenBitAddress, write: &[u8]) -> Result<(), Self::Error> {
        let old_addr = self.get_target_addr();
        self.set_target_addr(address);
        self.send_raw(&mut write.iter().copied())?;
        self.set_target_addr(old_addr);
        Ok(())
    }

    fn write_read(
        &mut self,
        address: SevenBitAddress,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.write(address, write)?;
        self.read(address, read)?;
        Ok(())
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
