//! GPIO0 - GPIO2 pin manipulation.
//! This module contains trait implementations for the active GPIO ports.
//!
//! # Examples
//!
//! Basic usage:
//! ```
//! let pin = gpio_port.get_pin_handle(0).unwrap().into_input_pin().unwrap();
//! assert_ne!(pin.is_low(), pin.is_high());
//!
//! let mut pin = pin.into_output_pin(PinState::High).unwrap();
//! pin.set_low().unwrap();
//! assert!(pin.is_set_low().unwrap());
//! pin.set_high().unwrap();
//! assert!(pin.is_set_high().unwrap());
//! ```

use core::marker::PhantomData;

use sealed::sealed;

use port_num_types::GpioPortNum;

use super::pin_traits::{ErrorType, InputPin, IoPin, OutputPin, PinState, StatefulOutputPin};
use super::private::NonConstructible;
use super::{
    GpioError, GpioPort, GpioPortMetadata, PinHandle, PinIoMode, PinOperatingMode,
    __seal_gpio_port_metadata, __seal_pin_handle,
};

pub mod port_num_types;

// TODO FOR ASTRA:
// [x] make input pin and output pin structs
//     [x] pin types should implement InputPin for input pin and StatefulOutputPin for output pin
//     [x] these pins should be a newtype wrapping the pin handle so drop works on it properly
//     [x] all pin types, including PinHandle, should implement GeneralIoPin<INPUT_PIN_TYPE, OUTPUT_PIN_TYPE>
//     [-] note: to implement the error checking for alternate functions,
//               implement it based on the port num given back with GpioPortNum::PORT_NUM
//
// [x] after implementing above trait functionality
//     [x] implement weak/strong pullup/pulldown resistor configuration (input mode only)
//     [x] also drive strength and power supply (output mode only)
//
// [x] see low power module for example on everything above
//
// [ ] add interrupt support (input mode only)
//     [ ] just need to support adding 1 listener per pin
//     [ ] the listener can either be low/high level triggered, rising/falling edge triggered, or dual edge triggered
//     [ ] if developer provides another listener through same function, overwrite previous listener
// [-] add documentation
//     [x] a module-level doc comment
//     [x] public functions within this module that aren't trait impl functions
//     [x] other public items like structs
//     [ ] on the super module with examples of how to use the API (can tell user to see user guide and datasheet too)
//     [ ] improve existing comments in entire driver to add more detail
// [x] add unit tests for acquiring handles, releasing handles and recaquiring
// [ ] add unit tests for each public function in the active pin API

/// Marker struct implementing `GpioPortMetadata` for
/// active GPIO ports.
pub struct ActiveGpio<PortNum: GpioPortNum>(PhantomData<PortNum>);

#[sealed]
impl<PortNum: GpioPortNum + 'static> GpioPortMetadata<'static> for ActiveGpio<PortNum> {
    type PinHandleType<'a, const PIN_CT: usize> = ActivePinHandle<'a, PortNum, PIN_CT>;
    type GpioRegs = PortNum::Peripheral;
}

/// `PinHandle` implementation for active GPIO ports.
pub struct ActivePinHandle<'a, PortNum: GpioPortNum + 'static, const PIN_CT: usize> {
    port: &'a GpioPort<'static, ActiveGpio<PortNum>, PIN_CT>,
    pin_idx: usize,
}

impl<PortNum: GpioPortNum + 'static, const PIN_CT: usize> Drop
    for ActivePinHandle<'_, PortNum, PIN_CT>
{
    fn drop(&mut self) {
        // When handle is dropped, allow the pin to be taken again.
        self.port.pin_taken[self.pin_idx].set(false);
    }
}

#[sealed]
impl<'a, PortNum: GpioPortNum + 'static, const PIN_CT: usize> PinHandle<'a>
    for ActivePinHandle<'a, PortNum, PIN_CT>
{
    type Port = GpioPort<'static, ActiveGpio<PortNum>, PIN_CT>;

    fn new(_private: NonConstructible, port: &'a Self::Port, pin_idx: usize) -> Self {
        // TODO: We can't get rid of the const generic here or otherwise prevent a bad pin count
        // from being entered until more complex exprs can be evaluated in const generics stably.
        // So there are asserts here to ensure they can't be constructed. The construction of these
        // handles are done privately and not able to be done externally so this is fine.
        assert!(PIN_CT <= 32); // Any active port can have up to 32 pins based on the registers
        assert!(pin_idx < PIN_CT);

        Self { port, pin_idx }
    }

    fn get_pin_idx(&self) -> usize {
        self.pin_idx
    }
}

