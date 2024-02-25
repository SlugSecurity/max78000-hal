//! Contains traits used by pins in the GPIO peripherals API.
//! Some traits in this module are re-exports from `embedded_hal` for GPIO pins.

pub use embedded_hal::digital::v2::*;

use super::{GpioError, PinIoMode, PinOperatingMode};

/// Trait for any GPIO pin on this board in either input or output mode.
pub trait GeneralIoPin<TInput, TOutput>: IoPin<TInput, TOutput>
where
    TInput: InputPin + IoPin<TInput, TOutput>,
    TOutput: StatefulOutputPin + IoPin<TInput, TOutput>,
{
    /// Sets what operating mode the pin is in. This can be digital I/O mode
    /// or an alternate function mode. For a list of what each alternate function
    /// does for each pin, see page 28 of [chip datasheet](https://www.analog.com/media/en/technical-documentation/data-sheets/MAX78000.pdf).
    ///
    /// # Errors:
    ///
    /// If the pin is in the wrong I/O mode when switching to an
    /// alternate function mode, like if the pin is in input mode when
    /// the target alternate function is for UART0 TX, [`GpioError::WrongIoMode`]
    /// is returned.
    ///
    /// If the pin doesn't have the alternate function mode requested,
    /// [`GpioError::BadOperatingMode``] is returned.
    ///
    /// Other error variants can be returned too in case of another error.
    fn set_operating_mode(&mut self, mode: PinOperatingMode) -> Result<(), GpioError>;

    /// Gets the pins operating mode. This can be digital I/O mode
    /// or an alternate function mode.
    fn get_operating_mode(&self) -> PinOperatingMode;

    /// Gets the pins I/O mode, which is either input or output mode.
    fn get_io_mode(&self) -> PinIoMode;
}
