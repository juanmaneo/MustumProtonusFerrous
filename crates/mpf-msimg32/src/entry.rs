/// `DLL_PROCESS_ATTACH` from `include/winnt.h`.
pub const DLL_PROCESS_ATTACH: u32 = 1;

/// What the default `DllMain` does for one call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DllMainOutcome {
    /// The `BOOL` return value. The C body returns `TRUE` for every reason.
    pub ret: bool,
    /// The `inst` passed to `DisableThreadLibraryCalls`, or `None` when that call is not made.
    pub disable_thread_library_calls: Option<usize>,
}

/// `DllMain` from `libs/winecrt0/dll_main.c`. `msimg32.spec` exports `DllInitialize` as
/// `-private` and maps it to this function. The `reserved` argument is not taken.
pub fn dll_main(inst: usize, reason: u32) -> DllMainOutcome {
    DllMainOutcome {
        ret: true,
        disable_thread_library_calls: (reason == DLL_PROCESS_ATTACH).then_some(inst),
    }
}
