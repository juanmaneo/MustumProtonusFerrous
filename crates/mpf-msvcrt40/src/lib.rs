//! Port of the portable logic in Wine's `dlls/msvcrt40/msvcrt40.spec` and `dlls/msvcrt40/Makefile.in`.
//!
//! `dlls/msvcrt40/` has no `.c`, `.idl`, `.rc` or `.tlb` sources. Its spec defines 1164 exports:
//! 1162 forwards to `msvcrt.dll` or `msvcirt.dll`, and two `@ stub` exports. The port models the
//! export table, the forward of each entry, the stub records and the DLL name. The verdict is
//! partial because the DLL has no code of its own.
//!
//! Ported:
//! - `msvcrt40.spec` export table: [`EXPORTS`], [`Export`], [`ExportKind`], [`find_export`]
//! - the forward of each export: [`forward`], [`Forward`]
//! - `@ stub` records: [`StubException`] and [`wine_spec_unimplemented_stub`], plus one helper per
//!   stub export: [`iostream_init_vector_deleting_dtor`] and [`iostream_init_scalar_deleting_dtor`]
//! - `MODULE = msvcrt40.dll` from `Makefile.in`: [`MSVCRT40_DLL_NAME`]
//!
//! Skipped:
//! - The forwarded functions. They are exports of `msvcrt.dll` and `msvcirt.dll`, which this DLL
//!   only points to.
//! - The ordinals. Winebuild numbers the entries from 1 in spec order (`assign_ordinals` in
//!   `tools/winebuild/parser.c`), and the table keeps spec order.
//! - The `-norelay` and `-ret64` flags on 12 forwards. Winebuild emits no relay for a forward
//!   (`tools/winebuild/spec32.c`), so the flags change nothing here.
//! - The `-private` flag on `atexit`. Winebuild leaves private entries out of the import list
//!   (`tools/winebuild/import.c`). The table keeps `atexit` as a forward.
//! - The argument lists, which describe the forwarded functions.

mod exports;
mod stub;

pub use exports::{find_export, forward, Export, ExportKind, Forward, EXPORTS};
pub use stub::{
    iostream_init_scalar_deleting_dtor, iostream_init_vector_deleting_dtor,
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
    MSVCRT40_DLL_NAME,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
