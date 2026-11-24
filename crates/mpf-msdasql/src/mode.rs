//! The `Mode` connection property name lookup (`convert_dbproperty_mode`).

use std::cmp::Ordering;

pub const DB_MODE_READ: u32 = 0x01;
pub const DB_MODE_WRITE: u32 = 0x02;
pub const DB_MODE_READWRITE: u32 = 0x03;
pub const DB_MODE_SHARE_DENY_READ: u32 = 0x04;
pub const DB_MODE_SHARE_DENY_WRITE: u32 = 0x08;
pub const DB_MODE_SHARE_EXCLUSIVE: u32 = 0x0c;
pub const DB_MODE_SHARE_DENY_NONE: u32 = 0x10;

/// The C table, in the order `bsearch` requires: sorted by name under case-insensitive comparison.
const DB_MODE_NAMES: [(&str, u32); 7] = [
    ("Read", DB_MODE_READ),
    ("ReadWrite", DB_MODE_READWRITE),
    ("Share Deny None", DB_MODE_SHARE_DENY_NONE),
    ("Share Deny Read", DB_MODE_SHARE_DENY_READ),
    ("Share Deny Write", DB_MODE_SHARE_DENY_WRITE),
    ("Share Exclusive", DB_MODE_SHARE_EXCLUSIVE),
    ("Write", DB_MODE_WRITE),
];

/// Ports `convert_dbproperty_mode`. `src` is read up to its first NUL, as `wcsicmp` reads it. The result is the
/// `VT_I4` value the C code stores in the `VARIANT`, or `None` where the C code returns `E_FAIL`.
///
/// Names are compared with ASCII letters folded. Wine's `wcsicmp` also folds non-ASCII letters, which this port does not model.
pub fn convert_dbproperty_mode(src: &[u16]) -> Option<u32> {
    let src = before_nul(src);
    DB_MODE_NAMES
        .binary_search_by(|&(name, _)| compare_folded(name, src))
        .ok()
        .map(|index| DB_MODE_NAMES[index].1)
}

fn before_nul(units: &[u16]) -> &[u16] {
    let end = units
        .iter()
        .position(|&unit| unit == 0)
        .unwrap_or(units.len());
    &units[..end]
}

/// Orders `name` against `src` as `wcsicmp(name, src)` does, for strings with no NUL.
fn compare_folded(name: &str, src: &[u16]) -> Ordering {
    name.encode_utf16()
        .map(fold)
        .cmp(src.iter().copied().map(fold))
}

fn fold(unit: u16) -> u16 {
    if (b'A' as u16..=b'Z' as u16).contains(&unit) {
        unit + 32
    } else {
        unit
    }
}
