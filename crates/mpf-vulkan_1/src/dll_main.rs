/// `DLL_PROCESS_ATTACH` from `include/winnt.h`.
pub const DLL_PROCESS_ATTACH: u32 = 1;

/// The OS calls `DllMain` makes, in the order it makes them.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DllMainEffects {
    /// `DisableThreadLibraryCalls(hinst)` was called.
    pub disable_thread_library_calls: bool,
    /// `GetDpiForSystem()` was called (to make sure that user32 is loaded).
    pub get_dpi_for_system: bool,
}

/// `DllMain` from `dlls/vulkan-1/vulkan.c`. Returns the `BOOL` result (always `TRUE`, as
/// `true`) and the OS calls the C body makes. Any `reason` other than `DLL_PROCESS_ATTACH`
/// returns at once with no calls.
pub fn dll_main(reason: u32) -> (bool, DllMainEffects) {
    if reason != DLL_PROCESS_ATTACH {
        return (true, DllMainEffects::default());
    }

    (
        true,
        DllMainEffects {
            disable_thread_library_calls: true,
            get_dpi_for_system: true,
        },
    )
}
