//! Port of the library name selection in Wine's `load_functions` in
//! `dlls/ctapi32/ctapi32.c`: the `library` registry value, the `libctapi.so`
//! fallback, and the `MAX_PATH` limit on the narrow name.

use crate::unixlib::{attach, STATUS_SUCCESS};

/// Windows `MAX_PATH`: the size of the narrow soname buffer, NUL included.
pub const MAX_PATH: usize = 260;
/// Windows `REG_SZ` value type.
pub const REG_SZ: u32 = 1;
/// Soname used when the registry value is absent, of the wrong type, or does not convert.
pub const FALLBACK_LIBCTAPI: &[u8] = b"libctapi.so";

/// Bytes `RegQueryValueExW` may return: `MAX_PATH` wide chars less the terminator.
const LIBRARY_VALUE_BYTES: usize = (MAX_PATH - 1) * 2;

/// Port of the `RegQueryValueExW` result handling. `value` is the `(type, data)` the
/// query returned, or `None` if the key or value is missing or the query failed.
/// Returns the wide chars of the value, or `None` for a missing value, a type other
/// than `REG_SZ`, or data too large for the buffer (`ERROR_MORE_DATA`). A trailing odd
/// byte is dropped.
pub fn registry_library(value: Option<(u32, &[u8])>) -> Option<Vec<u16>> {
    let (value_type, data) = value?;
    if value_type != REG_SZ || data.len() > LIBRARY_VALUE_BYTES {
        return None;
    }
    Some(
        data.as_chunks::<2>()
            .0
            .iter()
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect(),
    )
}

/// Port of `WideCharToMultiByte(CP_UNIXCP, 0, buffer_w, -1, soname, sizeof(soname))`:
/// converts up to the first NUL and returns `None` when the UTF-8 name would not fit
/// in `MAX_PATH` bytes together with its NUL.
pub fn narrow_name(units: &[u16]) -> Option<Vec<u8>> {
    let end = units
        .iter()
        .position(|&unit| unit == 0)
        .unwrap_or(units.len());
    let name = String::from_utf16_lossy(&units[..end]).into_bytes();
    (name.len() < MAX_PATH).then_some(name)
}

/// Port of the soname choice in `load_functions`: the registry `library` value when it
/// converts within `MAX_PATH`, otherwise `libctapi.so`.
pub fn soname(value: Option<(u32, &[u8])>) -> Vec<u8> {
    registry_library(value)
        .and_then(|units| narrow_name(&units))
        .unwrap_or_else(|| FALLBACK_LIBCTAPI.to_vec())
}

/// Port of `load_functions`: `dlopen` receives the soname and returns the `dlsym`
/// lookup on success. Returns whether `attach` reports `STATUS_SUCCESS`.
pub fn load_functions<F: FnMut(&str) -> bool>(
    value: Option<(u32, &[u8])>,
    dlopen: impl FnOnce(&[u8]) -> Option<F>,
) -> bool {
    attach(dlopen(&soname(value))) == STATUS_SUCCESS
}
