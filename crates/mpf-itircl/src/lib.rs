//! Port of the portable logic in Wine's `dlls/itircl/itircl.spec`. The DLL has no C sources in
//! the reference, so the port covers its export table and the stub record for its one `@ stub`
//! export.
//!
//! Ported:
//! - `itircl.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - `DllGetClassObject` (`@ stub`) and the `__wine_spec_unimplemented_stub` record it raises:
//!   [`dll_get_class_object`], [`wine_spec_unimplemented_stub`], [`StubException`]
//!
//! Skipped:
//! - `DllCanUnloadNow`, `DllRegisterServer` and `DllUnregisterServer` (`@ stdcall -private`).
//!   This reference has no C body for them, so their behaviour is not defined. Only their table
//!   entries are ported.
//! - The ordinal form of the `function` argument, which `winebuild` uses for unnamed exports.
//!   `itircl.spec` has no unnamed exports.
//! - The `__cdecl` calling convention and the `-Wb,--prefer-native` build flag.

mod exports;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    dll_get_class_object, wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
