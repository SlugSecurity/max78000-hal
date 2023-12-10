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
        const A1_RX: u8 = 0b0001;
        const A1_TX: u8 = 0b0010;
        const A1_AX: u8 = 0b0011;
        const A2_RX: u8 = 0b0100;
        const A2_TX: u8 = 0b1000;
        const A2_AX: u8 = 0b1100;
        const A2_NA: u8 = 0;

        const P0_TABLE: &[u8] = &[
            A1_RX | A2_NA, // P0.0    UART0A_RX           -
            A1_TX | A2_NA, // P0.1    UART0A_TX           -
            A1_AX | A2_AX, // P0.2    TMR0A__IOA          UART0B_CTS
            A1_AX | A2_AX, // P0.3    EXT_CLK/TMR0A_IOB   UART0B_RTS
            A1_AX | A2_AX, // P0.4    SPI0_SS0            TMR0B_IOAN
            A1_AX | A2_AX, // P0.5    SPI0_MOSI           TMR0B_IOBN
            A1_AX | A2_AX, // P0.6    SPI0_MISO           OWM_IO
            A1_AX | A2_AX, // P0.7    SPI0_SCK            OWM_PE
            A1_AX | A2_AX, // P0.8    SPI0_SDIO2          TMR0B_IOA
            A1_AX | A2_AX, // P0.9    SPI0_SDIO3          TMR0B_IOB
            A1_AX | A2_AX, // P0.10   I2C0_SCL            SPI0_SS2
            A1_AX | A2_AX, // P0.11   I2C0_SDA            SPI0_SS1
            A1_RX | A2_AX, // P0.12   UART1A_RX           TMR1B_IOAN
            A1_TX | A2_AX, // P0.13   UART1A_TX           TMR1B_IOBN
            A1_AX | A2_AX, // P0.14   TMR1A_IOA           I2S_CLKEXT
            A1_AX | A2_AX, // P0.15   TMR1A_IOB           PCIF_VSYNC
            A1_AX | A2_AX, // P0.16   I2C1_SCL            PT2
            A1_AX | A2_AX, // P0.17   I2C1_SDA            PT3
            A1_AX | A2_AX, // P0.18   PT0                 OWM_IO
            A1_AX | A2_AX, // P0.19   PT1                 OWM_PE
            A1_AX | A2_AX, // P0.20   SPI1_SS0            PCIF_D0
            A1_AX | A2_AX, // P0.21   SPI1_MOSI           PCIF_D1
            A1_AX | A2_AX, // P0.22   SPI1_MISO           PCIF_D2
            A1_AX | A2_AX, // P0.23   SPI1_SCK            PCIF_D3
            A1_AX | A2_AX, // P0.24   SPI1_SDIO2          PCIF_D4
            A1_AX | A2_AX, // P0.25   SPI1_SDIO3          PCIF_D5
            A1_AX | A2_AX, // P0.26   TMR2A_IOA           PCIF_D6
            A1_AX | A2_AX, // P0.27   TMR2A_IOB           PCIF_D7
            A1_AX | A2_NA, // P0.28   SWDIO               -
            A1_AX | A2_NA, // P0.29   SWCLK               -
            A1_AX | A2_AX, // P0.30   I2C2_SCL            PCIF_D8
            A1_AX | A2_AX, // P0.31   I2C2_SDA            PCIF_D9
        ];
        const P1_TABLE: &[u8] = &[
            A1_RX | A2_AX, // P1.0   UART2A_RX    RV_TCK
            A1_TX | A2_AX, // P1.1   UART2A_TX    RV_TMS
            A1_AX | A2_AX, // P1.2   I2S_SCK      RV_TDI
            A1_AX | A2_AX, // P1.3   I2S_WS       RV_TDO
            A1_AX | A2_AX, // P1.4   I2S_SDI      TMR3B_IOA
            A1_AX | A2_AX, // P1.5   I2S_SDO      TMR3B_IOB
            A1_AX | A2_AX, // P1.6   TMR3A_IOA    PCIF_D10
            A1_AX | A2_AX, // P1.7   TMR3A_IOB    PCIF_D11
            A1_AX | A2_AX, // P1.8   PCIF_HSYNC   RXEV0
            A1_AX | A2_AX, // P1.9   PCIF_PCLK    TXEV0
        ];
        const P2_TABLE: &[u8] = &[
            A1_AX | A2_NA, // P2.0   AIN0/AINON              -
            A1_AX | A2_NA, // P2.1   AIN1/AIN0P              -
            A1_AX | A2_NA, // P2.2   AIN2/AIN1N              -
            A1_AX | A2_NA, // P2.3   AIN3/AIN1P              -
            A1_AX | A2_AX, // P2.4   AIN4/AIN2N              LPTMR0B_IOA
            A1_AX | A2_AX, // P2.5   AIN5/AIN2P              LPTMR1_IOA
            A1_AX | A2_RX, // P2.6   LPTMR0_CLK/AIN6/AIN3N   LPUARTB_RX
            A1_AX | A2_TX, // P2.7   LPTMR1_CLK/AIN7/AIN3P   LPUARTB_TX
        ];

        let table = match Port::PORT_NUM {
            0 => P0_TABLE,
            1 => P1_TABLE,
            2 => P2_TABLE,
            _ => &[]
        };

        let entry = table.get(self.pin_idx).copied().unwrap_or_default();

        let (alt1, alt2) = match self.get_io_mode() {
            PinIoMode::Input => (entry & A1_RX != 0, entry & A2_RX != 0),
            PinIoMode::Output => (entry & A1_TX != 0, entry & A2_TX != 0),
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
