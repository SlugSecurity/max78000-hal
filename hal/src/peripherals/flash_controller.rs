//! Flash controller peripheral API.

use max78000::{FLC, ICC0};

/// TRNG peripheral.
pub struct FlashController<'a> {
    flc: FLC,
    icc: &'a ICC0,
}

// TODO: Implement with the peripheral API when available.

impl<'a> FlashController<'a> {
    /// Creates a new flash controller peripheral.
    // TODO: Make this function pub(crate) when the peripheral API is available. Tests needs it public until then.
    pub fn new(flc: FLC, icc: &'a ICC0) -> Self {
        Self { flc, icc }
    }

    /// Checks if the flash controller's clock divisor is correct and if not, sets it. Correct
    /// clock frequency is 1 MHz.
    ///
    /// This MUST be called before any non-read flash controller operations.
    fn set_clock_divisor(&self) {
        // TODO: Finish.
    }

    /// Flushes the data and instruction cache.
    ///
    /// This MUST be called after any write/erase flash controller operations.
    fn flush_icc(&self) {
        // TODO: Finish.
    }

    /// Reads data from flash.
    pub fn read(&self, address: u32, data: &mut [u8]) {
        // TODO: Finish.
    }

    /// Writes data to flash.
    pub fn write(&self, address: u32, data: &[u8]) {
        // TODO: Finish.
    }

    // TODO: Finish adding functions for the flash controller. No need to implement async/interrupts.
}
