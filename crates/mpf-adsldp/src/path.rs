//! Port of `parse_path` from `adsldp.c`: splits an `LDAP:` path into its host, port and object.

use crate::{cstr, icase_cmp, HResult, E_ADS_BAD_PATHNAME};

const SCHEME: [u16; 5] = [
    b'L' as u16,
    b'D' as u16,
    b'A' as u16,
    b'P' as u16,
    b':' as u16,
];
const SLASH: u16 = b'/' as u16;
const COLON: u16 = b':' as u16;
const MINUS: u16 = b'-' as u16;
const PLUS: u16 = b'+' as u16;

/// `2^31`. Magnitudes are capped here, so the saturated value matches `ntdll_wcstol`.
const MAGNITUDE_LIMIT: i64 = 1 << 31;

/// The parts of an `LDAP:` path. The C code returns allocated BSTRs; these are slices of the input.
pub struct LdapPath<'a> {
    pub host: Option<&'a [u16]>,
    pub port: u32,
    pub object: Option<&'a [u16]>,
}

/// Splits `text` as `parse_path` does. The text ends at its first NUL.
pub fn parse_ldap_path(text: &[u16]) -> Result<LdapPath<'_>, HResult> {
    let text = cstr(text);
    let rest = match text.get(..SCHEME.len()) {
        Some(scheme) if icase_cmp(scheme, &SCHEME).is_eq() => &text[SCHEME.len()..],
        _ => return Err(E_ADS_BAD_PATHNAME),
    };
    if rest.is_empty() {
        return Ok(LdapPath {
            host: None,
            port: 0,
            object: None,
        });
    }
    let rest = rest
        .strip_prefix(&[SLASH, SLASH])
        .filter(|rest| !rest.is_empty())
        .ok_or(E_ADS_BAD_PATHNAME)?;

    let host_len = rest
        .iter()
        .position(|&unit| unit == COLON || unit == SLASH)
        .unwrap_or(rest.len());
    let (host, after_host) = rest.split_at(host_len);
    if host.is_empty() {
        return Err(E_ADS_BAD_PATHNAME);
    }
    let (port, tail) = match after_host.strip_prefix(&[COLON]) {
        Some(after_colon) => {
            let (value, used) = wcstol10(after_colon);
            (value as u32, &after_colon[used..])
        }
        None => (0, after_host),
    };
    let object = match tail {
        [] => None,
        [SLASH, object @ ..] if !object.is_empty() => Some(object),
        _ => return Err(E_ADS_BAD_PATHNAME),
    };
    Ok(LdapPath {
        host: Some(host),
        port,
        object,
    })
}

/// `ntdll_wcstol(text, &end, 10)`, saturated to `i32`. Returns the value and the number of units
/// it consumed. Without digits it returns 0 and consumes nothing, as the C code leaves `end` alone.
fn wcstol10(text: &[u16]) -> (i32, usize) {
    let mut i = text.iter().take_while(|&&unit| is_space(unit)).count();
    let sign = text.get(i).copied();
    let negative = sign == Some(MINUS);
    if negative || sign == Some(PLUS) {
        i += 1;
    }
    let digits_start = i;
    let mut magnitude = 0i64;
    while let Some(digit) = text.get(i).and_then(|&unit| decimal_digit(unit)) {
        magnitude = (magnitude * 10 + digit).min(MAGNITUDE_LIMIT);
        i += 1;
    }
    if i == digits_start {
        return (0, 0);
    }
    let value = if negative { -magnitude } else { magnitude };
    (
        value.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32,
        i,
    )
}

/// `ntdll_iswspace`: tab through carriage return, space, and no-break space.
fn is_space(unit: u16) -> bool {
    matches!(unit, 0x09..=0x0d | 0x20 | 0xa0)
}

fn decimal_digit(unit: u16) -> Option<i64> {
    match unit {
        0x30..=0x39 => Some(i64::from(unit - 0x30)),
        _ => None,
    }
}
