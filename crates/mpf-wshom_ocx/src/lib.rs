//! Deterministic string logic from Wine's `dlls/wshom.ocx/shell.c`.
//!
//! Ported functions: `split_command`, `get_root_key`, and `split_reg_path`. Their COM callers
//! also use process, registry, environment, shell-link, or handle state, which is outside this
//! crate's scope. Inputs are UTF-16 strings and, like the source's `WCHAR *` calls, are truncated
//! at their first NUL. Returned slices borrow from the input rather than allocating copies.
//!
//! The port preserves Wine's behavior, including root-name prefix matches without a boundary
//! check and the single-separator `split_reg_path` case retaining its leading backslash.

mod command;
mod registry;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use command::split_command;
pub use registry::{root_key, split_reg_path, RegPath, RootKey};

/// A Win32 `HRESULT`.
pub type HResult = i32;

/// `HRESULT_FROM_WIN32(ERROR_PATH_NOT_FOUND)`.
pub const PATH_NOT_FOUND: HResult = 0x8007_0003u32 as i32;

/// The part of `s` before its first NUL, as the C string functions see it.
fn wide_str(s: &[u16]) -> &[u16] {
    let end = s.iter().position(|&c| c == 0).unwrap_or(s.len());
    &s[..end]
}
