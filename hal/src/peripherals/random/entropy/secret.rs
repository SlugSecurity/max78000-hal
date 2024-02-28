use cortex_m_semihosting::dbg;
use sha3::{digest::Update, Sha3_256};
use zeroize::Zeroize;

use crate::peripherals::random::{CsprngInitArgs, SECRET_SIZE};

use super::EntropySource;

/// This entropy source is a constant secret value.
///
/// IMPORTANT: This struct should not be moved to ensure the secret gets zeroed out on drop.
pub(crate) struct Secret<T: EntropySource> {
    next: T,
    secret: [u8; SECRET_SIZE],
}

impl<T: EntropySource> EntropySource for Secret<T> {
    fn init<F: FnMut(&mut [u8])>(mut csprng_init_args: CsprngInitArgs<F>) -> Self {
        let mut secret = [0; SECRET_SIZE];
        (csprng_init_args.get_rng_static_secret)(&mut secret);

        Secret {
            next: T::init(csprng_init_args),
            secret,
        }
    }

    fn add_to_hasher(&self, hasher: &mut Sha3_256) {
        let static_secret = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B,
            0x1C, 0x1D, 0x1E, 0x1F,
        ];
        assert_eq!(static_secret, self.secret);
        dbg!(
            "Secret entropy (already asserted against static test): ",
            &self.secret
        );

        hasher.update(&self.secret);
        self.next.add_to_hasher(hasher);
    }
}

impl<T: EntropySource> Drop for Secret<T> {
    fn drop(&mut self) {
        self.secret.zeroize();
    }
}
