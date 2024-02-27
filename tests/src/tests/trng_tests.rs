//! Tests for the TRNG peripheral.

use core::fmt::Write;

use bitvec::prelude::*;
use cortex_m_semihosting::hio;
use max78000_hal::peripherals::{trng::Trng, PeripheralHandle};

/// Runs all TRNG tests.
pub fn run_trng_tests(trng: PeripheralHandle<'_, Trng>, stdout: &mut hio::HostStream) {
    writeln!(stdout, "Starting TRNG peripheral tests...").unwrap();

    // Run tests.
    test_random_u32(&trng);
    test_fill_buffer(&trng, stdout);
    writeln!(stdout, "TRNG peripheral tests complete!\n").unwrap();
}

/// Tests the [`Trng::random_u32()`] function.
fn test_random_u32(trng: &Trng) {
    for _ in 0..100 {
        let random = trng.random_u32();
        assert_ne!(random, 0);
    }
}

/// Calculates the min-entropy of a buffer in bits per byte.
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

/// Prints a warning if the min-entropy of a buffer is less than a threshold.
fn find_and_warn_entropy(
    stdout: &mut hio::HostStream,
    buf: &[u8],
    entropy_req: f64,
    print_entropy: bool,
) {
    let entropy_actual = min_entropy(buf);

    if print_entropy {
        writeln!(
            stdout,
            "{}B TRNG buffer entropy: {} bits per byte",
            buf.len(),
            entropy_actual
        )
        .unwrap();
    }

    if entropy_actual < entropy_req {
        writeln!(
            stdout,
            "Error: entropy bits per byte <{} on {}B buffer: {} bits per byte",
            entropy_req,
            buf.len(),
            entropy_actual
        )
        .unwrap();
    }
}

/// Tests the [`Trng::fill_buffer()`] function.
fn test_fill_buffer(trng: &Trng, stdout: &mut hio::HostStream) {
    // Small buffer test.
    let mut buf = [0u8; 15];
    trng.fill_buffer(&mut buf);
    writeln!(stdout, "TRNG buffer: {:?}", buf).unwrap();
    find_and_warn_entropy(stdout, &buf, 6.0, true);
    assert_ne!(buf, [0u8; 15]);

    // Large buffer entropy test.
    let mut buf = [0u8; 80 * 1024]; // 80 KiB.
    trng.fill_buffer(&mut buf);
    find_and_warn_entropy(stdout, &buf, 7.5, true);

    // Repeated entropy test.
    for _ in 0..100 {
        let mut buf = [0u8; 103]; // Not a multiple of 32 bits.
        trng.fill_buffer(&mut buf);
        find_and_warn_entropy(stdout, &buf, 6.0, false);
        assert_ne!(buf, [0u8; 103]);

        let mut buf = [0u8; 124]; // Multiple of 32 bits.
        trng.fill_buffer(&mut buf);
        find_and_warn_entropy(stdout, &buf, 6.0, false);
        assert_ne!(buf, [0u8; 124]);
    }
}
