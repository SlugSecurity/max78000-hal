use max78000_hal::max78000::UART;
use max78000_hal::peripherals::uart::{TxChannel, UartBuilder};

pub fn run_uart_test(regs: &UART) {
    let uart = UartBuilder::build(regs, 115200);
    uart.send(b"hello world").unwrap();
}
