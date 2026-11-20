//! The character classes of `uri.c`, over UTF-16 code units, and the
//! percent-encoding and path-shape checks that use them.

use crate::scheme::URL_SCHEME_FILE;
use crate::{at, BACKSLASH, COLON, HASH, NUL, PERCENT, QUESTION, SLASH};

fn in_range(c: u16, lo: u8, hi: u8) -> bool {
    (u16::from(lo)..=u16::from(hi)).contains(&c)
}

fn is_one_of(c: u16, set: &[u8]) -> bool {
    c < 0x80 && set.contains(&(c as u8))
}

pub fn is_ascii(c: u16) -> bool {
    c < 0x80
}

pub fn is_alpha(c: u16) -> bool {
    in_range(c, b'a', b'z') || in_range(c, b'A', b'Z')
}

pub fn is_num(c: u16) -> bool {
    in_range(c, b'0', b'9')
}

pub fn is_hexdigit(c: u16) -> bool {
    in_range(c, b'a', b'f') || in_range(c, b'A', b'F') || is_num(c)
}

/// `unreserved = ALPHA / DIGIT / "-" / "." / "_" / "~"`
pub fn is_unreserved(c: u16) -> bool {
    is_alpha(c) || is_num(c) || is_one_of(c, b"-._~")
}

/// `sub-delims = "!" / "$" / "&" / "'" / "(" / ")" / "*" / "+" / "," / ";" / "="`
pub fn is_subdelim(c: u16) -> bool {
    is_one_of(c, b"!$&'()*+,;=")
}

/// The characters that end the authority. A backslash ends it only when
/// `accept_backslash` is set, which is the case for known schemes.
pub fn is_auth_delim(c: u16, accept_backslash: bool) -> bool {
    c == HASH || c == SLASH || c == QUESTION || c == NUL || (accept_backslash && c == BACKSLASH)
}

pub fn is_slash(c: u16) -> bool {
    c == SLASH || c == BACKSLASH
}

/// The characters that end a path. A `#` ends it for every scheme but file.
pub fn is_path_delim(scheme: i32, c: u16) -> bool {
    c == NUL || (c == HASH && scheme != URL_SCHEME_FILE) || c == QUESTION
}

pub fn is_forbidden_dos_path_char(c: u16) -> bool {
    c == u16::from(b'<') || c == u16::from(b'>') || c == u16::from(b'"')
}

/// Whether the units at `i` start a UNC path, `\\`.
pub fn is_unc_path(s: &[u16], i: usize) -> bool {
    at(s, i) == BACKSLASH && at(s, i + 1) == BACKSLASH
}

/// Whether the units at `i` start an implicit file path: a UNC path, or a
/// drive letter followed by `:`.
pub fn is_implicit_file_path(s: &[u16], i: usize) -> bool {
    is_unc_path(s, i) || (is_alpha(at(s, i)) && at(s, i + 1) == COLON)
}

/// `pct-encoded = "%" HEXDIG HEXDIG`. Moves `pos` past the octet on success.
pub fn check_pct_encoded(s: &[u16], pos: &mut usize) -> bool {
    let p = *pos;
    if at(s, p) != PERCENT || !is_hexdigit(at(s, p + 1)) || !is_hexdigit(at(s, p + 2)) {
        return false;
    }
    *pos = p + 3;
    true
}
