//! This module contains traits to implement the CommStack and error structs.
//!
//! ## CommStack
//!
//! The CommStack consists of three layers.
//! - The framing layer
//!     - The framing layer is responsible for turning a stream of data into frames.
//!       See the [`framing`](lower_layers::framing) module for the traits involved in this layer.
//! - The encryption and integrity layer
//!     - This layer is responsible for providing secure and authenticated methods of transportation.
//!       Implementations of this part of the CommStack are provided in the [`crypto`](lower_layers::crypto)
//!       module. See that module for more detail on the traits and structs provided for this layer.
//! - The application layer
//!     - The application layer is the layer responsible for incorporating the lower two layers together.
//!       There are no implementations provided on this layer, but one can easily put together types consisting
//!       of different channel primitives to guarantee different properties.

use core::time::Duration;

pub mod lower_layers;

/// Type definition for any [`CommunicationError`] [`Results`](core::result::Result).
pub type Result<T> = core::result::Result<T, CommunicationError>;

/// This trait represents a timeout that can be polled and reset. It is used to provide a platform-indepentdent
/// way to poll for if time is up and reset the timeout as necessary.
pub trait Timeout {
    /// Polls the timer to see if time is up, returning ``true`` if it is up.
    fn poll(&mut self) -> bool;

    /// Resets the timer back to its original duration.
    fn reset(&mut self);

    /// Gets the total duration of the timer.
    fn duration(&self) -> Duration;
}

/// A channel to receive data from. See the documentation for [`recv_with_timeout`](RxChannel::recv_with_timeout)
/// and [`recv_with_data_timeout`](RxChannel::recv_with_data_timeout) for more info.
pub trait RxChannel {
    /// Receives data from the channel, putting the data received into ``dest``, returning the
    /// number of bytes written to it upon success. The buffer provided should have enough
    /// space to store the data that needs to be received along with its metadata size. The provided timeout
    /// is reset on each byte received. If the timeout has passed and not enough bytes have been received, this
    /// function returns an error. Upon an error, a [`CommunicationError`] is given.
    ///
    /// # ERRORS:
    ///
    /// - [`CommunicationError::RecvError`] - There are a couple of cases when this can occur:
    ///   - If this is a channel receiving communications from a
    ///     [`FramedTxChannel`](lower_layers::framing::FramedTxChannel), then this error could occur
    ///     if the provided buffer is too small to fit a whole message sent in a frame or if a malformed
    ///     message was sent.
    ///   - The timeout is reached.
    ///   - If this is a channel receiving communications from a channel in the crypto layer, such
    ///     as from an [`XChachaPoly1305Channel`](lower_layers::crypto::XChacha20Poly1305TxChannel)
    ///     then this error could occur if the provided buffer isn't big enough to store the additional
    ///     metadata, which can include a nonce and/or an authentication tag. Additionally, if the message
    ///     sent couldn't be authenticated, which can occur due to data corruption, then this error
    ///     will be returned.
    ///  - [`CommunicationError::InternalError`]
    ///    - This can occur if some internal error happens. This should only occur if something is wrong
    ///      with the implementation.
    fn recv_with_data_timeout<T: Timeout>(&mut self, dest: &mut [u8], tmr: &mut T)
        -> Result<usize>;

    /// Receives data from the channel, putting the data received into ``dest``, returning the
    /// number of bytes written to it upon success. The buffer provided should have enough
    /// space to store the data that needs to be received along with its metadata size. The provided time to
    /// block is for the entire receive operation. If the timeout has passed and not enough bytes have been received,
    /// this function returns an error. Upon an error, a [`CommunicationError`] is given.
    ///
    /// # ERRORS:
    ///
    /// - [`CommunicationError::RecvError`] - There are a couple of cases when this can occur:
    ///   - If this is a channel receiving communications from a
    ///     [`FramedTxChannel`](lower_layers::framing::FramedTxChannel), then this error could occur
    ///     if the provided buffer is too small to fit a whole message sent in a frame or if a malformed
    ///     message was sent.
    ///   - The timeout is reached.
    ///   - If this is a channel receiving communications from a channel in the crypto layer, such
    ///     as from an [`XChachaPoly1305Channel`](lower_layers::crypto::XChacha20Poly1305TxChannel)
    ///     then this error could occur if the provided buffer isn't big enough to store the additional
    ///     metadata, which can include a nonce and/or an authentication tag. Additionally, if the message
    ///     sent couldn't be authenticated, which can occur due to data corruption, then this error
    ///     will be returned.
    ///  - [`CommunicationError::InternalError`]
    ///    - This can occur if some internal error happens. This should only occur if something is wrong
    ///      with the implementation.
    fn recv_with_timeout<T: Timeout>(&mut self, dest: &mut [u8], tmr: &mut T) -> Result<usize>
    where
        T: Timeout;
}

