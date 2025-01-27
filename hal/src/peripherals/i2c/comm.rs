use crate::communication::lower_layers::framing::{Frame, FramedTxChannel};
use crate::communication::{CommunicationError, InfTimeout, RxChannel, Timeout};
use crate::peripherals::i2c::{I2CMaster, I2CSlave, SlavePollResult, GCRI2C};
use cortex_m::asm::delay;
use embedded_hal::i2c::I2c;

// TODO: Eliminate the need for this
// Explanation: In actual testing the master side ends up being
// faster than the slave side in terms of re-sending requests
// before the slave software can wrap up and listen for them again.
// This delay is necessary to slow down the master enough to give
// the slave time to catch up.
static MASTER_DELAY: u32 = 1000;

trait CommStackRx {
    fn rx_channel_recv<TMR: Timeout>(
        &mut self,
        dest: &mut [u8],
        tmr: &mut TMR,
        rst_on_data: bool,
    ) -> crate::communication::Result<usize>;
}

impl<T: GCRI2C> CommStackRx for I2CSlave<'_, T> {
    fn rx_channel_recv<TMR: Timeout>(
        &mut self,
        dest: &mut [u8],
        tmr: &mut TMR,
        rst_on_data: bool,
    ) -> crate::communication::Result<usize> {
        if let Ok(SlavePollResult::IncomingTransmission) = self.slave_poll(tmr) {
            tmr.reset();
            let mut bytes_sent_buf = [0u8; 4];
            if let Ok((n, _)) = self.recv_raw(&mut bytes_sent_buf, tmr, rst_on_data) {
                if n != 4 {
                    return Err(CommunicationError::RecvError(0));
                }
                let expected_to_recv = u32::from_le_bytes(bytes_sent_buf);
                return if let Ok(SlavePollResult::IncomingTransmission) = self.slave_poll(tmr) {
                    let (n, _) = self
                        .recv_raw(dest, tmr, true)
                        .map_err(|_| CommunicationError::RecvError(0))?;
                    if n != expected_to_recv {
                        return Err(CommunicationError::RecvError(n as usize));
                    }
                    Ok(n as usize)
                } else {
                    Err(CommunicationError::RecvError(0))
                };
            }
        }
        Err(CommunicationError::RecvError(0))
    }
}

impl<T: GCRI2C> RxChannel for I2CSlave<'_, T> {
    fn recv_with_data_timeout<R: Timeout>(
        &mut self,
        dest: &mut [u8],
        tmr: &mut R,
    ) -> crate::communication::Result<usize> {
        self.rx_channel_recv(dest, tmr, true)
    }

    fn recv_with_timeout<R: Timeout>(
        &mut self,
        dest: &mut [u8],
        tmr: &mut R,
    ) -> crate::communication::Result<usize> {
        self.rx_channel_recv(dest, tmr, false)
    }
}

impl<T: GCRI2C> CommStackRx for I2CMaster<'_, T> {
    fn rx_channel_recv<TMR: Timeout>(
        &mut self,
        dest: &mut [u8],
        tmr: &mut TMR,
        rst_on_data: bool,
    ) -> crate::communication::Result<usize> {
        let mut bytes_sent_buf = [0u8; 4];
        delay(MASTER_DELAY);
        if let Ok(()) = self.recv_raw(&mut bytes_sent_buf, tmr, rst_on_data, 4) {
            let bytes_to_read = u32::from_le_bytes(bytes_sent_buf);
            for i in 0..(bytes_to_read / 256) as usize {
                delay(MASTER_DELAY); // TODO: mitigate these delays bc this is... a lot
                let Ok(_) = self.recv_raw(&mut dest[i * 256..], tmr, rst_on_data, 256) else {
                    return Err(CommunicationError::RecvError(i * 256));
                };
            }
            delay(MASTER_DELAY);
            let leftover = dest.len() - (dest.len() % 256);
            let leftover_len = dest.len() % 256;
            let Ok(_) = self.recv_raw(&mut dest[leftover..], tmr, rst_on_data, leftover_len) else {
                return Err(CommunicationError::RecvError(leftover));
            };
            return Ok(bytes_to_read as usize);
        }
        Err(CommunicationError::RecvError(0))
    }
}

impl<T: GCRI2C> RxChannel for I2CMaster<'_, T> {
    fn recv_with_data_timeout<TMT: Timeout>(
        &mut self,
        dest: &mut [u8],
        tmr: &mut TMT,
    ) -> crate::communication::Result<usize> {
        self.rx_channel_recv(dest, tmr, true)
    }

    fn recv_with_timeout<TMT: Timeout>(
        &mut self,
        dest: &mut [u8],
        tmr: &mut TMT,
    ) -> crate::communication::Result<usize> {
        self.rx_channel_recv(dest, tmr, false)
    }
}

impl<T: GCRI2C> FramedTxChannel for I2CSlave<'_, T> {
    fn frame<'a, const FRAME_CT: usize>(
        &mut self,
        frame: impl FnOnce() -> Result<Frame<'a, FRAME_CT>, CommunicationError>,
    ) -> Result<(), CommunicationError> {
        let frame = frame()?;
        let mut iter = frame.into_byte_iter();
        let len = iter.length();
        if let Ok(SlavePollResult::TransmitNeeded) = self.slave_poll(&mut InfTimeout::new()) {
            let Ok(_) = self.send_raw(&mut u32::to_le_bytes(len as u32).into_iter()) else {
                return Err(CommunicationError::SendError);
            };
            for _ in 0..len.div_ceil(256) {
                if let Ok(SlavePollResult::TransmitNeeded) = self.slave_poll(&mut InfTimeout::new())
                {
                    let Ok(_) = self.send_raw(&mut iter) else {
                        return Err(CommunicationError::SendError);
                    };
                } else {
                    return Err(CommunicationError::SendError);
                }
            }
            return Ok(());
        }
        Err(CommunicationError::InternalError)
    }
}

impl<T: GCRI2C> FramedTxChannel for I2CMaster<'_, T> {
    fn frame<'a, const FRAME_CT: usize>(
        &mut self,
        frame: impl FnOnce() -> Result<Frame<'a, FRAME_CT>, CommunicationError>,
    ) -> Result<(), CommunicationError> {
        let frame = frame()?;
        let mut iter = frame.into_byte_iter();
        let len = iter.length();
        let Ok(_) = self.write(self.target_addr, &u32::to_le_bytes(len as u32)) else {
            return Err(CommunicationError::SendError);
        };
        delay(MASTER_DELAY);
        let Ok(_) = self.send_raw(&mut iter) else {
            return Err(CommunicationError::SendError);
        };
        Ok(())
    }
}
