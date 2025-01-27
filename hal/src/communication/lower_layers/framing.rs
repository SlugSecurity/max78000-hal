//! This module encompasses the framing layer of the CommStack, which provides framing traits.
//! Any framing implementation must have channels implementing the aforementioned traits.
//! [`FramedTxChannels`](FramedTxChannel) differ from [`TxChannels`](TxChannel) in that they require
//! framing while [`TxChannels`](TxChannel) do not necessarily require any concept of framing. No framing
//! protocols are provided in this module.
//!
//! See the documentation for [`communication`](crate::communication) for a description of full communication
//! stack.

use crate::communication::{CommunicationError, TxChannel};

/// A trait to be implemented by all transmission channels in framing protocol implementations.
/// This contains one function to specify the slices that go into the frame to be transmitted.
pub trait FramedTxChannel: TxChannel {
    /// Transmits a frame through the [`TxChannel`] given a closure returning a [`Frame`] or
    /// a [`CommunicationError`]. The const generic, FRAME_CT, must be the number of
    /// slices in the created frame.
    ///
    /// # ERRORS:
    ///
    /// - [`CommunicationError::SendError`] - Occurs when there's no more space
    ///   in the frame for the number of slices provided or some error occurs when
    ///   sending the frame through the [`TxChannel`].
    fn frame<'a, const FRAME_CT: usize>(
        &mut self,
        frame: impl FnOnce() -> Result<Frame<'a, FRAME_CT>, CommunicationError>,
    ) -> Result<(), CommunicationError>;
}

impl<T: FramedTxChannel> TxChannel for T {
    fn send(&mut self, src: &mut [u8]) -> Result<(), CommunicationError> {
        self.frame::<1>(|| Frame::new().append(src))
    }
}

/// A struct that keeps track of slices of u8's to write as one frame
/// in a [`FramedTxChannel`]. This can be used to write discontiguous
/// pieces of memory into one frame. The const generic ``FRAME_SLICES``
/// indicates the number of slices in the [`Frame`].
#[derive(Default)]
pub struct Frame<'a, const FRAME_SLICES: usize> {
    frame_components: heapless::Vec<&'a [u8], FRAME_SLICES>,
    total_len: usize,
}

impl<'a, const FRAME_SLICES: usize> IntoIterator for Frame<'a, FRAME_SLICES> {
    type Item = &'a [u8];
    type IntoIter = <heapless::Vec<&'a [u8], FRAME_SLICES> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.frame_components.into_iter()
    }
}

impl<'a, const FRAME_CT: usize> Frame<'a, FRAME_CT> {
    /// Instantiates a new [`Frame`]. See the struct documentation for
    /// more information.
    pub fn new() -> Self {
        Frame {
            frame_components: heapless::Vec::new(),
            total_len: 0,
        }
    }

    /// Adds a slice to the frame.
    ///
    /// # ERRORS:
    ///
    /// - [`CommunicationError::InternalError`] - Occurs when there's no more space
    ///   in the frame for another slice.
    pub fn append(mut self, buff: &'a [u8]) -> Result<Self, CommunicationError> {
        match self.frame_components.push(buff) {
            Ok(_) => {
                self.total_len += buff.len();

                Ok(self)
            }
            Err(_) => Err(CommunicationError::InternalError),
        }
    }

    /// Gets the length of the frame in bytes.
    pub fn len(&self) -> usize {
        self.total_len
    }

    /// Checks if the [`Frame`] is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Converts the [`Frame`] into an iterator over the individual bytes of the frame
    // not using impl IntoIterator as that is already implemented
    pub fn into_byte_iter(self) -> FrameIterator<'a, FRAME_CT> {
        FrameIterator {
            frame: self.frame_components,
            current_slice_index: 0,
            current_byte_index: 0,
        }
    }
}

/// An iterator over the bytes of a frame, not the slices
pub struct FrameIterator<'a, const FRAME_SLICES: usize> {
    frame: heapless::Vec<&'a [u8], FRAME_SLICES>,
    current_slice_index: usize,
    current_byte_index: usize,
}

impl<const FRAME_SLICES: usize> FrameIterator<'_, FRAME_SLICES> {
    /// Computes the frame length, in bytes
    pub fn length(&self) -> usize {
        self.frame.iter().fold(0, |sum, el| sum + el.len())
    }
}

impl<const FRAME_SLICES: usize> Iterator for FrameIterator<'_, FRAME_SLICES> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(slice) = self.frame.get(self.current_slice_index) {
            if let Some(&byte) = slice.get(self.current_byte_index) {
                self.current_byte_index += 1;
                Some(byte)
            } else {
                self.current_slice_index += 1;
                self.current_byte_index = 0;
                self.next()
            }
        } else {
            None
        }
    }
}
