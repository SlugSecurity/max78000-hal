//! Peripheral drivers for the MAX78000.

pub mod adc;
pub mod aes;
pub mod bit_banding;
pub mod bootloader;
pub mod crc;
pub mod delay;
pub mod digital;
pub mod ecc;
pub mod flash_controller;
pub mod oscillator;
pub mod power;
pub mod raw;
pub mod rng;
pub mod rtc;
pub mod serial;
pub mod synchronization;
pub mod timer;
pub mod watchdog;

// TODO: Create peripheral manager and traits.
