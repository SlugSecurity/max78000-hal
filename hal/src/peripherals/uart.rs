//! Module for UART API.
//!
//! TODO: Add example and more desc.

use core::{marker::PhantomData, ops::Deref};

use sealed::sealed;

// TODO: Document this
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum CommunicationError {
    RecvError { amount_sent: usize },
    SendError { amount_sent: usize },
    InternalError,
}

pub type Result<T> = core::result::Result<T, CommunicationError>;

/// A trait for all instances of UART peripherals, ie: UART0, UART1, UART2, UART3.
#[sealed]
pub trait UartInstance {
    // TODO: Bound to proper pin trait when GPIO is in. Make new trait that encompasses
    //       the pin handle and (). That trait should have a fn for configuring an instance
    //       of the RxPin or TxPin to be used for UART -- no-op with ()

    /// The register block to use for this UART.
    type Registers: Deref<Target = max78000::uart::RegisterBlock>;
}

macro_rules! uart_instance_impl {
    ($uart:ident, $regs:ty) => {
        /// A UART instance containing types for which RX and TX pins to use
        /// for it.
        pub struct $uart;

        #[sealed]
        impl UartInstance for $uart {
            type Registers = $regs;
        }
    };
}

// TODO: Replace RX and TX pin types when GPIO is merged.
uart_instance_impl!(Uart0, max78000::UART);

pub struct UartBuilder<T: UartInstance> {
    regs: T::Registers,
}

impl UartBuilder<Uart0> {
    pub fn build<'a>(instance: &'a max78000::UART, baud: u32) -> Uart<'a, Uart0> {
        const IBRO_FREQUENCY: u32 = 7372800;
        instance.ctrl().write(|w| {
            w.rx_thd_val()
                .variant(1)
                .char_size()
                ._8bits()
                .par_en()
                .variant(false)
                .stopbits()
                .bit(true)
                // use IBRO
                .bclksrc()
                .clk2()
        });

        instance
            .clkdiv()
            .write(|w| w.clkdiv().variant(IBRO_FREQUENCY.div_ceil(baud)));

        Uart {
            regs: instance,
            _uart_instance: Default::default(),
        }
    }
}

// TODO: Move to its own crate/module
pub trait RxChannel {
    // TODO: Use timeout versions of these functions with timer API
    fn recv(&self, dest: &mut [u8]) -> Result<usize>;
}

pub trait TxChannel {
    fn send(&self, src: &[u8]) -> Result<()>;
}

// make trait for pin configuration for Tx and Rx generic params
// and call those functions on constructions
pub struct Uart<'a, T: UartInstance> {
    regs: &'a T::Registers,
    _uart_instance: PhantomData<T>,
}

impl<T: UartInstance> RxChannel for Uart<'_, T> {
    fn recv(&self, dest: &mut [u8]) -> Result<usize> {
        let mut index: usize = 0;
        while self.regs.status().read().rx_em().bit() && index < dest.len() {
            dest[index] = self.regs.fifo().read().data().bits();
            index += 1;
        }
        Ok(index)
    }
}

impl<T: UartInstance> TxChannel for Uart<'_, T> {
    fn send(&self, src: &[u8]) -> Result<()> {
        for (i, &byte) in src.iter().enumerate() {
            if self.regs.status().read().tx_full().bit() {
                return Err(CommunicationError::SendError { amount_sent: i });
            }
            self.regs.fifo().write(|w| w.data().variant(byte));
        }
        Ok(())
    }
}
