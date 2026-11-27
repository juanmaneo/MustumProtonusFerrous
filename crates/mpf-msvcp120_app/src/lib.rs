//! Port of the portable logic in Wine's `dlls/msvcp120_app/msvcp120_app.spec` and `dlls/msvcp120_app/Makefile.in`.
//!
//! `dlls/msvcp120_app` has no `.c`, `.idl`, `.rc` or `.tlb` sources, so there is no executable code to port.
//! Its only content is the export table: 3810 lines, of which 2288 are `@ cdecl`, 967 `@ thiscall`,
//! 160 `@ extern` and 395 `@ stub`. Every non-stub line forwards to `msvcp120.<name>`. This is a partial
//! port: the table, the forwards, the stub records and the DLL name are modelled.
//!
//! Ported:
//! - `msvcp120_app.spec` export table: [`EXPORTS`], [`Export`], [`ExportKind`], [`Arch`], [`find_export`]
//! - the forward of each non-stub export: [`forward`], [`Forward`], [`FORWARD_MODULE`]
//! - every `@ stub` export (395 lines): one helper per export in `stub_exports`, each built on
//!   [`wine_spec_unimplemented_stub`] and returning the [`StubException`] that the stub raises.
//!   Every `-arch=` variant is kept.
//! - the `__wine_spec_unimplemented_stub` record and its constants: [`wine_spec_unimplemented_stub`],
//!   [`StubException`], [`EXCEPTION_WINE_STUB`], [`EXCEPTION_NONCONTINUABLE`]
//!
//! Skipped:
//! - The forwarded functions and data. They are exports of `msvcp120.dll`, which this DLL only points to.
//! - The `-ret64` flag (79 lines) and the `-norelay` flag (5 lines): winebuild ABI and relay options that
//!   no runtime code in this DLL reads.
//! - Arch selection: a build exports only one variant of each `-arch=` group. The table keeps all of them.
//! - The C++ runtime behind the stubbed exports. Wine has no implementation, so the port returns the
//!   exception instead of looping on it.

mod exports;
mod stub;
mod stub_exports;

pub use exports::{
    find_export, forward, Arch, Export, ExportKind, Forward, EXPORTS, FORWARD_MODULE,
};
pub use stub::{
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};
pub use stub_exports::*;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
