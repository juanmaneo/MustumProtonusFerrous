//! Portable PA19 integer decoding logic from Wine's `dlls/mspatcha/pa19.c`.
//!
//! The patch container uses a terminated, little-endian base-128 encoding.
//! Unsigned values use seven payload bits per byte; signed values use six
//! magnitude bits and a sign bit in the first byte, then seven bits per byte.
//! At most nine bytes are consumed, matching Wine's readers.
//!
//! File, handle, buffer-output, timestamp, checksum, range application, and
//! LZX stream operations are not included: they depend on PatchAPI state or
//! the separate decoder.

/// Failure reported when a PA19 variable-length integer is absent or does not
/// terminate within Wine's nine-byte limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VliError {
    /// No byte was available at the start of the integer.
    Empty,
    /// The available bytes do not contain a terminator within nine bytes.
    Unterminated,
}

/// Reads one PA19 unsigned variable-length integer from the start of `bytes`.
///
/// Returns the decoded value and number of consumed bytes. Bytes after the
/// first terminating byte are ignored, as they are by Wine's cursor-based
/// parser. The encoding is limited to nine bytes.
pub fn read_uvli(bytes: &[u8]) -> Result<(u64, usize), VliError> {
    let limit = bytes.len().min(9);
    if limit == 0 {
        return Err(VliError::Empty);
    }

    let mut value = u64::from(bytes[0] & 0x7f);
    let mut index = 1;
    while index < limit && bytes[index - 1] < 0x80 {
        value += u64::from(bytes[index] & 0x7f) << (7 * index);
        index += 1;
    }

    if bytes[index - 1] < 0x80 {
        return Err(VliError::Unterminated);
    }

    Ok((value, index))
}

/// Reads one PA19 signed variable-length integer from the start of `bytes`.
///
/// The first byte's bit 6 is the sign and its low six bits begin the
/// magnitude. Returns the decoded value and number of consumed bytes. Bytes
/// after the first terminating byte are ignored; at most nine bytes are
/// consumed, matching Wine's parser.
pub fn read_svli(bytes: &[u8]) -> Result<(i64, usize), VliError> {
    let limit = bytes.len().min(9);
    if limit == 0 {
        return Err(VliError::Empty);
    }

    let mut value = i64::from(bytes[0] & 0x3f);
    let mut index = 1;
    while index < limit && bytes[index - 1] < 0x80 {
        value += i64::from(bytes[index] & 0x7f) << (7 * index - 1);
        index += 1;
    }

    if bytes[index - 1] < 0x80 {
        return Err(VliError::Unterminated);
    }

    if bytes[0] & 0x40 != 0 {
        value = -value;
    }

    Ok((value, index))
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
