//! Name checks from Wine's `dlls/dnsapi/name.c`: `DnsValidateName_W` and
//! `DnsNameCompare_W`.
//!
//! Names are UTF-16 code units, as Wine's `WCHAR` strings are. A name ends at
//! its first zero unit, and a missing name is `None`.

use crate::{until_nul, Error};

const HAS_EXTENDED: u32 = 0x0001;
const HAS_NUMERIC: u32 = 0x0002;
const HAS_NON_NUMERIC: u32 = 0x0004;
const HAS_DOT: u32 = 0x0008;
const HAS_DOT_DOT: u32 = 0x0010;
const HAS_SPACE: u32 = 0x0020;
const HAS_INVALID: u32 = 0x0040;
const HAS_ASTERISK: u32 = 0x0080;
const HAS_UNDERSCORE: u32 = 0x0100;
const HAS_LONG_LABEL: u32 = 0x0200;

/// The ASCII characters that `DnsValidateName_W` rejects outright.
const INVALID: &[u8] = b"{|}~[\\]^':;<=>?@!\"#$%&`()+/,";

const DOT: u16 = b'.' as u16;

/// The `DNS_NAME_FORMAT` values that `DnsValidateName_W` checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameFormat {
    /// `DnsNameDomain`.
    Domain,
    /// `DnsNameDomainLabel`.
    DomainLabel,
    /// `DnsNameHostnameFull`.
    HostnameFull,
    /// `DnsNameHostnameLabel`.
    HostnameLabel,
    /// `DnsNameWildcard`.
    Wildcard,
    /// `DnsNameSrvRecord`.
    SrvRecord,
}

impl NameFormat {
    /// The format for a raw `DNS_NAME_FORMAT` value, or `None` for a value that
    /// Wine does not check.
    pub fn from_raw(value: u32) -> Option<Self> {
        Some(match value {
            0 => NameFormat::Domain,
            1 => NameFormat::DomainLabel,
            2 => NameFormat::HostnameFull,
            3 => NameFormat::HostnameLabel,
            4 => NameFormat::Wildcard,
            5 => NameFormat::SrvRecord,
            _ => return None,
        })
    }
}

/// `DnsValidateName_W`: checks `name` against a raw `DNS_NAME_FORMAT`. The
/// general checks apply to every format, and an unknown format passes after
/// them, as in Wine.
pub fn validate_name(name: Option<&[u16]>, format: u32) -> Result<(), Error> {
    let name = until_nul(name.ok_or(Error::InvalidName)?);
    let state = scan(name);
    if name.is_empty()
        || name.len() > 255
        || state & HAS_LONG_LABEL != 0
        || state & HAS_DOT_DOT != 0
        || (unit(name, 0) == DOT && unit(name, 1) != 0)
    {
        return Err(Error::InvalidName);
    }
    match NameFormat::from_raw(format) {
        Some(NameFormat::Domain | NameFormat::HostnameFull) => check_hostname(state),
        Some(NameFormat::DomainLabel | NameFormat::HostnameLabel) => {
            if state & HAS_DOT != 0 {
                return Err(Error::InvalidName);
            }
            check_hostname(state)
        }
        Some(NameFormat::Wildcard) => check_wildcard(name, state),
        Some(NameFormat::SrvRecord) => check_srv(name, state),
        None => Ok(()),
    }
}

/// `DnsNameCompare_W`: compares two names ignoring case and trailing dots. Two
/// missing names are equal, and a missing name never equals a present one.
pub fn name_compare(a: Option<&[u16]>, b: Option<&[u16]>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => {
            let a = trim_dots(until_nul(a));
            let b = trim_dots(until_nul(b));
            a.len() == b.len() && a.iter().zip(b).all(|(&x, &y)| upcase(x) == upcase(y))
        }
        _ => false,
    }
}

/// Classifies the characters of `name` into the `HAS_*` flags.
fn scan(name: &[u16]) -> u32 {
    let mut state = 0;
    let mut label = 0;
    for (i, &c) in name.iter().enumerate() {
        if c == DOT {
            label = 0;
            state |= HAS_DOT;
            if unit(name, i + 1) == DOT {
                state |= HAS_DOT_DOT;
            }
        } else if !(u16::from(b'0')..=u16::from(b'9')).contains(&c) {
            state |= HAS_NON_NUMERIC;
        } else {
            state |= HAS_NUMERIC;
        }
        if label > 62 {
            state |= HAS_LONG_LABEL;
        }
        label += 1;
        if is_invalid(c) {
            state |= HAS_INVALID;
        } else if c > 127 {
            state |= HAS_EXTENDED;
        } else if c == u16::from(b' ') {
            state |= HAS_SPACE;
        } else if c == u16::from(b'_') {
            state |= HAS_UNDERSCORE;
        } else if c == u16::from(b'*') {
            state |= HAS_ASTERISK;
        }
    }
    state
}

/// The unit at `i`, or zero past the end, as a C string reads.
fn unit(name: &[u16], i: usize) -> u16 {
    name.get(i).copied().unwrap_or(0)
}

fn is_invalid(c: u16) -> bool {
    u8::try_from(c).is_ok_and(|b| INVALID.contains(&b))
}

fn all_numeric(state: u32) -> bool {
    state & HAS_NON_NUMERIC == 0 && state & HAS_NUMERIC != 0
}

fn check_hostname(state: u32) -> Result<(), Error> {
    if state & (HAS_EXTENDED | HAS_UNDERSCORE) != 0 {
        return Err(Error::NonRfcName);
    }
    if state & (HAS_SPACE | HAS_INVALID | HAS_ASTERISK) != 0 {
        return Err(Error::InvalidNameChar);
    }
    Ok(())
}

fn check_wildcard(name: &[u16], state: u32) -> Result<(), Error> {
    if all_numeric(state) {
        return Err(Error::InvalidName);
    }
    if unit(name, 0) != u16::from(b'*') {
        return Err(Error::InvalidName);
    }
    let second = unit(name, 1);
    if second != 0 && second != DOT {
        return Err(Error::InvalidNameChar);
    }
    if state & (HAS_EXTENDED | HAS_SPACE | HAS_INVALID) != 0 {
        return Err(Error::InvalidName);
    }
    Ok(())
}

fn check_srv(name: &[u16], state: u32) -> Result<(), Error> {
    if all_numeric(state) {
        return Err(Error::InvalidName);
    }
    if unit(name, 0) != u16::from(b'_') {
        return Err(Error::InvalidName);
    }
    if state & HAS_UNDERSCORE != 0 && unit(name, 1) == 0 {
        return Err(Error::NonRfcName);
    }
    if state & (HAS_EXTENDED | HAS_SPACE | HAS_INVALID) != 0 {
        return Err(Error::InvalidName);
    }
    Ok(())
}

fn trim_dots(mut name: &[u16]) -> &[u16] {
    while let [rest @ .., DOT] = name {
        name = rest;
    }
    name
}

/// The uppercase form of `u` when it maps to one code unit. Lone surrogates
/// and multi-character mappings keep their value.
fn upcase(u: u16) -> u16 {
    let Some(c) = char::from_u32(u32::from(u)) else {
        return u;
    };
    let mut upper = c.to_uppercase();
    match (upper.next(), upper.next()) {
        (Some(up), None) => u16::try_from(u32::from(up)).unwrap_or(u),
        _ => u,
    }
}
