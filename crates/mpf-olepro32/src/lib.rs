//! Port of the portable logic in Wine's `dlls/olepro32/olepro32stubs.c` and
//! `dlls/olepro32/olepro32.spec`.
//!
//! Ported:
//! - `DllGetClassObject` (explicit C that returns `S_OK`): [`dll_get_class_object`], with
//!   [`HRESULT`] and [`S_OK`]
//! - The seven `olepro32.spec` forwards to `oleaut32` (ordinals 248 to 254): [`FORWARDS`],
//!   [`Forward`] and [`forward_for`]
//!
//! Skipped:
//! - `DllCanUnloadNow`, `DllRegisterServer` and `DllUnregisterServer`: declared `-private` in
//!   `olepro32.spec` with no definition under `dlls/olepro32`, so there is no reference body to
//!   port. They are not `@ stub` lines either, so the stub model does not apply.
//! - The `rclsid`, `riid` and `ppv` parameters of `DllGetClassObject`: the C body never reads or
//!   writes them, so the port takes no arguments.
//! - The `FIXME` trace, and the i386 `stdcall` argument cleanup: forwards are data, not resolved.
//! - `olepro.idl`: type-library declarations (interface and coclass GUIDs) with no runtime code.

mod class_object;
mod forwards;

pub use class_object::{dll_get_class_object, HRESULT, S_OK};
pub use forwards::{forward_for, Forward, FORWARDS};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
