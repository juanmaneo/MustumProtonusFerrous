//! Port of the portable logic in Wine's `dlls/msisys.ocx/msisys.c` and `dlls/msisys.ocx/msisys.ocx.spec`.
//!
//! Ported:
//! - `DllGetClassObject`, which always returns `CLASS_E_CLASSNOTAVAILABLE`: [`dll_get_class_object`]
//! - `HRESULT` and `CLASS_E_CLASSNOTAVAILABLE` from `include/winerror.h`: [`Hresult`],
//!   [`CLASS_E_CLASSNOTAVAILABLE`]
//! - The export declarations in `msisys.ocx.spec`: [`ExportKind`], [`export_kind`]
//!
//! Skipped:
//! - The arguments of `DllGetClassObject` (`pclsid`, `piid`, `ppv`). The C body never reads
//!   them, and its only other use is a `FIXME` trace, so the port takes none. The result is the
//!   same for NULL and non-NULL arguments, and the C body never writes `*ppv`.
//! - `DllCanUnloadNow`, `DllRegisterServer`, `DllUnregisterServer`: declared `-private` in
//!   `msisys.ocx.spec` with no body in `msisys.c`, so there is no behaviour to port.
//!   [`export_kind`] classifies them only.
//! - The `__wine_spec_unimplemented_stub` model: `msisys.ocx.spec` has no `@ stub` exports.

mod class_object;
mod spec;

pub use class_object::{dll_get_class_object, Hresult, CLASS_E_CLASSNOTAVAILABLE};
pub use spec::{export_kind, ExportKind};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
