use core::convert::Infallible;
use core::marker::PhantomData;

use sealed::sealed;

use port_num_types::GpioPortNum;

use super::pin_traits::{GeneralIoPin, InputPin, IoPin, OutputPin, PinState, StatefulOutputPin};
use super::private::NonConstructible;
use super::{
    GpioError, GpioPort, GpioPortMetadata, PinHandle, PinIoMode, PinOperatingMode,
    __seal_gpio_port_metadata, __seal_pin_handle,
};

pub mod port_num_types;

// TODO FOR ASTRA:
// - make input pin and output pin structs
//       - pin types should implement InputPin for input pin and StatefulOutputPin for output pin
//       - these pins should be a newtype wrapping the pin handle so drop works on it properly
//       - all pin types, including PinHandle, should implement GeneralIoPin<INPUT_PIN_TYPE, OUTPUT_PIN_TYPE>
//       - note: to implement the error checking for alternate functions,
//               implement it based on the port num given back with GpioPortNum::PORT_NUM
//
// - after implementing above trait functionality
//       - implement weak/strong pullup/pulldown resistor configuration (input mode only)
//       - also drive strength and power supply (output mode only)
//
// - see low power module for example on everything above
//
// - add interrupt support (input mode only)
//       - just need to support adding 1 listener per pin
//       - the listener can either be low/high level triggered, rising/falling edge triggered, or dual edge triggered
//       - if developer provides another listener through same function, overwrite previous listener
// - add documentation
//     - a module-level doc comment
//     - public functions within this module that aren't trait impl functions
//     - other public items like structs
//     - on the super module with examples of how to use the API (can tell user to see user guide and datasheet too)
//     - improve existing comments in entire driver to add more detail
// - add unit tests for acquiring handles, releasing handles and recaquiring
// - add unit tests for each public function in the common pin API

/// Marker struct implementing `GpioPortMetadata` for
/// common GPIO ports.
pub struct CommonGpio<Port: GpioPortNum>(PhantomData<Port>);

#[sealed]
impl<Port: GpioPortNum + 'static> GpioPortMetadata<'static> for CommonGpio<Port> {
    type PinHandleType<'a, const PIN_CT: usize> = CommonPinHandle<'a, Port, PIN_CT>;
    type GpioRegs = Port::Peripheral;
}

/// `PinHandle` implementation for common GPIO ports.
pub struct CommonPinHandle<'a, Port: GpioPortNum + 'static, const PIN_CT: usize> {
    port: &'a GpioPort<'static, CommonGpio<Port>, PIN_CT>,
    pin_idx: usize,
}

impl<'a, Port: GpioPortNum + 'static, const PIN_CT: usize> Drop
    for CommonPinHandle<'a, Port, PIN_CT>
{
    fn drop(&mut self) {
        // When handle is dropped, allow the pin to be taken again.
        self.port.pin_taken[self.pin_idx].set(false);
    }
}

#[sealed]
impl<'a, Port: GpioPortNum + 'static, const PIN_CT: usize> PinHandle<'a>
    for CommonPinHandle<'a, Port, PIN_CT>
{
    type Port = GpioPort<'static, CommonGpio<Port>, PIN_CT>;

    fn new(_private: NonConstructible, port: &'a Self::Port, pin_idx: usize) -> Self {
        // We can't get rid of the const generic here or otherwise prevent a bad pin count
        // from being entered until more complex exprs can be evaluated in const generics stably.
        // So there are asserts here to ensure they can't be constructed. The construction of these
        // handles are done privately and not able to be done externally so this is fine.
        assert!(PIN_CT <= 32); // Any common port can have up to 32 pins based on the registers
        assert!(pin_idx < PIN_CT);

        Self { port, pin_idx }
    }

    fn get_pin_idx(&self) -> usize {
        self.pin_idx
    }
}

pub struct CommonInputPin<'a, Port: GpioPortNum + 'static, const PIN_CT: usize>(
    CommonPinHandle<'a, Port, PIN_CT>,
);

impl<Port: GpioPortNum + 'static, const PIN_CT: usize> InputPin
    for CommonInputPin<'_, Port, PIN_CT>
{
    type Error = Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.0.port.regs.in_().read().bits() & (1 << self.0.pin_idx) != 0)
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        self.is_high().map(|x| !x)
    }
}

pub struct CommonOutputPin<'a, Port: GpioPortNum + 'static, const PIN_CT: usize>(
    CommonPinHandle<'a, Port, PIN_CT>,
);

impl<Port: GpioPortNum + 'static, const PIN_CT: usize> OutputPin
    for CommonOutputPin<'_, Port, PIN_CT>
{
    type Error = Infallible;

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.0
            .port
            .regs
            .out_set()
            .write(|w| unsafe { w.bits(1 << self.0.pin_idx) });
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.0
            .port
            .regs
            .out_clr()
            .write(|w| unsafe { w.bits(1 << self.0.pin_idx) });
        Ok(())
    }
}

impl<Port: GpioPortNum + 'static, const PIN_CT: usize> StatefulOutputPin
    for CommonOutputPin<'_, Port, PIN_CT>
{
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(self.0.port.regs.out().read().bits() & (1 << self.0.pin_idx) != 0)
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> {
        self.is_set_high().map(|x| !x)
    }
}

