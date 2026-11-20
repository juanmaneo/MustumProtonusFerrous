//! Hex text, as `CRYPT_STRING_HEXRAW` writes it and `CRYPT_STRING_HEX` reads it.

use crate::Error;

const DIGITS: &[u8; 16] = b"0123456789abcdef";

/// Writes lowercase hex for `binary`, followed by `line_end`.
pub(crate) fn encode(binary: &[u8], line_end: &str) -> Vec<u8> {
    let mut text = Vec::new();
    for &b in binary {
        text.push(DIGITS[usize::from(b >> 4)]);
        text.push(DIGITS[usize::from(b & 0x0f)]);
    }
    text.extend_from_slice(line_end.as_bytes());
    text
}

/// Reads hex digit pairs. Leading separators (`-`, `,` and whitespace) are skipped. Between
/// pairs only `-` and `,` may appear, and after the last pair only separators may follow.
pub(crate) fn decode(input: &[u8]) -> Result<Vec<u8>, Error> {
    let mut out = Vec::new();
    let mut i = skip_run(input, 0, is_special);
    while let Some(hi) = input.get(i).copied().and_then(digit) {
        let lo = input
            .get(i + 1)
            .copied()
            .and_then(digit)
            .ok_or(Error::InvalidData)?;
        out.push((hi << 4) | lo);
        i = skip_run(input, i + 2, is_pair_gap);
    }
    if input[i..].iter().copied().all(is_special) {
        Ok(out)
    } else {
        Err(Error::InvalidData)
    }
}

fn skip_run(input: &[u8], from: usize, skip: fn(u8) -> bool) -> usize {
    from + input[from..].iter().take_while(|&&c| skip(c)).count()
}

fn is_special(c: u8) -> bool {
    matches!(c, b'-' | b',' | b' ' | b'\t' | b'\r' | b'\n')
}

fn is_pair_gap(c: u8) -> bool {
    matches!(c, b'-' | b',')
}

fn digit(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}
