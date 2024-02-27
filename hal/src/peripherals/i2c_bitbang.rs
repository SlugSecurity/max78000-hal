//! uh oh

use crate::peripherals::i2c::SlavePollResult::{Received, TransmitNeeded};
use core::ops::Deref;
use cortex_m::asm::delay;
use embedded_hal;
use embedded_hal::i2c::{ErrorKind, ErrorType, NoAcknowledgeSource, Operation, SevenBitAddress};
use max78000::{i2c0, GCR, tmr};
use max78000::{I2C0, I2C1, I2C2};
use crate::peripherals::timer::{Timer, TimerPeripheralGCR};
// use cortex_m::interrupt::free;

pub trait BBGCRI2C {
    /// Disable peripheral
    fn peripheral_clock_disable(gcr_reg: &GCR);
    /// Enable peripheral
    fn peripheral_clock_enable(gcr_reg: &GCR);
    /// Reset the peripheral
    fn reset_peripheral(gcr_reg: &GCR);
    /// Do not drive SCL (set pin high-impedance)
    fn set_scl(&mut self);
    /// Actively drive SCL signal low
    fn clear_scl(&mut self);
    /// Do not drive SDA (set pin high-impedance)
    fn set_sda(&mut self);
    /// Actively drive SDA signal low
    fn clear_sda(&mut self);
    /// Read SDA state
    fn read_sda(&self) -> bool;
    /// Read SCL state
    fn read_scl(&self) -> bool;
}

macro_rules! gen_impl_gcri2c {
    ($register:ty, $lowercaseName:ident, $rstReg:ident, $pclkdisReg:ident) => {
        impl BBGCRI2C for $register {
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

            fn set_scl(&mut self) {
                self.ctrl().modify(|_, w| w.scl_out().bit(true))
            }
            fn clear_scl(&mut self) {
                self.ctrl().modify(|_, w| w.scl_out().bit(false))
            }
            fn set_sda(&mut self) {
                self.ctrl().modify(|_, w| w.sda_out().bit(true))
            }
            fn clear_sda(&mut self) {
                self.ctrl().modify(|_, w| w.sda_out().bit(false))
            }
            fn read_scl(&self) -> bool {
                self.ctrl().read().scl().bit()
            }
            fn read_sda(&self) -> bool {
                self.ctrl().read().sda().bit()
            }
        }
    };
}

gen_impl_gcri2c!(I2C0, i2c0, rst0, pclkdis0);
gen_impl_gcri2c!(I2C1, i2c1, rst1, pclkdis0);
gen_impl_gcri2c!(I2C2, i2c2, rst1, pclkdis1);

/*
/// Master or slave, in the slave case provide a 7 bit address to be used
/// given it's a u8, the topmost bit will be ignored
pub enum I2CMode {
    /// Function as a master
    Master,
    /// Function as a slave
    Slave(u8)
}*/

/// The result of calling slave_poll, Received indicates how many bytes have been read,
/// and if bytes had to be dropped due to exceeding the buffer size
///
/// TransmitNeeded indicates you need to call slave_send with the data needed
pub enum SlavePollResult {
    Received(u32, bool),
    TransmitNeeded,
}

pub struct I2CMaster<'a, T: Deref<Target=i2c0::RegisterBlock> + BBGCRI2C, R: Sized + Deref<Target=tmr::RegisterBlock> + TimerPeripheralGCR> {
    i2c_regs: T,
    timer: Timer<'a, R>,
    started: bool
}

pub struct I2CSlave<T: Deref<Target=i2c0::RegisterBlock> + BBGCRI2C> {
    i2c_regs: T
}

impl<T: Deref<Target=i2c0::RegisterBlock> + BBGCRI2C> I2CSlave<T> {
    pub fn new(gcr_regs: &GCR, i2c_regs: T, address: u8) -> Self {
        T::reset_peripheral(gcr_regs);
        T::peripheral_clock_enable(gcr_regs);

        i2c_regs.ctrl().modify(|_, w| {
            w.mst_mode().bit(false)
                .en().bit(true)
                .bb_mode().bit(true)
                .scl_out().bit(true)
                .sda_out().bit(true)
        });

        Self {i2c_regs}
    }

    pub fn poll(&mut self) {

    }
}

// TODO: write code to initialize relevant registers for both master and slave operation

impl<'a, T: Deref<Target = i2c0::RegisterBlock> + BBGCRI2C, R: Sized + Deref<Target = tmr::RegisterBlock> + TimerPeripheralGCR> I2CMaster<'a, T, R> {
    pub fn new(gcr_regs: &GCR, i2c_regs: T, timer: Timer<'a, R>) -> Self {
        T::reset_peripheral(gcr_regs);
        T::peripheral_clock_enable(gcr_regs);

        i2c_regs.ctrl().modify(|_, w| {
            w.mst_mode().bit(true)
                .bb_mode().bit(true)
                .en().bit(true)
                .scl_out().bit(true)
                .sda_out().bit(true)
        });

        Self { i2c_regs, timer, started: false }
    }

