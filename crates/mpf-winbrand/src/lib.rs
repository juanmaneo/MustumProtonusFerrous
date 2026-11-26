//! Port of the portable logic in Wine's `dlls/winbrand/main.c` and `dlls/winbrand/winbrand.spec`.
//!
//! Ported:
//! - `BrandingFormatString` (`@ stdcall`): [`branding_format_string`], which expands the
//!   `WINDOWS_SHORT` and `WINDOWS_LONG` variables through [`expand_environment_strings`]
//! - `RtlExpandEnvironmentStrings` and `ENV_FindVariable` from `dlls/ntdll/env.c`:
//!   [`expand_environment_strings`], [`find_environment_variable`], [`NtStatus`] and the
//!   `STATUS_*` values it returns
//! - `winbrand.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The 14 `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per export
//!
//! Skipped:
//! - The `FIXME` trace and the `debugstr_w` call.
//! - The `renv == NULL` path of `RtlExpandEnvironmentStrings`, which reads the process
//!   environment block under the PEB lock. The port always takes the block as an argument.
//! - A NULL destination with a nonzero count. A slice cannot express it, and
//!   `BrandingFormatString` never passes it.
//! - The status checks in `BrandingFormatString` that cannot fire: the length query only
//!   reports `STATUS_BUFFER_TOO_SMALL`, and the second call into a buffer of the queried length
//!   always reports `STATUS_SUCCESS`. The checks were removed, not left uncovered.
//! - Case folding beyond ASCII. Wine folds with the `RtlUpcaseUnicodeChar` table, which is not
//!   ported.
//! - The `WINAPI` and `__cdecl` calling conventions and the OS heap: the allocator is a closure
//!   argument.

mod env;
mod exports;
mod format;
mod stub;

pub use env::{
    expand_environment_strings, find_environment_variable, NtStatus, STATUS_BUFFER_TOO_SMALL,
    STATUS_SUCCESS,
};
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use format::branding_format_string;
pub use stub::*;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