impl<'a, Port: GpioPortNum + 'static, const PIN_CT: usize>
    IoPin<CommonInputPin<'a, Port, PIN_CT>, CommonOutputPin<'a, Port, PIN_CT>>
    for CommonPinHandle<'a, Port, PIN_CT>
{
    type Error = Infallible;

    fn into_input_pin(self) -> Result<CommonInputPin<'a, Port, PIN_CT>, Self::Error> {
        self.port
            .regs
            .outen_clr()
            .write(|w| unsafe { w.bits(1 << self.pin_idx) });
        self.port
            .regs
            .inen()
            .modify(|r, w| unsafe { w.bits(r.bits() | (1 << self.pin_idx)) });
        Ok(CommonInputPin(self))
    }

    fn into_output_pin(
        self,
        state: PinState,
    ) -> Result<CommonOutputPin<'a, Port, PIN_CT>, Self::Error> {
        self.port
            .regs
            .inen()
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.pin_idx)) });
        self.port
            .regs
            .outen_set()
            .write(|w| unsafe { w.bits(1 << self.pin_idx) });
        let mut pin = CommonOutputPin(self);
        match state {
            PinState::Low => pin.set_low()?,
            PinState::High => pin.set_high()?,
        }
        Ok(pin)
    }
}

impl<'a, Port: GpioPortNum + 'static, const PIN_CT: usize>
    GeneralIoPin<CommonInputPin<'a, Port, PIN_CT>, CommonOutputPin<'a, Port, PIN_CT>>
    for CommonPinHandle<'a, Port, PIN_CT>
{
    fn set_operating_mode(&mut self, mode: PinOperatingMode) -> Result<(), GpioError> {
        match mode {
            PinOperatingMode::DigitalIo => {
                self.port
                    .regs
                    .en0_set()
                    .write(|w| w.all().variant(1 << self.pin_idx));
            }
            PinOperatingMode::AltFunction1 => {
                self.port
                    .regs
                    .en1_clr()
                    .write(|w| w.all().variant(1 << self.pin_idx));
                self.port
                    .regs
                    .en0_clr()
                    .write(|w| w.all().variant(1 << self.pin_idx));
            }
            PinOperatingMode::AltFunction2 => {
                self.port
                    .regs
                    .en1_set()
                    .write(|w| w.all().variant(1 << self.pin_idx));
                self.port
                    .regs
                    .en0_clr()
                    .write(|w| w.all().variant(1 << self.pin_idx));
            }
        }
        Ok(())
    }

    fn get_operating_mode(&self) -> PinOperatingMode {
        match (
            self.port.regs.en0().read().bits() & (1 << self.pin_idx) != 0,
            self.port.regs.en1().read().bits() & (1 << self.pin_idx) != 0,
        ) {
            (false, false) => PinOperatingMode::AltFunction1,
            (false, true) => PinOperatingMode::AltFunction2,
            (true, _) => PinOperatingMode::DigitalIo,
        }
    }

    fn get_io_mode(&self) -> PinIoMode {
        let is_out = self.port.regs.outen().read().bits() & (1 << self.pin_idx) != 0;
        if is_out {
            PinIoMode::Output
        } else {
            PinIoMode::Input
        }
    }
}

impl<'a, Port: GpioPortNum + 'static, const PIN_CT: usize>
    IoPin<CommonInputPin<'a, Port, PIN_CT>, CommonOutputPin<'a, Port, PIN_CT>>
    for CommonInputPin<'a, Port, PIN_CT>
{
    type Error = Infallible;

    fn into_input_pin(self) -> Result<CommonInputPin<'a, Port, PIN_CT>, Self::Error> {
        self.0.into_input_pin()
    }

    fn into_output_pin(
        self,
        state: PinState,
    ) -> Result<CommonOutputPin<'a, Port, PIN_CT>, Self::Error> {
        self.0.into_output_pin(state)
    }
}

impl<'a, Port: GpioPortNum + 'static, const PIN_CT: usize>
    GeneralIoPin<CommonInputPin<'a, Port, PIN_CT>, CommonOutputPin<'a, Port, PIN_CT>>
    for CommonInputPin<'a, Port, PIN_CT>
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

impl<'a, Port: GpioPortNum + 'static, const PIN_CT: usize>
    IoPin<CommonInputPin<'a, Port, PIN_CT>, CommonOutputPin<'a, Port, PIN_CT>>
    for CommonOutputPin<'a, Port, PIN_CT>
{
    type Error = Infallible;

    fn into_input_pin(self) -> Result<CommonInputPin<'a, Port, PIN_CT>, Self::Error> {
        self.0.into_input_pin()
    }

    fn into_output_pin(
        self,
        state: PinState,
    ) -> Result<CommonOutputPin<'a, Port, PIN_CT>, Self::Error> {
        self.0.into_output_pin(state)
    }
}

impl<'a, Port: GpioPortNum + 'static, const PIN_CT: usize>
    GeneralIoPin<CommonInputPin<'a, Port, PIN_CT>, CommonOutputPin<'a, Port, PIN_CT>>
    for CommonOutputPin<'a, Port, PIN_CT>
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
