use crate::hresult::{HResult, S_FALSE};

/// The `@ stdcall -private` exports of `dlls/wmadmod/wmadmod.spec`, in spec order.
pub const EXPORTS: [&str; 4] = [
    "DllCanUnloadNow",
    "DllGetClassObject",
    "DllRegisterServer",
    "DllUnregisterServer",
];

/// Whether `dlls/wmadmod/wmadmod.spec` exports `name`. Names are case-sensitive.
pub fn is_export(name: &str) -> bool {
    EXPORTS.contains(&name)
}

/// `DllCanUnloadNow`. `dlls/wmadmod` does not define it. The default from
/// `libs/winecrt0/dll_canunload.c`, which every DLL links, returns `S_FALSE`.
pub fn dll_can_unload_now() -> HResult {
    S_FALSE
}
