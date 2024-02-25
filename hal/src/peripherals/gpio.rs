//! GPIO peripherals API. Re-exports `embedded_hal` traits for GPIO pins in the `pin_traits` sub-module.

use core::{array, cell::Cell};

use max78000::{GPIO0, GPIO1, GPIO2, MCR};
use sealed::sealed;

use self::{
    active::{
        port_num_types::{GpioOne, GpioTwo, GpioZero},
        ActiveGpio,
    },
    low_power::LowPowerGpio,
    private::NonConstructible,
};

pub mod pin_traits;

pub mod active;

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
///
/// # Note:
///
/// This trait is sealed and cannot be implemented outside this crate.
#[sealed(pub(crate))]
pub trait GpioPortMetadata<'b> {
    /// The type of the pin handle.
    type PinHandleType<'a, const PIN_CT: usize>: PinHandle<'a, Port = GpioPort<'b, Self, PIN_CT>>
    where
        'b: 'a;

    /// The type of the struct to access the GPIO port registers.
    type GpioRegs;
}

mod private {
    pub struct NonConstructible;
}

/// This trait defines a pin handle. Dropping the pin handle should return it back.
///
/// # Note:
///
/// This trait is sealed and cannot be implemented outside this crate.
#[sealed(pub(crate))]
pub trait PinHandle<'a> {
    /// The type of the GPIO port struct.
    type Port;

    /// Creates a new `PinHandle``.
    /// # Panics
    ///
    /// This function panics if pin_idx is less than the number of pins
    /// of the port. It can also panic in cases where an invalid type
    /// is about to be constructed.
    #[doc(hidden)]
    fn new(_private: NonConstructible, port_ref: &'a Self::Port, pin_idx: usize) -> Self;

    /// Gets the pin index associated with this handle.
    fn get_pin_idx(&self) -> usize;
}

/// This struct is responsible for managing handles to GPIO pins within
/// a particular GPIO port. Only one handle can be taken at a time to
/// a particular pin. Dropping a pin handle will allow for it to be able
/// to be taken again.
#[derive(Debug)]
pub struct GpioPort<'regs, Metadata: GpioPortMetadata<'regs> + ?Sized, const PIN_CT: usize> {
    // TODO for implementor: The const generic `PIN_CT` can be removed once more complex
    // expressions are allowed within const generics like associated constants from generic types
    pub(crate) regs: Metadata::GpioRegs,
    pub(crate) pin_taken: [Cell<bool>; PIN_CT],
}

impl<'t, 'regs, Metadata: GpioPortMetadata<'regs> + ?Sized, const PIN_CT: usize>
    GpioPort<'regs, Metadata, PIN_CT>
{
    /// Creates a new GpioPort
    fn new(regs: Metadata::GpioRegs) -> Self {
        Self {
            regs,
            pin_taken: array::from_fn(|_| Default::default()),
        }
    }

    /// Gets a pin handle based on the provided index. Returns an Err if the
    /// pin index is out of bounds or a pin handle has already been taken out.
    // TODO: Statically constrain the pin index according to PIN_CT
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

        Ok(Metadata::PinHandleType::new(NonConstructible, self, idx))
    }
}

/// Creates a new [`GpioPort`] representing GPIO0.
// TODO: Make this pub(crate) when peripheral manager is made
pub fn new_gpio0(gpio0: GPIO0) -> GpioPort<'static, ActiveGpio<GpioZero>, 31> {
    GpioPort::<ActiveGpio<GpioZero>, 31>::new(gpio0)
}

/// Creates a new [`GpioPort`] representing GPIO1.
// TODO: Make this pub(crate) when peripheral manager is made
pub fn new_gpio1(gpio1: GPIO1) -> GpioPort<'static, ActiveGpio<GpioOne>, 10> {
    GpioPort::<ActiveGpio<GpioOne>, 10>::new(gpio1)
}

/// Creates a new [`GpioPort`] representing GPIO2.
// TODO: Make this pub(crate) when peripheral manager is made
pub fn new_gpio2(gpio2: GPIO2) -> GpioPort<'static, ActiveGpio<GpioTwo>, 8> {
    GpioPort::<ActiveGpio<GpioTwo>, 8>::new(gpio2)
}

/// Creates a new [`GpioPort`] representing GPIO3.
// TODO: Make this pub(crate) when peripheral manager is made
pub fn new_gpio3<'a>(gpio3: &'a MCR) -> GpioPort<'a, LowPowerGpio<'a>, 2> {
    GpioPort::<LowPowerGpio<'a>, 2>::new(gpio3)
}

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
