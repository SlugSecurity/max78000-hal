use core::ops::Deref;
use embedded_hal;
use embedded_hal::i2c::{ErrorType, Operation, SevenBitAddress};
use max78000::i2c0::rxctrl0::FLUSH_A;
use max78000::{i2c0, GCR};
use max78000::{I2C0, I2C1, I2C2};

pub enum I2CError {
    BadRead,
    Misc,
}

struct I2C<T: Deref<Target = i2c0::RegisterBlock>> {
    i2c_regs: T,
}

impl<T: Deref<Target = i2c0::RegisterBlock>> I2C<T> {
    pub fn new(gcr_regs: &GCR, i2c_regs: T) -> Self {
        Self { i2c_regs }
    }

    /*fn read_small(&mut self, address: SevenBitAddress, read: &mut [u8]) {
        // TODO: make safe for read.len() > 255
        // Write the number of data bytes to receive to the I2C receive count field (I2Cn_RXCTRL1.cnt).
        self.i2c_regs.rxctrl1().modify(|_, w| w.cnt().variant(read.len() as u8));
        // Write the I2C slave address byte to the I2Cn_FIFO register with the R/W bit set to 1
        // TODO: figure out what the r/w bit means
        self.i2c_regs.fifo().write(|w| w.data().variant(address));
        // Send a START condition by setting I2Cn_MSTCTRL.start = 1
        self.i2c_regs.mstctrl().modify(|_, w| w.start().variant(true));
        // The slave address is transmitted by the controller from the I2Cn_FIFO register.
        // The I2C controller receives an ACK from the slave, and the controller sets the address ACK interrupt flag
        // (I2Cn_INTFL0.addr_ack = 1).
        // The I2C controller receives data from the slave and automatically ACKs each byte. The software must retrieve this
        // data by reading the I2Cn_FIFO register.
        // TODO: make safe for 0xff, fifo full or empty
        for i in 0..read.len() {
            let byte = self.i2c_regs.fifo().read().data().bits();
            read[i] = byte;
        }
    }*/
}

impl<T: Deref<Target = i2c0::RegisterBlock>> ErrorType for I2C<T> {
    type Error = I2CError;
}

impl<T: Deref<Target = i2c0::RegisterBlock>> embedded_hal::i2c::I2c for I2C<T> {
    fn read(&mut self, address: SevenBitAddress, read: &mut [u8]) -> Result<(), Self::Error> {
        // TODO: strengthen against failure, write logic for if slave
        // Let's flush the FIFO buffers
        self.i2c_regs.rxctrl0().modify(|_, w| w.flush().bit(true));
        self.i2c_regs.txctrl0().modify(|_, w| w.flush().bit(true));

        // stall until flush completes
        while self.i2c_regs.rxctrl0().read().flush().bit()
            || self.i2c_regs.txctrl0().read().flush().bit()
        {}

        for offset in 0..read.len() / 255 {
            // Write the number of data bytes to receive to the I2C receive count field (I2Cn_RXCTRL1.cnt).
            self.i2c_regs.rxctrl1().modify(|_, w| w.cnt().variant(255));
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
            // The I2C controller receives data from the slave and automatically ACKs each byte. The software must retrieve this
            // data by reading the I2Cn_FIFO register.
            for i in 0..read.len() {
                let byte = self.i2c_regs.fifo().read().data().bits();
                read[offset * 255 + i] = byte;
            }
        }
        self.i2c_regs
            .rxctrl1()
            .modify(|_, w| w.cnt().variant((read.len() % 255) as u8));
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
        // The I2C controller receives data from the slave and automatically ACKs each byte. The software must retrieve this
        // data by reading the I2Cn_FIFO register.
        for i in 0..read.len() % 255 {
            let byte = self.i2c_regs.fifo().read().data().bits();
            read[(read.len() - read.len() % 255) + i] = byte;
        }
        Ok(())
    }

    fn write(&mut self, address: SevenBitAddress, write: &[u8]) -> Result<(), Self::Error> {
        todo!()
    }

    fn write_read(
        &mut self,
        address: SevenBitAddress,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn transaction(
        &mut self,
        address: SevenBitAddress,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        todo!()
    }
}
