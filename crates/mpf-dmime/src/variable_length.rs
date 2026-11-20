//! MIDI variable-length number decoding from `read_variable_length_number`.

/// The outcome of reading a MIDI variable-length number from the start of a byte slice.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecodeResult {
    /// A terminating byte was read.
    Complete { value: u32, bytes_read: usize },
    /// The input ended while the continuation bit was set.
    Incomplete { value: u32, bytes_read: usize },
}

/// Reads one variable-length number, preserving the `DWORD` wrapping behavior of Wine's decoder.
///
/// Bytes after the first byte without its high bit set are not consumed. An input ending before a
/// terminating byte returns the accumulated value and the number of bytes it contained.
pub fn read_variable_length_number(bytes: &[u8]) -> DecodeResult {
    let mut value: u32 = 0;

    for (index, &byte) in bytes.iter().enumerate() {
        value = value.wrapping_shl(7) | u32::from(byte & 0x7f);
        if byte & 0x80 == 0 {
            return DecodeResult::Complete {
                value,
                bytes_read: index + 1,
            };
        }
    }

    DecodeResult::Incomplete {
        value,
        bytes_read: bytes.len(),
    }
}
