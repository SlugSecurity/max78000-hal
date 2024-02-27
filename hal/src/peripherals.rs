//! Peripheral drivers for the MAX78000.

// Embedded HAL peripherals.
pub mod adc;
pub mod delay;
pub mod gpio;
pub mod serial;
pub mod timer;
pub mod trng;
pub mod watchdog;

// Non embedded HAL peripherals.
pub mod aes;
pub mod bit_banding;
pub mod bootloader;
pub mod crc;
pub mod ecc;
pub mod flash_controller;
pub mod i2c;
pub mod oscillator;
pub mod power;
pub mod raw;
pub mod rtc;
pub mod synchronization;
pub mod i2c_bitbang;

// TODO: Create peripheral manager and traits.
