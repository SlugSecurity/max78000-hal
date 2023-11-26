use core::convert::Infallible;

use max78000::mcr::{GPIO3_CTRL, OUTEN};

use super::pin_traits::{toggleable, GeneralIoPin, InputPin, IoPin, OutputPin, StatefulOutputPin};

use super::{GpioError, GpioPort, GpioPortMetadata, PinHandle, PinIoMode, PinOperatingMode};

// TODO for arelyx:
// - implement functions in traits that aren't complete below (see LowPowerPinHandle::set_operating_mode for example)
// - implement pullup resistor configuration (should be for just input mode, confirm this)
// - add documentation
//     - a module-level doc comment
//     - public functions within this module that aren't trait impl functions
//     - other public items like structs
//     - improve existing comments in entire driver to add more detail
// - add unit tests for each public function in the low power pin API

pub struct LowPowerGpioRegs {
    gpio_ctrl: GPIO3_CTRL,
    gpio3_alt_fn_enablers: OUTEN,
}

/// Marker struct implementing `GpioPortMetadata` for
/// low power GPIO ports.
pub struct LowPowerGpio;

impl GpioPortMetadata for LowPowerGpio {
    type PinHandleType<'a, const PIN_CT: usize> = LowPowerPinHandle<'a, PIN_CT>;
    type GpioRegs = LowPowerGpioRegs;
}

/// `PinHandle` implementation for low power GPIO ports.
pub struct LowPowerPinHandle<'a, const PIN_CT: usize> {
    port: &'a GpioPort<LowPowerGpio, PIN_CT>,
    pin_idx: usize,
}

impl<'a, const PIN_CT: usize> IoPin<LowPowerInputPin<'a, PIN_CT>, LowPowerOutputPin<'a, PIN_CT>>
    for LowPowerPinHandle<'a, PIN_CT>
{
    type Error = Infallible;

    fn into_input_pin(self) -> Result<LowPowerInputPin<'a, PIN_CT>, Self::Error> {
        todo!()
    }

    fn into_output_pin(
        self,
        state: embedded_hal::digital::v2::PinState,
    ) -> Result<LowPowerOutputPin<'a, PIN_CT>, Self::Error> {
        todo!()
    }
}

impl<'a, const PIN_CT: usize>
    GeneralIoPin<LowPowerInputPin<'a, PIN_CT>, LowPowerOutputPin<'a, PIN_CT>>
    for LowPowerPinHandle<'a, PIN_CT>
{
    fn set_operating_mode(&mut self, mode: PinOperatingMode) -> Result<(), GpioError> {
        use GpioError::*;
        use PinIoMode::*;

        // User guide is very confusing but
        // - GPIO 3 pin 0's alt fn 1 is for PDOWN (it outputs data on the pin)
        //     - the MCR_OUTEN register at the pdown_out_en bit is how you switch the alternate function
        // - GPIO 3 pin 1's alt fn 1 is for SQWOUT (it outputs data on the pin)
        //     - the MCR_OUTEN register at the sqwout_en bit is how you switch the alternate function

        let r = &self.port.regs.gpio3_alt_fn_enablers;

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

impl<'a, const PIN_CT: usize> Drop for LowPowerPinHandle<'a, PIN_CT> {
    fn drop(&mut self) {
        // When handle is dropped, allow the pin to be taken again.
        self.port.pin_taken[self.pin_idx].set(false);
    }
}

impl<'a, const PIN_CT: usize> PinHandle<'a> for LowPowerPinHandle<'a, PIN_CT> {
    type Port = GpioPort<LowPowerGpio, PIN_CT>;

    fn new(port: &'a Self::Port, pin_idx: usize) -> Self {
        Self { port, pin_idx }
    }
}

pub struct LowPowerInputPin<'a, const PIN_CT: usize>(LowPowerPinHandle<'a, PIN_CT>);

impl<'a, const PIN_CT: usize> InputPin for LowPowerInputPin<'a, PIN_CT> {
    type Error = Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        todo!()
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        todo!()
    }
}

impl<'a, const PIN_CT: usize> IoPin<LowPowerInputPin<'a, PIN_CT>, LowPowerOutputPin<'a, PIN_CT>>
    for LowPowerInputPin<'a, PIN_CT>
{
    type Error = Infallible;

    fn into_input_pin(self) -> Result<LowPowerInputPin<'a, PIN_CT>, Self::Error> {
        self.0.into_input_pin()
    }

    fn into_output_pin(
        self,
        state: embedded_hal::digital::v2::PinState,
    ) -> Result<LowPowerOutputPin<'a, PIN_CT>, Self::Error> {
        self.0.into_output_pin(state)
    }
}

impl<'a, const PIN_CT: usize>
    GeneralIoPin<LowPowerInputPin<'a, PIN_CT>, LowPowerOutputPin<'a, PIN_CT>>
    for LowPowerInputPin<'a, PIN_CT>
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

pub struct LowPowerOutputPin<'a, const PIN_CT: usize>(LowPowerPinHandle<'a, PIN_CT>);

impl<'a, const PIN_CT: usize> OutputPin for LowPowerOutputPin<'a, PIN_CT> {
    type Error = Infallible;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}

impl<'a, const PIN_CT: usize> StatefulOutputPin for LowPowerOutputPin<'a, PIN_CT> {
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        todo!()
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> {
        todo!()
    }
}

/// To implement ToggleableOutputPin from OutputPin + StatefulOutputPin.
impl<'a, const PIN_CT: usize> toggleable::Default for LowPowerOutputPin<'a, PIN_CT> {}

impl<'a, const PIN_CT: usize> IoPin<LowPowerInputPin<'a, PIN_CT>, LowPowerOutputPin<'a, PIN_CT>>
    for LowPowerOutputPin<'a, PIN_CT>
{
    type Error = Infallible;

    fn into_input_pin(self) -> Result<LowPowerInputPin<'a, PIN_CT>, Self::Error> {
        self.0.into_input_pin()
    }

    fn into_output_pin(
        self,
        state: embedded_hal::digital::v2::PinState,
    ) -> Result<LowPowerOutputPin<'a, PIN_CT>, Self::Error> {
        self.0.into_output_pin(state)
    }
}

impl<'a, const PIN_CT: usize>
    GeneralIoPin<LowPowerInputPin<'a, PIN_CT>, LowPowerOutputPin<'a, PIN_CT>>
    for LowPowerOutputPin<'a, PIN_CT>
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
