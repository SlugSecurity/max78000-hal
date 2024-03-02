use crate::communication::Timeout;
use crate::peripherals::gpio::GpioError;
use crate::peripherals::i2c::{BusSpeed, I2CSlave, SlavePollResult, GCRI2C};
use crate::peripherals::oscillator::SystemClock;
use core::cell::{Ref, RefMut};
use embedded_hal::i2c::{ErrorKind, SevenBitAddress};

impl<'a, T: GCRI2C> I2CSlave<'a, T> {
    /// Creates a new instance of an I2C slave
    pub(crate) fn new(
        address: SevenBitAddress,
        bus_speed: BusSpeed,
        system_clock: Ref<SystemClock>,
        i2c_regs: RefMut<'a, T>,
    ) -> Result<Self, GpioError> {
        i2c_regs.ctrl().modify(|_, w| w.en().bit(true));

        i2c_regs.ctrl().modify(|_, w| {
            w.mst_mode()
                .bit(false)
                .gc_addr_en()
                .bit(false)
                .irxm_en()
                .bit(false)
                .clkstr_dis()
                .bit(false)
                .hs_en()
                .bit(false)
        });

        i2c_regs.rxctrl0().modify(|_, w| {
            w.dnr().bit(true) // TODO: debatable
        });

        i2c_regs.txctrl0().modify(|_, w| {
            w.nack_flush_dis()
                .bit(false) // TODO: idk what this does
                .rd_addr_flush_dis()
                .bit(true)
                .wr_addr_flush_dis()
                .bit(true)
                .gc_addr_flush_dis()
                .bit(true)
                .preload_mode()
                .bit(false)
        });

        /*i2c_regs
        .timeout()
        .modify(|_, w| w.scl_to_val().variant(0xffff));*/

        // Configure clock speed values
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

        unsafe {
            i2c_regs.slave0().write(|w| w.bits(address as u32));
        }

        i2c_regs.ctrl().modify(|_, w| w.en().bit(true));

        Ok(Self { i2c_regs })
    }

    /// Poll for either a master read or write operation. Optional timeout
    pub fn slave_poll<TMT: Timeout>(
        &mut self,
        tmt: &mut TMT,
    ) -> Result<SlavePollResult, ErrorKind> {
        self.i2c_regs.clear_interrupt_flags();
        self.i2c_regs.flush_rx_fifo();
        // Wait for I2Cn_INTFL0.addr_match = 1
        self.i2c_regs.ctrl().modify(|_, w| w.en().bit(true));

        while !self.i2c_regs.intfl0().read().addr_match().bit() && !tmt.poll() {}
        if tmt.poll() {
            return Err(ErrorKind::Bus);
        }

        if !self.i2c_regs.ctrl().read().read().bit() {
            self.i2c_regs
                .intfl0()
                .modify(|_, w| w.addr_match().bit(false));
            return Ok(SlavePollResult::IncomingTransmission);
        }

        Ok(SlavePollResult::TransmitNeeded)
    }

    /// Receive message from master into read buffer
    pub fn recv_raw<TMT: Timeout>(
        &mut self,
        read_buffer: &mut [u8],
        tmt: &mut TMT,
        rst_on_byte: bool,
    ) -> Result<(u32, bool), ErrorKind> {
        let mut num_read = 0;
        let capacity = read_buffer.len();

        // read to fill read buffer
        while num_read < capacity {
            if self.i2c_regs.bus_error() || tmt.poll() {
                return Err(ErrorKind::Bus);
            }
            while !self.i2c_regs.is_rx_fifo_empty() {
                if self.i2c_regs.bus_error() || tmt.poll() {
                    return Err(ErrorKind::Bus);
                }
                if num_read < capacity {
                    read_buffer[num_read] = self.i2c_regs.fifo().read().data().bits();
                    num_read += 1;
                    if rst_on_byte {
                        tmt.reset()
                    }
                }
            }
            if self.i2c_regs.intfl0().read().done().bit() {
                break;
            };
        }

        let was_it_truncated = !self.i2c_regs.intfl0().read().done().bit();

        // discard remaining bytes that we can't put in the read buffer
        while !self.i2c_regs.intfl0().read().done().bit() {
            if self.i2c_regs.bus_error() || tmt.poll() {
                return Err(ErrorKind::Bus);
            }
            while !self.i2c_regs.is_rx_fifo_empty() {
                if self.i2c_regs.bus_error() || tmt.poll() {
                    return Err(ErrorKind::Bus);
                }
                self.i2c_regs.fifo().read().data().bits();
                num_read += 1;
                if rst_on_byte {
                    tmt.reset()
                }
            }
        }

        Ok((num_read as u32, was_it_truncated))
    }

    /// Respond to master on i2c buf using buffer as the message to send
    /// sends a chain of 0s if bus exceeded but master still wants more
    pub fn send_raw<I: Iterator<Item = u8>>(&mut self, buffer: &mut I) -> Result<u32, ErrorKind> {
        // With I2Cn_CTRL.en = 0, initialize all relevant registers, including specifically for this mode I2Cn_CTRL. clkstr_dis = 0,
        // I2Cn_TXCTRL0[5:2] = 0x8 and I2Cn_TXCTRL0.preload_mode = 0. Don't forget to program I2Cn_CLKHI.hi and
        // I2Cn_HSCLK.hsclk_hi with appropriate values satisfying tSU;DAT (and HS tSU;DAT).

        self.i2c_regs
            .intfl0()
            .modify(|_, w| w.addr_match().bit(false));

        self.i2c_regs
            .intfl0()
            .modify(|_, w| w.tx_lockout().variant(true));
        let mut num_written = 0;

        let mut done = false;
        while !self.i2c_regs.intfl0().read().done().bit() && !done {
            if self.i2c_regs.bus_error() {
                return Err(ErrorKind::Bus);
            }
            while !self.i2c_regs.is_tx_fifo_full() && !self.i2c_regs.intfl0().read().done().bit() {
                if self.i2c_regs.bus_error() {
                    return Err(ErrorKind::Bus);
                }
                // important: we must only pull out of the iterator if we know the master needs it
                if num_written >= 256 {
                    done = true;
                    break;
                }
                if let Some(byte) = buffer.next() {
                    self.i2c_regs.fifo().write(|w| w.data().variant(byte));
                    num_written += 1;
                } else {
                    done = true;
                    break;
                }
            }
        }

        // write zeros if we've exceeded buffer but master still wants more
        while !self.i2c_regs.intfl0().read().done().bit() {
            if self.i2c_regs.bus_error() {
                return Err(ErrorKind::Bus);
            }
            while !self.i2c_regs.is_tx_fifo_full() && !self.i2c_regs.intfl0().read().done().bit() {
                if self.i2c_regs.bus_error() {
                    return Err(ErrorKind::Bus);
                }
                self.i2c_regs.fifo().write(|w| w.data().variant(0));
            }
        }

        // clean up!
        self.i2c_regs.intfl0().modify(|_, w| w.done().bit(false));
        self.i2c_regs.inten0().modify(|_, w| w.tx_thd().bit(false));

        Ok(num_written as u32)
    }
}
