use max78000::gpio0;

use super::pin_traits::IoPin;
use super::{GpioPort, GpioPortMetadata, PinHandle};

trait CommonIoPin {
    // TODO: put functions that can be used regardless of whether the pin is in
    // input or output mode -- like fetching the state of the pin (whether in input or output mode)

    // Make CommonIoPin require IoPin from embedded_hal once the input and output pin types are made here.
    // Make input and output pin types implement InputPin and StatefulOutputPin, respectively.
    // See low power module for example.
}

/// Marker struct implementing `GpioPortMetadata` for
/// common GPIO ports.
pub struct CommonGpio;

impl GpioPortMetadata for CommonGpio {
    type PinHandleType<'a, const PIN_CT: usize> = CommonPinHandle<'a, PIN_CT>;
    type GpioRegs = gpio0::RegisterBlock;
}

/// `PinHandle` implementation for common GPIO ports.
pub struct CommonPinHandle<'a, const PIN_CT: usize> {
    port: &'a GpioPort<CommonGpio, PIN_CT>,
    pin_idx: usize,
}

impl<'a, const PIN_CT: usize> Drop for CommonPinHandle<'a, PIN_CT> {
    fn drop(&mut self) {
        // When handle is dropped, allow the pin to be taken again.
        self.port.pin_taken[self.pin_idx].set(false);
    }
}

impl<'a, const PIN_CT: usize> PinHandle<'a> for CommonPinHandle<'a, PIN_CT> {
    type Port = GpioPort<CommonGpio, PIN_CT>;

    fn new(port: &'a Self::Port, pin_idx: usize) -> Self {
        assert!(pin_idx < PIN_CT);

        Self { port, pin_idx }
    }
}
