//! DER header decoding for the signature stream read by `MsiSIPGetSignedDataMsg`.

/// Tag of the outer `SEQUENCE` that wraps a PKCS #7 signature.
pub const SEQUENCE_TAG: u8 = 0x30;
/// Length byte for indefinite-length encoding, which Wine rejects.
pub const INDEFINITE_LENGTH: u8 = 0x80;
/// Longest long-form length accepted: the size of a `DWORD`.
pub const MAX_LENGTH_OCTETS: u8 = 4;

/// The tag, length octets and content length of a definite-length `SEQUENCE`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DerHeader {
    /// Tag byte, length byte and any long-form length octets.
    pub header_len: u8,
    /// Content length from the length octets.
    pub content_len: u32,
}

impl DerHeader {
    /// `2 + lenBytes + dataBytes` in `DWORD` arithmetic, which wraps on overflow.
    pub fn message_len(&self) -> u32 {
        u32::from(self.header_len).wrapping_add(self.content_len)
    }
}

/// Reads the header that `MsiSIPGetSignedDataMsg` reads: the tag, then a
/// short-form or long-form definite length of at most `MAX_LENGTH_OCTETS` octets.
pub fn parse_der_header(stream: &[u8]) -> Option<DerHeader> {
    let &[tag, first_len, ..] = stream else {
        return None;
    };
    if tag != SEQUENCE_TAG || first_len == INDEFINITE_LENGTH {
        return None;
    }
    if first_len & 0x80 == 0 {
        return Some(DerHeader {
            header_len: 2,
            content_len: u32::from(first_len),
        });
    }
    let len_octets = first_len & 0x7f;
    if len_octets > MAX_LENGTH_OCTETS {
        return None;
    }
    let octets = stream.get(2..2 + usize::from(len_octets))?;
    let content_len = octets
        .iter()
        .fold(0u32, |acc, &b| (acc << 8) | u32::from(b));
    Some(DerHeader {
        header_len: 2 + len_octets,
        content_len,
    })
}
