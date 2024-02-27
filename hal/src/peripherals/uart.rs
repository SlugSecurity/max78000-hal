//! Module for UART API.
//!
//! TODO: Add example and more desc.

use core::borrow::Borrow;
use core::cell::BorrowMutError;
use core::{cell::RefMut, marker::PhantomData, ops::Deref};

use embedded_hal::digital::v2::IoPin;
use sealed::sealed;

use crate::peripherals::gpio::active::{port_num_types::GpioZero, ActiveInputPin, ActiveOutputPin};
use crate::peripherals::gpio::active::{DriveStrength, PowerSupply, PullMode};
use crate::peripherals::gpio::pin_traits::GeneralIoPin;
use crate::peripherals::gpio::PinOperatingMode;
use crate::peripherals::Gpio0;

use super::gpio::{GpioError, PinHandle};
use super::{PeripheralHandle, PeripheralManager};

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

/// Used to configure a UART instance
pub struct UartBuilder<'a, T: UartInstance> {
    uart_regs: PeripheralHandle<'a, T::Registers>,
    gpio: PeripheralHandle<'a, Gpio0>,
    tx: ActiveOutputPin<'a, GpioZero, 31>,
    rx: ActiveInputPin<'a, GpioZero, 31>,
}

pub enum UartBuilderError {
    BorrowPeripheral(BorrowMutError),
    GetPins(GpioError),
}

impl From<BorrowMutError> for UartBuilderError {
    fn from(value: BorrowMutError) -> Self {
        Self::BorrowPeripheral(value)
    }
}

impl From<GpioError> for UartBuilderError {
    fn from(value: GpioError) -> Self {
        Self::GetPins(value)
    }
}

impl<'a> UartBuilder<'a, Uart0> {
    /// Create a [`UartBuilder`] from a reference to the registers
    pub fn new(
        peripheral_manager: &'a PeripheralManager<'a>,
    ) -> core::result::Result<Self, UartBuilderError> {
        let gpio = peripheral_manager.gpio0()?;
        // these results have Infallible as the Err type so unwrap is ok
        let rx = gpio.get_pin_handle(0)?.into_input_pin().unwrap();
        let tx = gpio
            .get_pin_handle(1)?
            .into_output_pin(false.into())
            .unwrap();
        Ok(Self {
            uart_regs: peripheral_manager.uart()?,
            gpio,
            rx,
            tx,
        })
    }

    pub fn build(mut self, baud: u32) -> Uart<'a, Uart0> {
        const IBRO_FREQUENCY: u32 = 7372800;
        self.uart_regs.ctrl().modify(|_r, w| {
            // Set GPIO pins to mode alt 1
            self.tx.set_operating_mode(PinOperatingMode::AltFunction1);
            self.rx.set_operating_mode(PinOperatingMode::AltFunction1);
            // Set pin voltage to VDDIO
            self.tx.set_power_supply(PowerSupply::Vddio);
            self.rx.set_power_supply(PowerSupply::Vddio);
            // Set pull mode to none
            self.rx.set_pull_mode(PullMode::HighImpedance);
            // Set drive strength to 0
            self.tx.set_drive_strength(DriveStrength::S0);

            w.rx_thd_val()
                .variant(1)
                .char_size()
                ._8bits() // 8-bit character length
                .par_en()
                .variant(false) // No parity bit
                .stopbits()
                .bit(false) // 1 stop bit
                .bclksrc()
                .clk2() // use IBRO
        });

        // Set oversampling to 16x (this is when fdm is 0 so needs to be changed if it's not)
        self.uart_regs.osr().write(|w| w.osr().variant(5));

        self.uart_regs
            .clkdiv()
            .modify(|_r, w| w.clkdiv().variant(IBRO_FREQUENCY.div_ceil(baud)));

        // Enable the baud clock after setting clock divider.
        self.uart_regs.ctrl().modify(|_r, w| w.bclken().set_bit());

        // Wait for baud clock to be ready.
        while self.uart_regs.ctrl().read().bclkrdy().bit_is_clear() {}

        Uart {
            regs: self.uart_regs,
            tx: self.tx,
            rx: self.rx,
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
    regs: PeripheralHandle<'a, T::Registers>,
    tx: ActiveOutputPin<'a, GpioZero, 31>,
    rx: ActiveInputPin<'a, GpioZero, 31>,
    _uart_instance: PhantomData<T>,
}

impl<T: UartInstance> RxChannel for Uart<'_, T> {
    fn recv(&self, dest: &mut [u8]) -> Result<usize> {
        let mut index: usize = 0;
        while index < dest.len() {
            while self.regs.status().read().rx_em().bit() {}
            dest[index] = self.regs.fifo().read().data().bits();
            index += 1;
        }
        Ok(index)
    }
}

impl<T: UartInstance> TxChannel for Uart<'_, T> {
    fn send(&self, src: &[u8]) -> Result<()> {
        for &byte in src.iter() {
            while self.regs.status().read().tx_full().bit() {}
            self.regs.fifo().modify(|_r, w| w.data().variant(byte));
        }

        Ok(())
    }
}
