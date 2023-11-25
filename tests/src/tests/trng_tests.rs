//! Tests for the TRNG peripheral.

use core::fmt::Write;

use bitvec::prelude::*;
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
    gcr_regs.pclkdis1.write(|w| w.trng().en());

    // Run tests.
    let trng = Trng::new(trng_regs);
    test_random_u32(&trng);
    test_fill_buffer(&trng, stdout);
    writeln!(stdout, "TRNG peripheral tests complete!\n").unwrap();
}

/// Calculates the minimum entropy of a buffer in bits per byte.
fn min_entropy(buf: &[u8]) -> f64 {
    let mut ones = 0;

    for bit in buf.as_bits::<Lsb0>() {
        if *bit {
            ones += 1;
        }
    }

    let one_probability = ones as f64 / (buf.len() * 8) as f64;
    let zero_probability = 1.0 - one_probability;
    assert!(!one_probability.is_nan());
    assert!(!zero_probability.is_nan());

    -libm::log2(one_probability.max(zero_probability)) * 8.0
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
    let mut entropy = min_entropy(&buf);
    writeln!(stdout, "TRNG buffer: {:?}", buf).unwrap();
    writeln!(stdout, "TRNG buffer entropy: {} bits per byte", entropy).unwrap();
    assert_ne!(buf, [0u8; 15]);
    assert!(entropy >= 7.0);

    for _ in 0..100 {
        let mut buf = [0u8; 103]; // Not a multiple of 32 bits.
        trng.fill_buffer(&mut buf);
        entropy = min_entropy(&buf);
        assert_ne!(buf, [0u8; 103]);
        assert!(entropy >= 7.0);

        let mut buf = [0u8; 124]; // Multiple of 32 bits.
        trng.fill_buffer(&mut buf);
        entropy = min_entropy(&buf);
        assert_ne!(buf, [0u8; 124]);
        assert!(entropy >= 7.0);
    }
}
