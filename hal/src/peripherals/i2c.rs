//! I2C Peripheral Drivers

use core::cell::RefMut;
use core::ops::Deref;
use max78000::i2c0;
use max78000::{I2C0, I2C1, I2C2};

/// Implementation of the comm stack traits
pub mod comm;
/// Implementation of master mode
pub mod master;
/// Implementation of slave mode
pub mod slave;

/// Auxiliary trait that only the I2C0, I2C1, and I2C2 registers can implement;
/// Allows peripheral toggle and reset functionality to said peripherals if GCR regs
/// are provided.
pub trait GCRI2C: Deref<Target = i2c0::RegisterBlock> {
    /// Flush transmit and receive FIFO
    fn flush_fifo(&mut self);
    /// Flush only the receive FIFO
    fn flush_rx_fifo(&mut self);
    /// Is receive FIFO full?
    fn is_rx_fifo_full(&self) -> bool;
    /// Is receive FIFO empty?
    fn is_rx_fifo_empty(&self) -> bool;
    /// Is transmit FIFO full?
    fn is_tx_fifo_full(&self) -> bool;
    /// Is transmit FIFO empty?
    fn is_tx_fifo_empty(&self) -> bool;
    /// Clear interrupt flags
    fn clear_interrupt_flags(&mut self);
    /// Has a bus timeout occurred?
    fn bus_timeout(&self) -> bool;
    /// Is there a bus error?
    fn bus_error(&self) -> bool;
}

macro_rules! gen_impl_gcri2c {
    ($register:ty, $lowercaseName:ident, $rstReg:ident, $pclkdisReg:ident) => {
        impl GCRI2C for $register {
            fn flush_fifo(&mut self) {
                self.rxctrl0().modify(|_, w| w.flush().bit(true));
                self.txctrl0().modify(|_, w| w.flush().bit(true));
                while self.rxctrl0().read().flush().bit() || self.txctrl0().read().flush().bit() {}
            }
            fn flush_rx_fifo(&mut self) {
                self.rxctrl0().modify(|_, w| w.flush().bit(true));
                while self.rxctrl0().read().flush().bit() {}
            }
            fn is_rx_fifo_empty(&self) -> bool {
                self.status().read().rx_em().bit()
            }
            fn is_rx_fifo_full(&self) -> bool {
                self.status().read().rx_full().bit()
            }
            fn is_tx_fifo_empty(&self) -> bool {
                self.status().read().tx_em().bit()
            }
            fn is_tx_fifo_full(&self) -> bool {
                self.status().read().tx_full().bit()
            }
            fn bus_timeout(&self) -> bool {
                self.intfl0().read().to_err().bit()
            }
            fn bus_error(&self) -> bool {
                self.intfl0().read().data_err().bit()
                    || self.intfl0().read().addr_nack_err().bit()
                    //|| self.intfl0().read().to_err().bit()
                    || self.intfl0().read().stop_err().bit()
                    || self.intfl0().read().start_err().bit()
                    || self.intfl0().read().dnr_err().bit()
                    || self.intfl0().read().arb_err().bit()
            }
            fn clear_interrupt_flags(&mut self) {
                self.intfl0().modify(|_, w| {
                    w.wr_addr_match()
                        .bit(true)
                        .rd_addr_match()
                        .bit(true)
                        .tx_lockout()
                        .bit(true)
                        .stop_err()
                        .bit(true)
                        .start_err()
                        .bit(true)
                        .dnr_err()
                        .bit(true)
                        .data_err()
                        .bit(true)
                        .addr_nack_err()
                        .bit(true)
                        .to_err()
                        .bit(true)
                        .arb_err()
                        .bit(true)
                        .addr_ack()
                        .bit(true)
                        .stop()
                        .bit(true)
                        .rx_thd()
                        .bit(true)
                        .addr_match()
                        .bit(true)
                        .gc_addr_match()
                        .bit(true)
                        .irxm()
                        .bit(true)
                        .done()
                        .bit(true)
                });
            }
        }
    };
}

gen_impl_gcri2c!(I2C0, i2c0, rst0, pclkdis0);
gen_impl_gcri2c!(I2C1, i2c1, rst1, pclkdis0);
gen_impl_gcri2c!(I2C2, i2c2, rst1, pclkdis1);

/// The result of calling slave_poll, Received indicates how many bytes have been read,
/// and if bytes had to be dropped due to exceeding the buffer size
///
/// TransmitNeeded indicates you need to call slave_send with the data needed
pub enum SlavePollResult {
    /// Received #bytes and if given read buffer length was exceeded
    IncomingTransmission,
    /// The peripheral is currently clock stretching and a transmit operation
    /// is required ASAP
    TransmitNeeded,
}

/// Various I2C bus speeds
pub enum BusSpeed {
    /// Standard mode - 100kbps or 100khz
    Standard100kbps,
    /// Fast mode - 400kbps or 400khz
    Fast400kbps,
    /// Fast plus mode - 1mbps or 1mhz
    FastPlus1mbps,
}

/// An I2C peripheral operating as a master.
/// Important: Bus arbitration is not supported, so there can only be one
/// master on the bus
pub struct I2CMaster<'a, T: GCRI2C> {
    i2c_regs: RefMut<'a, T>,
    target_addr: u8,
}

/// An I2C peripheral operating as a slave.
pub struct I2CSlave<'a, T: GCRI2C> {
    i2c_regs: RefMut<'a, T>,
}
