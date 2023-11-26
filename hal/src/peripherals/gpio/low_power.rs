use max78000::mcr::GPIO3_CTRL;

use super::{GpioPort, GpioPortMetadata, PinHandle};

/// Marker struct implementing `GpioPortMetadata` for
/// low power GPIO ports.
pub struct LowPowerGpio;

impl GpioPortMetadata for LowPowerGpio {
    type PinHandleType<'a, const PIN_CT: usize> = LowPowerPinHandle<'a, PIN_CT>;
    type GpioRegs = GPIO3_CTRL;
}

/// `PinHandle` implementation for low power GPIO ports.
pub struct LowPowerPinHandle<'a, const PIN_CT: usize> {
    port: &'a GpioPort<LowPowerGpio, PIN_CT>,
    pin_idx: usize,
}

impl<'a, const PIN_CT: usize> LowPowerPinHandle<'a, PIN_CT> {
    // add pin state function to get whether the pin is in input or output mode
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
