//! Port of the portable logic in Wine's `dlls/msvcr120_app/msvcr120_app.spec` and `dlls/msvcr120_app/Makefile.in`.
//!
//! `dlls/msvcr120_app` has no `.c`, `.idl`, `.rc` or `.tlb` sources, so the DLL has no code of its own.
//! Its content is the export table: 1758 entries forwarded to `msvcr120.dll` under the same name, and
//! 400 `@ stub` entries.
//!
//! Ported:
//! - The `msvcr120_app.spec` export table: [`EXPORTS`], [`Export`], [`ExportKind`], [`Arch`], [`find_export`]
//! - `MODULE = msvcr120_app.dll` from `Makefile.in`: [`MSVCR120_APP_DLL_NAME`]
//! - `__wine_spec_unimplemented_stub` and the record it raises: [`wine_spec_unimplemented_stub`],
//!   [`StubException`], [`EXCEPTION_WINE_STUB`], [`EXCEPTION_NONCONTINUABLE`]
//! - One helper per `@ stub` export, 400 in all, each returning the record its stub raises
//!   (re-exported from the private `stubs` module)
//!
//! Skipped:
//! - The 1758 forwarded functions: their code lives in `dlls/msvcr120`, not in this DLL.
//! - The stub bodies: winebuild generates them, and each one raises the record and never returns.
//! - Calling conventions, argument lists, and the `-norelay` and `-ret64` flags. They describe the
//!   target's ABI or winebuild's thunks, not behaviour that runs without the OS.
//! - Ordinals, which the spec does not state.
//! - The two `# extern` comment lines (`__lconv` and `_mbcasemap`), which winebuild ignores.
//! - The `ExceptionRecord` (NULL) and `ExceptionAddress` fields of the stub record.

mod exports;
mod stub;
mod stubs;

pub use exports::{find_export, Arch, Export, ExportKind, EXPORTS};
pub use stub::{
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
    MSVCR120_APP_DLL_NAME,
};
pub use stubs::*;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