/// `InputPin` implementation for active GPIO ports.
pub struct ActiveInputPin<'a, PortNum: GpioPortNum + 'static, const PIN_CT: usize>(
    ActivePinHandle<'a, PortNum, PIN_CT>,
);

impl<PortNum: GpioPortNum + 'static, const PIN_CT: usize> ErrorType
    for ActiveInputPin<'_, PortNum, PIN_CT>
{
    type Error = GpioError;
}

impl<PortNum: GpioPortNum + 'static, const PIN_CT: usize> InputPin
    for ActiveInputPin<'_, PortNum, PIN_CT>
{
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.0.port.regs.in_().read().bits() & (1 << self.0.pin_idx) != 0)
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        self.is_high().map(|x| !x)
    }
}

/// `OutputPin` implementation for active GPIO ports.
pub struct ActiveOutputPin<'a, PortNum: GpioPortNum + 'static, const PIN_CT: usize>(
    ActivePinHandle<'a, PortNum, PIN_CT>,
);

impl<PortNum: GpioPortNum + 'static, const PIN_CT: usize> ErrorType
    for ActiveOutputPin<'_, PortNum, PIN_CT>
{
    type Error = GpioError;
}

impl<PortNum: GpioPortNum + 'static, const PIN_CT: usize> OutputPin
    for ActiveOutputPin<'_, PortNum, PIN_CT>
{
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.0
            .port
            .regs
            .out_set()
            .write(|w| w.gpio_out_set().variant(1 << self.0.pin_idx));
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.0
            .port
            .regs
            .out_clr()
            .write(|w| w.gpio_out_clr().variant(1 << self.0.pin_idx));
        Ok(())
    }
}

impl<PortNum: GpioPortNum + 'static, const PIN_CT: usize> StatefulOutputPin
    for ActiveOutputPin<'_, PortNum, PIN_CT>
{
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.0.port.regs.out().read().bits() & (1 << self.0.pin_idx) != 0)
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        self.is_set_high().map(|x| !x)
    }
}

impl<PortNum: GpioPortNum + 'static, const PIN_CT: usize> ErrorType
    for ActivePinHandle<'_, PortNum, PIN_CT>
{
    type Error = GpioError;
}

impl<PortNum: GpioPortNum + 'static, const PIN_CT: usize> ActivePinHandle<'_, PortNum, PIN_CT> {
    fn transition_operating_mode(&mut self) {
        self.port
            .regs
            .en0_set()
            .write(|w| w.all().variant(1 << self.pin_idx));
    }

