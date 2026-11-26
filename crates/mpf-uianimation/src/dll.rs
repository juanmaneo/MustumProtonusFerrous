use crate::class::{class_for_clsid, ClassKind};
use crate::com::{Guid, Out, CLASS_E_CLASSNOTAVAILABLE, HRESULT, S_FALSE};
use crate::factory::class_factory_query_interface;

/// The export table of `dlls/uianimation/uianimation.spec`, in spec order.
pub const EXPORTS: [&str; 4] = [
    "DllCanUnloadNow",
    "DllGetClassObject",
    "DllRegisterServer",
    "DllUnregisterServer",
];

/// Whether `name` is one of the [`EXPORTS`]. Names are case-sensitive.
pub fn is_export(name: &str) -> bool {
    EXPORTS.contains(&name)
}

/// `DllGetClassObject` in `dlls/uianimation/main.c`. When no class matches, the C code returns
/// `CLASS_E_CLASSNOTAVAILABLE` without writing `*obj`.
pub fn dll_get_class_object(clsid: Guid, iid: Guid) -> (HRESULT, Out<ClassKind>) {
    match class_for_clsid(clsid) {
        Some(factory) => class_factory_query_interface(factory, iid),
        None => (CLASS_E_CLASSNOTAVAILABLE, Out::Untouched),
    }
}

/// `DllCanUnloadNow`, the default in `libs/winecrt0/dll_canunload.c`: always `S_FALSE`.
pub fn dll_can_unload_now() -> HRESULT {
    S_FALSE
}

/// `DllRegisterServer`, the default in `libs/winecrt0/dll_register.c`, which returns
/// `__wine_register_resources()`. The registry work happens on the OS side, so the caller
/// supplies it as `register_resources`.
pub fn dll_register_server(register_resources: impl FnOnce() -> HRESULT) -> HRESULT {
    register_resources()
}

/// `DllUnregisterServer`, the default in `libs/winecrt0/dll_register.c`, which returns
/// `__wine_unregister_resources()`. The caller supplies the OS side as `unregister_resources`.
pub fn dll_unregister_server(unregister_resources: impl FnOnce() -> HRESULT) -> HRESULT {
    unregister_resources()
}
