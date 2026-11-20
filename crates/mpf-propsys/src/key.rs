//! `PSPropertyKeyFromString` and `PSStringFromPropertyKey` of `propsys_main.c`.

use crate::guid::guid_string;
use crate::{
    hex_value, unit, Guid, HResult, PropertyKey, E_INVALIDARG, E_NOT_SUFFICIENT_BUFFER, E_POINTER,
    GUIDSTRING_MAX, S_OK,
};

const NUL: u32 = 0;
const SPACE: u32 = 0x20;
const COMMA: u32 = 0x2C;
const MINUS: u32 = 0x2D;

/// The indices that each `Data4` byte is read from, as `string_to_guid` validates them.
const DATA4_RANGES: [(usize, usize); 8] = [
    (20, 21),
    (22, 24),
    (25, 26),
    (27, 28),
    (29, 30),
    (31, 32),
    (33, 34),
    (35, 37),
];

fn valid_at(c: u32, at: usize) -> bool {
    match at {
        0 => c == u32::from(b'{'),
        9 | 14 | 19 | 24 => c == u32::from(b'-'),
        37 => c == u32::from(b'}'),
        _ => hex_value(c).is_some(),
    }
}

fn valid_range(text: &[u16], min: usize, max: usize) -> bool {
    (min..=max).all(|at| valid_at(unit(text, at), at))
}

fn hex_run(text: &[u16], start: usize, len: usize) -> u32 {
    (start..start + len).fold(0, |value, at| {
        (value << 4) | hex_value(unit(text, at)).unwrap_or(0)
    })
}

/// Writes each field only once its indices have validated, so a bad string can leave the
/// earlier fields set, as the C code does.
fn string_to_guid(text: &[u16], guid: &mut Guid) -> bool {
    if !valid_range(text, 0, 8) {
        return false;
    }
    guid.data1 = hex_run(text, 1, 8);
    if !valid_range(text, 9, 14) {
        return false;
    }
    guid.data2 = hex_run(text, 10, 4) as u16;
    if !valid_range(text, 15, 19) {
        return false;
    }
    guid.data3 = hex_run(text, 15, 4) as u16;
    for (byte, &(min, max)) in guid.data4.iter_mut().zip(DATA4_RANGES.iter()) {
        if !valid_range(text, min, max) {
            return false;
        }
        *byte = hex_run(text, min, 2) as u8;
    }
    true
}

/// Parses `{GUID} pid` into `key`. The key is zeroed first, and then written field by field.
pub fn ps_property_key_from_string(text: Option<&[u16]>, key: Option<&mut PropertyKey>) -> HResult {
    let (Some(text), Some(key)) = (text, key) else {
        return E_POINTER;
    };
    *key = PropertyKey::default();
    if !string_to_guid(text, &mut key.fmtid) {
        return E_INVALIDARG;
    }

    let mut at = GUIDSTRING_MAX - 1;
    let mut has_comma = false;
    loop {
        match unit(text, at) {
            SPACE => {}
            COMMA => {
                if has_comma {
                    return S_OK;
                }
                has_comma = true;
            }
            _ => break,
        }
        at += 1;
    }
    if unit(text, at) == NUL {
        return E_INVALIDARG;
    }

    // With a comma, one minus sign counts. Without one, the first sign is skipped and a second
    // one is interpreted, with spaces allowed around each.
    let has_minus = if has_comma {
        let minus = unit(text, at) == MINUS;
        at += usize::from(minus);
        minus
    } else {
        at += usize::from(unit(text, at) == MINUS);
        while unit(text, at) == SPACE {
            at += 1;
        }
        let minus = unit(text, at) == MINUS;
        at += usize::from(minus);
        while unit(text, at) == SPACE {
            at += 1;
        }
        minus
    };

    let mut pid = 0_u32;
    while (u32::from(b'0')..=u32::from(b'9')).contains(&unit(text, at)) {
        pid = pid
            .wrapping_mul(10)
            .wrapping_add(unit(text, at) - u32::from(b'0'));
        at += 1;
    }
    key.pid = if has_minus { pid.wrapping_neg() } else { pid };
    S_OK
}

/// Writes `{GUID} pid` to `psz`. When the pid does not fit, the output is the C quirk: the
/// digits go in reverse, skipping the last one, after a NUL at the pid's start.
pub fn ps_string_from_property_key(key: Option<&PropertyKey>, psz: Option<&mut [u16]>) -> HResult {
    let Some(psz) = psz else {
        return E_POINTER;
    };
    if psz.len() <= GUIDSTRING_MAX + 1 {
        return E_NOT_SUFFICIENT_BUFFER;
    }
    let Some(key) = key else {
        psz[0] = 0;
        return E_NOT_SUFFICIENT_BUFFER;
    };

    let mut prefix: Vec<u8> = guid_string(&key.fmtid).into_bytes();
    prefix.push(b' ');
    let (head, tail) = psz.split_at_mut(GUIDSTRING_MAX);
    for (slot, c) in head.iter_mut().zip(prefix) {
        *slot = u16::from(c);
    }

    let digits: Vec<u16> = key.pid.to_string().bytes().map(u16::from).collect();
    if tail.len() > digits.len() {
        tail[..digits.len()].copy_from_slice(&digits);
        tail[digits.len()] = 0;
        return S_OK;
    }

    head[0] = 0;
    tail[0] = 0;
    for (slot, &digit) in tail[1..].iter_mut().zip(digits.iter().rev().skip(1)) {
        *slot = digit;
    }
    E_NOT_SUFFICIENT_BUFFER
}