    fn write_operating_mode(
        &mut self,
        mode: PinOperatingMode,
        io_mode: PinIoMode,
    ) -> Result<(), GpioError> {
        const A1_RX: u8 = 0b0001; // means AF1 is valid when this pin is an input pin
        const A1_TX: u8 = 0b0010; // means AF1 is valid when this pin is an output pin
        const A1_AX: u8 = 0b0011; // means AF1 is always valid for this pin

        const A2_RX: u8 = 0b0100; // means AF2 is valid when this pin is an input pin
        const A2_TX: u8 = 0b1000; // means AF2 is valid when this pin is an output pin
        const A2_AX: u8 = 0b1100; // means AF2 is always valid for this pin
        const A2_NA: u8 = 0b0000; // means AF2 is never valid for this pin

        // Tables based off of https://www.analog.com/media/en/technical-documentation/data-sheets/MAX78000.pdf
        // Page 29-31, table section `GPIO and Alternate Function`.
        // Note that the AF validation checks only covers UART when checking if the RX/TX state is valid.
        // TODO: check the RX/TX state for more than just UART
        // TODO: Statically constrain the pin operating mode according to PIN_CT and the pin index
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

        let table = match PortNum::PORT_NUM {
            0 => P0_TABLE,
            1 => P1_TABLE,
            2 => P2_TABLE,
            _ => &[],
        };

        let pin_entry = table.get(self.pin_idx).copied().unwrap_or_default();

        let (af1_is_valid, af2_is_valid) = match io_mode {
            PinIoMode::Input => (pin_entry & A1_RX != 0, pin_entry & A2_RX != 0),
            PinIoMode::Output => (pin_entry & A1_TX != 0, pin_entry & A2_TX != 0),
        };

        // https://www.analog.com/media/en/technical-documentation/user-guides/max78000-user-guide.pdf
        // Page 111, section 6.2.3, table 6-2.
        match mode {
            PinOperatingMode::DigitalIo => {}
            PinOperatingMode::AltFunction1 if af1_is_valid => {
                self.port
                    .regs
                    .en1_clr()
                    .write(|w| w.all().variant(1 << self.pin_idx));
                self.port
                    .regs
                    .en0_clr()
                    .write(|w| w.all().variant(1 << self.pin_idx));
            }
            PinOperatingMode::AltFunction2 if af2_is_valid => {
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
}

impl<'a, PortNum: GpioPortNum + 'static, const PIN_CT: usize>
    IoPin<ActiveInputPin<'a, PortNum, PIN_CT>, ActiveOutputPin<'a, PortNum, PIN_CT>>
    for ActivePinHandle<'a, PortNum, PIN_CT>
{
    type InputConfig = ActiveInputPinConfig;

    fn into_input_pin(
        self,
        config: ActiveInputPinConfig,
    ) -> Result<ActiveInputPin<'a, PortNum, PIN_CT>, Self::Error> {
        let mut pin = ActiveInputPin(self);

        pin.0.transition_operating_mode();
        pin.set_power_supply(config.power_supply);
        pin.set_pull_mode(config.pull_mode);

        pin.0
            .port
            .regs
            .outen_clr()
            .write(|w| w.all().variant(1 << pin.0.pin_idx));
        pin.0
            .port
            .regs
            .inen()
            .modify(|r, w| w.gpio_inen().variant(r.bits() | (1 << pin.0.pin_idx)));

        pin.0
            .write_operating_mode(config.operating_mode, PinIoMode::Input)?;

        Ok(pin)
    }

    type OutputConfig = ActiveOutputPinConfig;

    fn into_output_pin(
        self,
        state: PinState,
        config: ActiveOutputPinConfig,
    ) -> Result<ActiveOutputPin<'a, PortNum, PIN_CT>, Self::Error> {
        let mut pin = ActiveOutputPin(self);

        pin.0.transition_operating_mode();
        pin.set_power_supply(config.power_supply);
        pin.set_drive_strength(config.drive_strength);

        match state {
            PinState::Low => pin.set_low()?,
            PinState::High => pin.set_high()?,
        }

        pin.0
            .port
            .regs
            .inen()
            .modify(|r, w| w.gpio_inen().variant(r.bits() & !(1 << pin.0.pin_idx)));
        pin.0
            .port
            .regs
            .outen_set()
            .write(|w| w.all().variant(1 << pin.0.pin_idx));

        pin.0
            .write_operating_mode(config.operating_mode, PinIoMode::Output)?;

        Ok(pin)
    }

    fn set_operating_mode(&mut self, mode: PinOperatingMode) -> Result<(), GpioError> {
        self.transition_operating_mode();
        self.write_operating_mode(mode, self.get_io_mode())
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

impl<'a, PortNum: GpioPortNum + 'static, const PIN_CT: usize>
    IoPin<ActiveInputPin<'a, PortNum, PIN_CT>, ActiveOutputPin<'a, PortNum, PIN_CT>>
    for ActiveInputPin<'a, PortNum, PIN_CT>
{
    type InputConfig = ActiveInputPinConfig;

    fn into_input_pin(
        self,
        config: ActiveInputPinConfig,
    ) -> Result<ActiveInputPin<'a, PortNum, PIN_CT>, Self::Error> {
        self.0.into_input_pin(config)
    }

    type OutputConfig = ActiveOutputPinConfig;

    fn into_output_pin(
        self,
        state: PinState,
        config: ActiveOutputPinConfig,
    ) -> Result<ActiveOutputPin<'a, PortNum, PIN_CT>, Self::Error> {
        self.0.into_output_pin(state, config)
    }

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

impl<'a, PortNum: GpioPortNum + 'static, const PIN_CT: usize>
    IoPin<ActiveInputPin<'a, PortNum, PIN_CT>, ActiveOutputPin<'a, PortNum, PIN_CT>>
    for ActiveOutputPin<'a, PortNum, PIN_CT>
{
    type InputConfig = ActiveInputPinConfig;

    fn into_input_pin(
        self,
        config: ActiveInputPinConfig,
    ) -> Result<ActiveInputPin<'a, PortNum, PIN_CT>, Self::Error> {
        self.0.into_input_pin(config)
    }

    type OutputConfig = ActiveOutputPinConfig;

    fn into_output_pin(
        self,
        state: PinState,
        config: ActiveOutputPinConfig,
    ) -> Result<ActiveOutputPin<'a, PortNum, PIN_CT>, Self::Error> {
        self.0.into_output_pin(state, config)
    }

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

/// The configuration needed when converting an active GPIO pin into input mode.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct ActiveInputPinConfig {
    /// The operating mode of the pin to use when it's converted to an input pin.
    pub operating_mode: PinOperatingMode,
    /// The power supply of the pin to use when it's converted to an input pin.
    pub power_supply: PowerSupply,
    /// The pull mode of the pin to use when it's converted to an input pin.
    pub pull_mode: PullMode,
}

