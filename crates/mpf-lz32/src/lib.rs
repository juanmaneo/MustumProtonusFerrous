//! Port of the portable logic in Wine's `dlls/lz32/lz32.spec` and `dlls/lz32/Makefile.in`.
//!
//! `dlls/lz32` has no C sources. Its `lz32.spec` lists twelve `@ stdcall` exports, each with a
//! `kernel32.<name>` forward target, so the DLL carries no code of its own.
//!
//! Ported:
//! - `lz32.spec` export table: [`EXPORTS`], [`Export`], [`ArgKind`] and [`export`], with the
//!   i386 byte count that `winebuild` writes as `@N` ([`Export::stdcall_bytes`]), the forward
//!   target split ([`Export::forward`]) and the `.def` line ([`Export::def_line`])
//! - the `.def` body that `winebuild --def` derives from the table: [`def_body`]
//! - `Makefile.in` constants: [`MODULE_NAME`], [`IMPORT_LIB`] and [`PRODUCT_VERSION`]
//!
//! Skipped:
//! - The LZ routines (`LZOpenFile`, `LZRead`, `GetExpandedName` and the rest). The forwards
//!   resolve to `dlls/kernel32/lzexpand.c`, which is a different component.
//! - `dlls/lz32/tests/lzexpand_main.c`, the test suite for the forwarded functions.
//! - The `-Wl,-noentry` link flag, which only affects the PE entry point.
//! - The `@ stub` model: `lz32.spec` has no stub entries.

mod exports;
mod module;

pub use exports::{export, ArgKind, Export, EXPORTS};
pub use module::{def_body, IMPORT_LIB, MODULE_NAME, PRODUCT_VERSION};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
