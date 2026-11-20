//! Writer escaping and character entities from writer.c, over UTF-16 code units.

use crate::chars::{is_char, is_high_surrogate, is_low_surrogate};
use crate::{c_str, WC_E_XMLCHARACTER, WR_E_INVALIDSURROGATEPAIR};

const DASH: u16 = 0x2d;
const SPACE: u16 = 0x20;

/// `writer_get_next_write_count`: the number of units in the character at the start of
/// `text`. An empty text reads as NUL, which is not a character.
pub fn next_write_count(text: &[u16]) -> Result<usize, i32> {
    let first = text.first().copied().unwrap_or(0);
    if !is_char(first) {
        return Err(WC_E_XMLCHARACTER);
    }
    if is_high_surrogate(first) {
        match text.get(1) {
            Some(&low) if is_low_surrogate(low) => Ok(2),
            _ => Err(WR_E_INVALIDSURROGATEPAIR),
        }
    } else if is_low_surrogate(first) {
        Err(WR_E_INVALIDSURROGATEPAIR)
    } else {
        Ok(1)
    }
}

/// `write_escaped_string` for a counted text. `<`, `&` and `>` are written as entities and
/// every other character is copied. On an error, the output written so far is kept, as in C.
pub fn escape_chars(text: &[u16], out: &mut Vec<u16>) -> Result<(), i32> {
    let mut rest = text;
    while !rest.is_empty() {
        let count = next_write_count(rest)?;
        match rest[0] {
            0x3c => out.extend("&lt;".encode_utf16()),
            0x26 => out.extend("&amp;".encode_utf16()),
            0x3e => out.extend("&gt;".encode_utf16()),
            _ => out.extend_from_slice(&rest[..count]),
        }
        rest = &rest[count..];
    }
    Ok(())
}

/// `write_escaped_string` for a NUL-terminated text, which stops at the first NUL.
pub fn escape_string(text: &[u16], out: &mut Vec<u16>) -> Result<(), i32> {
    escape_chars(c_str(text), out)
}

/// `xmlwriter_WriteCharEntity`: a hexadecimal character reference. The character is not checked.
pub fn char_entity(ch: u16) -> Vec<u16> {
    format!("&#x{ch:x};").encode_utf16().collect()
}

/// `xmlwriter_WriteSurrogateCharEntity`: a hexadecimal reference to the code point of a
/// surrogate pair.
pub fn surrogate_entity(high: u16, low: u16) -> Result<Vec<u16>, i32> {
    if !(is_high_surrogate(high) && is_low_surrogate(low)) {
        return Err(WC_E_XMLCHARACTER);
    }
    let code_point = (u32::from(high) - 0xd800) * 0x400 + (u32::from(low) - 0xdc00) + 0x10000;
    Ok(format!("&#x{code_point:X};").encode_utf16().collect())
}

/// `xmlwriter_WriteComment`: the text between `<!--` and `-->`. A space follows each '-' that
/// starts a "--" pair, and a space follows a trailing '-', so the comment is well formed. The
/// text ends at the first NUL.
pub fn write_comment(text: &[u16]) -> Vec<u16> {
    let text = c_str(text);
    let mut out: Vec<u16> = "<!--".encode_utf16().collect();
    for (i, &unit) in text.iter().enumerate() {
        out.push(unit);
        if unit == DASH && text.get(i + 1) == Some(&DASH) {
            out.push(SPACE);
        }
    }
    if text.last() == Some(&DASH) {
        out.push(SPACE);
    }
    out.extend("-->".encode_utf16());
    out
}
