//! Module for UART API.
//!
//! Currently only UART0 is implemented, using pins 0 and 1 as those are connected to the
//! USB-to-UART bridge on the MAX78000FTHR board.
//!
//! # Example usage
//!
//! ```rs
//! use max78000_hal::communication::{RxChannel, TxChannel};
//! use max78000_hal::peripherals::timer::Time::Milliseconds;
//! use max78000_hal::{
//!     max78000::Peripherals,
//!     peripherals::{
//!         oscillator::{Ipo, IpoDivider, IpoFrequency},
//!         timer::{Oscillator, Prescaler},
//!         PeripheralManagerBuilder, SplittablePeripheral,
//!     },
//! };
//!
//! let (to_consume, to_borrow, rem) = Peripherals::take().unwrap().split();
//! let manager = PeripheralManagerBuilder::<Ipo>::new(
//!     &to_borrow,
//!     to_consume,
//!     IpoFrequency::_100MHz,
//!     IpoDivider::_1,
//! )
//! .configure_timer_0(Oscillator::ISO, Prescaler::_4096)
//! .build();
//! // we need timers for the timeout receive methods
//! let clk0 = manager.timer_0().unwrap();
//! // 115,200 baud
//! let mut uart = manager.build_uart().unwrap().build(115200);
//!! let mut timer = clk0.new_timer(Milliseconds(500));
//! let mut buf = [0u8; 16];
//! uart.recv_with_timeout(&mut buf, &mut timer).unwrap();
//!! buf = *b"0123456789abcdef";
//! // send must take a mutable parameter due to the txchannel trait,
//! // but UART send does not actually mutate the buffer
//! uart.send(&mut buf).unwrap();
//! ```

use core::cell::BorrowMutError;
use core::{marker::PhantomData, ops::Deref, result::Result};

use embedded_hal::digital::PinState;
use sealed::sealed;

use super::gpio::{
    active::{
        port_num_types::GpioZero, ActiveInputPin, ActiveInputPinConfig, ActiveOutputPin,
        ActiveOutputPinConfig, DriveStrength, PowerSupply, PullMode,
    },
    pin_traits::IoPin,
    GpioError, PinOperatingMode,
};
use super::{PeripheralHandle, PeripheralManager};
use crate::communication::{
    CommunicationError, LineDelimitedRxChannel, LineEnding, Result as CommunicationResult,
    RxChannel, Timeout, TxChannel,
};

/// A trait for all instances of UART peripherals, ie: UART0, UART1, UART2, UART3.
#[sealed]
pub trait UartInstance {
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

uart_instance_impl!(Uart0, max78000::UART);

/// Used to configure UART 0
pub struct UartBuilder<'a, T: UartInstance> {
    uart_regs: PeripheralHandle<'a, T::Registers>,
    tx: ActiveOutputPin<'a, GpioZero, 31>,
    rx: ActiveInputPin<'a, GpioZero, 31>,
}

/// Error that can be returned while creating a UartBuilders
#[derive(Debug)]
pub enum UartBuilderError {
    /// Error occurred while borrowing a peripheral
    BorrowPeripheral(BorrowMutError),
    /// Error occurred while claiming a GPIO pin
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
    pub fn new<'pc>(
        peripheral_manager: &'a PeripheralManager<'pc>,
    ) -> Result<Self, UartBuilderError>
    where
        'a: 'pc,
    {
        let gpio = peripheral_manager.gpio0();

        // these results have Infallible as the Err type so unwrap is ok
        // pin configs from https://github.com/analogdevicesinc/msdk/blob/c7dc24619e995f17cefd9c776292d318a8a04afb/Libraries/PeriphDrivers/Source/SYS/pins_ai85.c#L45-L46
        let rx = gpio
            .get_pin_handle(0)?
            .into_input_pin(ActiveInputPinConfig {
                operating_mode: PinOperatingMode::AltFunction1,
                power_supply: PowerSupply::Vddio,
                pull_mode: PullMode::HighImpedance,
            })
            .unwrap();
        let tx = gpio
            .get_pin_handle(1)?
            .into_output_pin(
                PinState::Low,
                ActiveOutputPinConfig {
                    operating_mode: PinOperatingMode::AltFunction1,
                    power_supply: PowerSupply::Vddio,
                    drive_strength: DriveStrength::S0,
                },
            )
            .unwrap();
        Ok(Self {
            uart_regs: peripheral_manager.uart()?,
            rx,
            tx,
        })
    }

    /// Set up and return a UART instance for the given baud rate
    pub fn build(self, baud: u32) -> Uart<'a, Uart0> {
        const IBRO_FREQUENCY: u32 = 7372800;
        self.uart_regs.ctrl().modify(|_r, w| {
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
            _tx: self.tx,
            _rx: self.rx,
            _uart_instance: Default::default(),
        }
    }
}

