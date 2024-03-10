use crate::communication::lower_layers::framing::{Frame, FramedTxChannel};
use crate::communication::{CommunicationError, InfTimeout, RxChannel, Timeout};
use crate::peripherals::i2c::{I2CMaster, I2CSlave, SlavePollResult, GCRI2C};
use cortex_m::asm::delay;
use embedded_hal::i2c::I2c;

static MASTER_DELAY: u32 = 10000;

trait CommStackRx {
    fn rx_channel_recv<TMR: Timeout>(
        &mut self,
        dest: &mut [u8],
        tmr: &mut TMR,
        rst_on_data: bool
    ) -> crate::communication::Result<usize>;
}

impl<'a, T: GCRI2C> CommStackRx for I2CSlave<'a, T> {
    fn rx_channel_recv<TMR: Timeout>(&mut self, dest: &mut [u8], tmr: &mut TMR, rst_on_data: bool) -> crate::communication::Result<usize> {
        if let Ok(SlavePollResult::IncomingTransmission) = self.slave_poll(tmr) {
            tmr.reset();
            let mut bytes_sent_buf = [0u8; 4];
            if let Ok((n, _)) = self.recv_raw(&mut bytes_sent_buf, tmr, rst_on_data) {
                if n != 4 {
                    return Err(CommunicationError::RecvError(0));
                }
                let mut num_read = 0;
                return if let Ok(SlavePollResult::IncomingTransmission) = self.slave_poll(tmr) {
                    if let Ok((n, _)) = self.recv_raw(dest, tmr, rst_on_data) {
                        num_read += n;
                        Ok(num_read as usize)
                    } else {
                        Err(CommunicationError::RecvError(num_read as usize))
                    }
                } else {
                    Err(CommunicationError::RecvError(num_read as usize))
                };
            }
        }
        Err(CommunicationError::RecvError(0))
    }
}

impl<'a, T: GCRI2C> RxChannel for I2CSlave<'a, T> {
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
    ) -> crate::communication::Result<usize>
    where
        R: Timeout,
    {
        self.rx_channel_recv(dest, tmr, false)
    }
}

impl<'a, T: GCRI2C> CommStackRx for I2CMaster<'a, T> {
    fn rx_channel_recv<TMR: Timeout>(&mut self, dest: &mut [u8], tmr: &mut TMR, rst_on_data: bool) -> crate::communication::Result<usize> {
        // TODO: remove unwraps
        let mut bytes_sent_buf = [0u8; 4];
        delay(MASTER_DELAY);
        if let Ok(()) = self.recv_raw(&mut bytes_sent_buf, tmr, rst_on_data) {
            let bytes_to_read = u32::from_le_bytes(bytes_sent_buf);
            for i in 0..(bytes_to_read / 256) as usize {
                delay(MASTER_DELAY); // TODO: mitigate these delays bc this is... a lot
                self.recv_raw(&mut dest[i * 256..], tmr, rst_on_data).unwrap();
            }
            let leftover = dest.len() - (dest.len() % 256);
            delay(MASTER_DELAY);
            self.recv_raw(&mut dest[leftover..], tmr, rst_on_data).unwrap();
            return Ok(bytes_to_read as usize);
        }
        Err(CommunicationError::RecvError(0))
    }
}

impl<'a, T: GCRI2C> RxChannel for I2CMaster<'a, T> {
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
    ) -> crate::communication::Result<usize>
    where
        TMT: Timeout,
    {
        self.rx_channel_recv(dest, tmr, false)
    }
}

impl<'b, T: GCRI2C> FramedTxChannel for I2CSlave<'b, T> {
    fn frame<'a, const FRAME_CT: usize>(
        &mut self,
        frame: impl FnOnce() -> Result<Frame<'a, FRAME_CT>, CommunicationError>,
    ) -> Result<(), CommunicationError> {
        let frame = frame()?;
        let mut iter = frame.into_byte_iter();
        let len = iter.length();
        if let Ok(SlavePollResult::TransmitNeeded) = self.slave_poll(&mut InfTimeout::new()) {
            self.send_raw(&mut u32::to_le_bytes(len as u32).into_iter())
                .unwrap();
            for _ in 0..((len - 1) / 256) + 1 {
                if let Ok(SlavePollResult::TransmitNeeded) = self.slave_poll(&mut InfTimeout::new())
                {
                    self.send_raw(&mut iter).unwrap();
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
        self.send_raw(&mut iter).unwrap();
        Ok(())
    }
}
