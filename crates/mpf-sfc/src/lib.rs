//! Port of the portable logic in Wine's `dlls/sfc/sfc.spec` and `dlls/sfc/Makefile.in`.
//!
//! `dlls/sfc` has no C sources and no entry point (`-Wl,-noentry`). All sixteen `sfc.spec`
//! entries forward to `sfc_os.dll`, so the DLL itself runs no code.
//!
//! Ported:
//! - The `sfc.spec` export table in spec order, with the ordinals winebuild assigned: [`EXPORTS`],
//!   [`Export`]
//! - The name table that winebuild sorts for the name pointers, with the index as the hint:
//!   [`name_table`]
//! - Lookup by name and by ordinal: [`export_by_name`], [`export_by_ordinal`]
//! - The split of a forward target at its last `.`, as the loader does it:
//!   [`Export::forward_target`]
//!
//! Skipped:
//! - The forwarded functions. They are the `sfc_os.dll` exports, ported in `mpf-sfc_os`.
//! - `@ stub` records and `__wine_spec_unimplemented_stub`. `sfc.spec` has no `@ stub` entry.
//! - The `winebuild` parser. Its ordinal assignment is recorded as the literal values in
//!   [`EXPORTS`], and the oracle rows check those values.

mod exports;

pub use exports::{export_by_name, export_by_ordinal, name_table, Export, EXPORTS};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
