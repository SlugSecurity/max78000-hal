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

const INTERJECTION: [u8; 2568] =
    *b"I'd just like to interject for a moment.  What you're referring to as Linux,
is in fact, GNU/Linux, or as I've recently taken to calling it, GNU plus Linux.
Linux is not an operating system unto itself, but rather another free component
of a fully functioning GNU system made useful by the GNU corelibs, shell
utilities and vital system components comprising a full OS as defined by POSIX.

Many computer users run a modified version of the GNU system every day,
without realizing it.  Through a peculiar turn of events, the version of GNU
which is widely used today is often called \"Linux\", and many of its users are
not aware that it is basically the GNU system, developed by the GNU Project.

There really is a Linux, and these people are using it, but it is just a
part of the system they use.  Linux is the kernel: the program in the system
that allocates the machine's resources to the other programs that you run.
The kernel is an essential part of an operating system, but useless by itself;
it can only function in the context of a complete operating system.  Linux is
normally used in combination with the GNU operating system: the whole system
is basically GNU with Linux added, or GNU/Linux.  All the so-called \"Linux\"
distributions are really distributions of GNU/Linux.
I'd just like to interject for a moment.  What you're referring to as Linux,
is in fact, GNU/Linux, or as I've recently taken to calling it, GNU plus Linux.
Linux is not an operating system unto itself, but rather another free component
of a fully functioning GNU system made useful by the GNU corelibs, shell
utilities and vital system components comprising a full OS as defined by POSIX.

Many computer users run a modified version of the GNU system every day,
without realizing it.  Through a peculiar turn of events, the version of GNU
which is widely used today is often called \"Linux\", and many of its users are
not aware that it is basically the GNU system, developed by the GNU Project.

There really is a Linux, and these people are using it, but it is just a
part of the system they use.  Linux is the kernel: the program in the system
that allocates the machine's resources to the other programs that you run.
The kernel is an essential part of an operating system, but useless by itself;
it can only function in the context of a complete operating system.  Linux is
normally used in combination with the GNU operating system: the whole system
is basically GNU with Linux added, or GNU/Linux.  All the so-called \"Linux\"
distributions are really distributions of GNU/Linux.
";

/// Run all UART tests
pub fn run_uart_test(
    stdout: &mut hio::HostStream,
    uart_builder: UartBuilder<'_, Uart0>,
    clk0: PeripheralHandle<Clock<TMR>>,
) {
    writeln!(stdout, "Starting UART tests...\n").unwrap();

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

    timer = clk0.new_timer(Milliseconds(500));
    buf = [0; 14];
    assert_eq!(uart.recv_with_data_timeout(&mut buf, &mut timer), Ok(14));
    assert_eq!(
        buf,
        *b"womp womp womp",
        "actually received: {}",
        core::str::from_utf8(&buf).unwrap_or("[junk]")
    );

    // long receive test
    let mut big_buf = [0u8; 2568];
    timer = clk0.new_timer(Milliseconds(5000));
    let ret = uart.recv_with_data_timeout(&mut big_buf, &mut timer);
    for (i, &byte) in big_buf.iter().enumerate() {
        if byte != INTERJECTION[i] {
            writeln!(stdout, "first mismatch at {i}").unwrap();
            break;
        }
    }
    assert_eq!(ret, Ok(big_buf.len()));
    assert_eq!(
        big_buf,
        INTERJECTION,
        "actually received: {}",
        core::str::from_utf8(&big_buf).unwrap_or("[junk]")
    );

    writeln!(stdout, "Finished UART tests...\n").unwrap();
}
