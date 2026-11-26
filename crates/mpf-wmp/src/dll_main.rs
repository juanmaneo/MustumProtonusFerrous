/// `DLL_PROCESS_DETACH` from `include/winnt.h`.
pub const DLL_PROCESS_DETACH: u32 = 0;

/// `DLL_PROCESS_ATTACH` from `include/winnt.h`.
pub const DLL_PROCESS_ATTACH: u32 = 1;

/// The effects of `DllMain`, apart from the OS calls it makes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DllMainOutcome {
    /// The `BOOL` the C function returns. It is always `TRUE`.
    pub ret: bool,
    /// `DisableThreadLibraryCalls` runs and the instance is stored (`DLL_PROCESS_ATTACH`).
    pub attached: bool,
    /// `unregister_wmp_class`, `unregister_player_msg_class` and `release_typelib` run
    /// (`DLL_PROCESS_DETACH` with a NULL `lpv`).
    pub cleanup: bool,
}

/// `DllMain` from `dlls/wmp/wmp_main.c`. `lpv_is_null` is true when the `lpv` argument is NULL.
pub fn dll_main(fdw_reason: u32, lpv_is_null: bool) -> DllMainOutcome {
    DllMainOutcome {
        ret: true,
        attached: fdw_reason == DLL_PROCESS_ATTACH,
        cleanup: fdw_reason == DLL_PROCESS_DETACH && lpv_is_null,
    }
}
