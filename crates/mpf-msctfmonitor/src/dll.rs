//! `DllCanUnloadNow`, `DllRegisterServer` and `DllUnregisterServer`. The spec declares them
//! `-private` with no body in `dlls/msctfmonitor/main.c`; the bodies come from
//! `libs/winecrt0/dll_canunload.c` and `libs/winecrt0/dll_register.c`.

/// `HRESULT`.
pub type Hresult = i32;

/// `S_OK` from `include/winerror.h`.
pub const S_OK: Hresult = 0;

/// `S_FALSE` from `include/winerror.h`.
pub const S_FALSE: Hresult = 1;

/// `DllCanUnloadNow` from `libs/winecrt0/dll_canunload.c`: always `S_FALSE`.
pub fn dll_can_unload_now() -> Hresult {
    S_FALSE
}

/// `DllRegisterServer` from `libs/winecrt0/dll_register.c`, which returns
/// `__wine_register_resources()`. That enumerates the module's `WINE_REGISTRY` resources. This
/// module has none, so the enumeration never calls back and the initial `S_OK` is returned.
pub fn dll_register_server() -> Hresult {
    S_OK
}

/// `DllUnregisterServer` from `libs/winecrt0/dll_register.c`, which returns
/// `__wine_unregister_resources()`. The reasoning in [`dll_register_server`] applies.
pub fn dll_unregister_server() -> Hresult {
    S_OK
}
