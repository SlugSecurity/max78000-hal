use core::ops::Deref;
use embedded_hal;
use embedded_hal::i2c::{ErrorKind, ErrorType, Operation, SevenBitAddress};
use max78000::{i2c0, GCR};
use max78000::{I2C0, I2C1, I2C2};

pub trait GCRI2C {
    /// Disable peripheral
    fn peripheral_clock_disable(gcr_reg: &GCR);
    /// Enable peripheral
    fn peripheral_clock_enable(gcr_reg: &GCR);
    /// Reset the peripheral
    fn reset_peripheral(gcr_reg: &GCR);
}

macro_rules! gen_impl_gcri2c {
    ($register:ty, $lowercaseName:ident, $rstReg:ident, $pclkdisReg:ident) => {
        impl GCRI2C for $register {
            fn peripheral_clock_disable(gcr_reg: &GCR) {
                gcr_reg
                    .$pclkdisReg()
                    .modify(|_, w| w.$lowercaseName().bit(true))
            }
            fn peripheral_clock_enable(gcr_reg: &GCR) {
                gcr_reg
                    .$pclkdisReg()
                    .modify(|_, w| w.$lowercaseName().bit(false))
            }
            fn reset_peripheral(gcr_reg: &GCR) {
                gcr_reg
                    .$rstReg()
                    .modify(|_, w| w.$lowercaseName().bit(true));
                while gcr_reg.$rstReg().read().$lowercaseName().bit() {}
            }
        }
    };
}

gen_impl_gcri2c!(I2C0, i2c0, rst0, pclkdis0);
gen_impl_gcri2c!(I2C1, i2c1, rst1, pclkdis0);
gen_impl_gcri2c!(I2C2, i2c2, rst1, pclkdis1);

/// Master or slave, in the slave case provide a 7 bit address to be used
/// given it's a u8, the topmost bit will be ignored
pub enum I2CMode {
    /// Function as a master
    Master,
    /// Function as a slave
    Slave(u8)
}

struct I2C<T: Deref<Target = i2c0::RegisterBlock> + GCRI2C> {
    i2c_regs: T,
}

impl<T: Deref<Target = i2c0::RegisterBlock> + GCRI2C> I2C<T> {
    pub fn new(gcr_regs: &GCR, i2c_regs: T) -> Self {
        T::peripheral_clock_enable(gcr_regs);
        T::reset_peripheral(gcr_regs);
        Self { i2c_regs }
    }

    // Reads up to 256 bytes to read slice, in single i2c transaction
    fn master_recv(&mut self, address: SevenBitAddress, read: &mut [u8]) -> Result<(), ErrorKind> {
        // Let's flush the FIFO buffers
        self.i2c_regs.rxctrl0().modify(|_, w| w.flush().bit(true));
        self.i2c_regs.txctrl0().modify(|_, w| w.flush().bit(true));

        // stall until flush completes
        while self.i2c_regs.rxctrl0().read().flush().bit()
            || self.i2c_regs.txctrl0().read().flush().bit()
        {}

        let bytes_to_read = if read.len() >= 256 {256} else {read.len()};

        // Write the number of data bytes to receive to the I2C receive count field (I2Cn_RXCTRL1.cnt).
        self.i2c_regs.rxctrl1().modify(|_, w| w.cnt().variant(bytes_to_read as u8));
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
        for cell in read.iter_mut().take(bytes_to_read) {
            *cell = self.i2c_regs.fifo().read().data().bits();
        }
        Ok(())
    }
}

impl<T: Deref<Target = i2c0::RegisterBlock> + GCRI2C> ErrorType for I2C<T> {
    type Error = ErrorKind;
}

impl<T: Deref<Target = i2c0::RegisterBlock> + GCRI2C> embedded_hal::i2c::I2c for I2C<T> {
    fn read(&mut self, address: SevenBitAddress, read: &mut [u8]) -> Result<(), Self::Error> {
        let bytes_to_read = read.len();
        for i in 0..bytes_to_read/256 {
            self.master_recv(address, &mut read[i*256..])?;
        }
        let leftover = read.len() - (read.len() % 256);
        self.master_recv(address, &mut read[leftover..])?;
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
