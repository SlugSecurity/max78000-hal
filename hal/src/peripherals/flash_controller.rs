//! Flash controller peripheral API.

use max78000::{FLC, ICC0};

/// Flash Controller peripheral.
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

    /// Unlocks memory protection to allow flash operations
    fn unlock_write_protection(&self) {
        self.flc.ctrl().modify(|_, w| w.unlock().unlocked());
    }

    fn lock_write_protection(&self) {
        self.flc.ctrl().modify(|_, w| w.unlock().locked());
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
}
pub(crate) mod private {
    impl<'a> super::FlashController<'a> {
        /// Reads data from flash.
        pub fn read(&self, address: u32, data: &mut [u8]) {
            // TODO: Finish.
        }

        /// Writes data to flash.
        pub fn write(&self, address: u32, data: &[u8]) {
            // If desired, enable the flash controller interrupts by setting the
            // FLC_INTR.afie and FLC_INTR.doneie bits.
            self.flc.ctrl().modify(|r, w| {
                while r.pend().bit_is_clear() == false {}
                w
            });

            self.set_clock_divisor();
            self.flc.addr().modify(|_, w| w.bits(address));

            // Each word of the data to write follows the little-endian format where the
            // least significant byte of the word is stored at the lowest-numbered byte, and
            // the most significant byte is stored at the highest- numbered byte.

            // Least significant
            self.flc.data(0).modify(|r, w| w);
            self.flc.data(1).modify(|r, w| w);
            self.flc.data(2).modify(|r, w| w);
            // Most significant
            self.flc.data(3).modify(|r, w| w);

            self.unlock_write_protection();
            // Turn on write bit
            // The hardware automatically clears this field when the write
            // operation is complete.
            self.flc.ctrl().modify(|r, w| w.wr().set_bit());
            // An interrupt is generated if the FLC_INTR.doneie field is set to 1.
            self.flc.intr().modify(|r, w| {
                while r.done().bit_is_set() == false {}
                w
            });

            // If an error occurred, the FLC_INTR.af field is set to 1 by
            // hardware. An interrupt is generated if the FLC_INTR.afie field is
            // set to 1.

            self.lock_write_protection();
        }

        // TODO: Finish adding functions for the flash controller. No need to
        // implement async/interrupts.
    }
}
