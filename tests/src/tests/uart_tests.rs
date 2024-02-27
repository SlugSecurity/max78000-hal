//! Tests for UART API

use core::fmt::Write;
use cortex_m_semihosting::hio;
use max78000_hal::communication::{CommunicationError, RxChannel, TxChannel};
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

    // send, host should receive the same data
    let mut buf = *b"bleh bleh bleh";
    assert_eq!(uart.send(&mut buf), Ok(()));

    // receive, should get the same as what host sent
    let mut timer = clk0.new_timer(Milliseconds(5000));
    buf = [0; 14];
    assert_eq!(uart.recv_with_timeout(&mut buf, &mut timer), Ok(14));
    assert_eq!(
        buf,
        *b"meow meow meow",
        "actually received: {}",
        core::str::from_utf8(&buf).unwrap_or("[junk]")
    );

    // for these timeout tests the host sleeps 100ms before each byte
    // recv_with_timeout should fail, since it takes 1400ms in total,
    // but recv_with_data_timeout should pass
    timer = clk0.new_timer(Milliseconds(1000));
    buf = [0; 14];
    assert_eq!(
        uart.recv_with_timeout(&mut buf, &mut timer),
        Err(CommunicationError::RecvError)
    );

    // flush the fifo bleh bleh bleh
    let mut byte = [0u8];
    timer = clk0.new_timer(Milliseconds(500));
    while uart.recv_with_timeout(&mut byte, &mut timer) != Ok(1) || byte != [0xff] {}

    // ping the host to say we are ready for the next test
    // uart.send(&mut buf[0..1]).unwrap();

    timer = clk0.new_timer(Milliseconds(500));
    buf = [0; 14];
    assert_eq!(uart.recv_with_data_timeout(&mut buf, &mut timer), Ok(14));
    assert_eq!(
        buf,
        *b"womp womp womp",
        "actually received: {}",
        core::str::from_utf8(&buf).unwrap_or("[junk]")
    );
}
