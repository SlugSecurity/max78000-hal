//! Tests for UART API

use core::fmt::Write;
use cortex_m_semihosting::hio;
use max78000_hal::communication::{
    CommunicationError, LineDelimitedRxChannel, LineEnding, RxChannel, TxChannel,
};
use max78000_hal::max78000::TMR2;
use max78000_hal::peripherals::timer::Time::Milliseconds;
use max78000_hal::peripherals::{
    timer::Clock,
    uart::{Uart0, UartBuilder},
    PeripheralHandle,
};

const INTERJECTION: [u8; 2824] =
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
\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1f\x20\x21\x22\x23\x24\x25\x26\x27\x28\x29\x2a\x2b\x2c\x2d\x2e\x2f\x30\x31\x32\x33\x34\x35\x36\x37\x38\x39\x3a\x3b\x3c\x3d\x3e\x3f\x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4a\x4b\x4c\x4d\x4e\x4f\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5a\x5b\x5c\x5d\x5e\x5f\x60\x61\x62\x63\x64\x65\x66\x67\x68\x69\x6a\x6b\x6c\x6d\x6e\x6f\x70\x71\x72\x73\x74\x75\x76\x77\x78\x79\x7a\x7b\x7c\x7d\x7e\x7f\x80\x81\x82\x83\x84\x85\x86\x87\x88\x89\x8a\x8b\x8c\x8d\x8e\x8f\x90\x91\x92\x93\x94\x95\x96\x97\x98\x99\x9a\x9b\x9c\x9d\x9e\x9f\xa0\xa1\xa2\xa3\xa4\xa5\xa6\xa7\xa8\xa9\xaa\xab\xac\xad\xae\xaf\xb0\xb1\xb2\xb3\xb4\xb5\xb6\xb7\xb8\xb9\xba\xbb\xbc\xbd\xbe\xbf\xc0\xc1\xc2\xc3\xc4\xc5\xc6\xc7\xc8\xc9\xca\xcb\xcc\xcd\xce\xcf\xd0\xd1\xd2\xd3\xd4\xd5\xd6\xd7\xd8\xd9\xda\xdb\xdc\xdd\xde\xdf\xe0\xe1\xe2\xe3\xe4\xe5\xe6\xe7\xe8\xe9\xea\xeb\xec\xed\xee\xef\xf0\xf1\xf2\xf3\xf4\xf5\xf6\xf7\xf8\xf9\xfa\xfb\xfc\xfd\xfe\xff";

const EGO: [u8; 2294] = *b"don't interpret it as ego. Long is computer researcher more than a professor, he's the highest profile person to teach us the way of the industry, he would be better off doing research for the betterment of the computer science field but he decide to nurture us students to have a good impact on the industry as a whole. If you are wondering why that's a good thing, I'll give you 2 sides of the picture to compare: 
1. Veenstra has 240 students, that means complete chaos with grading, less interactions with students, TAs are gonna be overwhelmed, etc, etc, pretty much what's happening in this class. Unless he can manage it well (which is a pretty tall task) 
2. Long has 51 students, that means 51 students are gonna get the content closest to the industry to get jobs from a high profile person of the campus. While being chill with the grading, more engagement with the students, etc. 

I don't usually make comments like this, but I had to get it out there cause he's being misunderstood hard by alot of people
don't interpret it as ego. Long is computer researcher more than a professor, he's the highest profile person to teach us the way of the industry, he would be better off doing research for the betterment of the computer science field but he decide to nurture us students to have a good impact on the industry as a whole. If you are wondering why that's a good thing, I'll give you 2 sides of the picture to compare: 
1. Veenstra has 240 students, that means complete chaos with grading, less interactions with students, TAs are gonna be overwhelmed, etc, etc, pretty much what's happening in this class. Unless he can manage it well (which is a pretty tall task) 
2. Long has 51 students, that means 51 students are gonna get the content closest to the industry to get jobs from a high profile person of the campus. While being chill with the grading, more engagement with the students, etc. 

