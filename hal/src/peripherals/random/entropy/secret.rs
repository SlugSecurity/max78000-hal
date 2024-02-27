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
    fn init(csprng_init_args: CsprngInitArgs) -> Self {
        let mut secret = [0; SECRET_SIZE];
        (csprng_init_args.get_rng_static_secret)(&mut secret);

        Secret {
            next: T::init(csprng_init_args),
            secret,
        }
    }

    fn add_to_hasher(&self, hasher: &mut Sha3_256) {
        hasher.update(&self.secret);
        self.next.add_to_hasher(hasher);
    }
}

impl<T: EntropySource> Drop for Secret<T> {
    fn drop(&mut self) {
        self.secret.zeroize();
    }
}
