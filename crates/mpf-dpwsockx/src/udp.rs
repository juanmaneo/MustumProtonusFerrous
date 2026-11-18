//! The datagram framing of `DPWS_UdpReceiveCompleted`.
//!
//! A framed datagram starts with a `DPSP_MSG_HEADER` whose body begins with the `play`
//! signature. Anything else is passed on as raw bytes.

use crate::header::{msg_size, HEADER_SIZE};
use crate::MsgHeader;

/// The signature at byte 20 of a framed datagram: `0x79616c70` read as a little-endian DWORD.
pub const SIGNATURE: &[u8] = b"play";

/// A datagram as `HandleMessage` receives it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Datagram<'a> {
    /// The header of a framed datagram, or `None` for raw data.
    pub header: Option<MsgHeader>,
    /// The message body. For a framed datagram it starts with the signature, as Wine passes it.
    pub body: &'a [u8],
}

/// A framed datagram that `DPWS_UdpReceiveCompleted` refuses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatagramError {
    /// The header's size is smaller than the header itself.
    TooShort,
    /// The header's size is larger than the datagram.
    Truncated,
}

/// Splits a received datagram. On a [`DatagramError`] Wine skips `HandleMessage` and does not
/// re-arm the receive, which stops datagram reception. The caller decides whether to re-arm.
pub fn parse_datagram(data: &[u8]) -> Result<Datagram<'_>, DatagramError> {
    match MsgHeader::from_bytes(data) {
        Some(header) if data.get(HEADER_SIZE..HEADER_SIZE + SIGNATURE.len()) == Some(SIGNATURE) => {
            let size = msg_size(header.mixed) as usize;
            if size < HEADER_SIZE {
                Err(DatagramError::TooShort)
            } else if size > data.len() {
                Err(DatagramError::Truncated)
            } else {
                Ok(Datagram {
                    header: Some(header),
                    body: &data[HEADER_SIZE..size],
                })
            }
        }
        _ => Ok(Datagram {
            header: None,
            body: data,
        }),
    }
}
