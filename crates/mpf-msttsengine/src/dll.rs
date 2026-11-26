//! `DllMain` and `DllGetClassObject` from `dlls/msttsengine/main.c`, and the `DllCanUnloadNow`,
//! `DllRegisterServer` and `DllUnregisterServer` defaults from `libs/winecrt0` that the spec exports.

use crate::class_factory::{class_factory_query_interface, ClassFactory};
use crate::com::{
    is_equal_guid, Guid, Hresult, CLASS_E_CLASSNOTAVAILABLE, CLSID_TTS_ENGINE_COM, S_FALSE,
};

/// `DLL_PROCESS_ATTACH`, from `include/winnt.h`.
pub const DLL_PROCESS_ATTACH: u32 = 1;

/// `DllMain`. On `DLL_PROCESS_ATTACH` it calls `disable_thread_library_calls`, which stands in for
/// `DisableThreadLibraryCalls(instance)`. It returns `TRUE` for every reason.
pub fn dll_main(reason: u32, disable_thread_library_calls: impl FnOnce()) -> bool {
    if reason == DLL_PROCESS_ATTACH {
        disable_thread_library_calls();
    }
    true
}

/// `DllGetClassObject`. For `CLSID_TTSEngineCom` it returns the class factory's `QueryInterface` result. For any
/// other CLSID it returns `CLASS_E_CLASSNOTAVAILABLE`, and the C leaves `*obj` untouched.
pub fn dll_get_class_object(clsid: &Guid, riid: &Guid) -> Result<ClassFactory, Hresult> {
    if !is_equal_guid(clsid, &CLSID_TTS_ENGINE_COM) {
        return Err(CLASS_E_CLASSNOTAVAILABLE);
    }
    class_factory_query_interface(riid)
}

/// `DllCanUnloadNow`, the `libs/winecrt0` default: always `S_FALSE`, so the DLL is never unloadable.
pub fn dll_can_unload_now() -> Hresult {
    S_FALSE
}

/// `DllRegisterServer`, the `libs/winecrt0` default. It returns the result of `__wine_register_resources`, which
/// `register_resources` stands in for.
pub fn dll_register_server(register_resources: impl FnOnce() -> Hresult) -> Hresult {
    register_resources()
}

/// `DllUnregisterServer`, the `libs/winecrt0` default. It returns the result of `__wine_unregister_resources`,
/// which `unregister_resources` stands in for.
pub fn dll_unregister_server(unregister_resources: impl FnOnce() -> Hresult) -> Hresult {
    unregister_resources()
}
