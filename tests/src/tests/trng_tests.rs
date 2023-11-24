//! Tests for the TRNG peripheral.

use core::fmt::Write;

use cortex_m_semihosting::hio;
use max78000_hal::{
    max78000::{GCR, TRNG},
    peripherals::trng::Trng,
};

/// Runs all TRNG tests.
pub fn run_trng_tests(trng_regs: TRNG, gcr_regs: &GCR, stdout: &mut hio::HostStream) {
    writeln!(stdout, "Starting TRNG peripheral tests...").unwrap();

    // Enable TRNG clock. This will be done by the peripheral API when available.
    // TODO: Remove this when the peripheral API is available.
    gcr_regs.pclkdis1.modify(|_, w| w.trng().en());

    // Run tests.
    let trng = Trng::new(trng_regs);
    test_random_u32(&trng);
    test_fill_buffer(&trng, stdout);
    writeln!(stdout, "TRNG peripheral tests complete!\n").unwrap();
}

/// Tests the [`trng::Trng::random_u32()`] function.
fn test_random_u32(trng: &Trng) {
    for _ in 0..100 {
        let random = trng.random_u32();
        assert_ne!(random, 0);
    }
}

/// Tests the [`trng::Trng::fill_buffer()`] function.
fn test_fill_buffer(trng: &Trng, stdout: &mut hio::HostStream) {
    let mut buf = [0u8; 15];
    trng.fill_buffer(&mut buf);
    assert_ne!(buf, [0u8; 15]);
    writeln!(stdout, "TRNG buffer: {:?}", buf).unwrap();

    for _ in 0..100 {
        let mut buf = [0u8; 103]; // Not a multiple of 32 bits.
        trng.fill_buffer(&mut buf);
        assert_ne!(buf, [0u8; 103]);

        let mut buf = [0u8; 124]; // Multiple of 32 bits.
        trng.fill_buffer(&mut buf);
        assert_ne!(buf, [0u8; 124]);
    }
}
