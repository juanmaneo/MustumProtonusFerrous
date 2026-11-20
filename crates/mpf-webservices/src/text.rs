//! Port of the pure text helpers in `dlls/webservices/reader.c`: the scalar parsers
//! `str_to_bool`, `str_to_int64`, `str_to_uint64` and `str_to_bytes` (base64), and the entity
//! decoding of `decode_text`.
//!
//! The C functions take a length and a `WS_HEAP`. Here each takes a slice and returns a value,
//! so the heap and its allocation-failure paths do not appear.

use crate::Error;

/// The named entities that `decode_text` accepts, each with the text it stands for.
const NAMED_ENTITIES: [(&[u8], &[u8]); 5] = [
    (b"lt;", b"<"),
    (b"gt;", b">"),
    (b"quot;", b"\""),
    (b"amp;", b"&"),
    (b"apos;", b"'"),
];

/// `read_isspace`: the only bytes that the scalar parsers trim from both ends.
fn is_space(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\r' | b'\n')
}

pub(crate) fn trim(text: &[u8]) -> &[u8] {
    let start = text
        .iter()
        .position(|&byte| !is_space(byte))
        .unwrap_or(text.len());
    let end = text
        .iter()
        .rposition(|&byte| !is_space(byte))
        .map_or(start, |last| last + 1);
    &text[start..end]
}

fn decimal_digit(byte: u8) -> Result<u8, Error> {
    if byte.is_ascii_digit() {
        Ok(byte - b'0')
    } else {
        Err(Error::InvalidFormat)
    }
}

/// `str_to_bool`: accepts exactly `true`, `1`, `false` or `0`. Whitespace is not trimmed.
pub fn str_to_bool(text: &[u8]) -> Result<bool, Error> {
    match text {
        b"true" | b"1" => Ok(true),
        b"false" | b"0" => Ok(false),
        _ => Err(Error::InvalidFormat),
    }
}

/// Splits the optional minus sign off the trimmed text, and rejects text with no digits.
fn signed_digits(text: &[u8]) -> Result<(bool, &[u8]), Error> {
    let text = trim(text);
    let (negative, digits) = match text.strip_prefix(b"-") {
        Some(rest) => (true, rest),
        None => (false, text),
    };
    if digits.is_empty() {
        return Err(Error::InvalidFormat);
    }
    Ok((negative, digits))
}

/// `str_to_int64`: parses a signed decimal number in `min..=max`. The digit check comes before
/// the range check for each character, so `99x` with a maximum of 9 overflows at the second digit,
/// before the bad character is reached.
pub fn str_to_int64(text: &[u8], min: i64, max: i64) -> Result<i64, Error> {
    let (negative, digits) = signed_digits(text)?;
    let (min, max) = (i128::from(min), i128::from(max));
    let mut value: i128 = 0;
    for &byte in digits {
        let digit = i128::from(decimal_digit(byte)?);
        let signed = if negative { -digit } else { digit };
        value = value * 10 + signed;
        if (negative && value < min) || (!negative && value > max) {
            return Err(Error::NumericOverflow);
        }
    }
    Ok(value as i64)
}

/// `str_to_uint64`: parses an unsigned decimal number no larger than `max`.
pub fn str_to_uint64(text: &[u8], max: u64) -> Result<u64, Error> {
    let digits = trim(text);
    if digits.is_empty() {
        return Err(Error::InvalidFormat);
    }
    let max = u128::from(max);
    let mut value: u128 = 0;
    for &byte in digits {
        value = value * 10 + u128::from(decimal_digit(byte)?);
        if value > max {
            return Err(Error::NumericOverflow);
        }
    }
    Ok(value as u64)
}

/// `str_to_bytes`: decodes base64 text. The length after trimming must be a multiple of four.
/// Text that is not valid base64 gives an empty result, as in Wine.
pub fn str_to_bytes(text: &[u8]) -> Result<Vec<u8>, Error> {
    let text = trim(text);
    if !text.len().is_multiple_of(4) {
        return Err(Error::InvalidFormat);
    }
    Ok(decode_base64(text).unwrap_or_default())
}

