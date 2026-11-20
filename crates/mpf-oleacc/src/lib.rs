//! Deterministic, headless portions of Wine's `dlls/oleacc/main.c`.
//!
//! Ported functions:
//! - `convert_child_id`: converts a `VT_I4` child variant to its value and returns `-1` for
//!   other variant types.
//! - `GetOleaccVersionInfo`: reports the fixed `7.0.0.0` version and zero build number.
//!
//! Window, COM, registry, resource-loading, and process-handle operations are intentionally not
//! included because they depend on operating-system state.

mod child_id;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use child_id::VT_I4;

/// Returns the value of a `VT_I4` child-id variant, or `-1` for another variant type.
///
/// This follows `convert_child_id` in Wine's `dlls/oleacc/main.c`.
pub fn convert_child_id(vt: u16, val: i32) -> i32 {
    if child_id::is_supported_child_id_type(vt) {
        val
    } else {
        -1
    }
}

/// Returns Wine's fixed `GetOleaccVersionInfo` values: version 7.0 and build 0.
pub const fn get_oleacc_version_info() -> (u32, u32) {
    (0x0007_0000, 0)
}
