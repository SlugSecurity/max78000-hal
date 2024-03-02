use super::{KeyedChannel, RandomSource};
use crate::communication::{
    self,
    lower_layers::framing::{Frame, FramedTxChannel},
    CommunicationError, RxChannel, Timeout, TxChannel,
};
use chacha20poly1305::{AeadCore, AeadInPlace, KeyInit, XChaCha20Poly1305};
use generic_array::GenericArray;
use typenum::Unsigned;

pub use chacha20poly1305::Key;

/// This typedef can be used to change what algorithm the channel in this module uses.
type ChannelAlgorithm = XChaCha20Poly1305;

type TagSize = <ChannelAlgorithm as AeadCore>::TagSize;
type NonceSize = <ChannelAlgorithm as AeadCore>::NonceSize;

const TAG_SIZE: usize = <TagSize as Unsigned>::USIZE;
const NONCE_SIZE: usize = <NonceSize as Unsigned>::USIZE;

/// The total metadata size required when receiving on a [`XChacha20Poly1305RxChannel`].
pub const METADATA_SIZE: usize = TAG_SIZE + NONCE_SIZE;

/// This [`RxChannel`] wraps around another [`RxChannel`] to decrypt communications encrypted
/// by a [`XChacha20Poly1305TxChannel`], providing message authenticity and confidentiality.
/// When reading from an [`XChacha20Poly1305RxChannel`], care must be taken to ensure that
/// there is sufficient space to store the 16-byte tag and 24-byte nonce as well.
/// If a received message doesn't contain a nonce or authentication tag or has an invalid
/// authentication tag, a [`CommunicationError::RecvError`] is given. If the underlying
/// channel gives this error, it will be propagated up. Data sent and received through
/// this channel must be at least 1 byte long.
///
/// # ERRORS:
///
/// - [`CommunicationError::RecvError`] - The message didn't contain a nonce of the right size,
/// didn't match the authentication tag provided, didn't contain an authentication tag, couldn't
/// be read into the buffer because it was too small, or an error occurred while receiving the
/// message from the wrapped channel.
///
/// See the [`module`](super) documentation for more information on the cipher used.
pub struct XChacha20Poly1305RxChannel<T: RxChannel> {
    channel: T,
    decryptor: ChannelAlgorithm,
}

impl<T: RxChannel> XChacha20Poly1305RxChannel<T> {
    /// Creates a new [`XChacha20Poly1305RxChannel`] given an inner [`RxChannel`] and a
    /// decryption [`Key`].
    pub fn new(channel: T, rx_key: &Key) -> Self {
        Self {
            channel,
            decryptor: ChannelAlgorithm::new(rx_key),
        }
    }

    fn recv_with<U: Timeout>(
        &mut self,
        dest: &mut [u8],
        read_fn: impl FnOnce(&mut Self, &mut [u8], &mut U) -> communication::Result<usize>,
        timer: &mut U,
    ) -> communication::Result<usize> {
        const METADATA_SIZE: usize = TAG_SIZE + NONCE_SIZE;

        // Check that the destination buffer has space for at least one byte of ciphertext.
        if dest.len() <= METADATA_SIZE {
            return Err(CommunicationError::RecvError(0));
        }

        // Read message from inner channel.
        let bytes_read = read_fn(self, dest, timer)?;
        let dest = &mut dest[..bytes_read];

        // Check we have at least one byte of ciphertext.
        if dest.len() <= METADATA_SIZE {
            return Err(CommunicationError::RecvError(0));
        }

        // Split message from metadata.
        let (msg_body, metadata) = dest.split_at_mut(dest.len() - METADATA_SIZE);

        // Take nonce and tag
        let (&mut ref nonce, &mut ref tag) = metadata.split_at_mut(NONCE_SIZE);

        // Decrypt in place using the ciphertext, nonce, and tag
        self.decryptor
            .decrypt_in_place_detached(nonce.into(), b"", msg_body, tag.into())
            .map_err(|_| CommunicationError::RecvError(0))?;

        // Our decrypted buffer is at the beginning of our slice and we return the length of it.
        Ok(msg_body.len())
    }
}

impl<T: RxChannel> KeyedChannel for XChacha20Poly1305RxChannel<T> {
    type KeyType = Key;

    fn change_key(&mut self, new_key: &Self::KeyType) {
        self.decryptor = ChannelAlgorithm::new(new_key);
    }
}

