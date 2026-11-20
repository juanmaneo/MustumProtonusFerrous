//! The userinfo and port parsers of `uri.c`: the username, the password and
//! the port that follows the host.

use crate::chars::{check_pct_encoded, is_auth_delim, is_num};
use crate::flags::{ALLOW_NULL_TERM_PASSWORD, ALLOW_NULL_TERM_USER_NAME};
use crate::scheme::URL_SCHEME_UNKNOWN;
use crate::{at, has, ParseData, AT, COLON, NUL, PERCENT};

/// Scans up to the first of `stops`. A `%` must start a valid escape, and a
/// known scheme also rejects any authority delimiter. A NUL ends the field
/// when `null_term` is set. Returns `None`, and leaves `pos` alone, on failure.
fn scan_field<'a>(
    s: &'a [u16],
    pos: &mut usize,
    known: bool,
    null_term: bool,
    stops: &[u16],
) -> Option<&'a [u16]> {
    let start = *pos;
    let mut p = start;
    while !stops.contains(&at(s, p)) {
        let c = at(s, p);
        if c == PERCENT {
            if check_pct_encoded(s, &mut p) {
                continue;
            }
            if known {
                return None;
            }
        } else if null_term && c == NUL {
            break;
        } else if is_auth_delim(c, known) {
            return None;
        }
        p += 1;
    }
    *pos = p;
    Some(&s[start..p])
}

/// Parses the username at `pos`, which ends at `:` or `@`.
pub fn parse_username<'a>(
    s: &'a [u16],
    pos: &mut usize,
    data: &mut ParseData<'a>,
    extras: u32,
) -> bool {
    let known = data.scheme_type != URL_SCHEME_UNKNOWN;
    let null_term = has(extras, ALLOW_NULL_TERM_USER_NAME);
    data.username = scan_field(s, pos, known, null_term, &[COLON, AT]);
    data.username.is_some()
}

/// Parses the password at `pos`, which ends at `@`.
pub fn parse_password<'a>(
    s: &'a [u16],
    pos: &mut usize,
    data: &mut ParseData<'a>,
    extras: u32,
) -> bool {
    let known = data.scheme_type != URL_SCHEME_UNKNOWN;
    let null_term = has(extras, ALLOW_NULL_TERM_PASSWORD);
    data.password = scan_field(s, pos, known, null_term, &[AT]);
    data.password.is_some()
}

/// Parses `username [":" password] "@"` at `pos`. Without the `@` the URI has
/// no userinfo, so `pos` and both fields are left as they were.
pub fn parse_userinfo<'a>(s: &'a [u16], pos: &mut usize, data: &mut ParseData<'a>) {
    let start = *pos;
    if !parse_username(s, pos, data, 0) {
        return;
    }
    if at(s, *pos) == COLON {
        *pos += 1;
        if !parse_password(s, pos, data, 0) {
            *pos = start;
            data.username = None;
            return;
        }
    }
    *pos += 1;
}

/// Parses the port digits at `pos`, which end at an authority delimiter. A
/// port above 65535 fails.
pub fn parse_port<'a>(s: &'a [u16], pos: &mut usize, data: &mut ParseData<'a>) -> bool {
    let known = data.scheme_type != URL_SCHEME_UNKNOWN;
    let start = *pos;
    let mut p = start;
    let mut port: u32 = 0;
    while !is_auth_delim(at(s, p), known) {
        let c = at(s, p);
        if !is_num(c) {
            data.port = None;
            return false;
        }
        port = port * 10 + u32::from(c - u16::from(b'0'));
        if port > u32::from(u16::MAX) {
            data.port = None;
            return false;
        }
        p += 1;
    }
    data.port_value = port;
    data.has_port = p > start;
    data.port = Some(&s[start..p]);
    *pos = p;
    true
}
