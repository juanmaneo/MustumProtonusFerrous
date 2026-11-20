//! Interface names: `ConvertInterfaceLuidToNameW` and `ConvertInterfaceNameToLuidW`.
//!
//! A name is `<prefix>_<index>` for the interface types in [`NAME_PREFIXES`], and
//! `iftype<n>_<index>` for any other type.

use crate::Error;

/// `IF_MAX_STRING_SIZE`.
const IF_MAX_STRING_SIZE: usize = 256;

const INDEX_MASK: u32 = 0x00ff_ffff;

const IFTYPE: &str = "iftype";

/// Wine's `name_prefixes`: the `IF_TYPE_*` value and the name prefix for each.
const NAME_PREFIXES: [(&str, u16); 9] = [
    ("other", 1),
    ("ethernet", 6),
    ("tokenring", 9),
    ("ppp", 23),
    ("loopback", 24),
    ("atm", 37),
    ("wireless", 71),
    ("tunnel", 131),
    ("ieee1394", 144),
];

/// A `NET_LUID`: the interface type (16 bits) and the 24-bit `NetLuidIndex`.
/// The 24 reserved bits are zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NetLuid {
    pub if_type: u16,
    pub index: u32,
}

impl NetLuid {
    /// Builds a LUID, keeping the low 24 bits of `index`.
    pub fn new(if_type: u16, index: u32) -> Self {
        Self {
            if_type,
            index: index & INDEX_MASK,
        }
    }
}

/// `ConvertInterfaceLuidToNameW`: the name of `luid`, for a buffer of `len` units
/// including the NUL. The name must fit with its NUL, so `len` must exceed its length.
pub fn luid_to_name(luid: &NetLuid, len: usize) -> Result<String, Error> {
    let name = match NAME_PREFIXES.iter().find(|(_, t)| *t == luid.if_type) {
        Some((prefix, _)) => format!("{prefix}_{}", luid.index),
        None => format!("iftype{}_{}", luid.if_type, luid.index),
    };
    if name.len() >= len {
        return Err(Error::NotEnoughMemory);
    }
    Ok(name)
}

/// `ConvertInterfaceNameToLuidW`: parses `name`, read up to its first NUL.
pub fn name_to_luid(name: &[u16]) -> Result<NetLuid, Error> {
    let end = name.iter().position(|&c| c == 0).unwrap_or(name.len());
    let name = &name[..end];
    let sep = name
        .iter()
        .position(|&c| c == u16::from(b'_'))
        .ok_or(Error::InvalidName)?;
    if sep > IF_MAX_STRING_SIZE {
        return Err(Error::InvalidName);
    }
    let prefix = &name[..sep];

    let if_type = if sep > IFTYPE.len() && eq_ascii(&prefix[..IFTYPE.len()], IFTYPE) {
        let value = wcstol(&prefix[IFTYPE.len()..]) as u32;
        if value == u32::MAX {
            return Err(Error::InvalidName);
        }
        value as u16
    } else {
        NAME_PREFIXES
            .iter()
            .find(|(p, _)| eq_ascii(prefix, p))
            .ok_or(Error::InvalidName)?
            .1
    };
    Ok(NetLuid::new(if_type, wcstol(&name[sep + 1..]) as u32))
}

fn eq_ascii(units: &[u16], s: &str) -> bool {
    units.len() == s.len() && units.iter().zip(s.bytes()).all(|(&u, b)| u == u16::from(b))
}

/// `wcstol(s, NULL, 10)` with a 64-bit `long`: leading ASCII white space, an
/// optional sign, then decimal digits up to the first other unit. Overflow
/// saturates at the limits of `long`, as glibc does.
fn wcstol(s: &[u16]) -> i64 {
    let mut i = 0;
    while s.get(i).is_some_and(|&c| matches!(c, 0x20 | 0x09..=0x0d)) {
        i += 1;
    }
    let negative = match s.get(i) {
        Some(&c) if c == u16::from(b'-') => {
            i += 1;
            true
        }
        Some(&c) if c == u16::from(b'+') => {
            i += 1;
            false
        }
        _ => false,
    };
    let mut magnitude: u128 = 0;
    while let Some(digit) = s
        .get(i)
        .and_then(|&c| char::from_u32(u32::from(c)).and_then(|ch| ch.to_digit(10)))
    {
        magnitude = (magnitude * 10 + u128::from(digit)).min(1 << 64);
        i += 1;
    }
    let limit: u128 = if negative { 1 << 63 } else { (1 << 63) - 1 };
    let value = magnitude.min(limit) as i128;
    (if negative { -value } else { value }) as i64
}
