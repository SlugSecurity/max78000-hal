//! Tests for GPIO0-GPIO3.

use core::fmt::Write;
use cortex_m_semihosting::hio;
use max78000_hal::peripherals::gpio::{
    active::{
        port_num_types::GpioPortNum, ActiveGpio, ActiveInputPinConfig, ActiveOutputPinConfig,
    },
    pin_traits::{InputPin, IoPin, OutputPin, PinState, StatefulOutputPin},
    Gpio0, Gpio1, Gpio2, GpioError, GpioPort, PinIoMode, PinOperatingMode,
};

/// Runs all GPIO tests.
pub fn run_gpio_tests(
    gpio0_port: &Gpio0,
    gpio1_port: &Gpio1,
    gpio2_port: &Gpio2,
    stdout: &mut hio::HostStream,
) {
    writeln!(stdout, "Starting GPIO peripheral tests...").unwrap();

    // Run tests.

    // Note: Tests should be made generic over traits like GeneralIoPin, InputPin, and StatefulOutputPin
    // Write sanity checks for now (writing a value then reading it) -- physical tests will come later

    test_active_port(gpio0_port);
    test_active_port(gpio1_port);
    test_active_port(gpio2_port);

    writeln!(stdout, "GPIO peripheral tests complete!\n").unwrap();
}

fn test_active_port<const PIN_CT: usize, PortNum: GpioPortNum + 'static>(
    port: &GpioPort<'static, ActiveGpio<PortNum>, PIN_CT>,
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

    let mut pin = pin.into_input_pin(ActiveInputPinConfig::default()).unwrap();
    assert_eq!(pin.get_io_mode(), PinIoMode::Input);
    assert_eq!(pin.get_operating_mode(), Default::default());
    assert_eq!(pin.get_power_supply(), Default::default());
    assert_eq!(pin.get_pull_mode(), Default::default());

    assert_ne!(pin.is_low(), pin.is_high());

    let mut pin = pin
        .into_output_pin(PinState::High, ActiveOutputPinConfig::default())
        .unwrap();
    assert_eq!(pin.get_io_mode(), PinIoMode::Output);
    assert_eq!(pin.get_operating_mode(), Default::default());
    assert_eq!(pin.get_power_supply(), Default::default());
    assert_eq!(pin.get_drive_strength(), Default::default());

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

    if matches!(PortNum::PORT_NUM, 0 | 2) {
        let mut rx = port
            .get_pin_handle(0)
            .unwrap()
            .into_input_pin(ActiveInputPinConfig {
                operating_mode: PinOperatingMode::AltFunction1,
                ..Default::default()
            })
            .unwrap();
        let mut tx = port
            .get_pin_handle(1)
            .unwrap()
            .into_output_pin(
                PinState::Low,
                ActiveOutputPinConfig {
                    operating_mode: PinOperatingMode::AltFunction1,
                    ..Default::default()
                },
            )
            .unwrap();
        assert_eq!(rx.get_operating_mode(), PinOperatingMode::AltFunction1);
        assert_eq!(tx.get_operating_mode(), PinOperatingMode::AltFunction1);
        assert_eq!(
            rx.set_operating_mode(PinOperatingMode::AltFunction2),
            Err(GpioError::BadOperatingMode)
        );
        assert_eq!(
            tx.set_operating_mode(PinOperatingMode::AltFunction2),
            Err(GpioError::BadOperatingMode)
        );
    }
}