/// The configuration needed when converting an active GPIO pin into output mode.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct ActiveOutputPinConfig {
    /// The operating mode of the pin to use when it's converted to an output pin.
    pub operating_mode: PinOperatingMode,
    /// The power supply of the pin to use when it's converted to an output pin.
    pub power_supply: PowerSupply,
    /// The drive strength of the pin to use when it's converted to an output pin.
    pub drive_strength: DriveStrength,
}

/// Represents the associated power supply of a pin.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum PowerSupply {
    /// VDDIO (1.8V).
    #[default]
    Vddio,
    /// VDDIOH (3.0V).
    Vddioh,
}

impl<PortNum: GpioPortNum + 'static, const PIN_CT: usize> ActivePinHandle<'_, PortNum, PIN_CT> {
    /// Sets the pin's associated power supply.
    pub fn set_power_supply(&self, ps: PowerSupply) {
        self.port.regs.vssel().modify(|r, w| match ps {
            PowerSupply::Vddio => w.all().variant(r.bits() & !(1 << self.pin_idx)),
            PowerSupply::Vddioh => w.all().variant(r.bits() | (1 << self.pin_idx)),
        });
    }

    /// Gets the pin's associated power supply.
    pub fn get_power_supply(&self) -> PowerSupply {
        if self.port.regs.vssel().read().bits() & (1 << self.pin_idx) == 0 {
            PowerSupply::Vddio
        } else {
            PowerSupply::Vddioh
        }
    }
}

/// Represents the pull mode of an input pin.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum PullMode {
    /// High impedance mode (the default after power-on-reset).
    #[default]
    HighImpedance,
    /// Weak pullup mode (1 megaohm).
    WeakPullup,
    /// Strong pullup mode (25 kiloohms).
    StrongPullup,
    /// Weak pulldown mode (1 megaohm).
    WeakPulldown,
    /// Strong pulldown mode (25 kiloohms).
    StrongPulldown,
}

