//! True random number generator (TRNG) peripheral API.

use core::mem;

use max78000::TRNG;

/// TRNG peripheral.
pub struct Trng {
    trng: TRNG,
}

// TODO: Implement with the peripheral API when available.

impl Trng {
    /// Creates a new TRNG peripheral.
    // TODO: Make this function pub(crate) when the peripheral API is available. Tests needs it public until then.
    pub(crate) fn new(trng: TRNG) -> Self {
        Self { trng }
    }

    /// Returns a random number.
    pub fn random_u32(&self) -> u32 {
        while self.trng.status().read().rdy().is_busy() {}
        self.trng.data().read().bits()
    }

    /// Fills a buffer with random bytes.
    pub fn fill_buffer(&self, buf: &mut [u8]) {
        buf.chunks_mut(mem::size_of::<u32>()).for_each(|chunk| {
            let random = self.random_u32();
            chunk.copy_from_slice(&random.to_ne_bytes()[..chunk.len()]);
        });
    }
}