/// Decodes whole quads, then the last quad by its padding. The last character of a quad that
/// ends in `=` is never checked, as in `decode_base64` in `reader.c`.
fn decode_base64(mut text: &[u8]) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    while text.len() > 4 {
        let quad = [
            sextet(text[0])?,
            sextet(text[1])?,
            sextet(text[2])?,
            sextet(text[3])?,
        ];
        push_bytes(&mut out, quad, 3);
        text = &text[4..];
    }
    match *text {
        [a, b, b'=', _] => push_bytes(&mut out, [sextet(a)?, sextet(b)?, 0, 0], 1),
        [a, b, c, b'='] => push_bytes(&mut out, [sextet(a)?, sextet(b)?, sextet(c)?, 0], 2),
        [a, b, c, d] => push_bytes(
            &mut out,
            [sextet(a)?, sextet(b)?, sextet(c)?, sextet(d)?],
            3,
        ),
        _ => {}
    }
    Some(out)
}

fn push_bytes(out: &mut Vec<u8>, [s0, s1, s2, s3]: [u8; 4], keep: usize) {
    let bytes = [(s0 << 2) | (s1 >> 4), (s1 << 4) | (s2 >> 2), (s2 << 6) | s3];
    out.extend_from_slice(&bytes[..keep]);
}

fn sextet(byte: u8) -> Option<u8> {
    match byte {
        b'A'..=b'Z' => Some(byte - b'A'),
        b'a'..=b'z' => Some(byte - b'a' + 26),
        b'0'..=b'9' => Some(byte - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

/// `decode_text`: replaces the entities in `text` with the characters they stand for. An `&`
/// that does not start a known entity is an error.
pub fn decode_text(text: &[u8]) -> Result<Vec<u8>, Error> {
    let mut out = Vec::with_capacity(text.len());
    let mut rest = text;
    while let Some((&byte, after)) = rest.split_first() {
        if byte == b'&' {
            let (decoded, next) = entity(after).ok_or(Error::InvalidFormat)?;
            out.extend_from_slice(&decoded);
            rest = next;
        } else {
            out.push(byte);
            rest = after;
        }
    }
    Ok(out)
}

/// Decodes the entity whose text follows the `&`. Returns the UTF-8 bytes and the rest.
fn entity(text: &[u8]) -> Option<(Vec<u8>, &[u8])> {
    NAMED_ENTITIES
        .iter()
        .find_map(|(name, value)| text.strip_prefix(*name).map(|rest| (value.to_vec(), rest)))
        .or_else(|| numeric_entity(text))
}

/// A character reference: `#` then decimal digits, or `#x` then hexadecimal digits, then `;`.
/// Decimal references may have leading zeros, which do not count toward the digit limit.
fn numeric_entity(text: &[u8]) -> Option<(Vec<u8>, &[u8])> {
    let body = text.strip_prefix(b"#")?;
    let (value, rest) = match body.strip_prefix(b"x") {
        Some(hex) => code_point(hex, 16, 6)?,
        None => {
            let zeros = body.iter().take_while(|&&byte| byte == b'0').count();
            code_point(&body[zeros..], 10, 7)?
        }
    };
    Some((codepoint_to_utf8(value)?, rest))
}

/// Reads up to `max_digits` digits in `radix`, then the `;` that ends them.
fn code_point(text: &[u8], radix: u32, max_digits: usize) -> Option<(u32, &[u8])> {
    let run = text
        .iter()
        .take_while(|&&byte| char::from(byte).is_digit(radix))
        .count();
    if run == 0 || run > max_digits {
        return None;
    }
    let value = text[..run].iter().fold(0, |value, &byte| {
        value * radix + char::from(byte).to_digit(radix).unwrap_or(0)
    });
    let rest = text[run..].strip_prefix(b";")?;
    Some((value, rest))
}

/// `codepoint_to_utf8`: the UTF-8 bytes of a character reference. Zero, the surrogates, the
/// noncharacters U+FFFE and U+FFFF, and values past U+10FFFF are rejected.
pub fn codepoint_to_utf8(value: u32) -> Option<Vec<u8>> {
    char::from_u32(value)
        .filter(|&c| !matches!(c, '\0' | '\u{FFFE}' | '\u{FFFF}'))
        .map(|c| c.to_string().into_bytes())
}
