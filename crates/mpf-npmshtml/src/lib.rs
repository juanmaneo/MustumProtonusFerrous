//! Port of the portable logic in Wine's `dlls/npmshtml/main.c` and `dlls/npmshtml/npmshtml.spec`.
//!
//! Ported:
//! - `NP_Initialize` and `NP_Shutdown` return values: [`np_initialize`], [`np_shutdown`]
//! - `npmshtml.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`], including the
//!   `NP_GetEntryPoints` forward to `mshtml`
//!
//! Skipped:
//! - The `arg` of `NP_Initialize`: the C body never reads it, so the port takes it as an opaque
//!   pointer and does not dereference it.
//! - The code behind `NP_GetEntryPoints`: the spec forwards it to `mshtml`, which is not ported.
//!   Only the forward target is modelled.
//! - The `stdcall` calling convention, which the Rust ABI does not model.
//! - The `VER_*` and `MIMEType` values in `dlls/npmshtml/Makefile.in`, which are build metadata.
//! - `@ stub` exports: the spec has none, so `__wine_spec_unimplemented_stub` is not modelled.

mod exports;
mod plugin;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use plugin::{np_initialize, np_shutdown};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
