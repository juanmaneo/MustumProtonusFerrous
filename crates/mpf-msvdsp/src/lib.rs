//! Port of the portable logic in Wine's `dlls/msvdsp/msvdsp.c` and `dlls/msvdsp/msvdsp.spec`.
//! `DllCanUnloadNow` is not defined in msvdsp.c; msvdsp.spec exports it from
//! `libs/winecrt0/dll_canunload.c`.
//!
//! Ported:
//! - `DllRegisterServer` and `DllUnregisterServer`: the OS call order, the stop at the first
//!   failed HRESULT, and the MFT and DMO registration tables for "Frame Rate Converter" and
//!   "Color Control": [`dll_register_server`], [`dll_unregister_server`], [`Registrar`],
//!   [`MftRegisterTypeInfo`], [`DmoPartialMediaType`]
//! - `DllGetClassObject`, which serves no class: [`dll_get_class_object`]
//! - `DllCanUnloadNow`, which always returns `S_FALSE`: [`dll_can_unload_now`]
//! - The GUIDs the tables use: [`Guid`], [`CLSID_C_FRAME_RATE_CONVERT_DMO`],
//!   [`CLSID_C_COLOR_CONTROL_DMO`], [`MFT_CATEGORY_VIDEO_EFFECT`], [`DMOCATEGORY_VIDEO_EFFECT`]
//! - The `HRESULT` values and the `FAILED` test: [`HResult`], [`failed`]
//!
//! Skipped:
//! - The COM and OS work behind [`Registrar`] (`__wine_register_resources`, `MFTRegister`,
//!   `DMORegister` and their unregister counterparts). The port passes the same arguments to a
//!   caller-supplied `Registrar` and registers nothing itself.
//! - The `TRACE` and `FIXME` output.
//! - `MFVideoFormat_ABGR32`, which msvdsp.c defines but no table uses.
//! - The `attributes` argument of `MFTRegister`, which the C call always passes as NULL.

mod class_object;
mod guid;
mod hresult;
mod register;

pub use class_object::{dll_can_unload_now, dll_get_class_object};
pub use guid::{
    Guid, CLSID_C_COLOR_CONTROL_DMO, CLSID_C_FRAME_RATE_CONVERT_DMO, DMOCATEGORY_VIDEO_EFFECT,
    MFT_CATEGORY_VIDEO_EFFECT,
};
pub use hresult::{
    failed, HResult, CLASS_E_CLASSNOTAVAILABLE, E_ACCESSDENIED, E_FAIL, E_OUTOFMEMORY, S_FALSE,
    S_OK,
};
pub use register::{
    dll_register_server, dll_unregister_server, DmoPartialMediaType, MftRegisterTypeInfo, Registrar,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
