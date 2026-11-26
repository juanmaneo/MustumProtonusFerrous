//! Port of the portable logic in Wine's `dlls/dxtrans/main.c`, plus the
//! stub-export behaviour declared in `dlls/dxtrans/dxtrans.spec`.
//!
//! Ported:
//! - `DllGetClassObject`: always `CLASS_E_CLASSNOTAVAILABLE`, ignoring its arguments:
//!   [`dll_get_class_object`], [`CLASS_E_CLASSNOTAVAILABLE`]
//! - Unimplemented `@ stub` exports: the `__wine_spec_unimplemented_stub` path
//!   from `libs/winecrt0/stub.c`, which never returns and raises
//!   `EXCEPTION_WINE_STUB` non-continuably: [`unimplemented_stub`], [`StubException`],
//!   [`STUB_EXPORTS`]
//!
//! Skipped:
//! - `DllCanUnloadNow`, `DllRegisterServer`, `DllUnregisterServer`: listed as
//!   `-private` exports in `dxtrans.spec` but not defined in `main.c`.
//! - The `DXConstOverArray`, `DXDitherArray` and similar DirectX transform
//!   implementations: not present in this Wine source; they are stubs only.
//! - The i386 and ARM mangled names of the DX stubs: the same stub behaviour
//!   applies; only the win64 names are listed in [`STUB_EXPORTS`].

mod class_object;
mod stub_exports;

pub use class_object::{dll_get_class_object, CLASS_E_CLASSNOTAVAILABLE};
pub use stub_exports::{
    unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB, STUB_EXPORTS,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