impl<T: RxChannel> RxChannel for XChacha20Poly1305RxChannel<T> {
    /// Receives data from the channel, putting the data received into ``dest``, returning the
    /// number of bytes written to it upon success. The buffer provided should have enough
    /// space to store the data that needs to be received along with its metadata size. The provided timeout
    /// is reset on each byte received. If the timeout has passed and not enough bytes have been received, this
    /// function returns an error. Upon an error, a [`CommunicationError`] is given.
    ///
    /// # ERRORS:
    ///
    /// - [`CommunicationError::RecvError`] - This error can occur in the following cases:
    ///   - If the provided buffer is too small to fit a whole message sent in a frame or if a malformed
    ///     message was sent. In this channel, there must be enough space to accomodate for [`METADATA_SIZE`]
    ///     bytes + 1 additional byte of message data. A blank message can neither be sent nor received.
    ///   - If the timeout is reached.
    ///  - [`CommunicationError::InternalError`]
    ///    - This can occur if some internal error happens. This should only occur if something is wrong
    ///      with the implementation.
    fn recv_with_data_timeout<U: Timeout>(
        &mut self,
        dest: &mut [u8],
        timer: &mut U,
    ) -> communication::Result<usize> {
        self.recv_with(
            dest,
            |ch, d, t| ch.channel.recv_with_data_timeout(d, t),
            timer,
        )
    }

    /// Receives data from the channel, putting the data received into ``dest``, returning the
    /// number of bytes written to it upon success. The buffer provided should have enough
    /// space to store the data that needs to be received along with its metadata size. The provided time to
    /// block is for the entire receive operation. If the timeout has passed and not enough bytes have been received,
    /// this function returns an error. Upon an error, a [`CommunicationError`] is given.
    ///
    /// # ERRORS:
    ///
    /// - [`CommunicationError::RecvError`] - This error can occur in the following cases:
    ///   - If the provided buffer is too small to fit a whole message sent in a frame or if a malformed
    ///     message was sent. In this channel, there must be enough space to accomodate for [`METADATA_SIZE`]
    ///     bytes + 1 additional byte of message data. A blank message can neither be sent nor received.
    ///   - If the timeout is reached.
    ///  - [`CommunicationError::InternalError`]
    ///    - This can occur if some internal error happens. This should only occur if something is wrong
    ///      with the implementation.
    fn recv_with_timeout<U: Timeout>(
        &mut self,
        dest: &mut [u8],
        timer: &mut U,
    ) -> communication::Result<usize> {
        self.recv_with(dest, |ch, d, t| ch.channel.recv_with_timeout(d, t), timer)
    }
}

/// This [`TxChannel`] wraps around a [`FramedTxChannel`] to encrypt communications encrypted by a [`XChacha20Poly1305TxChannel`],
/// providing message authenticity and confidentiality. This channel requires a [`RandomSource`] to generate a random nonce.
///
/// See the module-level documentation for more information on the cipher used.
pub struct XChacha20Poly1305TxChannel<T: FramedTxChannel, U: RandomSource> {
    channel: T,
    random_source: U,
    encryptor: ChannelAlgorithm,
}

impl<T: FramedTxChannel, U: RandomSource> XChacha20Poly1305TxChannel<T, U> {
    /// Creates a new [`XChacha20Poly1305TxChannel`] given an inner [`FramedTxChannel`] and an
    /// encryption [`Key`].
    pub fn new(channel: T, random_source: U, tx_key: &Key) -> Self {
        Self {
            channel,
            random_source,
            encryptor: ChannelAlgorithm::new(tx_key),
        }
    }
}

impl<T: FramedTxChannel, U: RandomSource> KeyedChannel for XChacha20Poly1305TxChannel<T, U> {
    type KeyType = Key;

    fn change_key(&mut self, new_key: &Self::KeyType) {
        self.encryptor = ChannelAlgorithm::new(new_key);
    }
}

impl<T: FramedTxChannel, U: RandomSource> TxChannel for XChacha20Poly1305TxChannel<T, U> {
    /// Sends the data from ``src`` through the channel. Upon an error, a [`CommunicationError`]
    /// is given. The data in this buffer is modified because it is encrypted in place.
    ///
    /// # ERRORS:
    ///
    /// - [`CommunicationError::SendError`]
    ///   - This could occur if any implementation-based error occurs while sending data.
    ///     This could be because:
    ///         - The message was too short. With this channel, at least one byte of data must be sent.
    ///         - An error occurred during message encryption.
    /// - [`CommunicationError::InternalError`]
    ///   - This can occur if some internal error happens. This should only occur if something is wrong
    ///     with the implementation.
    fn send(&mut self, buff: &mut [u8]) -> communication::Result<()> {
        if buff.is_empty() {
            return Err(CommunicationError::SendError);
        }

        let mut nonce: GenericArray<u8, NonceSize> = Default::default();

        // Fill nonce with random bytes.
        self.random_source.fill_rand_slice(&mut nonce);

        // Encrypt buff completely in place with no associated data, returning the auth tag.
        let tag = self
            .encryptor
            .encrypt_in_place_detached(&nonce, b"", buff)
            .map_err(|_| CommunicationError::SendError)?;

        // Write message in following order: Ciphertext + Nonce + Tag
        self.channel
            .frame::<3>(|| Frame::new().append(buff)?.append(&nonce)?.append(&tag))
    }
}
