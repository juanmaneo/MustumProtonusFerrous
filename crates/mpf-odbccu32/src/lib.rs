//! Port of the portable logic in Wine's `dlls/odbccu32/odbccu32.spec` and `dlls/odbccu32/Makefile.in`.
//!
//! `dlls/odbccu32` has no C sources. Its code is the export table of the spec file, plus the
//! `__wine_spec_unimplemented_stub` call that its one `stub` export makes.
//!
//! Ported:
//! - The 37 entries of `odbccu32.spec` with their ordinals: 36 `stdcall` forwards to
//!   `odbc32.<name>` and one `stub`: [`EXPORTS`], [`Export`], [`ExportKind`], [`find_export`]
//! - `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`), the record each stub raises:
//!   [`wine_spec_unimplemented_stub`], [`StubException`], [`EXCEPTION_WINE_STUB`],
//!   [`EXCEPTION_NONCONTINUABLE`]
//! - `ReleaseCLStmtResources`, the `stub` export: [`release_cl_stmt_resources`]
//!
//! Skipped:
//! - The bodies of the 36 forwarded functions: they live in `dlls/odbc32`, not in this DLL.
//! - The argument lists of the `stdcall` lines. winebuild reads them only for the i386 `@N`
//!   suffix on export names (`get_args_size` in `tools/winebuild/utils.c`). The x86_64 output has
//!   no suffix.
//! - `ExceptionAddress` of the stub record: it is the address of the stub routine, not a fixed value.
//! - The `PRIVATE` word that winebuild's `.def` output adds to stub lines: a `.def` detail, not
//!   runtime behaviour.

mod exports;
mod stubs;

pub use exports::{find_export, Export, ExportKind, EXPORTS};
pub use stubs::{
    release_cl_stmt_resources, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
