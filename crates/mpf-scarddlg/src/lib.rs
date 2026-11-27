//! Port of the portable logic in Wine's `dlls/scarddlg/scarddlg.spec` and `dlls/scarddlg/Makefile.in`.
//!
//! The DLL has no `.c`, `.idl`, `.rc` or `.tlb` sources. All five exports are `@ stub` lines in
//! `scarddlg.spec`, so the port models the export table, the stub records and the constants they use.
//!
//! Ported:
//! - `scarddlg.spec` export table: [`EXPORTS`], [`export_ordinal`]
//! - The five `@ stub` exports: [`get_open_card_name_a`], [`get_open_card_name_w`],
//!   [`scard_dlg_extended_error`], [`scard_ui_dlg_select_card_a`], [`scard_ui_dlg_select_card_w`]
//! - `__wine_spec_unimplemented_stub` and the record it raises: [`wine_spec_unimplemented_stub`],
//!   [`StubException`], with [`EXCEPTION_WINE_STUB`] and [`EXCEPTION_NONCONTINUABLE`]
//!
//! Skipped:
//! - The `__cdecl` calling convention and the `RtlRaiseException` loop: the port returns the record
//!   instead of raising it.
//! - `ExceptionRecord` (NULL in every oracle row) and `ExceptionAddress` (the stub's own address):
//!   the record does not model them.
//! - The ordinal form of `function` that winebuild uses for unnamed exports. Every export in
//!   `scarddlg.spec` has a name.
//! - `EXTRADLLFLAGS = -Wb,--prefer-native` in `Makefile.in`: a build flag, not runtime behaviour.

mod exports;
mod stub;

pub use exports::{export_ordinal, EXPORTS};
pub use stub::{
    get_open_card_name_a, get_open_card_name_w, scard_dlg_extended_error,
    scard_ui_dlg_select_card_a, scard_ui_dlg_select_card_w, wine_spec_unimplemented_stub,
    StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
