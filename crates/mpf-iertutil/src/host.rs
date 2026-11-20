//! The IPv4 and IPvFuture host parsers of `uri.c`.

use crate::authority::parse_port;
use crate::chars::{is_auth_delim, is_hexdigit, is_num, is_subdelim, is_unreserved};
use crate::scheme::URL_SCHEME_UNKNOWN;
use crate::{at, ParseData, COLON, DOT, URI_HOST_IPV4, URI_HOST_UNKNOWN};

/// Checks one `dec-octet` at `pos`. Wine's test for an octet above 255 is
/// `c1 >= '2' && c2 >= '5' && c3 >= '5'`, kept as written. It rejects `255`
/// and accepts `300` and `260`.
pub fn check_dec_octet(s: &[u16], pos: &mut usize) -> bool {
    let p = *pos;
    if !is_num(at(s, p)) {
        return false;
    }
    if !is_num(at(s, p + 1)) {
        *pos = p + 1;
        return true;
    }
    if !is_num(at(s, p + 2)) {
        *pos = p + 2;
        return true;
    }
    let (c1, c2, c3) = (at(s, p), at(s, p + 1), at(s, p + 2));
    if c1 >= u16::from(b'2') && c2 >= u16::from(b'5') && c3 >= u16::from(b'5') {
        return false;
    }
    *pos = p + 3;
    true
}

/// Checks for an implicit IPv4 address: a run of digits whose value is at most
/// `u32::MAX`.
pub fn check_implicit_ipv4(s: &[u16], pos: &mut usize) -> bool {
    let start = *pos;
    let mut p = start;
    let mut value: u64 = 0;
    while is_num(at(s, p)) {
        value = value * 10 + u64::from(at(s, p) - u16::from(b'0'));
        if value > u64::from(u32::MAX) {
            return false;
        }
        p += 1;
    }
    if p == start {
        return false;
    }
    *pos = p;
    true
}

/// Checks for `dec-octet "." dec-octet "." dec-octet "." dec-octet`. Without
/// `strict`, the address may stop after two or three octets.
pub fn check_ipv4address(s: &[u16], pos: &mut usize, strict: bool) -> bool {
    let mut p = *pos;
    for octet in 1..=4 {
        if !check_dec_octet(s, &mut p) {
            return false;
        }
        if octet == 4 {
            break;
        }
        if at(s, p) != DOT {
            if octet == 1 || strict {
                return false;
            }
            break;
        }
        p += 1;
    }
    *pos = p;
    true
}

/// Parses an IPv4 host at `pos`, either dotted or implicit, followed by an
/// optional port. For an unknown scheme with a `:` but no port digits, the
/// host keeps the `:`.
pub fn parse_ipv4address<'a>(s: &'a [u16], pos: &mut usize, data: &mut ParseData<'a>) -> bool {
    let is_unknown = data.scheme_type == URL_SCHEME_UNKNOWN;
    let start = *pos;
    let mut p = start;
    if !check_ipv4address(s, &mut p, false) && !check_implicit_ipv4(s, &mut p) {
        data.host = None;
        return false;
    }
    data.host_type = URI_HOST_IPV4;
    if at(s, p) == COLON {
        let colon = p;
        p += 1;
        if !parse_port(s, &mut p, data) {
            data.host = None;
            return false;
        }
        let end = if !data.has_port && is_unknown {
            p
        } else {
            colon
        };
        data.host = Some(&s[start..end]);
    } else if is_auth_delim(at(s, p), !is_unknown) {
        data.host = Some(&s[start..p]);
    } else {
        data.host = None;
        return false;
    }
    *pos = p;
    true
}

fn is_ipvfuture_char(c: u16) -> bool {
    is_unreserved(c) || is_subdelim(c) || c == COLON
}

/// Parses `"v" 1*HEXDIG "." 1*(unreserved / sub-delims / ":")` at `pos`. Sets
/// the host type to unknown and leaves `data.host` alone.
pub fn parse_ipvfuture<'a>(s: &'a [u16], pos: &mut usize, data: &mut ParseData<'a>) -> bool {
    let start = *pos;
    let c = at(s, start);
    if c != u16::from(b'v') && c != u16::from(b'V') {
        return false;
    }
    let mut p = start + 1;
    if !is_hexdigit(at(s, p)) {
        return false;
    }
    p += 1;
    while is_hexdigit(at(s, p)) {
        p += 1;
    }
    if at(s, p) != DOT {
        return false;
    }
    p += 1;
    if !is_ipvfuture_char(at(s, p)) {
        return false;
    }
    p += 1;
    while is_ipvfuture_char(at(s, p)) {
        p += 1;
    }
    data.host_type = URI_HOST_UNKNOWN;
    *pos = p;
    true
}
