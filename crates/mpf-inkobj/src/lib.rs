//! Port of the portable logic in Wine's `dlls/inkobj/inkobj.c` and `dlls/inkobj/inkobj.spec`.
//!
//! Ported:
//! - `DllGetClassObject`: always `CLASS_E_CLASSNOTAVAILABLE`, for any class and interface ID:
//!   [`dll_get_class_object`], [`Guid`], [`Hresult`], [`CLASS_E_CLASSNOTAVAILABLE`]
//! - `inkobj.spec` export declarations: [`ExportKind`], [`export_kind`]
//!
//! Skipped:
//! - `DllCanUnloadNow`, `DllRegisterServer` and `DllUnregisterServer`: declared `-private` in
//!   `inkobj.spec` with no body in `dlls/inkobj/inkobj.c`, so [`export_kind`] classifies them
//!   only. The default bodies are in `libs/winecrt0`. Its `DllCanUnloadNow` returns `S_FALSE`.
//!   The registration pair runs the module's `WINE_REGISTRY` resources through ATL's
//!   `IRegistrar`, which needs the OS registry and COM, so it has no portable behaviour to port.
//! - The `FIXME` trace of `rclsid` (`debugstr_guid`), and the `ppv` out-pointer, which the C body
//!   never reads or writes.
//! - The `WINAPI` calling convention and the `WINE_DEFAULT_DEBUG_CHANNEL` trace channel.

mod class_object;
mod exports;

pub use class_object::{dll_get_class_object, Guid, Hresult, CLASS_E_CLASSNOTAVAILABLE};
pub use exports::{export_kind, ExportKind};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
