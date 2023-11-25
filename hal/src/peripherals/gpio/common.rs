use max78000::gpio0;

use super::{GpioPort, GpioPortMetadata, PinHandle};

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

impl<'a, const PIN_CT: usize> CommonPinHandle<'a, PIN_CT> {
    // add pin state function (input or output)
}

// TODO: impl IoPin
// TODO: impl Drop

impl<'a, const PIN_CT: usize> PinHandle<'a> for CommonPinHandle<'a, PIN_CT> {
    type Port = GpioPort<CommonGpio, PIN_CT>;

    fn new(port: &'a Self::Port, pin_idx: usize) -> Self {
        Self { port, pin_idx }
    }
}
