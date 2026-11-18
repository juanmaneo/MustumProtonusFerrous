//! The stream receive decisions of `DPWS_TcpReceiveCompleted` and `DPWS_HeaderReceiveCompleted`.
//!
//! The socket calls and the completion routines stay with the caller. These functions say
//! what Wine does next.

use crate::header::{msg_size, HEADER_SIZE};

/// `DPWS_GUARANTEED_MAXBUFFERSIZE`: the largest message body a guaranteed message may carry.
pub const GUARANTEED_MAX_BUFFER_SIZE: u32 = 1_048_547;

/// The next step after a completed stream read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TcpStep {
    /// The read failed or the peer closed the stream: the connection is removed.
    Drop,
    /// The buffer is not full: post a new read for the `remaining` bytes still missing.
    Rearm { remaining: u32 },
    /// The buffer is full: run the completion routine.
    Complete,
}

/// Decides the next step after a read of `transferred` bytes. `remaining` is the length the
/// read was posted with, and `error` is the Win32 error of the read (0 on success).
pub fn tcp_receive_step(error: u32, transferred: u32, remaining: u32) -> TcpStep {
    if error != 0 || transferred == 0 {
        TcpStep::Drop
    } else if transferred < remaining {
        TcpStep::Rearm {
            remaining: remaining - transferred,
        }
    } else {
        TcpStep::Complete
    }
}

/// A header that `DPWS_HeaderReceiveCompleted` refuses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeaderError {
    /// The size is smaller than the header itself.
    TooShort,
    /// The body is larger than [`GUARANTEED_MAX_BUFFER_SIZE`].
    TooLong,
}

/// The body length that follows a header with this `mixed` word.
pub fn body_size(mixed: u32) -> Result<u32, HeaderError> {
    let size = msg_size(mixed);
    let header_len = HEADER_SIZE as u32;
    if size < header_len {
        Err(HeaderError::TooShort)
    } else if size - header_len > GUARANTEED_MAX_BUFFER_SIZE {
        Err(HeaderError::TooLong)
    } else {
        Ok(size - header_len)
    }
}

/// The buffer growth rule of `DPWS_HeaderReceiveCompleted`: the new buffer size when a body of
/// `needed` bytes does not fit in a buffer of `current` bytes, or `None` when it fits. The
/// buffer doubles but never ends up smaller than `needed`.
///
/// Wine computes this in `int`. The doubling saturates here instead, which only matters for
/// sizes [`body_size`] never admits, since `needed` is capped at the guaranteed body limit.
pub fn grown_buffer_size(current: u32, needed: u32) -> Option<u32> {
    (current < needed).then(|| current.saturating_mul(2).max(needed))
}
