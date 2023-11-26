//! GPIO peripherals API. Re-exports `embedded_hal` traits for GPIO pins in the `pin_traits` sub-module.

// TODO:
//      - add an assert if pin_idx is out of bounds
//      - make GpioPort::get_handle fallible if already taken or idx is out of bounds
//      - make sure everything has the proper privacy so nothing can be constructed, even through associated types
//      - implement embedded_hal traits -- PinHandle should imply `IoPin`
//      - make newtypes for input pins and output pins
//      - describe this module and give example
//      - improve docs and add examples for each public item

// TODO: implement drop and deref for handles (force deref on trait)

use core::{array, cell::Cell};

pub mod pin_traits;

/// Contains implementations of traits defined in this module for the common
/// GPIO ports (GPIO0 - GPIO2).
pub mod common;

/// Contains implementations of traits defined in this module for the low power
/// GPIO port (GPIO3).
pub mod low_power;

/// Error type for GPIO operations
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum GpioError {
    /// Pin handle is already taken and so cannot be taken again
    HandleAlreadyTaken,

    /// Pin index provided is out of bounds.
    InvalidPinIndex,

    /// GPIO pin was in the wrong I/O mode.
    WrongIoMode,

    /// GPIO pin doesn't have the selected operating mode.
    BadOperatingMode,
}

/// This trait defines two associated types for a particular GPIO port.
/// These are the pin handle type and the GPIO register block type.
pub trait GpioPortMetadata {
    /// The type of the pin handle.
    type PinHandleType<'a, const PIN_CT: usize>: PinHandle<'a, Port = GpioPort<Self, PIN_CT>>;

    /// The type of the struct to access the GPIO port registers.
    type GpioRegs;
}

/// This trait defines a pin handle. Dropping the pin handle should return it back.
/// A pin handle must also implement IoPin.
pub trait PinHandle<'a> {
    /// The type of the GPIO port struct.
    type Port;

    /// Creates a new `PinHandle.
    /// # Panics
    ///
    /// This function panics if pin_idx is less than the number of pins
    /// of the port.
    fn new(port_ref: &'a Self::Port, pin_idx: usize) -> Self;
}

/// This struct is responsible for managing handles to GPIO pins within
/// a particular GPIO port. Only one handle can be taken at a time to
/// a particular pin. Dropping a pin handle will allow for it to be able
/// to be taken again.
#[derive(Debug)]
pub struct GpioPort<Metadata: GpioPortMetadata + ?Sized, const PIN_CT: usize> {
    pub(crate) regs: Metadata::GpioRegs,
    pub(crate) pin_taken: [Cell<bool>; PIN_CT],
}

impl<'t, Metadata: GpioPortMetadata + ?Sized, const PIN_CT: usize> GpioPort<Metadata, PIN_CT> {
    /// Creates a new GpioPort
    fn new(regs: Metadata::GpioRegs) -> Self {
        Self {
            regs,
            pin_taken: array::from_fn(|_| Default::default()),
        }
    }

    /// Gets a pin handle based on the provided index. Returns an Err if the
    /// pin index is out of bounds or a pin handle has already been taken out.
    pub fn get_pin_handle(
        &'t self,
        idx: usize,
    ) -> Result<Metadata::PinHandleType<'t, PIN_CT>, GpioError> {
        let pin_taken_cell = self.pin_taken.get(idx).ok_or(GpioError::InvalidPinIndex)?;

        // Pin was already taken and hasn't been returned yet.
        if pin_taken_cell.get() {
            return Err(GpioError::HandleAlreadyTaken);
        }

        pin_taken_cell.set(true);

        Ok(Metadata::PinHandleType::new(self, idx))
    }
}

// TODO: impl new_gpio0 ... new_gpio3

/// Represents the I/O mode of a pin.
pub enum PinIoMode {
    /// Input mode (The default after power-on-reset).
    Input,

    /// Output mode.
    Output,
}

/// Represents the operating mode of a pin. For a list of what each alternate function
/// does for each pin, see page 28 of [chip datasheet](https://www.analog.com/media/en/technical-documentation/data-sheets/MAX78000.pdf).
pub enum PinOperatingMode {
    /// This operating mode allows the pin to be used for general purpose I/O. This is
    /// the default operating mode after power-on-reset for all pins except the pins
    /// for SWDIO and SWCLK. (See page 28 of datasheet linked in the enum's doc comment)
    DigitalIo,

    /// This operating mode is to allow the pin to perform some designated alternate
    /// function defined on page 28 of the datasheet linked in the enum's doc comment.
    /// This is the default operating mode of the pins for SWDIO and SWCLK after power-on-reset.
    AltFunction1,

    /// This operating mode is to allow the pin to perform some designated alternate
    /// function apart from [`PinOperatingMode::AltFunction1`]. The alternate functions
    /// for each pin is defined on page 28 of the datasheet linked in the enum's doc comment.
    AltFunction2,
}
