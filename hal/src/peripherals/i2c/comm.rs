use crate::communication::lower_layers::framing::{Frame, FramedTxChannel};
use crate::communication::{CommunicationError, RxChannel, Timeout};
use crate::peripherals::i2c::master::InfTimeout;
use crate::peripherals::i2c::{I2CMaster, I2CSlave, SlavePollResult, GCRI2C};
use cortex_m::asm::delay;
use embedded_hal::i2c::{ErrorKind, I2c, SevenBitAddress};

static MASTER_DELAY: u32 = 10;

/// Allows for bounded transmit and receive operations
pub trait BoundedTransmitMaster {
    /// Send bytes, letting the slave know in advance how many there are
    fn send_bounded(address: SevenBitAddress, buf: &[u8]) -> Result<(), ErrorKind>;
    /// Request bytes, letting the slave know in advance how many you require
    fn request_bounded(address: SevenBitAddress, write_buf: &[u8]) -> Result<(), ErrorKind>;
}

/// Result of polling for a master request
pub enum MasterRequest {
    /// The master is sending u32 bytes over
    Receive(u32),
    /// The master wants u32 bytes sent over
    Send(u32),
}

/// Allows for bounded transmit and receive operations
pub trait BoundedTransmitSlave {
    /// Send bytes to master
    fn reply_send_bounded(buf: &[u8]) -> Result<(), ErrorKind>;
    /// Receive bytes from master
    fn receive_bounded(write_buf: &[u8]) -> Result<(), ErrorKind>;
    /// Poll for a master request
    fn poll_bounded() -> Result<MasterRequest, ErrorKind>;
}

impl<'a, T: GCRI2C> RxChannel for I2CSlave<'a, T> {
    fn recv_with_data_timeout<R: Timeout>(
        &mut self,
        dest: &mut [u8],
        tmr: &mut R,
    ) -> crate::communication::Result<usize> {
        if let Ok(SlavePollResult::IncomingTransmission) = self.slave_poll(tmr) {
            tmr.reset();
            let mut bytes_sent_buf = [0u8; 4];
            if let Ok((n, _)) = self.slave_recv(&mut bytes_sent_buf, tmr, true) {
                if n != 4 {
                    return Err(CommunicationError::CustomError(1));
                }
                let mut num_read = 0;
                return if let Ok(SlavePollResult::IncomingTransmission) = self.slave_poll(tmr) {
                    if let Ok((n, _)) = self.slave_recv(dest, tmr, true) {
                        num_read += n;
                        Ok(num_read as usize)
                    } else {
                        Err(CommunicationError::CustomError(2))
                    }
                } else {
                    Err(CommunicationError::CustomError(3))
                };
            }
        }
        Err(CommunicationError::CustomError(4))
    }

    fn recv_with_timeout<R: Timeout>(
        &mut self,
        dest: &mut [u8],
        tmr: &mut R,
    ) -> crate::communication::Result<usize>
    where
        R: Timeout,
    {
        // TODO: do not duplicate code
        if let Ok(SlavePollResult::IncomingTransmission) = self.slave_poll(tmr) {
            tmr.reset();
            let mut bytes_sent_buf = [0u8; 4];
            if let Ok((n, _)) = self.slave_recv(&mut bytes_sent_buf, tmr, false) {
                if n != 4 {
                    return Err(CommunicationError::CustomError(1));
                }
                // let num_incoming = u32::from_le_bytes(bytes_sent_buf);
                let mut num_read = 0;
                if let Ok(SlavePollResult::IncomingTransmission) = self.slave_poll(tmr) {
                    return if let Ok((n, _)) = self.slave_recv(dest, tmr, true) {
                        num_read += n;
                        Ok(num_read as usize)
                    } else {
                        Err(CommunicationError::CustomError(2))
                    };
                } else {
                    return Err(CommunicationError::CustomError(3));
                }
                /*for i in 0..(num_incoming as usize >> 8) {
                    if let Ok(SlavePollResult::IncomingTransmission) = self.slave_poll(tmr) {
                        if let Ok((n, _)) = self.slave_recv(&mut dest[i * 256..], tmr, false) {
                            num_read += n;
                        } else {
                            return Err(CommunicationError::CustomError(4));
                        }
                    } else {
                        return Err(CommunicationError::CustomError(num_incoming));
                    }
                }
                let remaining = num_incoming % 256;
                if remaining > 0 {
                    if let Ok(SlavePollResult::IncomingTransmission) = self.slave_poll(tmr) {
                        if let Ok((n, _)) = self.slave_recv(
                            &mut dest[(num_incoming as usize >> 8 << 8)..],
                            tmr,
                            false,
                        ) {
                            num_read += n;
                            return Ok(num_read as usize);
                        } else {
                            return Err(CommunicationError::CustomError(1));
                        }
                    } else {
                        return Err(CommunicationError::CustomError(2));
                    }
                }
                return Ok(num_read as usize); */
            }
        }
        Err(CommunicationError::CustomError(4))
    }
}

