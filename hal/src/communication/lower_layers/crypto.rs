//! This module encompasses the encryption and integrity layer of the CommStack, which provides
//! wrapper types over [`RxChannels`](crate::communication::RxChannel) and
//! [`FramedTxChannels`](crate::communication::lower_layers::framing::FramedTxChannel), providing
//! securer and more robust ways to send and receive messages through channels. Additionally, this
//! module adds a trait called [`KeyedChannel`] that allows for encryption/decryption keys associated
//! with a secure channel to be modified after channel creation. Secure channels often need random number
//! generation and when they do, they'll require some implementation of [`RandomSource`].
//!
//! # Current secure channel implementations:
//! ## [`XChacha20Poly1305RxChannel`] and [`XChacha20Poly1305TxChannel`]
//! These channels provide message integrity and confidentiality by using XChacha20Poly1305.
//! Each message sent will contain a 24-byte nonce, a 16-byte authentication tag, and the
//! ciphertext given a 32-byte symmetric key to encrypt and decrypt communications. The
//! authentication tag provided will be checked against the message body to prevent message tampering.
//! This means that any buffers used to received messages from an [`XChacha20Poly1305RxChannel`] must
//! have enough space to store the additional metadata, totaling 40 bytes. This is stored in the
//! constant ``XChacha20Poly1305RxChannel::METADATA_SIZE``. This channel requires random number
//! generation. Because of this, it requires a [`RandomSource`].
//!
//! See the documentation for [`communication`](crate::communication) for a description of the CommStack
//! and more info on the other layers of the CommStack.

mod chachapoly1305;

pub use chachapoly1305::*;

/// Implemented for any channel that has encryption/decryption keys that can be changed after channel
/// creation.
pub trait KeyedChannel {
    /// The type of the key used in the channel.
    type KeyType;

    /// Changes the encryption/decryption key for this channel to a new key
    fn change_key(&mut self, new_key: &Self::KeyType);
}

/// Trait used for secure channels when they need random number generation.
pub trait RandomSource {
    /// Fills the provided slice with random bytes.
    fn fill_rand_slice<T: AsMut<[u8]>>(&mut self, slice_ref: T);
}
