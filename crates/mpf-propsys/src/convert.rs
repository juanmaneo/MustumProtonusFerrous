//! The `PropVariantTo*` conversions of `propvar.c`. The integer and double conversions write
//! their output only on `S_OK`, as the C code does.

use crate::number::convert_number;
use crate::{
    cstr, widen, HResult, PropValue, DISP_E_TYPEMISMATCH, E_FAIL, E_INVALIDARG, S_OK, VARIANT_TRUE,
};

pub fn prop_variant_to_int16(value: &PropValue, out: &mut i16) -> HResult {
    let (hr, res) = convert_number(value, 16, true);
    if hr == S_OK {
        *out = res as i16;
    }
    hr
}

pub fn prop_variant_to_int32(value: &PropValue, out: &mut i32) -> HResult {
    let (hr, res) = convert_number(value, 32, true);
    if hr == S_OK {
        *out = res as i32;
    }
    hr
}

pub fn prop_variant_to_int64(value: &PropValue, out: &mut i64) -> HResult {
    let (hr, res) = convert_number(value, 64, true);
    if hr == S_OK {
        *out = res;
    }
    hr
}

pub fn prop_variant_to_uint16(value: &PropValue, out: &mut u16) -> HResult {
    let (hr, res) = convert_number(value, 16, false);
    if hr == S_OK {
        *out = res as u16;
    }
    hr
}

pub fn prop_variant_to_uint32(value: &PropValue, out: &mut u32) -> HResult {
    let (hr, res) = convert_number(value, 32, false);
    if hr == S_OK {
        *out = res as u32;
    }
    hr
}

pub fn prop_variant_to_uint64(value: &PropValue, out: &mut u64) -> HResult {
    let (hr, res) = convert_number(value, 64, false);
    if hr == S_OK {
        *out = res as u64;
    }
    hr
}

pub fn prop_variant_to_uint32_with_default(value: &PropValue, default: u32) -> u32 {
    match convert_number(value, 32, false) {
        (S_OK, res) => res as u32,
        _ => default,
    }
}

pub fn prop_variant_to_double(value: &PropValue, out: &mut f64) -> HResult {
    match value {
        PropValue::R8(v) => {
            *out = *v;
            S_OK
        }
        PropValue::R4(v) => {
            *out = f64::from(*v);
            S_OK
        }
        _ => {
            let (hr, res) = convert_number(value, 64, true);
            if hr == S_OK {
                *out = res as f64;
            }
            hr
        }
    }
}

pub fn prop_variant_to_boolean(value: &PropValue, out: &mut bool) -> HResult {
    *out = false;
    let literal = match value {
        PropValue::Bool(v) => {
            *out = *v == VARIANT_TRUE;
            return S_OK;
        }
        PropValue::LpStr(None) | PropValue::LpWStr(None) | PropValue::Bstr(None) => {
            return DISP_E_TYPEMISMATCH;
        }
        PropValue::LpStr(Some(text)) => literal_bool(&widen(text)),
        PropValue::LpWStr(Some(text)) | PropValue::Bstr(Some(text)) => literal_bool(text),
        _ => None,
    };
    if let Some(b) = literal {
        *out = b;
        return S_OK;
    }
    let (hr, res) = convert_number(value, 64, true);
    *out = res != 0;
    hr
}

/// `lstrcmpiW` with `"true"` and `lstrcmpW` with `"#TRUE#"`, then the same for `false`.
fn literal_bool(text: &[u16]) -> Option<bool> {
    let text = cstr(text);
    if matches_text(text, "true", true) || matches_text(text, "#TRUE#", false) {
        Some(true)
    } else if matches_text(text, "false", true) || matches_text(text, "#FALSE#", false) {
        Some(false)
    } else {
        None
    }
}

fn matches_text(text: &[u16], pattern: &str, fold: bool) -> bool {
    text.len() == pattern.len()
        && text.iter().zip(pattern.bytes()).all(|(&c, want)| {
            let c = if fold { fold_ascii(c) } else { c };
            c == u16::from(want)
        })
}

fn fold_ascii(c: u16) -> u16 {
    if (u16::from(b'A')..=u16::from(b'Z')).contains(&c) {
        c + 0x20
    } else {
        c
    }
}

pub fn prop_variant_to_buffer(value: &PropValue, out: &mut [u8]) -> HResult {
    let PropValue::VectorUi1(bytes) = value else {
        return E_INVALIDARG;
    };
    if out.len() > bytes.len() {
        return E_FAIL;
    }
    out.copy_from_slice(&bytes[..out.len()]);
    S_OK
}

pub fn prop_variant_to_string_with_default<'a>(
    value: &'a PropValue,
    default: Option<&'a [u16]>,
) -> Option<&'a [u16]> {
    match value {
        PropValue::Bstr(text) => Some(cstr(text.as_deref().unwrap_or_default())),
        PropValue::LpWStr(Some(text)) => Some(cstr(text)),
        _ => default.map(cstr),
    }
}
