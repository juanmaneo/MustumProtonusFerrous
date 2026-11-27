//! Port of the portable logic in Wine's `dlls/psapi/psapi.spec` and `dlls/psapi/Makefile.in`.
//!
//! `dlls/psapi` has no C sources. Its `psapi.spec` lists 27 `@ stdcall -import` exports. Each
//! one is an import thunk that jumps to the `K32` symbol named after the signature, whose body
//! lives in `dlls/kernel32` and `dlls/kernelbase`. The DLL therefore carries no code of its own,
//! and the port is partial.
//!
//! Ported:
//! - `psapi.spec` export table: [`EXPORTS`], [`Export`], [`ArgKind`] and [`export`], with the
//!   i386 byte count that `winebuild` writes as `@N` ([`Export::stdcall_bytes`]) and the `.def`
//!   line ([`Export::def_line`]), which carries the `K32` link target
//! - the `.def` body that `winebuild --def` derives from the table: [`def_body`]
//! - `Makefile.in` constants: [`MODULE_NAME`], [`IMPORT_LIB`], [`PRODUCT_VERSION`] and
//!   [`FILE_DESCRIPTION`]
//!
//! Skipped:
//! - The `K32*` function bodies, which are not in `dlls/psapi`.
//! - `dlls/psapi/tests/psapi_main.c`, the test suite for the imported functions.
//! - The `@ stub` model (`__wine_spec_unimplemented_stub`): `psapi.spec` has no stub entries.
//! - Argument kinds other than `long` and `ptr`, which `psapi.spec` never uses.
//! - The `NONAME`, `DATA` and `PRIVATE` `.def` flags, which no psapi export sets.

mod exports;
mod module;

pub use exports::{export, ArgKind, Export, EXPORTS};
pub use module::{def_body, FILE_DESCRIPTION, IMPORT_LIB, MODULE_NAME, PRODUCT_VERSION};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
