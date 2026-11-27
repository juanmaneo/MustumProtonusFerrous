//! Port of the portable logic in Wine's `dlls/icuin/icuin.spec` and `dlls/icuin/Makefile.in`.
//!
//! `dlls/icuin` has no `.c`, `.idl`, `.rc` or `.tlb` sources and no `@ stub` exports. Its only
//! content is the export table: 416 `@ cdecl` and `@ varargs` lines, each forwarding to
//! `icu.<name>`. Wine's `dlls/icu/icu.spec` exports every forwarded name. The port models the
//! table, the forward of each entry and the DLL name. It is a partial port: the DLL has no code.
//!
//! Ported:
//! - `icuin.spec` export table: [`EXPORTS`], [`Export`], [`ExportKind`], [`find_export`]
//! - the forward of each export: [`forward`], [`Forward`], [`FORWARD_MODULE`]
//! - `MODULE = icuin.dll` from `Makefile.in`: [`ICUIN_DLL_NAME`]
//!
//! Skipped:
//! - The forwarded functions. They are exports of `icu.dll`, which this DLL only points to.
//! - The `__wine_spec_unimplemented_stub` model. No export is `@ stub`, so nothing here raises
//!   `EXCEPTION_WINE_STUB`.
//! - `EXTRADLLFLAGS = -Wl,-noentry` in `Makefile.in`: a link option that leaves the DLL without
//!   an entry point.

mod exports;

pub use exports::{
    find_export, forward, Export, ExportKind, Forward, EXPORTS, FORWARD_MODULE, ICUIN_DLL_NAME,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
