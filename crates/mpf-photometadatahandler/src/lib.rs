//! Port of the portable logic in Wine's `dlls/photometadatahandler/photometadatahandler.spec` and
//! `dlls/photometadatahandler/Makefile.in`.
//!
//! The directory has no `.c`, `.idl`, `.rc` or `.tlb` sources, so there is no executable C logic to
//! port. The port covers the spec's export table, its one `@ stub` export, and the makefile settings.
//!
//! Ported:
//! - `photometadatahandler.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`], [`is_exported`]
//! - `DllGetClassObject` (`@ stub`) and the `__wine_spec_unimplemented_stub` record it raises:
//!   [`dll_get_class_object`], [`wine_spec_unimplemented_stub`], [`StubException`]
//! - `Makefile.in` settings: [`makefile_settings`], [`MakefileSettings`]
//!
//! Skipped:
//! - `DllCanUnloadNow`, `DllRegisterServer` and `DllUnregisterServer`: `stdcall -private` in the spec,
//!   with no implementation in this directory. Winebuild keeps private names out of the export table
//!   and emits no stub for them, so only their spec entries are ported.
//! - The ordinal form of `__wine_spec_unimplemented_stub`: every export in the spec has a name.

mod exports;
mod makefile;
mod stub;

pub use exports::{export_kind, is_exported, ExportKind, EXPORTS};
pub use makefile::{makefile_settings, MakefileSettings};
pub use stub::{
    dll_get_class_object, wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