I don't usually make comments like this, but I had to get it out there cause he's being misunderstood hard by alot of people
\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1f\x20\x21\x22\x23\x24\x25\x26\x27\x28\x29\x2a\x2b\x2c\x2d\x2e\x2f\x30\x31\x32\x33\x34\x35\x36\x37\x38\x39\x3a\x3b\x3c\x3d\x3e\x3f\x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4a\x4b\x4c\x4d\x4e\x4f\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5a\x5b\x5c\x5d\x5e\x5f\x60\x61\x62\x63\x64\x65\x66\x67\x68\x69\x6a\x6b\x6c\x6d\x6e\x6f\x70\x71\x72\x73\x74\x75\x76\x77\x78\x79\x7a\x7b\x7c\x7d\x7e\x7f\x80\x81\x82\x83\x84\x85\x86\x87\x88\x89\x8a\x8b\x8c\x8d\x8e\x8f\x90\x91\x92\x93\x94\x95\x96\x97\x98\x99\x9a\x9b\x9c\x9d\x9e\x9f\xa0\xa1\xa2\xa3\xa4\xa5\xa6\xa7\xa8\xa9\xaa\xab\xac\xad\xae\xaf\xb0\xb1\xb2\xb3\xb4\xb5\xb6\xb7\xb8\xb9\xba\xbb\xbc\xbd\xbe\xbf\xc0\xc1\xc2\xc3\xc4\xc5\xc6\xc7\xc8\xc9\xca\xcb\xcc\xcd\xce\xcf\xd0\xd1\xd2\xd3\xd4\xd5\xd6\xd7\xd8\xd9\xda\xdb\xdc\xdd\xde\xdf\xe0\xe1\xe2\xe3\xe4\xe5\xe6\xe7\xe8\xe9\xea\xeb\xec\xed\xee\xef\xf0\xf1\xf2\xf3\xf4\xf5\xf6\xf7\xf8\xf9\xfa\xfb\xfc\xfd\xfe\xff";

/// Run all UART tests
pub fn run_uart_test(
    stdout: &mut hio::HostStream,
    uart_builder: UartBuilder<'_, Uart0>,
    clk0: PeripheralHandle<Clock<TMR2>>,
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

    assert!(matches!(
        uart.recv_with_timeout(&mut buf, &mut timer),
        Err(CommunicationError::RecvError(_))
    ));

    // flush the fifo bleh bleh bleh (otherwise the bytes from the last test before we hit timeout are still there)
    // host sends 0xff at the start of the big buffer test
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
    let mut big_buf = [0u8; INTERJECTION.len()];
    timer = clk0.new_timer(Milliseconds(500));
    let ret = uart.recv_with_data_timeout(&mut big_buf, &mut timer);
    assert_eq!(ret, Ok(big_buf.len()));
    assert_eq!(
        big_buf,
        INTERJECTION,
        "actually received: {}",
        core::str::from_utf8(&big_buf).unwrap_or("[junk]")
    );

    // long transmit test
    big_buf[0..EGO.len()].copy_from_slice(&EGO);
    assert_eq!(uart.send(&mut big_buf[0..EGO.len()]), Ok(()));

    writeln!(stdout, "Finished UART tests...\n").unwrap();

    // recv_line test
    let mut line_buf = [0u8; 30];
    timer = clk0.new_timer(Milliseconds(500));
    assert_eq!(
        uart.recv_line_with_data_timeout(&mut line_buf, &mut timer, LineEnding::LF),
        Ok(11)
    );
    assert_eq!(&line_buf[0..11], b"short line\n");

    line_buf = [0u8; 30];
    assert_eq!(
        uart.recv_line_with_data_timeout(&mut line_buf, &mut timer, LineEnding::CR),
        Ok(19)
    );
    assert_eq!(&line_buf[0..19], b"another short line\r");

    line_buf = [0u8; 30];
    assert_eq!(
        uart.recv_line_with_data_timeout(&mut line_buf, &mut timer, LineEnding::CRLF),
        Ok(11)
    );
    assert_eq!(&line_buf[0..11], b"CRLF line\r\n");

    line_buf = [0u8; 30];
    assert_eq!(
        uart.recv_line_with_data_timeout(&mut line_buf, &mut timer, LineEnding::LF),
        Err(CommunicationError::RecvError(30))
    );
    assert_eq!(line_buf, *b"a line that fills up the buffe");

    line_buf = [0u8; 30];
    assert_eq!(
        uart.recv_line_with_data_timeout(&mut line_buf, &mut timer, LineEnding::LF),
        Ok(19)
    );
    assert_eq!(&line_buf[0..19], b"r before a newline\n");
}
