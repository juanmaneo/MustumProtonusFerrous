//! Port of the portable logic in Wine's `dlls/msauddecmft/msauddecmft.c` and `dlls/msauddecmft/msauddecmft.spec`,
//! and of the `DllCanUnloadNow` default in `libs/winecrt0/dll_canunload.c` that the spec exports.
//!
//! Ported:
//! - The `IClassFactory` vtable: [`class_factory_query_interface`], [`class_factory_add_ref`],
//!   [`class_factory_release`], [`class_factory_lock_server`], [`ClassFactory`]
//! - `aac_decoder_factory_CreateInstance`, with the `CLSID_wg_aac_decoder` it requests:
//!   [`aac_decoder_factory_create_instance`]
//! - `DllGetClassObject` and `DllCanUnloadNow`: [`dll_get_class_object`], [`dll_can_unload_now`]
//! - `DllRegisterServer` and `DllUnregisterServer`, with the `MFTRegister` table of the AAC decoder:
//!   [`dll_register_server`], [`dll_unregister_server`], [`AAC_DECODER_REGISTRATION`],
//!   [`MftRegistration`], [`MftRegisterTypeInfo`]
//! - The `msauddecmft.spec` export names: [`EXPORTS`], [`is_export`]
//! - The GUID and `HRESULT` values the above use: [`Guid`], [`is_equal_guid`], [`Hresult`], [`failed`]
//!
//! Skipped:
//! - `TRACE` and `FIXME` logging, and `WINE_DEFAULT_DEBUG_CHANNEL`.
//! - The OS calls, which are closure arguments: `CoCreateInstance`, `__wine_register_resources`,
//!   `__wine_unregister_resources`, `MFTRegister` and `MFTUnregister`.
//! - The `outer` and `out` arguments of `CreateInstance`, and the `attributes` argument of `MFTRegister`.
//! - The COM vtable dispatch: the vtable functions are plain functions.
//! - The AAC decoder itself: `CLSID_wg_aac_decoder` is implemented in `dlls/winegstreamer`, not in this DLL.
//! - `__wine_spec_unimplemented_stub`: the spec has no `@ stub` exports.

mod class_factory;
mod dll;
mod exports;
mod guid;
mod hresult;
mod registration;

pub use class_factory::{
    aac_decoder_factory_create_instance, class_factory_add_ref, class_factory_lock_server,
    class_factory_query_interface, class_factory_release, ClassFactory, CLSCTX_INPROC_SERVER,
};
pub use dll::{dll_can_unload_now, dll_get_class_object};
pub use exports::{is_export, EXPORTS};
pub use guid::{
    is_equal_guid, Guid, CLSID_MSAAC_DEC_MFT, CLSID_WG_AAC_DECODER, IID_CLASS_FACTORY, IID_UNKNOWN,
};
pub use hresult::{failed, Hresult, CLASS_E_CLASSNOTAVAILABLE, E_NOINTERFACE, S_FALSE, S_OK};
pub use registration::{
    dll_register_server, dll_unregister_server, MftRegisterTypeInfo, MftRegistration,
    AAC_DECODER_REGISTRATION, MFT_CATEGORY_AUDIO_DECODER, MFT_ENUM_FLAG_SYNCMFT,
    MF_AUDIO_FORMAT_AAC, MF_AUDIO_FORMAT_ADTS, MF_AUDIO_FORMAT_FLOAT, MF_AUDIO_FORMAT_PCM,
    MF_AUDIO_FORMAT_RAW_AAC, MF_MEDIA_TYPE_AUDIO,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
