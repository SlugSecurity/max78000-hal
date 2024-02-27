//! Tests for UART API

use core::fmt::Write;
use cortex_m_semihosting::hio;
use max78000_hal::communication::{RxChannel, TxChannel};
use max78000_hal::max78000::TMR;
use max78000_hal::peripherals::timer::Time::Milliseconds;
use max78000_hal::peripherals::{
    timer::Clock,
    uart::{Uart0, UartBuilder},
    PeripheralHandle,
};

/// Run all UART tests
pub fn run_uart_test(
    stdout: &mut hio::HostStream,
    uart_builder: UartBuilder<'_, Uart0>,
    clk0: PeripheralHandle<Clock<TMR>>,
) {
    let mut uart = uart_builder.build(115200);
    let mut buf = *b"bleh bleh bleh";

    uart.send(&mut buf).unwrap();
    writeln!(stdout, "sent").unwrap();

    let mut timer = clk0.new_timer(Milliseconds(5000));
    let mut buf2 = [0u8; 14];
    writeln!(
        stdout,
        "{}",
        uart.recv_with_timeout(&mut buf2, &mut timer).unwrap()
    )
    .unwrap();
    assert_eq!(buf2, *b"meow meow meow");
}
