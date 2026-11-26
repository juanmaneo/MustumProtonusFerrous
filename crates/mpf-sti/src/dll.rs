//! The `sti.dll` exports that have a body in `dlls/sti/sti_main.c`, or that come from
//! `libs/winecrt0` (`DllCanUnloadNow`).
//!
//! `DllRegisterServer`, `DllUnregisterServer` and the `DllGetClassObject` fallback forward to
//! `STI_DllRegisterServer`, `STI_DllUnregisterServer` and `STI_DllGetClassObject`. Those come
//! from the midl-generated proxy-stub, which is not in this tree, so the port takes the forwarded
//! call as a closure and returns its result.

use crate::class_factory::class_factory_query_interface;
use crate::com::{Guid, Hresult, CLSID_STI, S_FALSE};

/// `DllGetClassObject`: `CLSID_Sti` is served by the class factory, and any other class goes
/// to the proxy-stub.
pub fn dll_get_class_object(rclsid: &Guid, iid: &Guid, proxy: impl FnOnce() -> Hresult) -> Hresult {
    if *rclsid == CLSID_STI {
        class_factory_query_interface(iid)
    } else {
        proxy()
    }
}

/// `DllRegisterServer`, which forwards to `STI_DllRegisterServer`.
pub fn dll_register_server(proxy: impl FnOnce() -> Hresult) -> Hresult {
    proxy()
}

/// `DllUnregisterServer`, which forwards to `STI_DllUnregisterServer`.
pub fn dll_unregister_server(proxy: impl FnOnce() -> Hresult) -> Hresult {
    proxy()
}

/// `DllCanUnloadNow`. It has no body in `sti`; `libs/winecrt0/dll_canunload.c` returns `S_FALSE`.
pub fn dll_can_unload_now() -> Hresult {
    S_FALSE
}
