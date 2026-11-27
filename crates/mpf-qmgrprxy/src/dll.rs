/// `HRESULT` from `include/winerror.h`.
pub type Hresult = i32;

/// `S_OK` from `include/winerror.h`.
pub const S_OK: Hresult = 0;

/// `TRUE` from `include/minwindef.h`.
pub const TRUE: i32 = 1;

/// `DLL_PROCESS_ATTACH` from `include/winnt.h`.
pub const DLL_PROCESS_ATTACH: u32 = 1;

/// What `DllMain` from `REGISTER_PROXY_DLL_ROUTINES` does for one `fdwReason`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DllMainOutcome {
    /// The `BOOL` that `DllMain` returns.
    pub ret: i32,
    /// Whether `DisableThreadLibraryCalls(hinstDLL)` runs.
    pub disable_thread_library_calls: bool,
    /// Whether `hProxyDll` is set to `hinstDLL`.
    pub stores_instance: bool,
}

/// `DllMain` from `REGISTER_PROXY_DLL_ROUTINES` in `include/rpcproxy.h`. On process attach it
/// disables thread library calls and keeps the instance handle. It returns `TRUE` for every reason.
pub fn dll_main(fdw_reason: u32) -> DllMainOutcome {
    let attach = fdw_reason == DLL_PROCESS_ATTACH;
    DllMainOutcome {
        ret: TRUE,
        disable_thread_library_calls: attach,
        stores_instance: attach,
    }
}

/// The function that `DllRegisterServer` and `DllUnregisterServer` call in `include/rpcproxy.h`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegisterRoute {
    /// `__wine_register_resources` or `__wine_unregister_resources`.
    WineResources,
    /// `NdrDllRegisterProxy` or `NdrDllUnregisterProxy` in rpcrt4. Not ported.
    NdrProxy,
}

/// The route for a build where `WINE_REGISTER_DLL` is `wine_register_dll`. `Makefile.in` sets it
/// through `dlldata_EXTRADEFS = -DWINE_REGISTER_DLL`.
pub fn register_route(wine_register_dll: bool) -> RegisterRoute {
    if wine_register_dll {
        RegisterRoute::WineResources
    } else {
        RegisterRoute::NdrProxy
    }
}

/// `__wine_register_resources` from `libs/winecrt0/register.c` for a DLL with no `WINE_REGISTRY`
/// resources. The enumeration never calls its callback, so the initial `S_OK` is returned.
pub fn wine_register_resources() -> Hresult {
    S_OK
}

/// `__wine_unregister_resources` from `libs/winecrt0/register.c`. See [`wine_register_resources`].
pub fn wine_unregister_resources() -> Hresult {
    S_OK
}
