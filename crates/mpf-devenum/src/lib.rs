//! Pure logic from Wine's `dlls/devenum`: the `IParseDisplayName` parser (`parsedisplayname.c`),
//! moniker display names (`GetDisplayName` in `mediacatenum.c`), GUID text (`guid_from_string`
//! and `StringFromGUID2` from `combase`) and the `array_reserve` growth rule.
//!
//! Registry enumeration, COM objects, the filter and DMO enumerators and the ProgID fallback of
//! `CLSIDFromString` stay in Wine.

pub mod array;
pub mod guid;
pub mod moniker;

use guid::{guid_prefix, CHARS_IN_GUID};
use moniker::{DeviceKind, Moniker};

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// Port of `devenum_parser_ParseDisplayName`. The name has the form `@device:sw:`, `@device:cm:`
/// or `@device:dmo:` followed by GUID text and the device name. Returns `None` where Wine returns
/// `MK_E_SYNTAX`: no `:`, an unknown device type, or a DMO name without two valid GUIDs.
///
/// A filter or codec whose GUID does not parse is returned without a class. Wine would first try
/// a registry ProgID lookup, which this port does not model.
pub fn parse_display_name(name: &[u16]) -> Option<Moniker<'_>> {
    let colon = name.iter().position(|&unit| unit == u16::from(b':'))?;
    let name = &name[colon + 1..];
    if let Some(rest) = strip_prefix(name, "sw:") {
        return Some(parse_device(DeviceKind::Filter, rest));
    }
    if let Some(rest) = strip_prefix(name, "cm:") {
        return Some(parse_device(DeviceKind::Codec, rest));
    }
    let rest = strip_prefix(name, "dmo:")?;
    let clsid = guid_prefix(rest)?;
    let class = guid_prefix(rest.get(CHARS_IN_GUID - 1..).unwrap_or_default())?;
    Some(Moniker::Dmo { clsid, class })
}

fn parse_device(kind: DeviceKind, name: &[u16]) -> Moniker<'_> {
    match guid_prefix(name) {
        Some(class) => Moniker::Device {
            kind,
            class: Some(class),
            name: name.get(CHARS_IN_GUID..).unwrap_or_default(),
        },
        None => Moniker::Device {
            kind,
            class: None,
            name,
        },
    }
}

fn strip_prefix<'a>(name: &'a [u16], prefix: &str) -> Option<&'a [u16]> {
    name.strip_prefix(wide(prefix).as_slice())
}

/// UTF-16 units of an ASCII literal.
pub(crate) fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}
