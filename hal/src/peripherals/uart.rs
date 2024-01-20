//! Module for UART API.
//!
//! TODO: Add example and more desc.

use core::marker::PhantomData;

use sealed::sealed;

/// A trait for all instances of UART peripherals, ie: UART0, UART1, UART2, UART3.
#[sealed]
pub trait UartInstance {
    // TODO: Bound to proper pin trait when GPIO is in. Make new trait that encompasses
    //       the pin handle and (). That trait should have a fn for configuring an instance
    //       of the RxPin or TxPin to be used for UART -- no-op with ()

    /// The GPIO pin to use for UART RX.
    type RxPin;

    /// The GPIO pin to use for UART TX.
    type TxPin;
}

macro_rules! uart_instance_impl {
    ($uart:ident, $rx_pin:ty, $tx_pin:ty) => {
        /// A UART instance containing types for which RX and TX pins to use
        /// for it.
        pub struct $uart;

        #[sealed]
        impl UartInstance for $uart {
            type RxPin = $rx_pin;
            type TxPin = $tx_pin;
        }
    };
}

// TODO: Replace RX and TX pin types when GPIO is merged.
uart_instance_impl!(Uart0, (), ());
uart_instance_impl!(Uart1, (), ());
uart_instance_impl!(Uart2, (), ());
uart_instance_impl!(Uart3, (), ());

pub struct UartBuilder<T: UartInstance> {
    tx_pin: T::TxPin,
    rx_pin: T::RxPin,
}

impl UartBuilder<Uart0> {
    pub fn build_with_usb(self) -> Uart<Uart0, (), ()> {
        todo!()
    }
}

impl<T: UartInstance> UartBuilder<T> {
    pub fn build_with_pins(self) -> Uart<T, T::TxPin, T::RxPin> {
        todo!()
    }
}

// make trait for pin configuration for Tx and Rx generic params
// and call those functions on constructions
pub struct Uart<T: UartInstance, Tx, Rx> {
    tx_pin: Tx,
    rx_pin: Rx,
    _uart_instance: PhantomData<T>,
}

impl<T: UartInstance, Tx, Rx> Uart<T, Tx, Rx> {}
