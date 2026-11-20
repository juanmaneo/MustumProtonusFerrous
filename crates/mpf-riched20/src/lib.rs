//! Deterministic RTF parsing logic from Wine's `dlls/riched20/reader.c`.
//!
//! This crate ports `RTFCharToHex`, the helper used to decode the two digits
//! following an RTF `\\'` escape. It intentionally preserves Wine's behavior
//! for non-hex ASCII bytes rather than validating them.

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// Converts an input byte using Wine's `RTFCharToHex` arithmetic.
///
/// ASCII uppercase letters are folded to lowercase; ASCII digits map to 0–9,
/// and all other bytes follow the source helper's `c - 'a' + 10` calculation.
pub const fn rtf_char_to_hex(byte: u8) -> i32 {
    let byte = if byte.is_ascii_uppercase() {
        byte + (b'a' - b'A')
    } else {
        byte
    };

    if byte.is_ascii_digit() {
        (byte - b'0') as i32
    } else {
        (byte as i32) - (b'a' as i32) + 10
    }
}
