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
        let (alt1_in, alt1_out, alt2_in, alt2_out) = match (Port::PORT_NUM, self.pin_idx) {
            (0, 0) => (true, false, false, false), //    UART0A_RX                -
            (0, 1) => (false, true, false, false), //    UART0A_TX                -
            (0, 2) => (false, false, false, false), //   TMR0A__IOA               UART0B_CTS
            (0, 3) => (false, false, false, false), //   EXT_CLK/TMR0A_IOB        UART0B_RTS
            (0, 4) => (false, false, false, false), //   SPI0_SS0                 TMR0B_IOAN
            (0, 5) => (false, false, false, false), //   SPI0_MOSI                TMR0B_IOBN
            (0, 6) => (false, false, false, false), //   SPI0_MISO                OWM_IO
            (0, 7) => (false, false, false, false), //   SPI0_SCK                 OWM_PE
            (0, 8) => (false, false, false, false), //   SPI0_SDIO2               TMR0B_IOA
            (0, 9) => (false, false, false, false), //   SPI0_SDIO3               TMR0B_IOB
            (0, 10) => (false, false, false, false), //  I2C0_SCL                 SPI0_SS2
            (0, 11) => (false, false, false, false), //  I2C0_SDA                 SPI0_SS1
            (0, 12) => (true, false, false, false), //   UART1A_RX                TMR1B_IOAN
            (0, 13) => (false, true, false, false), //   UART1A_TX                TMR1B_IOBN
            (0, 14) => (false, false, false, false), //  TMR1A_IOA                I2S_CLKEXT
            (0, 15) => (false, false, false, false), //  TMR1A_IOB                PCIF_VSYNC
            (0, 16) => (false, false, false, false), //  I2C1_SCL                 PT2
            (0, 17) => (false, false, false, false), //  I2C1_SDA                 PT3
            (0, 18) => (false, false, false, false), //  PT0                      OWM_IO
            (0, 19) => (false, false, false, false), //  PT1                      OWM_PE
            (0, 20) => (false, false, false, false), //  SPI1_SS0                 PCIF_D0
            (0, 21) => (false, false, false, false), //  SPI1_MOSI                PCIF_D1
            (0, 22) => (false, false, false, false), //  SPI1_MISO                PCIF_D2
            (0, 23) => (false, false, false, false), //  SPI1_SCK                 PCIF_D3
            (0, 24) => (false, false, false, false), //  SPI1_SDIO2               PCIF_D4
            (0, 25) => (false, false, false, false), //  SPI1_SDIO3               PCIF_D5
            (0, 26) => (false, false, false, false), //  TMR2A_IOA                PCIF_D6
            (0, 27) => (false, false, false, false), //  TMR2A_IOB                PCIF_D7
            (0, 28) => (false, false, false, false), //  SWDIO                    -
            (0, 29) => (false, false, false, false), //  SWCLK                    -
            (0, 30) => (false, false, false, false), //  I2C2_SCL                 PCIF_D8
            (0, 31) => (false, false, false, false), //  I2C2_SDA                 PCIF_D9
            (1, 0) => (true, false, false, false), //    UART2A_RX                RV_TCK
            (1, 1) => (false, true, false, false), //    UART2A_TX                RV_TMS
            (1, 2) => (false, false, false, false), //   I2S_SCK                  RV_TDI
            (1, 3) => (false, false, false, false), //   I2S_WS                   RV_TDO
            (1, 4) => (false, false, false, false), //   I2S_SDI                  TMR3B_IOA
            (1, 5) => (false, false, false, false), //   I2S_SDO                  TMR3B_IOB
            (1, 6) => (false, false, false, false), //   TMR3A_IOA                PCIF_D10
            (1, 7) => (false, false, false, false), //   TMR3A_IOB                PCIF_D11
            (1, 8) => (false, false, false, false), //   PCIF_HSYNC               RXEV0
            (1, 9) => (false, false, false, false), //   PCIF_PCLK                TXEV0
            (2, 0) => (false, false, false, false), //   AIN0/AINON               -
            (2, 1) => (false, false, false, false), //   AIN1/AIN0P               -
            (2, 2) => (false, false, false, false), //   AIN2/AIN1N               -
            (2, 3) => (false, false, false, false), //   AIN3/AIN1P               -
            (2, 4) => (false, false, false, false), //   AIN4/AIN2N               LPTMR0B_IOA
            (2, 5) => (false, false, false, false), //   AIN5/AIN2P               LPTMR1_IOA
            (2, 6) => (false, false, false, false), //   LPTMR0_CLK/AIN6/AIN3N    LPUARTB_RX
            (2, 7) => (false, false, false, false), //   LPTMR1_CLK/AIN7/AIN3P    LPUARTB_TX
            (3, 0) => (false, false, false, false), //   PDOWN                    WAKEUP
            (3, 1) => (false, false, false, false), //   SQWOUT                   WAKEUP
            _ => (false, false, false, false),
        };

        let (alt1, alt2) = match self.get_io_mode() {
            PinIoMode::Input => (alt1_in, alt2_in),
            PinIoMode::Output => (alt1_out, alt2_out),
        };

        match mode {
            PinOperatingMode::DigitalIo => {
                self.port
                    .regs
                    .en0_set()
                    .write(|w| w.all().variant(1 << self.pin_idx));
            }
            PinOperatingMode::AltFunction1 if alt1 => {
                self.port
                    .regs
                    .en1_clr()
                    .write(|w| w.all().variant(1 << self.pin_idx));
                self.port
                    .regs
                    .en0_clr()
                    .write(|w| w.all().variant(1 << self.pin_idx));
            }
            PinOperatingMode::AltFunction2 if alt2 => {
                self.port
                    .regs
                    .en1_set()
                    .write(|w| w.all().variant(1 << self.pin_idx));
                self.port
                    .regs
                    .en0_clr()
                    .write(|w| w.all().variant(1 << self.pin_idx));
            }
            _ => return Err(GpioError::BadOperatingMode),
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
