//! Port of the portable logic in Wine's `dlls/qedit/main.c`, `dlls/qedit/qedit.spec` and the
//! `DllCanUnloadNow` default in `libs/winecrt0/dll_canunload.c`.
//!
//! Ported:
//! - `DllGetClassObject` IID and CLSID dispatch and the class table: [`dll_get_class_object`],
//!   [`ClassId`]
//! - The class factory's reference counting, `QueryInterface`, `CreateInstance` gate and
//!   `LockServer`: [`ClassFactory`]
//! - `DllCanUnloadNow`: [`dll_can_unload_now`]
//! - The filter registration data `reg_sample_grabber` and `reg_null_renderer` as `REGFILTER2`:
//!   [`RegFilter2`], [`RegFilterPins2`], [`RegPinTypes`], [`SAMPLE_GRABBER_FILTER`],
//!   [`NULL_RENDERER_FILTER`]
//! - The `DllRegisterServer` and `DllUnregisterServer` sequencing: [`dll_register_server`],
//!   [`dll_unregister_server`], [`FilterMapper`]
//!
//! Skipped:
//! - `qedit.spec` has no `@ stub` entries. All four exports are `-private`, so the
//!   `__wine_spec_unimplemented_stub` model does not apply.
//! - `timeline_create`, `media_detector_create`, `null_renderer_create` and
//!   `sample_grabber_create`: the COM objects are not ported. [`ClassFactory::create_instance`]
//!   takes a closure in their place.
//! - `__wine_register_resources` and `__wine_unregister_resources` (they read the PE resources)
//!   and `CoCreateInstance`: their HRESULTs are inputs.
//! - `CoTaskMemAlloc` and `CoTaskMemFree`: an allocation flag is the input, and the port has
//!   nothing to free.
//! - `DllMain`, the `TRACE`, `WARN` and `FIXME` calls, the `InterlockedIncrement` and
//!   `InterlockedDecrement` atomics, and the `ppobj` out-parameter of `CreateInstance`.

mod class_factory;
mod com;
mod dll;
mod registration;

pub use class_factory::{ClassFactory, ClassId};
pub use com::{
    Guid, Hresult, CLASS_E_CLASSNOTAVAILABLE, CLSID_AMTIMELINE, CLSID_MEDIADET, CLSID_NULLRENDERER,
    CLSID_SAMPLEGRABBER, E_FAIL, E_NOINTERFACE, E_OUTOFMEMORY, GUID_NULL, IID_ICLASSFACTORY,
    IID_IUNKNOWN, REGDB_E_CLASSNOTREG, S_FALSE, S_OK,
};
pub use dll::{dll_can_unload_now, dll_get_class_object};
pub use registration::{
    dll_register_server, dll_unregister_server, FilterMapper, RegFilter2, RegFilterPins2,
    RegPinTypes, MERIT_DO_NOT_USE, NULL_RENDERER_FILTER, REG_PINFLAG_B_OUTPUT,
    SAMPLE_GRABBER_FILTER,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
