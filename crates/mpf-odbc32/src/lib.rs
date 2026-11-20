//! Headless port of the deterministic helpers in Wine's `dlls/odbc32/proxyodbc.c`.
//!
//! The DLL is mostly a thunk. Its exports forward to a driver, a registry key, a Win32 string conversion
//! or a handle table. The functions below run on plain values and keep the C control flow.
//!
//! Ported (names in `dlls/odbc32/proxyodbc.c`):
//! - `parse_connect_string`, `build_connect_string`, `get_attribute`: [parse_connect_string], [build_connect_string], [get_attribute].
//! - `has_suffix`, used to recognise `.dll` names: [has_suffix].
//! - `map_type`: [map_type].
//! - `len_to_user`, `len_from_user`: [len_to_user], [len_from_user].
//!
//! Skipped: the exports that call a driver, the registry, handle tables or `MultiByteToWideChar`
//! (`strnAtoW`, `strnWtoA`, `strdupAW` and `strdupWA`), and the debug helpers.
//!
//! Limits of the port:
//! - Case folding is ASCII only. Wine's `wcsicmp` also folds non-ASCII letters.
//! - `SQLLEN` is the 64-bit Win64 type. The 32-bit `SQLLEN` layout is not modelled.
//! - Inputs the C code reads out of bounds return `Err` or stop early instead of reading past the buffer.
//! - Names and values are compared and copied as whole slices. A NUL unit in a caller-built `Attribute` is an
//!   ordinary unit here, whereas the C code stops at it. `parse_connect_string` never produces one.

mod connect_string;
mod lengths;
mod types;
mod wide;

use wide::{before_nul, eq_ignore_case};

pub use connect_string::{
    build_connect_string, get_attribute, parse_connect_string, Attribute, ParseError,
};
pub use lengths::{len_from_user, len_to_user, LengthError};
pub use types::map_type;

/// Ports `has_suffix`: both strings are read up to their first NUL and the tail is compared without regard to ASCII case.
pub fn has_suffix(s: &[u16], suffix: &[u16]) -> bool {
    let (s, suffix) = (before_nul(s), before_nul(suffix));
    s.len() >= suffix.len() && eq_ignore_case(&s[s.len() - suffix.len()..], suffix)
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
