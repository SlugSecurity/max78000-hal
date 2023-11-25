//! GPIO peripherals API.

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

/// Contains implementations of traits defined in this module for the common
/// GPIO ports (GPIO0 - GPIO2).
pub mod common;

/// Contains implementations of traits defined in this module for the low power
/// GPIO port (GPIO3).
pub mod low_power;

/// This trait defines two associated types for a particular GPIO port.
/// These are the pin handle type and the GPIO register block type.
pub trait GpioPortMetadata {
    /// The type of the pin handle.
    type PinHandleType<'a, const PIN_CT: usize>: PinHandle<'a, Port = GpioPort<Self, PIN_CT>>;

    /// The type of the struct to access the GPIO port registers.
    type GpioRegs;
}

/// This trait defines a pin handle.
pub trait PinHandle<'a> {
    /// The type of the GPIO port struct.
    type Port;

    /// Creates a new `PinHandle`.
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
    pub fn new(regs: Metadata::GpioRegs) -> Self {
        Self {
            regs,
            pin_taken: array::from_fn(|_| Default::default()),
        }
    }

    /// Gets a pin handle based on the provided index. Returns an Err if the
    /// pin index is out of bounds or a pin handle has already been taken out.
    pub fn get_pin_handle(&'t self, idx: usize) -> Metadata::PinHandleType<'t, PIN_CT> {
        self.pin_taken[idx].set(true);

        Metadata::PinHandleType::new(self, idx)
    }
}
