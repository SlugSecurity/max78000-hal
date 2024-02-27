use core::fmt::Write;
use cortex_m_semihosting::hio;
use max78000_hal::peripherals::uart::{TxChannel, Uart0, UartBuilder};

pub fn run_uart_test(stdout: &mut hio::HostStream, builder: UartBuilder<'_, Uart0>) {
    let uart = builder.build(115200);
    let result = uart.send(b"hello world");
    writeln!(stdout, "{result:?}").unwrap();
}
