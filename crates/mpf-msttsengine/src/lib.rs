//! Port of the portable logic in Wine's `dlls/msttsengine/main.c` and `dlls/msttsengine/tts.c`, and of the
//! `libs/winecrt0` defaults for `DllCanUnloadNow`, `DllRegisterServer` and `DllUnregisterServer`, which
//! `dlls/msttsengine/msttsengine.spec` exports. The spec has no `@ stub` exports, so no stub model applies.
//!
//! Ported:
//! - `DllMain`: [`dll_main`], [`DLL_PROCESS_ATTACH`]
//! - `DllGetClassObject`, which serves `CLSID_TTSEngineCom`: [`dll_get_class_object`]
//! - The `IClassFactory` vtable of `ttsengine_cf`: [`ClassFactory`], [`class_factory_query_interface`],
//!   [`class_factory_add_ref`], [`class_factory_release`], [`class_factory_lock_server`],
//!   [`class_factory_create_instance`]
//! - The `struct ttsengine` object and its `ISpTTSEngine` and `ISpObjectWithToken` vtables: [`TtsEngine`],
//!   [`Interface`], [`ttsengine_create`], [`speak`], [`get_output_format`]
//! - `DllCanUnloadNow`, `DllRegisterServer` and `DllUnregisterServer`: [`dll_can_unload_now`],
//!   [`dll_register_server`], [`dll_unregister_server`]
//! - The `msttsengine.spec` export names: [`EXPORTS`], [`is_export`]
//! - The GUID, `HRESULT` and `ComPtr` values the above use, in `com.rs`: [`Guid`], [`is_equal_guid`],
//!   [`Hresult`], [`ComPtr`]
//!
//! Skipped:
//! - `TRACE` and `FIXME` logging, and `WINE_DEFAULT_DEBUG_CHANNEL`.
//! - The OS calls, which are arguments or passthroughs: `DisableThreadLibraryCalls` (a closure in
//!   [`dll_main`]), `malloc` (a `bool` in [`ttsengine_create`]), `__wine_register_resources` and
//!   `__wine_unregister_resources` (closures in [`dll_register_server`] and [`dll_unregister_server`]).
//! - The `ISpObjectToken` methods, which [`ComPtr`] models.
//! - The COM vtable dispatch. The vtable entries are plain functions. The `ISpObjectWithToken` entries forward
//!   to the `ISpTTSEngine` entries, so [`TtsEngine`] keeps one count and one `QueryInterface`.
//! - Raw `out` pointers: a `Result` stands for `*out`, and `Err` means the C leaves `*out` NULL, or untouched
//!   for the `CLSID` mismatch in `DllGetClassObject`.
//! - The `create_instance` pointer in `ttsengine_cf`: the factory calls [`ttsengine_create`] directly.

mod class_factory;
mod com;
mod dll;
mod exports;
mod tts_engine;

pub use class_factory::{
    class_factory_add_ref, class_factory_create_instance, class_factory_lock_server,
    class_factory_query_interface, class_factory_release, ClassFactory,
};
pub use com::{
    is_equal_guid, ComPtr, Guid, Hresult, CLASS_E_CLASSNOTAVAILABLE, CLASS_E_NOAGGREGATION,
    CLSID_TTS_ENGINE_COM, E_INVALIDARG, E_NOINTERFACE, E_NOTIMPL, E_OUTOFMEMORY, E_POINTER,
    IID_CLASS_FACTORY, IID_SP_OBJECT_WITH_TOKEN, IID_SP_TTS_ENGINE, IID_UNKNOWN,
    SPERR_ALREADY_INITIALIZED, S_FALSE, S_OK,
};
pub use dll::{
    dll_can_unload_now, dll_get_class_object, dll_main, dll_register_server, dll_unregister_server,
    DLL_PROCESS_ATTACH,
};
pub use exports::{is_export, EXPORTS};
pub use tts_engine::{get_output_format, speak, ttsengine_create, Interface, TtsEngine};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
