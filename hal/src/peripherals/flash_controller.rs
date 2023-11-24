//! Flash controller peripheral API.

use max78000::FLC;

/// TRNG peripheral.
pub struct FlashController {
    flc: FLC,
}

// TODO: Implement with the peripheral API when available.

impl FlashController {
    /// Creates a new flash controller peripheral.
    // TODO: Make this function pub(crate) when the peripheral API is available. Tests needs it public until then.
    pub fn new(flc: FLC) -> Self {
        Self { flc }
    }

    // TODO: Add functions for the flash controller.
}
