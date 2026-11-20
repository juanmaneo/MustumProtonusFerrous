//! Base64 symbols, lines and padding, as `base64.c` reads and writes them.

use crate::Error;

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const LINE_LENGTH: usize = 64;

/// Encodes `binary` in lines of 64 symbols, each line followed by `line_end`.
pub(crate) fn encode(binary: &[u8], line_end: &str) -> Vec<u8> {
    let mut symbols = Vec::new();
    for group in binary.chunks(3) {
        let bits = group
            .iter()
            .enumerate()
            .fold(0u32, |acc, (i, &b)| acc | (u32::from(b) << (16 - 8 * i)));
        for i in 0..4usize {
            symbols.push(if i <= group.len() {
                ALPHABET[((bits >> (18 - 6 * i)) & 0x3f) as usize]
            } else {
                b'='
            });
        }
    }

    let mut text = Vec::new();
    for line in symbols.chunks(LINE_LENGTH) {
        text.extend_from_slice(line);
        text.extend_from_slice(line_end.as_bytes());
    }
    text
}

/// Decodes base64 text. Whitespace is skipped, and the first padding symbol ends a complete
/// group. Padding that leaves a group incomplete is an error.
pub(crate) fn decode(input: &[u8]) -> Result<Vec<u8>, Error> {
    let values = values(input)?;
    let mut out = Vec::new();
    for quad in values.chunks(4) {
        let mut v = [0u8; 4];
        v[..quad.len()].copy_from_slice(quad);
        out.push((v[0] << 2) | (v[1] >> 4));
        if quad.len() > 2 {
            out.push((v[1] << 4) | (v[2] >> 2));
        }
        if quad.len() > 3 {
            out.push((v[2] << 6) | v[3]);
        }
    }
    Ok(out)
}

enum Symbol {
    Value(u8),
    Padding,
    Space,
}

fn symbol(c: u8) -> Result<Symbol, Error> {
    match c {
        b'=' => Ok(Symbol::Padding),
        b' ' | b'\t' | b'\r' | b'\n' => Ok(Symbol::Space),
        _ => ALPHABET
            .iter()
            .position(|&a| a == c)
            .map(|value| Symbol::Value(value as u8))
            .ok_or(Error::InvalidData),
    }
}

/// Collects the 6-bit values before the padding.
fn values(input: &[u8]) -> Result<Vec<u8>, Error> {
    let mut out = Vec::new();
    let mut seen = 0usize;
    let mut padded = false;
    for &c in input {
        match symbol(c)? {
            Symbol::Space => {}
            Symbol::Value(_) if padded => return Err(Error::InvalidData),
            Symbol::Padding if seen.is_multiple_of(4) => break,
            Symbol::Padding => {
                padded = true;
                seen += 1;
                if seen.is_multiple_of(4) {
                    break;
                }
            }
            Symbol::Value(value) => {
                seen += 1;
                out.push(value);
            }
        }
    }
    if padded && !seen.is_multiple_of(4) {
        return Err(Error::InvalidData);
    }
    Ok(out)
}
