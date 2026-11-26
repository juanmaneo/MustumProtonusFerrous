//! The exports in `dlls/objsel/objsel.spec`. `DllGetClassObject` is in `dlls/objsel/objsel.c`.
//! `DllCanUnloadNow`, `DllRegisterServer` and `DllUnregisterServer` are declared `-private`, and
//! `libs/winecrt0` provides them.

use crate::class_factory::ClassFactory;
use crate::guid::{is_equal_guid, Guid, CLSID_DS_OBJECT_PICKER};
use crate::hresult::{Hresult, CLASS_E_CLASSNOTAVAILABLE, S_FALSE};
use crate::object_picker::Interface;

/// `DllGetClassObject`. `*out` is cleared first. Only `CLSID_DsObjectPicker` is served, by the factory's
/// `QueryInterface`. Any other CLSID returns `CLASS_E_CLASSNOTAVAILABLE` with `*out` NULL. The C writes
/// `*out` before any check, so the port takes `out` as a reference, not an optional pointer.
pub fn dll_get_class_object(
    factory: &mut ClassFactory,
    clsid: &Guid,
    iid: &Guid,
    out: &mut Option<Interface>,
) -> Hresult {
    *out = None;
    if is_equal_guid(clsid, &CLSID_DS_OBJECT_PICKER) {
        return factory.query_interface(iid, Some(out));
    }
    CLASS_E_CLASSNOTAVAILABLE
}

/// `DllCanUnloadNow` from `libs/winecrt0/dll_canunload.c`: always `S_FALSE`.
pub fn dll_can_unload_now() -> Hresult {
    S_FALSE
}

/// `DllRegisterServer` from `libs/winecrt0/dll_register.c`, which returns `__wine_register_resources()`.
/// `register_resources` stands in for that call. The OS registrar it uses is not ported.
pub fn dll_register_server(register_resources: impl FnOnce() -> Hresult) -> Hresult {
    register_resources()
}

/// `DllUnregisterServer` from `libs/winecrt0/dll_register.c`, which returns
/// `__wine_unregister_resources()`. `unregister_resources` stands in for that call.
pub fn dll_unregister_server(unregister_resources: impl FnOnce() -> Hresult) -> Hresult {
    unregister_resources()
}
