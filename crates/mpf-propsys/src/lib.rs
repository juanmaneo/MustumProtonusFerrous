//! Deterministic part of Wine's `dlls/propsys`: the `PROPVARIANT` conversions of `propvar.c`
//! (`PropVariantTo*`, `PROPVAR_ConvertNumber` and `PROPVAR_WCHARToGUID`) and the property key
//! string helpers of `propsys_main.c` (`PSPropertyKeyFromString` and `PSStringFromPropertyKey`).
//!
//! Divergences from the C code, all on paths where C reads memory it should not, or where the
//! C result is undefined:
//! - A NULL `pszVal`, `pwszVal` or `bstrVal` reads as empty text. C dereferences it. A NULL
//!   `LPWSTR` therefore gives `E_INVALIDARG` from `prop_variant_to_guid`.
//! - A NULL `puuid` in `VT_CLSID` gives `E_POINTER`. C crashes in `memcpy`.
//! - A `double` outside `-2^63..2^63`, and NaN, converts to `i64::MIN`. C's conversion is
//!   undefined there.
//! - `wcstol` is modelled with the 32-bit `long` of Windows, so wide text clamps to `i32`.
//! - Case-insensitive literal comparison folds ASCII letters only.
//! - `prop_variant_to_boolean` returns `false` on `E_NOTIMPL`. C returns an uninitialised value.
//! - `VT_ARRAY | VT_UI1` has no `PropValue` variant, so `prop_variant_to_buffer` returns
//!   `E_INVALIDARG` for every value other than `VectorUi1`.
//! - Text results are truncated at the first NUL, as a C caller reads them. A BSTR with an
//!   embedded NUL therefore reads short, except in `prop_variant_to_guid`, which uses its full
//!   length as `SysStringLen` does.

mod convert;
mod guid;
mod key;
mod number;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use convert::{
    prop_variant_to_boolean, prop_variant_to_buffer, prop_variant_to_double, prop_variant_to_int16,
    prop_variant_to_int32, prop_variant_to_int64, prop_variant_to_string_with_default,
    prop_variant_to_uint16, prop_variant_to_uint32, prop_variant_to_uint32_with_default,
    prop_variant_to_uint64,
};
pub use guid::prop_variant_to_guid;
pub use key::{ps_property_key_from_string, ps_string_from_property_key};

pub type HResult = u32;

pub const S_OK: HResult = 0;
pub const E_NOTIMPL: HResult = 0x8000_4001;
pub const E_POINTER: HResult = 0x8000_4003;
pub const E_FAIL: HResult = 0x8000_4005;
pub const E_INVALIDARG: HResult = 0x8007_0057;
pub const E_NOT_SUFFICIENT_BUFFER: HResult = 0x8007_007A;
pub const DISP_E_TYPEMISMATCH: HResult = 0x8002_0005;
pub const ERROR_ARITHMETIC_OVERFLOW_HR: HResult = 0x8007_0216;

/// `VARIANT_TRUE`.
pub const VARIANT_TRUE: i16 = -1;

/// `GUIDSTRING_MAX`: a formatted GUID with its NUL terminator.
pub const GUIDSTRING_MAX: usize = 39;

/// A `GUID`, as used for `CLSID`, `IID` and `FMTID` values.
#[derive(Clone, Copy, Default)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// A `PROPERTYKEY`: a format identifier and a property identifier.
#[derive(Default)]
pub struct PropertyKey {
    pub fmtid: Guid,
    pub pid: u32,
}

/// The `PROPVARIANT` shapes that the conversions read. `None` in a text or `Clsid` variant
/// models a NULL pointer.
pub enum PropValue {
    Empty,
    Null,
    I1(i8),
    Ui1(u8),
    I2(i16),
    Ui2(u16),
    I4(i32),
    Ui4(u32),
    I8(i64),
    Ui8(u64),
    Bool(i16),
    R4(f32),
    R8(f64),
    LpStr(Option<Vec<u8>>),
    LpWStr(Option<Vec<u16>>),
    Bstr(Option<Vec<u16>>),
    Clsid(Option<Guid>),
    VectorUi1(Vec<u8>),
}

/// The unit at `at`, or NUL past the end, so that slices read as NUL-terminated strings.
fn unit(text: &[u16], at: usize) -> u32 {
    text.get(at).copied().map_or(0, u32::from)
}

/// The text before its first NUL.
fn cstr(text: &[u16]) -> &[u16] {
    match text.iter().position(|&c| c == 0) {
        Some(len) => &text[..len],
        None => text,
    }
}

/// Widens byte text one unit per byte, as the `LPSTR` conversions do.
fn widen(bytes: &[u8]) -> Vec<u16> {
    bytes.iter().copied().map(u16::from).collect()
}

/// The value of an ASCII hex digit.
fn hex_value(c: u32) -> Option<u32> {
    match c {
        0x30..=0x39 => Some(c - 0x30),
        0x41..=0x46 => Some(c - 0x41 + 10),
        0x61..=0x66 => Some(c - 0x61 + 10),
        _ => None,
    }
}
