//! Port of the portable logic in Wine's `dlls/wofutil/main.c` and `dlls/wofutil/wofutil.spec`.
//!
//! Ported:
//! - `WofIsExternalFile` and `WofShouldCompressBinaries`: the return values and the
//!   out-parameter writes: [`wof_is_external_file`], [`wof_should_compress_binaries`], with the
//!   `S_OK` and `FALSE` values
//! - `wofutil.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The nine `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`STUB_EXPORTS`], [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per
//!   export such as [`wof_enum_entries`]
//!
//! Skipped:
//! - The `FIXME` traces of the arguments. The `path` and `ptr` arguments of `WofIsExternalFile`
//!   and the `volume` and `alg` arguments of `WofShouldCompressBinaries` are never read or
//!   written by the C bodies, so the port accepts and ignores them.
//! - The `WINAPI` calling convention and the Windows headers.

mod calls;
mod exports;
mod stubs;

pub use calls::{wof_is_external_file, wof_should_compress_binaries, FALSE, S_OK};
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stubs::*;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
