//! Peripheral drivers for the MAX78000.

// Embedded HAL peripherals.
pub mod adc;
pub mod delay;
pub mod digital;
pub mod rng;
pub mod serial;
pub mod timer;
pub mod watchdog;

// Non embedded HAL peripherals.
pub mod aes;
pub mod bit_banding;
pub mod bootloader;
pub mod crc;
pub mod ecc;
pub mod flash_controller;
pub mod oscillator;
pub mod power;
pub mod raw;
pub mod rtc;
pub mod synchronization;

// TODO: Create peripheral manager and traits.
