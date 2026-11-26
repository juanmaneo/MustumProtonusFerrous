//! Port of the portable logic in Wine's `dlls/esent/esent.spec` and `dlls/esent/Makefile.in`.
//!
//! Wine's esent has no `.c`, `.idl`, `.rc` or `.tlb` source, so no export has a body to port.
//! All 336 exports are `@ stub`, and the port models what those stubs do.
//!
//! Ported:
//! - The 336 `@ stub` exports, in spec order: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The `__wine_spec_unimplemented_stub` record each stub raises: [`wine_spec_unimplemented_stub`],
//!   [`StubException`], [`EXCEPTION_WINE_STUB`], [`EXCEPTION_NONCONTINUABLE`]
//! - One helper per export, named after the C export in snake_case (`JetAddColumn` becomes
//!   `jet_add_column`). Each raises the record for its export with the module `esent.dll`, from
//!   `MODULE` in `dlls/esent/Makefile.in`. The helpers are re-exported at the crate root.
//!
//! Skipped:
//! - Ordinals: `esent.spec` gives no ordinal numbers, so the record carries the export name.
//!   The ordinal form of `__wine_spec_unimplemented_stub` is not modelled.
//! - The winebuild thunk assembly and the `.L__wine_spec_file_name` string. The port passes the
//!   module and function names directly.
//! - `EXTRADLLFLAGS = -Wb,--prefer-native` in `Makefile.in`, which is a build flag.

mod export_stubs;
mod exports;
mod stub;

pub use export_stubs::*;
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
