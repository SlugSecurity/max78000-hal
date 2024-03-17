use core::fmt::Write;
use core::{borrow::BorrowMut, ptr::write_volatile};
use cortex_m_semihosting::hio;
use fault_injection_protection_arm as fip;
use fip::{FaultInjectionPrevention, SecureBool};
use max78000_hal::communication::lower_layers::crypto::RandomSource;
use max78000_hal::peripherals::{rand_chacha::ChaChaRng, PeripheralHandle};
use subtle::ConstantTimeEq;

pub fn run_fip_tests(mut csprng: PeripheralHandle<'_, ChaChaRng>, stdout: &mut hio::HostStream) {
    writeln!(stdout, "Starting fip peripheral tests...").unwrap();

    let fip = fip::FaultInjectionPrevention::new();
    test_critical_if(&fip, csprng.borrow_mut(), stdout);

    // Run tests.
    writeln!(stdout, "FIP tests complete!\n").unwrap();
}

/// Tests the [`ChaCha20Rng::fill_bytes()`] function for the initialized CSPRNG.
fn test_critical_if(
    fip: &FaultInjectionPrevention,
    csprng: &mut ChaChaRng,
    stdout: &mut hio::HostStream,
) {
    fip.critical_if(
        || (true == true).into(),
        || writeln!(stdout, "Success: True path").unwrap(),
        || panic!("Error: False path"),
        csprng,
    );

    fip.critical_if(
        || (true == false).into(),
        || panic!("Error: True path"),
        || writeln!(stdout, "Success: False path").unwrap(),
        csprng,
    );

    writeln!(stdout, "Testing using rng in critical-if closure").unwrap();
    let mut secure_buffer = [0u8; 24];
    csprng.fill_rand_slice(&mut secure_buffer);

    let mut other_secure_buffer = [0u8; 24];

    fip.critical_if(
        || true.into(),
        |rng: &mut ChaChaRng| {
            fip.critical_write(
                &mut other_secure_buffer,
                secure_buffer,
                |dst, src| unsafe { write_volatile(dst, src) },
                rng,
            )
        },
        || (),
        csprng,
    );

    fip.critical_if(
        || {
            SecureBool::from(<subtle::Choice as Into<bool>>::into(
                other_secure_buffer.ct_eq(&secure_buffer),
            ))
        },
        || {
            writeln!(
                stdout,
                "Success: critical-write work inside of critical-if closure"
            )
            .unwrap()
        },
        || panic!("Error: critical-write fails"),
        csprng,
    );

    if fip.critical_read(&other_secure_buffer, csprng) == secure_buffer {
        writeln!(stdout, "Success: critical-read works").unwrap();
    } else {
        writeln!(stdout, "Error: critical-read fails").unwrap();
    }
}
