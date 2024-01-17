//! GPIO3 pin manipulation.
//!
//! # Examples
//!
//! Basic usage:
//! ```
//! let pin = gpio_port.get_pin_handle(0).unwrap().into_input_pin().unwrap();
//! assert_ne!(pin.is_low(), pin.is_high());
//!
//! let mut pin = pin.into_output_pin(PinState::High).unwrap();
//! pin.set_low().unwrap();
//! assert!(pin.is_set_low().unwrap());
//! pin.set_high().unwrap();
//! assert!(pin.is_set_high().unwrap());
//! ```

use core::convert::Infallible;
use core::marker::PhantomData;

use max78000::MCR;
use sealed::sealed;

use super::pin_traits::{
    toggleable, GeneralIoPin, InputPin, IoPin, OutputPin, PinState, StatefulOutputPin,
};

use super::private::NonConstructible;
use super::{
    GpioError, GpioPort, GpioPortMetadata, PinHandle, PinIoMode, PinOperatingMode,
    __seal_gpio_port_metadata, __seal_pin_handle,
};

// TODO for arelyx:
// - implement functions with todo!() in them (see LowPowerPinHandle::set_operating_mode for example)
// - add documentation
//     - a module-level doc comment
//     - public functions within this module that aren't trait impl functions
//     - other public items like structs
//     - improve existing comments in entire driver to add more detail
// - add unit tests for each public function in the low power pin API

/// Marker struct implementing `GpioPortMetadata` for
/// low power GPIO ports.
pub struct LowPowerGpio<'mcr>(PhantomData<&'mcr ()>);

#[sealed]
impl<'mcr> GpioPortMetadata<'mcr> for LowPowerGpio<'mcr> {
    type PinHandleType<'a, const PIN_CT: usize> = LowPowerPinHandle<'a, 'mcr, PIN_CT> where 'mcr: 'a;
    type GpioRegs = &'mcr MCR;
}

/// `PinHandle` implementation for low power GPIO ports.
pub struct LowPowerPinHandle<'a, 'mcr, const PIN_CT: usize> {
    port: &'a GpioPort<'mcr, LowPowerGpio<'mcr>, PIN_CT>,
    pin_idx: usize,
}

impl<'a, 'mcr, const PIN_CT: usize>
    IoPin<LowPowerInputPin<'a, 'mcr, PIN_CT>, LowPowerOutputPin<'a, 'mcr, PIN_CT>>
    for LowPowerPinHandle<'a, 'mcr, PIN_CT>
{
    type Error = Infallible;

    fn into_input_pin(self) -> Result<LowPowerInputPin<'a, 'mcr, PIN_CT>, Self::Error> {
        Ok(LowPowerInputPin(self))
    }

    fn into_output_pin(
        self,
        state: PinState,
    ) -> Result<LowPowerOutputPin<'a, 'mcr, PIN_CT>, Self::Error> {
        let mut output_pin = LowPowerOutputPin(self);
        output_pin.set_state(state)?;

        Ok(output_pin)
    }
}

