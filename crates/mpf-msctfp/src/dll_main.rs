/// `DLL_PROCESS_ATTACH` from `include/winnt.h`.
pub const DLL_PROCESS_ATTACH: u32 = 1;

/// The state that `REGISTER_PROXY_DLL_ROUTINES` keeps: the `hProxyDll` global, and the effect
/// of `DisableThreadLibraryCalls`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ProxyDll {
    /// `hProxyDll`. 0 stands for NULL, its initial value.
    pub hinstance: usize,
    /// Whether `DisableThreadLibraryCalls(hinstDLL)` has run.
    pub thread_library_calls_disabled: bool,
}

/// `DllMain` from `REGISTER_PROXY_DLL_ROUTINES`. On `DLL_PROCESS_ATTACH` it disables thread
/// notifications and stores the instance. Every reason returns TRUE.
pub fn dll_main(state: &mut ProxyDll, hinstance: usize, reason: u32) -> bool {
    if reason == DLL_PROCESS_ATTACH {
        state.thread_library_calls_disabled = true;
        state.hinstance = hinstance;
    }
    true
}