impl<'a, T: GCRI2C> RxChannel for I2CMaster<'a, T> {
    fn recv_with_data_timeout<TMT: Timeout>(
        &mut self,
        dest: &mut [u8],
        tmr: &mut TMT,
    ) -> crate::communication::Result<usize> {
        // TODO: remove unwraps
        let mut bytes_sent_buf = [0u8; 4];
        delay(MASTER_DELAY);
        if let Ok(()) = self.master_recv(self.target_addr, &mut bytes_sent_buf, tmr, true) {
            let bytes_to_read = u32::from_le_bytes(bytes_sent_buf);
            for i in 0..(bytes_to_read / 256) as usize {
                delay(MASTER_DELAY); // TODO: mitigate these delays bc this is... a lot
                self.master_recv(self.target_addr, &mut dest[i * 256..], tmr, true)
                    .unwrap();
            }
            let leftover = dest.len() - (dest.len() % 256);
            delay(MASTER_DELAY);
            self.master_recv(self.target_addr, &mut dest[leftover..], tmr, true)
                .unwrap();
            return Ok(bytes_to_read as usize);
        }
        Err(CommunicationError::InternalError)
    }

    fn recv_with_timeout<TMT: Timeout>(
        &mut self,
        dest: &mut [u8],
        tmr: &mut TMT,
    ) -> crate::communication::Result<usize>
    where
        TMT: Timeout,
    {
        let mut bytes_sent_buf = [0u8; 4];
        delay(MASTER_DELAY);
        if let Ok(()) = self.master_recv(self.target_addr, &mut bytes_sent_buf, tmr, false) {
            let bytes_to_read = u32::from_le_bytes(bytes_sent_buf);
            for i in 0..(bytes_to_read / 256) as usize {
                delay(MASTER_DELAY); // This delay is necessary for the slave to catch up
                self.master_recv(self.target_addr, &mut dest[i * 256..], tmr, false)
                    .unwrap();
            }
            let leftover = dest.len() - (dest.len() % 256);
            delay(MASTER_DELAY);
            self.master_recv(self.target_addr, &mut dest[leftover..], tmr, false)
                .unwrap();
            return Ok(bytes_to_read as usize);
        }
        Err(CommunicationError::InternalError)
    }
}

/*impl<'a, T: GCRI2C> TxChannel for I2CSlave<'a, T> {
    fn send(&mut self, src: &mut [u8]) -> crate::communication::Result<()> {
        if let Ok(SlavePollResult::TransmitNeeded) = self.slave_poll(&mut InfTimeout::new()) {
            self.slave_send(&mut u32::to_le_bytes(src.len() as u32).into_iter())
                .unwrap();
            for i in 0..((src.len() - 1) / 256) + 1 {
                if let Ok(SlavePollResult::TransmitNeeded) = self.slave_poll(&mut InfTimeout::new())
                {
                    self.slave_send(&src[i * 256..]).unwrap();
                }
            }
            return Ok(());
        }
        Err(CommunicationError::InternalError)
    }
}

impl<'a, T: GCRI2C> TxChannel for I2CMaster<'a, T> {
    fn send(&mut self, src: &mut [u8]) -> crate::communication::Result<()> {
        self.write(self.target_addr, &u32::to_le_bytes(src.len() as u32))
            .unwrap();
        delay(MASTER_DELAY);
        self.write(self.target_addr, src).unwrap();
        Ok(())
    }
}*/

impl<'b, T: GCRI2C> FramedTxChannel for I2CSlave<'b, T> {
    fn frame<'a, const FRAME_CT: usize>(
        &mut self,
        frame: impl FnOnce() -> Result<Frame<'a, FRAME_CT>, CommunicationError>,
    ) -> Result<(), CommunicationError> {
        let frame = frame()?;
        let mut iter = frame.into_byte_iter();
        let len = iter.length();
        if let Ok(SlavePollResult::TransmitNeeded) = self.slave_poll(&mut InfTimeout::new()) {
            self.slave_send(&mut u32::to_le_bytes(len as u32).into_iter())
                .unwrap();
            for _ in 0..((len - 1) / 256) + 1 {
                if let Ok(SlavePollResult::TransmitNeeded) = self.slave_poll(&mut InfTimeout::new())
                {
                    self.slave_send(&mut iter).unwrap();
                }
            }
            return Ok(());
        }
        Err(CommunicationError::InternalError)
    }
}

impl<'b, T: GCRI2C> FramedTxChannel for I2CMaster<'b, T> {
    fn frame<'a, const FRAME_CT: usize>(
        &mut self,
        frame: impl FnOnce() -> Result<Frame<'a, FRAME_CT>, CommunicationError>,
    ) -> Result<(), CommunicationError> {
        let frame = frame()?;
        let mut iter = frame.into_byte_iter();
        let len = iter.length();
        self.write(self.target_addr, &u32::to_le_bytes(len as u32))
            .unwrap();
        delay(MASTER_DELAY);
        self.master_send(self.target_addr, &mut iter).unwrap();
        Ok(())
    }
}
