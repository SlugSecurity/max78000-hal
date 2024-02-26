//! Tests for GPIO0-GPIO3.

use core::fmt::Write;
use cortex_m_semihosting::hio;
use max78000_hal::{
    max78000::{GCR, GPIO0, GPIO1, GPIO2, LPGCR},
    peripherals::gpio::{
        active::{port_num_types::GpioPortNum, ActiveGpio},
        new_gpio0, new_gpio1, new_gpio2,
        pin_traits::{GeneralIoPin, InputPin, IoPin, OutputPin, PinState, StatefulOutputPin},
        GpioError, GpioPort, PinIoMode,
    },
};

/// Runs all GPIO tests.
pub fn run_gpio_tests(
    gpio0: GPIO0,
    gpio1: GPIO1,
    gpio2: GPIO2,
    gcr: &GCR,
    lpgcr: &LPGCR,
    stdout: &mut hio::HostStream,
) {
    writeln!(stdout, "Starting GPIO peripheral tests...").unwrap();

    // Enable GPIO port clocks. This will be done by the peripheral API when available.
    // TODO: Remove this when the peripheral API is available.
    gcr.pclkdis0().write(|w| w.gpio0().en().gpio1().en());
    lpgcr.pclkdis().write(|w| w.gpio2().en());

    // Run tests.
    let gpio0_port = new_gpio0(gpio0);
    let gpio1_port = new_gpio1(gpio1);
    let gpio2_port = new_gpio2(gpio2);

    // Note: Tests should be made generic over traits like GeneralIoPin, InputPin, and StatefulOutputPin
    // Write sanity checks for now (writing a value then reading it) -- physical tests will come later

    test_active_port(gpio0_port);
    test_active_port(gpio1_port);
    test_active_port(gpio2_port);

    writeln!(stdout, "GPIO peripheral tests complete!\n").unwrap();
}

fn test_active_port<const PIN_CT: usize>(
    port: GpioPort<'static, ActiveGpio<impl GpioPortNum + 'static>, PIN_CT>,
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