/// A channel to receive data from which supports reading until a line delimiter.
pub trait LineDelimitedRxChannel: RxChannel {
    /// Receives one line of data from the channel, reading until the specified
    /// [`line_ending`](LineEnding) or the timeout is reached, putting the data into `dest`,
    /// returning the number of bytes received (including the line ending) upon success. `tmr` is
    /// reset for every byte read. If the end of the buffer is reached before seeing a line ending,
    /// returns [`RecvError(dest.len())`](CommunicationError::RecvError).
    ///
    /// See [rev_with_data_timeout](RxChannel::recv_with_data_timeout) for more documentation on the
    /// other error conditions.
    fn recv_line_with_data_timeout<T: Timeout>(
        &mut self,
        dest: &mut [u8],
        tmr: &mut T,
        line_ending: LineEnding,
    ) -> Result<usize>
    where
        T: Timeout;

    /// Receives one line of data from the channel, reading until the specified
    /// [`line_ending`](LineEnding) or the timeout is reached, putting the data into `dest`,
    /// returning the number of bytes received (including the line ending) upon success. `tmr`
    /// indicates a timeout for the entire operation. If the end of the buffer is reached before
    /// seeing a line ending, returns [`RecvError(dest.len())`](CommunicationError::RecvError).
    ///
    /// See [rev_with_data_timeout](RxChannel::recv_with_data_timeout) for more documentation on the
    /// other error conditions.
    fn recv_line_with_timeout<T: Timeout>(
        &mut self,
        dest: &mut [u8],
        tmr: &mut T,
        line_ending: LineEnding,
    ) -> Result<usize>
    where
        T: Timeout;
}

/// A channel to send data through. See the documentation for [`send`](TxChannel::send) for
/// more info.
pub trait TxChannel {
    /// Sends the data from ``src`` through the channel. Upon an error, a [`CommunicationError`]
    /// is given. The data in this buffer is not guaranteed to be intact after this function
    /// sends the data. In crypto channels for example, encryption can occur in place.
    ///
    /// # ERRORS:
    ///
    /// - [`CommunicationError::SendError`]
    ///   - This could occur if any implementation-based error occurs while sending data.
    /// - [`CommunicationError::InternalError`]
    ///   - This can occur if some internal error happens. This should only occur if something is wrong
    ///     with the implementation.
    fn send(&mut self, src: &mut [u8]) -> Result<()>;
}

/// The possible errors that can occur while sending or receiving data through an [`RxChannel`] or a
/// [`TxChannel`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CommunicationError {
    /// An error that can occur during a receive operation. See [RxChannel::recv_with_timeout] and
    /// [RxChannel::recv_with_data_timeout] for more details. Contains the number of bytes that were
    /// read successfully before the error occurred, if any.
    RecvError(usize),

    /// An error that can occur during a send operation. See [TxChannel::send] for more details.
    SendError,

    /// An error that can occur if an internal error is encountered that should never happen.
    InternalError,
}

/// Specifies what is counted as the end of a line for the RxChannel::recv_line_* methods
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LineEnding {
    /// Carriage return (\r)
    CR,
    /// Newline (\n)
    LF,
    /// Carriage return, then newline (\r\n)
    CRLF,
}

impl LineEnding {
    /// Check if a buffer of bytes ends with the specified line ending
    pub fn matches_end(&self, buf: &[u8]) -> bool {
        match *self {
            LineEnding::CR => buf.last().copied() == Some(b'\r'),
            LineEnding::LF => buf.last().copied() == Some(b'\n'),
            LineEnding::CRLF => {
                buf.len() >= 2 && buf[buf.len() - 2] == b'\r' && buf[buf.len() - 1] == b'\n'
            }
        }
    }
}
