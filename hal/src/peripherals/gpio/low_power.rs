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
// - implement pullup resistor configuration (should be for just input mode, confirm this)
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
        todo!()
    }

    fn into_output_pin(
        self,
        state: PinState,
    ) -> Result<LowPowerOutputPin<'a, 'mcr, PIN_CT>, Self::Error> {
        todo!()
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

        let r = &self.port.regs.outen;

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
        todo!()
    }

    fn get_io_mode(&self) -> PinIoMode {
        todo!()
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

impl<'a, 'mcr, const PIN_CT: usize> InputPin for LowPowerInputPin<'a, 'mcr, PIN_CT> {
    type Error = Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        todo!()
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        todo!()
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
        todo!()
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}

impl<'a, 'mcr, const PIN_CT: usize> StatefulOutputPin for LowPowerOutputPin<'a, 'mcr, PIN_CT> {
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        todo!()
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> {
        todo!()
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
