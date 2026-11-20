//! FMTID/property-storage-name conversion from Wine's `stg_prop.c`.

use crate::{
    Error, Fmtid, FMTID_DOC_SUMMARY_INFORMATION, FMTID_SUMMARY_INFORMATION,
    FMTID_USER_DEFINED_PROPERTIES,
};

const SUMMARY_NAME: &[u16] = &[
    5,
    b'S' as u16,
    b'u' as u16,
    b'm' as u16,
    b'm' as u16,
    b'a' as u16,
    b'r' as u16,
    b'y' as u16,
    b'I' as u16,
    b'n' as u16,
    b'f' as u16,
    b'o' as u16,
    b'r' as u16,
    b'm' as u16,
    b'a' as u16,
    b't' as u16,
    b'i' as u16,
    b'o' as u16,
    b'n' as u16,
];
const DOC_SUMMARY_NAME: &[u16] = &[
    5,
    b'D' as u16,
    b'o' as u16,
    b'c' as u16,
    b'u' as u16,
    b'm' as u16,
    b'e' as u16,
    b'n' as u16,
    b't' as u16,
    b'S' as u16,
    b'u' as u16,
    b'm' as u16,
    b'm' as u16,
    b'a' as u16,
    b'r' as u16,
    b'y' as u16,
    b'I' as u16,
    b'n' as u16,
    b'f' as u16,
    b'o' as u16,
    b'r' as u16,
    b'm' as u16,
    b'a' as u16,
    b't' as u16,
    b'i' as u16,
    b'o' as u16,
    b'n' as u16,
];
const FORMAT_MAP: &[u16; 32] = &[
    b'a' as u16,
    b'b' as u16,
    b'c' as u16,
    b'd' as u16,
    b'e' as u16,
    b'f' as u16,
    b'g' as u16,
    b'h' as u16,
    b'i' as u16,
    b'j' as u16,
    b'k' as u16,
    b'l' as u16,
    b'm' as u16,
    b'n' as u16,
    b'o' as u16,
    b'p' as u16,
    b'q' as u16,
    b'r' as u16,
    b's' as u16,
    b't' as u16,
    b'u' as u16,
    b'v' as u16,
    b'w' as u16,
    b'x' as u16,
    b'y' as u16,
    b'z' as u16,
    b'0' as u16,
    b'1' as u16,
    b'2' as u16,
    b'3' as u16,
    b'4' as u16,
    b'5' as u16,
];

/// Converts an optional FMTID to its property-storage name, including the trailing NUL.
///
/// The name is represented as UTF-16 units. A `None` FMTID maps to Wine's `E_INVALIDARG`.
pub fn fmt_id_to_prop_stg_name(fmtid: Option<&Fmtid>) -> Result<Vec<u16>, Error> {
    let fmtid = fmtid.ok_or(Error::InvalidArg)?;
    if *fmtid == FMTID_SUMMARY_INFORMATION {
        return Ok(nul_terminated(SUMMARY_NAME));
    }
    if *fmtid == FMTID_DOC_SUMMARY_INFORMATION || *fmtid == FMTID_USER_DEFINED_PROPERTIES {
        return Ok(nul_terminated(DOC_SUMMARY_NAME));
    }

    let bytes = fmtid.to_bytes();
    let mut name = Vec::with_capacity(28);
    name.push(5);
    let mut byte_index = 0;
    let mut bits_remaining = 8;
    while byte_index < bytes.len() {
        let mut value = u32::from(bytes[byte_index]) >> (8 - bits_remaining);
        if bits_remaining >= 5 {
            let mut unit = FORMAT_MAP[(value & 0x1f) as usize];
            if bits_remaining == 8 && (b'a' as u16..=b'z' as u16).contains(&unit) {
                unit -= u16::from(b'a' - b'A');
            }
            name.push(unit);
            bits_remaining -= 5;
            if bits_remaining == 0 {
                byte_index += 1;
                bits_remaining = 8;
            }
        } else {
            byte_index += 1;
            if byte_index < bytes.len() {
                value |= u32::from(bytes[byte_index]) << bits_remaining;
            }
            name.push(FORMAT_MAP[(value & 0x1f) as usize]);
            bits_remaining += 3;
        }
    }
    name.push(0);
    Ok(name)
}

/// Converts a NUL-terminated UTF-16 property-storage name to its FMTID.
///
/// Wine's parser reads 26 encoded units after the initial prefix. Short, non-special input is
/// rejected safely rather than reading past the Rust slice. As in Wine, the initial unit is not
/// validated for generic names, and letters in the encoded portion are case-insensitive.
pub fn prop_stg_name_to_fmt_id(name: Option<&[u16]>) -> Result<Fmtid, Error> {
    let name = name.ok_or(Error::InvalidName)?;
    let name = before_nul(name);
    if equals_ascii_case_insensitive(name, DOC_SUMMARY_NAME) {
        return Ok(FMTID_DOC_SUMMARY_INFORMATION);
    }
    if equals_ascii_case_insensitive(name, SUMMARY_NAME) {
        return Ok(FMTID_SUMMARY_INFORMATION);
    }
    if name.len() < 27 {
        return Err(Error::InvalidName);
    }

    let mut bytes = [0; 16];
    let mut byte_index = 0;
    for bits in (0..128).step_by(5) {
        let bits_used = bits % 8;
        if bits_used == 0 {
            byte_index += 1;
        }
        let value = decode_unit(name[bits / 5 + 1]).ok_or(Error::InvalidName)?;
        bytes[byte_index - 1] |= (u32::from(value) << bits_used) as u8;
        let bits_stored = (8 - bits_used).min(5);
        if bits_stored < 5 {
            let remaining = value >> (8 - bits_used);
            if bits + bits_stored == 128 {
                if remaining != 0 {
                    return Err(Error::InvalidName);
                }
                break;
            }
            byte_index += 1;
            bytes[byte_index - 1] |= remaining as u8;
        }
    }
    Ok(Fmtid::from_bytes(bytes))
}

fn nul_terminated(units: &[u16]) -> Vec<u16> {
    let mut result = Vec::with_capacity(units.len() + 1);
    result.extend_from_slice(units);
    result.push(0);
    result
}

fn before_nul(units: &[u16]) -> &[u16] {
    let end = units
        .iter()
        .position(|&unit| unit == 0)
        .unwrap_or(units.len());
    &units[..end]
}

fn equals_ascii_case_insensitive(left: &[u16], right: &[u16]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|(&left, &right)| fold_ascii(left) == fold_ascii(right))
}

fn fold_ascii(unit: u16) -> u16 {
    if (b'A' as u16..=b'Z' as u16).contains(&unit) {
        unit + u16::from(b'a' - b'A')
    } else {
        unit
    }
}

fn decode_unit(unit: u16) -> Option<u16> {
    let mut value = unit.wrapping_sub(u16::from(b'A'));
    if value > 26 {
        value = value.wrapping_sub(32);
        if value > 26 {
            value = value.wrapping_add(u16::from(b'a' - b'0' + 26));
            if value > 31 {
                return None;
            }
        }
    }
    Some(value)
}