    fn delay(&mut self) {
        delay(100);
        return;
        self.timer.reset();
        while !self.timer.poll() {}
    }

    fn start_cond(&mut self) -> Result<(), ErrorKind> {
        if self.started {
            // if started, do a restart condition
            self.i2c_regs.set_sda();
            self.delay();
            self.i2c_regs.set_scl();
            // clock stretching
            self.clock_stretch()?;
            self.delay();
        }
        if !self.i2c_regs.read_sda() {
            return Err(ErrorKind::ArbitrationLoss)
        }
        self.i2c_regs.clear_sda();
        self.delay();
        self.i2c_regs.clear_scl();
        self.started = true;
        Ok(())
    }

    fn stop_cond(&mut self) -> Result<(), ErrorKind> {
        self.i2c_regs.clear_sda();
        self.delay();
        self.i2c_regs.set_scl();

        // clock stretching
        self.clock_stretch()?;
        self.delay();
        self.i2c_regs.set_sda();
        self.delay();
        if !self.i2c_regs.read_sda() {
            return Err(ErrorKind::ArbitrationLoss)
        }

        self.started = false;
        Ok(())
    }

    fn write_bit(&mut self, bit: bool) -> Result<(), ErrorKind> {
        if bit {
            self.i2c_regs.set_sda();
        } else {
            self.i2c_regs.clear_sda();
        }

        self.delay();

        self.i2c_regs.set_scl();

        self.delay();

        // clock stretching
        self.clock_stretch()?;

        if bit && !self.i2c_regs.read_sda() {
            return Err(ErrorKind::ArbitrationLoss)
        }

        self.i2c_regs.clear_scl();
        Ok(())
    }

    fn clock_stretch(&mut self) -> Result<(), ErrorKind> {
        while !self.i2c_regs.read_scl() {
            // TODO: add timeout
        }
        Ok(())
    }

    fn read_bit(&mut self) -> Result<bool, ErrorKind> {
        self.i2c_regs.set_sda();
        self.delay();
        self.i2c_regs.set_scl();

        // clock stretching
        self.clock_stretch()?;

        self.delay();

        let bit = self.i2c_regs.read_sda();

        self.i2c_regs.clear_scl();

        Ok(bit)
    }

    fn write_byte(&mut self, byte: u8) -> Result<(), ErrorKind> {
        for b in 0..8 {
            self.write_bit(byte & (1 << b) != 0)?;
            if self.read_bit()? {
                return Err(ErrorKind::NoAcknowledge(NoAcknowledgeSource::Unknown))
            }
        }
        Ok(())
    }

    fn read_byte(&mut self, nack: bool) -> Result<u8, ErrorKind> {
        let mut byte = 0;
        for _ in 0..8 {
            byte = (byte << 1) | if self.read_bit()? {1} else {0};
        }
        self.write_bit(nack)?;
        Ok(byte)
    }

    fn master_recv(&mut self, address: SevenBitAddress, read: &mut [u8]) -> Result<(), ErrorKind> {
        self.start_cond()?;
        self.write_byte((address << 1) | 1)?;

        for cell in read.iter_mut() {
            *cell = self.read_byte(false)?;
        }

        self.stop_cond()?;

        Ok(())
    }

    fn master_send(&mut self, address: SevenBitAddress, write: &[u8]) -> Result<(), ErrorKind> {
        self.start_cond()?;

        self.write_byte(address << 1)?;

        for byte in write.iter() {
            self.write_byte(*byte)?;
        }

        self.stop_cond()?;

        Ok(())
    }
}

impl<'a, T: Deref<Target = i2c0::RegisterBlock> + BBGCRI2C, R: Sized + Deref<Target = tmr::RegisterBlock> + TimerPeripheralGCR> ErrorType for I2CMaster<'a, T, R> {
    type Error = ErrorKind;
}

impl<'a, T: Deref<Target = i2c0::RegisterBlock> + BBGCRI2C, R: Sized + Deref<Target = tmr::RegisterBlock> + TimerPeripheralGCR> embedded_hal::i2c::I2c for I2CMaster<'a, T, R> {
    fn read(&mut self, address: SevenBitAddress, read: &mut [u8]) -> Result<(), Self::Error> {
        self.master_recv(address, read)?;
        Ok(())
    }

    fn write(&mut self, address: SevenBitAddress, write: &[u8]) -> Result<(), Self::Error> {
        self.master_send(address, write)
    }

    fn write_read(&mut self, address: SevenBitAddress, write: &[u8], read: &mut [u8]) -> Result<(), Self::Error> {
        //free(|_| -> Result<(), Self::Error> {
        self.write(address, write)?;
        self.read(address, read)?;
        Ok(())
        //})
    }

    fn transaction(&mut self, address: SevenBitAddress, operations: &mut [Operation<'_>]) -> Result<(), Self::Error> {
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


