//! Tests for GPIO0-GPIO3.

use core::fmt::Write;
use cortex_m_semihosting::hio;
use max78000_hal::peripherals::{
    gpio::{
        active::{port_num_types::GpioPortNum, ActiveGpio},
        pin_traits::{GeneralIoPin, InputPin, IoPin, OutputPin, PinState, StatefulOutputPin},
        Gpio0, Gpio1, Gpio2, GpioError, GpioPort, PinIoMode,
    },
    PeripheralHandle,
};

/// Runs all GPIO tests.
pub fn run_gpio_tests(
    gpio0_port: PeripheralHandle<'_, Gpio0>,
    gpio1_port: PeripheralHandle<'_, Gpio1>,
    gpio2_port: PeripheralHandle<'_, Gpio2>,
    stdout: &mut hio::HostStream,
) {
    writeln!(stdout, "Starting GPIO peripheral tests...").unwrap();

    // Run tests.

    // Note: Tests should be made generic over traits like GeneralIoPin, InputPin, and StatefulOutputPin
    // Write sanity checks for now (writing a value then reading it) -- physical tests will come later

    test_active_port(&gpio0_port);
    test_active_port(&gpio1_port);
    test_active_port(&gpio2_port);

    writeln!(stdout, "GPIO peripheral tests complete!\n").unwrap();
}

fn test_active_port<const PIN_CT: usize>(
    port: &GpioPort<'static, ActiveGpio<impl GpioPortNum + 'static>, PIN_CT>,
) {
    let pin = port.get_pin_handle(PIN_CT - 1).unwrap();
    assert!(matches!(
        port.get_pin_handle(PIN_CT - 1),
        Err(GpioError::HandleAlreadyTaken)
    ));
    assert!(matches!(
        port.get_pin_handle(PIN_CT),
        Err(GpioError::InvalidPinIndex)
    ));

    let mut pin = pin.into_input_pin().unwrap();
    assert!(matches!(pin.get_io_mode(), PinIoMode::Input));
    assert_ne!(pin.is_low(), pin.is_high());

    let mut pin = pin.into_output_pin(PinState::High).unwrap();
    assert!(matches!(pin.get_io_mode(), PinIoMode::Output));
    assert!(pin.is_set_high().unwrap());
    pin.set_low().unwrap();
    assert!(pin.is_set_low().unwrap());
    pin.set_high().unwrap();
    assert!(pin.is_set_high().unwrap());
    drop(pin);

    let _pin = port.get_pin_handle(PIN_CT - 1).unwrap();
    assert!(matches!(
        port.get_pin_handle(PIN_CT - 1),
        Err(GpioError::HandleAlreadyTaken)
    ));
}
