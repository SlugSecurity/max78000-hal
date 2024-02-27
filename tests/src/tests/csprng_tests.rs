//! Tests for the CSPRNG peripheral.

use core::{borrow::BorrowMut, fmt::Write};

use cortex_m_semihosting::hio;
use max78000_hal::peripherals::{
    rand_chacha::{rand_core::RngCore, ChaCha20Rng},
    PeripheralHandle,
};

use super::trng_tests::find_and_warn_entropy;

/// Runs all CSPRNG tests.
pub fn run_csprng_tests(
    mut csprng: PeripheralHandle<'_, ChaCha20Rng>,
    stdout: &mut hio::HostStream,
) {
    writeln!(stdout, "Starting CSPRNG peripheral tests...").unwrap();

    // Run tests.
    test_fill_buffer(csprng.borrow_mut(), stdout);
    writeln!(stdout, "CSPRNG peripheral tests complete!\n").unwrap();
}

/// Tests the [`csprng::fill_bytes()`] function.
fn test_fill_buffer(csprng: &mut ChaCha20Rng, stdout: &mut hio::HostStream) {
    // Large buffer entropy test.
    let mut buf = [0u8; 80 * 1024]; // 80 KiB.
    csprng.fill_bytes(&mut buf);
    find_and_warn_entropy(stdout, &buf, 7.5, true);
}
