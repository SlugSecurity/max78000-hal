//! Tests for the MAX78000 HAL.

#![warn(missing_docs)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use max78000_hal::max78000::Peripherals;

extern crate panic_halt;

/// Entry point for tests.
#[entry]
fn main() -> ! {
    // TODO: Use peripheral API when available.
    Peripherals::take().unwrap();

    #[allow(clippy::empty_loop)]
    loop {}
}
