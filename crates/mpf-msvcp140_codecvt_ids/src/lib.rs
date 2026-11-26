//! Port of the portable logic in Wine's `dlls/msvcp140_codecvt_ids/main.c` and
//! `dlls/msvcp140_codecvt_ids/msvcp140_codecvt_ids.spec`.
//!
//! Ported:
//! - The four `@ extern` data exports and the `locale_id` storage that `main.c` defines for
//!   each one: [`EXPORTS`], [`LocaleIdExport`], [`LocaleId`]
//! - The lookup from an exported name to its C symbol and initial id: [`export_by_name`]
//!
//! Skipped:
//! - The id numbering. This DLL only defines zeroed storage. The `++_Id_cnt` numbering that
//!   gives an id its value on first use lives in the msvcp runtime, for example in
//!   `dlls/msvcp90/locale.c`.
//! - The `@ stub` model. The spec has no `@ stub` exports, and `main.c` has no C functions.

mod exports;

pub use exports::{export_by_name, LocaleId, LocaleIdExport, EXPORTS};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
