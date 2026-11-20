//! Port of the deterministic logic in Wine's `dlls/iertutil/uri.c`: the scheme
//! parser ([`scheme`]), the `Uri_CREATE` flag checks ([`flags`]), the userinfo
//! and port parsers ([`authority`]), the IPv4 and IPvFuture host parsers
//! ([`host`]), the path, query and fragment parsers and the dot segment and
//! file extension searches ([`path`]), and the domain name search ([`domain`]).
//!
//! Skipped: `parse_ipv6address`, which calls `RtlIpv6StringToAddressW`,
//! `parse_reg_name`, which calls `IdnToUnicode`, and the functions that reach
//! them (`parse_ip_literal`, `parse_host`, `parse_authority`, `parse_hierpart`
//! and `parse_uri`). Also skipped: canonicalization and URI display,
//! `pre_process_uri`, the `Uri` COM object, and the helpers that only that code
//! reaches (`ui2str`, `is_default_port`, `hex_to_int`, `decode_pct_val`,
//! `pct_encode_val`, `is_gendelim`, `is_reserved`, `is_drive_path`, `strdupwn`).
//!
//! Deviations: strings are UTF-16 code units (`&[u16]`) and a position is an
//! index into the slice. Reads past the end yield NUL, as Wine's NUL-terminated
//! strings do. A `ParseData` field holds a sub-slice of the input where Wine
//! keeps a pointer and a length. Case-insensitive comparisons fold ASCII
//! letters only. `check_dec_octet` keeps Wine's test `c1 >= '2' && c2 >= '5'
//! && c3 >= '5'`, which rejects `255` but accepts `300` and `260`.
//!
//! Dead code dropped: the length check in `find_domain_name` for a host that
//! starts with `.` (the host is at least four units long there), the `'@'`
//! recheck in `parse_userinfo` (the username and password scans always stop on
//! `'@'`), and the empty-path branch in `parse_path_hierarchical` (the scan
//! always consumes one unit).
//!
//! `find_domain_name` and `is_hierarchical_scheme` also exist in `mpf-urlmon`.
//! This crate is self-contained and does not depend on that one.

pub mod authority;
pub mod chars;
pub mod domain;
pub mod flags;
pub mod host;
pub mod path;
pub mod scheme;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub const URI_HOST_UNKNOWN: i32 = 0;
pub const URI_HOST_DNS: i32 = 1;
pub const URI_HOST_IPV4: i32 = 2;
pub const URI_HOST_IPV6: i32 = 3;
pub const URI_HOST_IDN: i32 = 4;

const NUL: u16 = 0;
const PERCENT: u16 = b'%' as u16;
const COLON: u16 = b':' as u16;
const AT: u16 = b'@' as u16;
const DOT: u16 = b'.' as u16;
const SLASH: u16 = b'/' as u16;
const BACKSLASH: u16 = b'\\' as u16;
const HASH: u16 = b'#' as u16;
const QUESTION: u16 = b'?' as u16;
const STAR: u16 = b'*' as u16;
const PLUS: u16 = b'+' as u16;
const MINUS: u16 = b'-' as u16;

/// The parsed parts of a URI, as Wine's `parse_data` holds them. Each part is
/// a sub-slice of the input, or `None` when the URI has no such part.
#[derive(Default)]
pub struct ParseData<'a> {
    pub is_relative: bool,
    pub has_implicit_scheme: bool,
    pub must_have_path: bool,
    pub scheme: Option<&'a [u16]>,
    pub scheme_type: i32,
    pub username: Option<&'a [u16]>,
    pub password: Option<&'a [u16]>,
    pub host: Option<&'a [u16]>,
    pub host_type: i32,
    pub has_port: bool,
    pub port: Option<&'a [u16]>,
    pub port_value: u32,
    pub path: Option<&'a [u16]>,
    pub query: Option<&'a [u16]>,
    pub fragment: Option<&'a [u16]>,
}

/// The unit at `i`, or NUL past the end of `s`.
fn at(s: &[u16], i: usize) -> u16 {
    s.get(i).copied().unwrap_or(NUL)
}

fn has(flags: u32, flag: u32) -> bool {
    flags & flag != 0
}

/// Folds an ASCII upper-case letter to lower case, as `StrCmpNIW` does.
fn fold(c: u16) -> u16 {
    if (u16::from(b'A')..=u16::from(b'Z')).contains(&c) {
        c + 32
    } else {
        c
    }
}

/// Compares an ASCII name with a string of units, ignoring ASCII case.
fn eq_ignore_ascii_case(name: &str, s: &[u16]) -> bool {
    name.len() == s.len()
        && name
            .bytes()
            .zip(s)
            .all(|(a, &b)| fold(u16::from(a)) == fold(b))
}
