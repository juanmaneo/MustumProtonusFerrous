//! `PropVariantToGUID` and `PROPVAR_WCHARToGUID` of `propvar.c`.

use crate::{
    cstr, hex_value, unit, Guid, HResult, PropValue, E_INVALIDARG, E_NOTIMPL, E_POINTER, S_OK,
};

pub fn prop_variant_to_guid(value: &PropValue, guid: &mut Guid) -> HResult {
    match value {
        PropValue::Bstr(text) => store(wchar_to_guid(text.as_deref().unwrap_or_default()), guid),
        PropValue::LpWStr(text) => store(
            wchar_to_guid(cstr(text.as_deref().unwrap_or_default())),
            guid,
        ),
        PropValue::Clsid(Some(source)) => {
            *guid = *source;
            S_OK
        }
        PropValue::Clsid(None) => E_POINTER,
        _ => E_NOTIMPL,
    }
}

fn store(parsed: Option<Guid>, guid: &mut Guid) -> HResult {
    match parsed {
        Some(parsed) => {
            *guid = parsed;
            S_OK
        }
        None => {
            *guid = Guid::default();
            E_INVALIDARG
        }
    }
}

/// The separators of `{XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX}`, by index.
const SEPARATORS: [(usize, u32); 6] = [
    (0, b'{' as u32),
    (9, b'-' as u32),
    (14, b'-' as u32),
    (19, b'-' as u32),
    (24, b'-' as u32),
    (37, b'}' as u32),
];

/// The index of the first digit of each of the 16 hex pairs in the text.
const PAIR_AT: [usize; 16] = [1, 3, 5, 7, 10, 12, 15, 17, 20, 22, 25, 27, 29, 31, 33, 35];

fn wchar_to_guid(text: &[u16]) -> Option<Guid> {
    if text.len() != 38 || !SEPARATORS.iter().all(|&(at, c)| unit(text, at) == c) {
        return None;
    }
    let mut bytes = [0_u8; 16];
    for (i, &at) in PAIR_AT.iter().enumerate() {
        match hex_pair(text, at) {
            Some(byte) => bytes[i] = byte,
            // C stores `(BYTE)-1` for a bad `Data4` pair and goes on, so only `Data1` to
            // `Data3` reject the text.
            None if i < 8 => return None,
            None => bytes[i] = 0xFF,
        }
    }
    let mut data4 = [0_u8; 8];
    data4.copy_from_slice(&bytes[8..]);
    Some(Guid {
        data1: u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
        data2: u16::from_be_bytes([bytes[4], bytes[5]]),
        data3: u16::from_be_bytes([bytes[6], bytes[7]]),
        data4,
    })
}

fn hex_pair(text: &[u16], at: usize) -> Option<u8> {
    let hi = hex_value(unit(text, at))?;
    let lo = hex_value(unit(text, at + 1))?;
    Some(((hi << 4) | lo) as u8)
}

/// The `{XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX}` form that `PSStringFromPropertyKey` prints.
pub(crate) fn guid_string(guid: &Guid) -> String {
    let d = guid.data4;
    format!(
        "{{{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}",
        guid.data1, guid.data2, guid.data3, d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7]
    )
}
