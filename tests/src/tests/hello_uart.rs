use core::fmt::Write;
use cortex_m_semihosting::hio;
use max78000_hal::max78000::UART;
use max78000_hal::peripherals::uart::{TxChannel, UartBuilder};

pub fn run_uart_test(regs: &UART, stdout: &mut hio::HostStream) {
    let uart = UartBuilder::build(regs, 115200);
    let result = uart.send(b"hello world");
    writeln!(stdout, "{result:?}").unwrap();
}
