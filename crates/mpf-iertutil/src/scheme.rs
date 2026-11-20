//! The scheme parser of `uri.c`: the scheme name, the `URL_SCHEME` value that
//! the name maps to, and the test for a hierarchical URI after the scheme.

use crate::chars::{is_alpha, is_implicit_file_path, is_num};
use crate::flags::{
    ALLOW_NULL_TERM_SCHEME, URI_CREATE_ALLOW_IMPLICIT_FILE_SCHEME,
    URI_CREATE_ALLOW_IMPLICIT_WILDCARD_SCHEME, URI_CREATE_ALLOW_RELATIVE,
};
use crate::{
    at, eq_ignore_ascii_case, has, ParseData, BACKSLASH, COLON, DOT, MINUS, NUL, PLUS, SLASH, STAR,
};

pub const URL_SCHEME_UNKNOWN: i32 = 0;
pub const URL_SCHEME_FTP: i32 = 1;
pub const URL_SCHEME_HTTP: i32 = 2;
pub const URL_SCHEME_GOPHER: i32 = 3;
pub const URL_SCHEME_MAILTO: i32 = 4;
pub const URL_SCHEME_NEWS: i32 = 5;
pub const URL_SCHEME_NNTP: i32 = 6;
pub const URL_SCHEME_TELNET: i32 = 7;
pub const URL_SCHEME_WAIS: i32 = 8;
pub const URL_SCHEME_FILE: i32 = 9;
pub const URL_SCHEME_MK: i32 = 10;
pub const URL_SCHEME_HTTPS: i32 = 11;
pub const URL_SCHEME_SHELL: i32 = 12;
pub const URL_SCHEME_SNEWS: i32 = 13;
pub const URL_SCHEME_LOCAL: i32 = 14;
pub const URL_SCHEME_JAVASCRIPT: i32 = 15;
pub const URL_SCHEME_VBSCRIPT: i32 = 16;
pub const URL_SCHEME_ABOUT: i32 = 17;
pub const URL_SCHEME_RES: i32 = 18;
pub const URL_SCHEME_MSSHELLROOTED: i32 = 19;
pub const URL_SCHEME_MSSHELLIDLIST: i32 = 20;
pub const URL_SCHEME_MSHELP: i32 = 21;
pub const URL_SCHEME_MSSHELLDEVICE: i32 = 22;
pub const URL_SCHEME_WILDCARD: i32 = 23;

/// The schemes that have a `URL_SCHEME` value, in the order `uri.c` searches them.
const RECOGNIZED_SCHEMES: [(&str, i32); 22] = [
    ("ftp", URL_SCHEME_FTP),
    ("http", URL_SCHEME_HTTP),
    ("gopher", URL_SCHEME_GOPHER),
    ("mailto", URL_SCHEME_MAILTO),
    ("news", URL_SCHEME_NEWS),
    ("nntp", URL_SCHEME_NNTP),
    ("telnet", URL_SCHEME_TELNET),
    ("wais", URL_SCHEME_WAIS),
    ("file", URL_SCHEME_FILE),
    ("mk", URL_SCHEME_MK),
    ("https", URL_SCHEME_HTTPS),
    ("shell", URL_SCHEME_SHELL),
    ("snews", URL_SCHEME_SNEWS),
    ("local", URL_SCHEME_LOCAL),
    ("javascript", URL_SCHEME_JAVASCRIPT),
    ("vbscript", URL_SCHEME_VBSCRIPT),
    ("about", URL_SCHEME_ABOUT),
    ("res", URL_SCHEME_RES),
    ("ms-shell-rooted", URL_SCHEME_MSSHELLROOTED),
    ("ms-shell-idlist", URL_SCHEME_MSSHELLIDLIST),
    ("hcp", URL_SCHEME_MSHELP),
    ("*", URL_SCHEME_WILDCARD),
];

const FILE_NAME: &[u16] = &[b'f' as u16, b'i' as u16, b'l' as u16, b'e' as u16];
const WILDCARD_NAME: &[u16] = &[STAR];