/// A running UART instance
pub struct Uart<'a, T: UartInstance> {
    regs: PeripheralHandle<'a, T::Registers>,
    _tx: ActiveOutputPin<'a, GpioZero, 31>,
    _rx: ActiveInputPin<'a, GpioZero, 31>,
    _uart_instance: PhantomData<T>,
}

impl<T: UartInstance> Uart<'_, T> {
    #[inline(always)]
    fn internal_recv<const RESET_EVERY_BYTE: bool, const USE_DELIMITER: bool>(
        &mut self,
        dest: &mut [u8],
        tmr: &mut impl Timeout,
        line_ending: LineEnding,
    ) -> CommunicationResult<usize> {
        let mut index: usize = 0;
        while index < dest.len() {
            // spin until there is a byte to be read
            while self.regs.status().read().rx_em().bit() {
                if tmr.poll() {
                    return Err(CommunicationError::RecvError(index));
                }
            }

            if self.regs.int_fl().read().rx_ov().bit() {
                panic!("rx fifo overrun");
            }

            dest[index] = self.regs.fifo().read().data().bits();
            index += 1;
            if USE_DELIMITER && line_ending.matches_end(&dest[0..index]) {
                return Ok(index);
            }

            if RESET_EVERY_BYTE {
                tmr.reset();
            }
        }

        if USE_DELIMITER && !line_ending.matches_end(dest) {
            Err(CommunicationError::RecvError(index))
        } else {
            Ok(index)
        }
    }
}

impl<T: UartInstance> RxChannel for Uart<'_, T> {
    fn recv_with_data_timeout<U: Timeout>(
        &mut self,
        dest: &mut [u8],
        tmr: &mut U,
    ) -> CommunicationResult<usize> {
        // the line ending here is arbitrary since USE_DELIMITER is false
        self.internal_recv::<true, false>(dest, tmr, LineEnding::CR)
    }

    fn recv_with_timeout<U: Timeout>(
        &mut self,
        dest: &mut [u8],
        tmr: &mut U,
    ) -> CommunicationResult<usize> {
        self.internal_recv::<false, false>(dest, tmr, LineEnding::CR)
    }
}

impl<T: UartInstance> LineDelimitedRxChannel for Uart<'_, T> {
    fn recv_line_with_data_timeout<U: Timeout>(
        &mut self,
        dest: &mut [u8],
        tmr: &mut U,
        line_ending: LineEnding,
    ) -> CommunicationResult<usize>
    where
        U: Timeout,
    {
        self.internal_recv::<true, true>(dest, tmr, line_ending)
    }

    fn recv_line_with_timeout<U: Timeout>(
        &mut self,
        dest: &mut [u8],
        tmr: &mut U,
        line_ending: LineEnding,
    ) -> CommunicationResult<usize>
    where
        U: Timeout,
    {
        self.internal_recv::<false, true>(dest, tmr, line_ending)
    }
}

impl<T: UartInstance> TxChannel for Uart<'_, T> {
    fn send(&mut self, src: &mut [u8]) -> CommunicationResult<()> {
        for &byte in src.iter() {
            while self.regs.status().read().tx_full().bit() {}
            self.regs.fifo().modify(|_r, w| w.data().variant(byte));
        }

        Ok(())
    }
}
