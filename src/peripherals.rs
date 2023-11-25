//! Peripheral drivers for the MAX78000.

use core::ops::Deref;

use critical_section::Mutex;
use heapless::{pool::singleton::arc::Pool, Arc};

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

mod internals;

// TODO: Create peripheral manager and traits.
// TODO: Create a new pool for each peripheral? Use some attribute macro to do this?

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
/// Peripheral errors.
pub enum PeripheralError {
    /// Peripheral is not enabled and operation requires it to be enabled.
    NotEnabled,
    /// Peripheral is not disabled and operation requires it to be disabled.
    NotDisabled,
    /// Peripheral failed to initialize.
    InitFailure,
    /// Peripheral is in an illegal state.
    IllegalState,
}

// initialization of peripheral? do we need to enable power first and then initialize? that means we need to add more functions to the private peripheral right?
