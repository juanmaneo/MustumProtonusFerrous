//! XML name, token and value checks from reader.c. Each reads its text up to the first NUL,
//! as the C does, and returns the length the C writes to its out parameter.

use crate::chars::{
    is_namechar, is_namestartchar, is_ncnamechar, is_ncnamestartchar, is_pubchar, is_wchar_space,
};
use crate::{c_str, eq_ascii, WC_E_NAMECHARACTER, WC_E_PUBLICID};

const COLON: u16 = 0x3a;

/// `is_valid_ncname` and `is_valid_name`. Empty text is a valid name of length 0.
fn check_name(name: &[u16], start: fn(u16) -> bool, part: fn(u16) -> bool) -> Result<usize, i32> {
    let name = c_str(name);
    let Some((&first, rest)) = name.split_first() else {
        return Ok(0);
    };
    if start(first) && rest.iter().all(|&unit| part(unit)) {
        Ok(name.len())
    } else {
        Err(WC_E_NAMECHARACTER)
    }
}

pub fn is_valid_ncname(name: &[u16]) -> Result<usize, i32> {
    check_name(name, is_ncnamestartchar, is_ncnamechar)
}

pub fn is_valid_name(name: &[u16]) -> Result<usize, i32> {
    check_name(name, is_namestartchar, is_namechar)
}

/// `is_valid_qname`: an NCName, optionally followed by a colon and an NCName that may be
/// empty. Unlike the other name checks, empty text is an error.
pub fn is_valid_qname(name: &[u16]) -> Result<usize, i32> {
    let name = c_str(name);
    let (prefix, local) = match name.iter().position(|&unit| unit == COLON) {
        Some(colon) => (&name[..colon], Some(&name[colon + 1..])),
        None => (name, None),
    };
    let prefix_ok = prefix.split_first().is_some_and(|(&first, rest)| {
        is_ncnamestartchar(first) && rest.iter().all(|&unit| is_ncnamechar(unit))
    });
    let local_ok = local.is_none_or(|local| local.iter().all(|&unit| is_ncnamechar(unit)));
    if prefix_ok && local_ok {
        Ok(name.len())
    } else {
        Err(WC_E_NAMECHARACTER)
    }
}

/// `is_valid_pubid`. Empty text is a valid public id of length 0.
pub fn is_valid_pubid(name: &[u16]) -> Result<usize, i32> {
    valid_units(name, is_pubchar).ok_or(WC_E_PUBLICID)
}

/// `is_valid_nmtoken`.
pub fn is_valid_nmtoken(name: &[u16]) -> Option<usize> {
    valid_units(name, is_namechar)
}

/// `is_valid_whitespace`.
pub fn is_valid_whitespace(text: &[u16]) -> Option<usize> {
    valid_units(text, is_wchar_space)
}

/// `is_valid_xml_space_value`: exactly `preserve` or `default`.
pub fn is_valid_xml_space_value(value: &[u16]) -> bool {
    let value = c_str(value);
    eq_ascii(value, "preserve") || eq_ascii(value, "default")
}

/// The length of the text when every unit is valid, which an empty text always is.
fn valid_units(text: &[u16], valid: fn(u16) -> bool) -> Option<usize> {
    let text = c_str(text);
    text.iter().all(|&unit| valid(unit)).then_some(text.len())
}
