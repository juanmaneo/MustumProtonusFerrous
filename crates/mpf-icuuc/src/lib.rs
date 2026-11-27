//! Port of the portable logic in Wine's `dlls/icuuc/icuuc.spec`.
//!
//! Ported:
//! - the `icuuc.spec` export table: [`EXPORTS`], [`Export`], [`ExportKind`] and [`find_export`]
//! - the forwarder each `@` line sets up, `icu.<name>`: [`forward_target`] and [`FORWARD_MODULE`]
//!
//! Skipped:
//! - Function bodies. `dlls/icuuc` has no `.c`, `.idl`, `.rc` or `.tlb` file. Every export is a
//!   forwarder to the `icu` module, and the code it reaches is not part of this DLL.
//! - `@ stub` records and `__wine_spec_unimplemented_stub`. The spec has no `@ stub` lines.
//! - Ordinals. The spec gives none, so the port keeps spec order and does not model how
//!   winebuild assigns them.
//! - Argument types stay as the spec's text and are not converted to C types.

mod exports;

pub use exports::{find_export, forward_target, Export, ExportKind, EXPORTS, FORWARD_MODULE};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