impl<'a, 'mcr, const PIN_CT: usize>
    GeneralIoPin<LowPowerInputPin<'a, 'mcr, PIN_CT>, LowPowerOutputPin<'a, 'mcr, PIN_CT>>
    for LowPowerPinHandle<'a, 'mcr, PIN_CT>
{
    fn set_operating_mode(&mut self, mode: PinOperatingMode) -> Result<(), GpioError> {
        use GpioError::*;
        use PinIoMode::*;

        // User guide is very confusing but
        // - GPIO 3 pin 0's alt fn 1 is for PDOWN (it outputs data on the pin)
        //     - the MCR_OUTEN register at the pdown_out_en bit is how you switch the alternate function
        // - GPIO 3 pin 1's alt fn 1 is for SQWOUT (it outputs data on the pin)
        //     - the MCR_OUTEN register at the sqwout_en bit is how you switch the alternate function

        let r = self.port.regs.outen();

        match (mode, self.pin_idx, self.get_io_mode()) {
            (PinOperatingMode::DigitalIo, 0, _) => r.write(|w| w.pdown_out_en().clear_bit()),
            (PinOperatingMode::DigitalIo, 1, _) => r.write(|w| w.sqwout_en().clear_bit()),
            (PinOperatingMode::AltFunction1, 0, Output) => r.write(|w| w.pdown_out_en().set_bit()),
            (PinOperatingMode::AltFunction1, 1, Output) => r.write(|w| w.sqwout_en().set_bit()),

            // Pin is in input mode when AltFunction1 was requested
            (PinOperatingMode::AltFunction1, _, _) => return Err(WrongIoMode),

            // AltFunction2 was given
            _ => return Err(BadOperatingMode),
        };

        Ok(())
    }

    fn get_operating_mode(&self) -> PinOperatingMode {
        let reg = self.port.regs.outen();

        if self.pin_idx == 0 {
            match reg.read().pdown_out_en().bit_is_set() {
                true => PinOperatingMode::AltFunction1,
                false => PinOperatingMode::DigitalIo,
            }
        } else {
            // Pin 1
            match reg.read().sqwout_en().bit_is_set() {
                true => PinOperatingMode::AltFunction1,
                false => PinOperatingMode::DigitalIo,
            }
        }
    }

    fn get_io_mode(&self) -> PinIoMode {
        let reg = self.port.regs.gpio3_ctrl();

        if self.pin_idx == 0 {
            match reg.read().p30_oe().bit_is_set() {
                true => PinIoMode::Output,
                false => PinIoMode::Input,
            }
        } else {
            match reg.read().p31_oe().bit_is_set() {
                true => PinIoMode::Output,
                false => PinIoMode::Input,
            }
        }
    }
}

impl<'a, 'mcr, const PIN_CT: usize> Drop for LowPowerPinHandle<'a, 'mcr, PIN_CT> {
    fn drop(&mut self) {
        // When handle is dropped, allow the pin to be taken again.
        self.port.pin_taken[self.pin_idx].set(false);
    }
}

#[sealed]
impl<'a, 'mcr, const PIN_CT: usize> PinHandle<'a> for LowPowerPinHandle<'a, 'mcr, PIN_CT> {
    type Port = GpioPort<'mcr, LowPowerGpio<'mcr>, PIN_CT>;

    fn new(_private: NonConstructible, port: &'a Self::Port, pin_idx: usize) -> Self {
        // We can't get rid of the const generic here or otherwise prevent a bad pin count
        // from being entered until more complex exprs can be evaluated in const generics stably.
        // So there are asserts here to ensure they can't be constructed. The construction of these
        // handles are done privately and not able to be done externally so this is fine.
        assert!(PIN_CT == 2);
        assert!(pin_idx < PIN_CT);

        Self { port, pin_idx }
    }

    fn get_pin_idx(&self) -> usize {
        self.pin_idx
    }
}

pub struct LowPowerInputPin<'a, 'mcr, const PIN_CT: usize>(LowPowerPinHandle<'a, 'mcr, PIN_CT>);

impl<'a, 'mcr, const PIN_CT: usize> LowPowerInputPin<'a, 'mcr, PIN_CT> {
    /// Enables the pin's pull-up resistor.
    pub fn enable_pullup_resistor(&self, enable: bool) {
        let reg = self.0.port.regs.gpio3_ctrl();

        match self.0.pin_idx == 0 {
            true => reg.write(|w| w.p30_pe().bit(enable)),
            false => reg.write(|w| w.p31_pe().bit(enable)),
        }
    }
}

impl<'a, 'mcr, const PIN_CT: usize> InputPin for LowPowerInputPin<'a, 'mcr, PIN_CT> {
    type Error = Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        let reg = self.0.port.regs.gpio3_ctrl();

        match self.0.pin_idx == 0 {
            true => Ok(reg.read().p30_in().bit_is_set()),
            false => Ok(reg.read().p31_in().bit_is_set()),
        }
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        let reg = self.0.port.regs.gpio3_ctrl();

        match self.0.pin_idx == 0 {
            true => Ok(reg.read().p30_in().bit_is_clear()),
            false => Ok(reg.read().p31_in().bit_is_clear()),
        }
    }
}

impl<'a, 'mcr, const PIN_CT: usize>
    IoPin<LowPowerInputPin<'a, 'mcr, PIN_CT>, LowPowerOutputPin<'a, 'mcr, PIN_CT>>
    for LowPowerInputPin<'a, 'mcr, PIN_CT>
{
    type Error = Infallible;

    fn into_input_pin(self) -> Result<LowPowerInputPin<'a, 'mcr, PIN_CT>, Self::Error> {
        self.0.into_input_pin()
    }

    fn into_output_pin(
        self,
        state: PinState,
    ) -> Result<LowPowerOutputPin<'a, 'mcr, PIN_CT>, Self::Error> {
        self.0.into_output_pin(state)
    }
}

