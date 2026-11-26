use crate::Hresult;

/// `S_OK` from `winerror.h`.
pub const S_OK: Hresult = 0;

/// `S_FALSE` from `winerror.h`.
pub const S_FALSE: Hresult = 1;

/// `DllCanUnloadNow` from `libs/winecrt0/dll_canunload.c`. The body always
/// returns `S_FALSE`, so COM is told the DLL must stay loaded. `cdosys.spec`
/// lists it, but `main.c` does not define it.
pub fn dll_can_unload_now() -> Hresult {
    S_FALSE
}

/// `DllRegisterServer` from `libs/winecrt0/dll_register.c`, which returns
/// `__wine_register_resources()`. That function starts from `S_OK` and only
/// changes it inside its `WINE_REGISTRY` resource callback. cdosys has no
/// `.rc` source, so the enumeration visits nothing and the result is `S_OK`.
pub fn dll_register_server() -> Hresult {
    S_OK
}

/// `DllUnregisterServer` from `libs/winecrt0/dll_register.c`. Same reasoning
/// as [`dll_register_server`], with `__wine_unregister_resources()`.
pub fn dll_unregister_server() -> Hresult {
    S_OK
}