impl<PortNum: GpioPortNum + 'static, const PIN_CT: usize> ActiveInputPin<'_, PortNum, PIN_CT> {
    /// Sets the pin's pull mode.
    pub fn set_pull_mode(&self, mode: PullMode) {
        let (padctrl0, padctrl1, ps) = match mode {
            PullMode::HighImpedance => (false, false, None),
            PullMode::WeakPullup => (true, false, Some(false)),
            PullMode::StrongPullup => (true, false, Some(true)),
            PullMode::WeakPulldown => (false, true, Some(false)),
            PullMode::StrongPulldown => (false, true, Some(true)),
        };

        let bit = |r, bit| (r & !(1 << self.0.pin_idx)) | ((bit as u32) << self.0.pin_idx);

        // https://www.analog.com/media/en/technical-documentation/user-guides/max78000-user-guide.pdf
        // Page 111, section 6.2.4, table 6-3.
        self.0
            .port
            .regs
            .padctrl0()
            .modify(|r, w| w.gpio_padctrl0().variant(bit(r.bits(), padctrl0)));
        self.0
            .port
            .regs
            .padctrl1()
            .modify(|r, w| w.gpio_padctrl1().variant(bit(r.bits(), padctrl1)));
        if let Some(ps) = ps {
            self.0
                .port
                .regs
                .ps()
                .modify(|r, w| w.all().variant(bit(r.bits(), ps)));
        }
    }

    /// Gets the pin's pull mode.
    pub fn get_pull_mode(&self) -> PullMode {
        let padctrl0 = self.0.port.regs.padctrl0().read().gpio_padctrl0().bits();
        let padctrl1 = self.0.port.regs.padctrl1().read().gpio_padctrl1().bits();
        let ps = self.0.port.regs.ps().read().all().bits();

        match [padctrl0, padctrl1, ps].map(|x| x & (1 << self.0.pin_idx) != 0) {
            [true, false, false] => PullMode::WeakPullup,
            [true, false, true] => PullMode::StrongPullup,
            [false, true, false] => PullMode::WeakPulldown,
            [false, true, true] => PullMode::StrongPulldown,
            _ => PullMode::HighImpedance,
        }
    }

    /// Sets the pin's associated power supply.
    pub fn set_power_supply(&self, ps: PowerSupply) {
        self.0.set_power_supply(ps);
    }

    /// Gets the pin's associated power supply.
    pub fn get_power_supply(&self) -> PowerSupply {
        self.0.get_power_supply()
    }
}

/// Represents the drive strength of an output pin.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum DriveStrength {
    /// Drive strength 0
    #[default]
    S0,
    /// Drive strength 1.
    S1,
    /// Drive strength 2.
    S2,
    /// Drive strength 3.
    S3,
}

impl<PortNum: GpioPortNum + 'static, const PIN_CT: usize> ActiveOutputPin<'_, PortNum, PIN_CT> {
    /// Sets the pin's drive strength.
    pub fn set_drive_strength(&self, ds: DriveStrength) {
        let (ds0, ds1) = match ds {
            DriveStrength::S0 => (false, false),
            DriveStrength::S1 => (true, false),
            DriveStrength::S2 => (false, true),
            DriveStrength::S3 => (true, true),
        };

        let bit = |r, bit| (r & !(1 << self.0.pin_idx)) | ((bit as u32) << self.0.pin_idx);

        // https://www.analog.com/media/en/technical-documentation/user-guides/max78000-user-guide.pdf
        // Page 112, section 6.2.5, table 6-4.
        self.0
            .port
            .regs
            .ds0()
            .modify(|r, w| w.gpio_ds0().variant(bit(r.bits(), ds0)));
        self.0
            .port
            .regs
            .ds1()
            .modify(|r, w| w.gpio_ds1().variant(bit(r.bits(), ds1)));
    }

    /// Gets the pin's drive strength.
    pub fn get_drive_strength(&self) -> DriveStrength {
        let ds0 = self.0.port.regs.ds0().read().gpio_ds0().bits();
        let ds1 = self.0.port.regs.ds1().read().gpio_ds1().bits();

        match [ds0, ds1].map(|x| x & (1 << self.0.pin_idx) != 0) {
            [false, false] => DriveStrength::S0,
            [true, false] => DriveStrength::S1,
            [false, true] => DriveStrength::S2,
            [true, true] => DriveStrength::S3,
        }
    }

    /// Sets the pin's associated power supply.
    pub fn set_power_supply(&self, ps: PowerSupply) {
        self.0.set_power_supply(ps);
    }

    /// Gets the pin's associated power supply.
    pub fn get_power_supply(&self) -> PowerSupply {
        self.0.get_power_supply()
    }
}
