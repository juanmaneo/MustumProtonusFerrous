//! Port of the portable logic in Wine's `dlls/wmadmod/wmadmod.c`, and of the `DllCanUnloadNow`
//! default in `libs/winecrt0/dll_canunload.c` that `dlls/wmadmod/wmadmod.spec` exports.
//!
//! Ported:
//! - Class factory methods `class_factory_QueryInterface`, `class_factory_AddRef`,
//!   `class_factory_Release` and `class_factory_LockServer`: [`class_factory_query_interface`],
//!   [`class_factory_add_ref`], [`class_factory_release`], [`class_factory_lock_server`]
//! - `DllGetClassObject` clsid dispatch: [`dll_get_class_object`]
//! - `DllRegisterServer` and `DllUnregisterServer` step chains and their MFT and DMO
//!   registration tables: [`dll_register_server`], [`dll_unregister_server`], [`WMA_DECODER_MFT`],
//!   [`WMA_DECODER_DMO`]
//! - `DllCanUnloadNow` (the winecrt0 default, `S_FALSE`): [`dll_can_unload_now`]
//! - The `dlls/wmadmod/wmadmod.spec` export list: [`EXPORTS`], [`is_export`]
//! - The `HRESULT` values and the `FAILED` test they rely on: [`HResult`], [`failed`]
//!
//! Skipped:
//! - `wma_decoder_factory_CreateInstance`, which forwards to `winegstreamer_create_wma_decoder`
//!   (a COM object created by `dlls/winegstreamer`).
//! - `__wine_register_resources`, `__wine_unregister_resources`, `MFTRegister`, `DMORegister`,
//!   `MFTUnregister` and `DMOUnregister`. These are OS and registry calls. The port takes each as a
//!   closure argument, and the step results drive the control flow.
//! - The `FIXME` and `TRACE` logging, the COM vtable layout and reference counts, and the
//!   `IClassFactory_QueryInterface` macro dispatch. The factory object is not modelled, so the
//!   methods take no receiver.
//! - The spec has no `@ stub` exports, so no `StubException` is modelled.

mod class_factory;
mod exports;
mod guid;
mod hresult;
mod registration;

pub use class_factory::{
    class_factory_add_ref, class_factory_lock_server, class_factory_query_interface,
    class_factory_release, dll_get_class_object,
};
pub use exports::{dll_can_unload_now, is_export, EXPORTS};
pub use guid::{
    is_equal_guid, Guid, CLSID_WMA_DEC_MEDIA_OBJECT, DMO_CATEGORY_AUDIO_DECODER,
    IID_ICLASS_FACTORY, IID_IUNKNOWN, MFT_CATEGORY_AUDIO_DECODER,
};
pub use hresult::{
    failed, HResult, CLASS_E_CLASSNOTAVAILABLE, E_ACCESSDENIED, E_FAIL, E_NOINTERFACE, E_NOTIMPL,
    E_UNEXPECTED, S_FALSE, S_OK,
};
pub use registration::{
    dll_register_server, dll_unregister_server, DmoRegistration, MediaType, MftRegistration,
    MEDIA_TYPE_AUDIO, MFT_ENUM_FLAG_SYNCMFT, SUBTYPE_IEEE_FLOAT, SUBTYPE_MSAUDIO1, SUBTYPE_PCM,
    SUBTYPE_WMAUDIO2, SUBTYPE_WMAUDIO3, SUBTYPE_WMAUDIO_LOSSLESS, WMA_DECODER_DMO,
    WMA_DECODER_INPUTS, WMA_DECODER_MFT, WMA_DECODER_OUTPUTS,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