impl<'a, 'mcr, const PIN_CT: usize>
    GeneralIoPin<LowPowerInputPin<'a, 'mcr, PIN_CT>, LowPowerOutputPin<'a, 'mcr, PIN_CT>>
    for LowPowerInputPin<'a, 'mcr, PIN_CT>
{
    fn set_operating_mode(&mut self, mode: PinOperatingMode) -> Result<(), GpioError> {
        self.0.set_operating_mode(mode)
    }

    fn get_operating_mode(&self) -> PinOperatingMode {
        self.0.get_operating_mode()
    }

    fn get_io_mode(&self) -> PinIoMode {
        self.0.get_io_mode()
    }
}

pub struct LowPowerOutputPin<'a, 'mcr, const PIN_CT: usize>(LowPowerPinHandle<'a, 'mcr, PIN_CT>);

impl<'a, 'mcr, const PIN_CT: usize> OutputPin for LowPowerOutputPin<'a, 'mcr, PIN_CT> {
    type Error = Infallible;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        let reg = self.0.port.regs.gpio3_ctrl();
        match self.0.pin_idx == 0 {
            true => Ok(reg.write(|w| w.p30_do().clear_bit())),
            false => Ok(reg.write(|w| w.p31_do().clear_bit())),
        }
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        let reg = self.0.port.regs.gpio3_ctrl();
        match self.0.pin_idx == 0 {
            true => Ok(reg.write(|w| w.p30_do().set_bit())),
            false => Ok(reg.write(|w| w.p31_do().set_bit())),
        }
    }
}

impl<'a, 'mcr, const PIN_CT: usize> StatefulOutputPin for LowPowerOutputPin<'a, 'mcr, PIN_CT> {
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        let reg = self.0.port.regs.gpio3_ctrl();
        match self.0.pin_idx == 0 {
            true => Ok(reg.read().p30_do().bit_is_set()),
            false => Ok(reg.read().p31_do().bit_is_set()),
        }
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> {
        let reg = self.0.port.regs.gpio3_ctrl();
        match self.0.pin_idx == 0 {
            true => Ok(reg.read().p30_do().bit_is_clear()),
            false => Ok(reg.read().p31_do().bit_is_clear()),
        }
    }
}

/// Provides [`ToggleableOutputPin`] as a blanket implementation from OutputPin + StatefulOutputPin.
///
/// [`ToggleableOutputPin`]: #impl-ToggleableOutputPin-for-LowPowerOutputPin<'a,+PIN_CT>
impl<'a, 'mcr, const PIN_CT: usize> toggleable::Default for LowPowerOutputPin<'a, 'mcr, PIN_CT> {}

impl<'a, 'mcr, const PIN_CT: usize>
    IoPin<LowPowerInputPin<'a, 'mcr, PIN_CT>, LowPowerOutputPin<'a, 'mcr, PIN_CT>>
    for LowPowerOutputPin<'a, 'mcr, PIN_CT>
{
    type Error = Infallible;

    fn into_input_pin(self) -> Result<LowPowerInputPin<'a, 'mcr, PIN_CT>, Self::Error> {
        self.0.into_input_pin()
    }

    fn into_output_pin(
        self,
        state: PinState,
    ) -> Result<LowPowerOutputPin<'a, 'mcr, PIN_CT>, Self::Error> {
        self.0.into_output_pin(state)
    }
}

impl<'a, 'mcr, const PIN_CT: usize>
    GeneralIoPin<LowPowerInputPin<'a, 'mcr, PIN_CT>, LowPowerOutputPin<'a, 'mcr, PIN_CT>>
    for LowPowerOutputPin<'a, 'mcr, PIN_CT>
{
    fn set_operating_mode(&mut self, mode: PinOperatingMode) -> Result<(), GpioError> {
        self.0.set_operating_mode(mode)
    }

    fn get_operating_mode(&self) -> PinOperatingMode {
        self.0.get_operating_mode()
    }

    fn get_io_mode(&self) -> PinIoMode {
        self.0.get_io_mode()
    }
}