/// Parses `scheme ":"` at `pos`. A scheme may also end at the end of the input
/// when `ALLOW_NULL_TERM_SCHEME` is set, and `pos` then moves one past it.
pub fn parse_scheme_name<'a>(
    s: &'a [u16],
    pos: &mut usize,
    data: &mut ParseData<'a>,
    extras: u32,
) -> bool {
    let start = *pos;
    let mut p = start;
    data.scheme = None;
    while at(s, p) != NUL {
        let c = at(s, p);
        if c == STAR && p == start {
            p += 1;
            break;
        }
        if !is_num(c) && !is_alpha(c) && c != PLUS && c != MINUS && c != DOT {
            break;
        }
        p += 1;
    }
    if p == start {
        return false;
    }
    let terminated = at(s, p) == COLON || (has(extras, ALLOW_NULL_TERM_SCHEME) && at(s, p) == NUL);
    if !terminated {
        return false;
    }
    data.scheme = Some(&s[start..p]);
    *pos = p + 1;
    true
}

/// Sets `data.scheme_type` from `data.scheme`. A scheme that is not in the
/// table is `URL_SCHEME_UNKNOWN`. A relative URI has no scheme, which is also
/// `URL_SCHEME_UNKNOWN`. Any other URI without a scheme is an error.
pub fn parse_scheme_type(data: &mut ParseData<'_>) -> bool {
    if let Some(scheme) = data.scheme {
        data.scheme_type = RECOGNIZED_SCHEMES
            .iter()
            .find(|(name, _)| eq_ignore_ascii_case(name, scheme))
            .map_or(URL_SCHEME_UNKNOWN, |&(_, value)| value);
        true
    } else if data.is_relative {
        data.scheme_type = URL_SCHEME_UNKNOWN;
        true
    } else {
        false
    }
}

/// Parses the scheme at `pos`. When there is none, the flags decide whether
/// the URI gets an implicit scheme, is relative, or is rejected.
pub fn parse_scheme<'a>(
    s: &'a [u16],
    pos: &mut usize,
    data: &mut ParseData<'a>,
    flags: u32,
    extras: u32,
) -> bool {
    if is_implicit_file_path(s, *pos) {
        if !has(flags, URI_CREATE_ALLOW_IMPLICIT_FILE_SCHEME) {
            return false;
        }
        data.scheme = Some(FILE_NAME);
        data.has_implicit_scheme = true;
    } else if !parse_scheme_name(s, pos, data, extras) {
        if has(flags, URI_CREATE_ALLOW_IMPLICIT_WILDCARD_SCHEME) {
            data.scheme = Some(WILDCARD_NAME);
            data.has_implicit_scheme = true;
        } else if has(flags, URI_CREATE_ALLOW_RELATIVE) {
            data.is_relative = true;
        } else {
            return false;
        }
    }
    parse_scheme_type(data)
}

/// The schemes that Windows treats as hierarchical.
pub fn is_hierarchical_scheme(scheme: i32) -> bool {
    matches!(
        scheme,
        URL_SCHEME_HTTP
            | URL_SCHEME_FTP
            | URL_SCHEME_GOPHER
            | URL_SCHEME_NNTP
            | URL_SCHEME_TELNET
            | URL_SCHEME_WAIS
            | URL_SCHEME_FILE
            | URL_SCHEME_HTTPS
            | URL_SCHEME_RES
    )
}

/// Consumes `//` at `pos` if it is there.
fn check_hierarchical(s: &[u16], pos: &mut usize) -> bool {
    let p = *pos;
    if at(s, p) != SLASH || at(s, p + 1) != SLASH {
        return false;
    }
    *pos = p + 2;
    true
}

/// Whether the URI at `pos` is hierarchical: a wildcard or implicit file
/// scheme, a `\\` after a hierarchical scheme, or a `//` after any scheme
/// but mailto. Consumes the marker on success.
pub fn is_hierarchical_uri(s: &[u16], pos: &mut usize, data: &ParseData<'_>) -> bool {
    let p = *pos;
    if data.scheme_type == URL_SCHEME_WILDCARD {
        return true;
    }
    if data.scheme_type == URL_SCHEME_FILE && data.has_implicit_scheme {
        return true;
    }
    if is_hierarchical_scheme(data.scheme_type)
        && at(s, p) == BACKSLASH
        && at(s, p + 1) == BACKSLASH
    {
        *pos = p + 2;
        return true;
    }
    data.scheme_type != URL_SCHEME_MAILTO && check_hierarchical(s, pos)
}
