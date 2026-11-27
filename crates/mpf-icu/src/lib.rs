//! Port of the portable logic in Wine's `dlls/icu/icu.spec` and `dlls/icu/Makefile.in`.
//!
//! `dlls/icu` has no C sources. Its `@ cdecl` and `@ varargs` exports are ICU library functions,
//! and its six `@ stub` exports are the only ones the DLL defines itself. The port covers the
//! export table and the stub records.
//!
//! Ported:
//! - `icu.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The six `@ stub` exports, each modelled as the record `__wine_spec_unimplemented_stub`
//!   raises: [`close_default_icu_grouping_letters`], [`get_default_icu_grouping_letters`],
//!   [`get_icu_grouping_letter`], [`sort_close_handle`], [`sort_get_handle`],
//!   [`sort_get_search_key`], [`wine_spec_unimplemented_stub`], [`StubException`]
//! - `EXCEPTION_WINE_STUB` and `EXCEPTION_NONCONTINUABLE`, the codes that record carries
//!
//! Skipped:
//! - The bodies of the 1053 `@ cdecl` and `@ varargs` exports. They are ICU library functions
//!   linked through `ICUI18N_PE_LIBS` and `ICUCOMMON_PE_LIBS` (set by `configure`), and no
//!   source for them is in `dlls/icu`. The export table records their names and kinds only.
//! - The argument type lists in the spec, and the ordinals (winebuild numbers the exports in
//!   spec order; every export here has a name).
//! - The ordinal form of the function argument to `__wine_spec_unimplemented_stub`. The port
//!   takes `&'static str` names, and `icu.spec` has no unnamed exports.
//! - The `__cdecl` calling convention and the `for (;;)` loop around `RtlRaiseException`: the
//!   port returns the record.

mod exports;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    close_default_icu_grouping_letters, get_default_icu_grouping_letters, get_icu_grouping_letter,
    sort_close_handle, sort_get_handle, sort_get_search_key, wine_spec_unimplemented_stub,
    StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
