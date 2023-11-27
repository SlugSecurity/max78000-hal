use core::convert::Infallible;
use core::marker::PhantomData;

use max78000::mcr::{GPIO3_CTRL, OUTEN};
use max78000::MCR;
use sealed::sealed;

use super::pin_traits::{
    toggleable, GeneralIoPin, InputPin, IoPin, OutputPin, PinState, StatefulOutputPin,
};

use super::{
    GpioError, GpioPort, GpioPortMetadata, PinHandle, PinIoMode, PinOperatingMode,
    __seal_gpio_port_metadata,
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
    type PinHandleType<'a, const PIN_CT: usize> = LowPowerPinHandle<'a, 'mcr, PIN_CT>    where
    'mcr: 'a;
    // How do we prove to the compiler that this is always true in the lifetimes we provide PinHandleType?

    // basically the lifetime provided into here is used for the lifetime of &MCR in LowPowerGpioRegs.
    // that way, the lifetime doesnt need to be specified inside GpioPort
    // but b/c LowPowerPinHandle has a reference to GpioPort, it also needs to specify &MCR's lifetime
    // but we know that whatever lifetime is provided into there already will outlive 'a
    // but coming back to LowPowerGpio, we need to express that the 'mcr lifetime provided into there outlives the lifetime
    // provided to PinHandleType which is 'a (this lifetime is the lifetime of the struct in GpioPort so it always does).
    // but the compiler doesnt know that so we need to enforce that bound here but how?

    // can we bound PinHandleType's 'a lifetime to the lifetime of the struct, perhaps by making GpioPortMetadata generic over some lifetime?

    // if we make gpioportmetadata generic over <'b> and constrain PinHandleType to <'b: 'a> where LowPowerGpio<'mcr> implements GpioPortMetadata<'mcr>
    // then CommonGpio can implement GpioPortMetadata<'static>?

    // then GpioPort will need to use GpioPortMetadata

    // alternativly, can we selectively implement GpioPortMetadata only if 'mcr is constrained to 'a? probs not b/c any 'a can be chosen unless we can somehow defer that error to later?

    // is there a way to express below that idgaf about the 'mcr lifetime in LowPowerPinHandle as long as I get smth?

    // maybe there is, what if i make low power pin handle generic over GpioPortMetadata rather than LowPowerGpio<'mcr>??
    // but then how do i do things specific to GpioPort<LowPowerGpio<'_>, PIN_CT>?? like peripheral access
    // u can get the GpioRegs type but it'll be an unknown type, not LowPowerGpioRegs

    // i think the main issue is that we need to know 'mcr's lifetime in case we give it out but in this case
    // we're never giving it out so we dont care about mcr's lifetime b/c we're always using it within LowPowerPinHandle

    type GpioRegs = &'mcr MCR;
}

/// `PinHandle` implementation for low power GPIO ports.
pub struct LowPowerPinHandle<'a, 'mcr, const PIN_CT: usize> {
    port: &'a GpioPort<'mcr, LowPowerGpio<'mcr>, PIN_CT>, // 'mcr here shouldnt be determined by LowPOwerPinHandle
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

impl<'a, 'mcr, const PIN_CT: usize> PinHandle<'a> for LowPowerPinHandle<'a, 'mcr, PIN_CT> {
    type Port = GpioPort<'mcr, LowPowerGpio<'mcr>, PIN_CT>;

    fn new(port: &'a Self::Port, pin_idx: usize) -> Self {
        Self { port, pin_idx }
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
