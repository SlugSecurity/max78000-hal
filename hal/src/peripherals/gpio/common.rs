use core::marker::PhantomData;

use sealed::sealed;

use self::port_num_types::GpioPortNum;

use super::{GpioPort, GpioPortMetadata, PinHandle, __seal_gpio_port_metadata};

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
impl<Port> GpioPortMetadata<'static> for CommonGpio<Port>
where
    for<'ah> Port: GpioPortNum + 'ah,
{
    type PinHandleType<'a, const PIN_CT: usize> = CommonPinHandle<'a, Port, PIN_CT>;
    type GpioRegs = Port::Peripheral;
}

// TODO: we could move the const generic into GpioPortNum as associated constant and def should

// TODO: seal PinHandle and privatize PinHandle::new so only the associated type is publicly visible?

/// `PinHandle` implementation for common GPIO ports.
pub struct CommonPinHandle<'a, Port, const PIN_CT: usize>
where
    for<'ah> Port: GpioPortNum + 'ah,
{
    port: &'a GpioPort<'static, CommonGpio<Port>, PIN_CT>,
    pin_idx: usize,
}

impl<'a, Port, const PIN_CT: usize> Drop for CommonPinHandle<'a, Port, PIN_CT>
where
    for<'ah> Port: GpioPortNum + 'ah,
{
    fn drop(&mut self) {
        // When handle is dropped, allow the pin to be taken again.
        self.port.pin_taken[self.pin_idx].set(false);
    }
}

impl<'a, Port, const PIN_CT: usize> PinHandle<'a> for CommonPinHandle<'a, Port, PIN_CT>
where
    for<'ah> Port: GpioPortNum + 'ah,
{
    type Port = GpioPort<'static, CommonGpio<Port>, PIN_CT>;

    fn new(port: &'a Self::Port, pin_idx: usize) -> Self {
        assert!(pin_idx < PIN_CT);

        Self { port, pin_idx }
    }
}
