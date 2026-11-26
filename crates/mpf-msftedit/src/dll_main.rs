/// `DLL_PROCESS_DETACH` from `include/winnt.h`.
pub const DLL_PROCESS_DETACH: u32 = 0;

/// `DLL_PROCESS_ATTACH` from `include/winnt.h`.
pub const DLL_PROCESS_ATTACH: u32 = 1;

/// `DLL_THREAD_ATTACH` from `include/winnt.h`.
pub const DLL_THREAD_ATTACH: u32 = 2;

/// `DLL_THREAD_DETACH` from `include/winnt.h`.
pub const DLL_THREAD_DETACH: u32 = 3;

/// The name `DllMain` passes to `LoadLibraryW`, from `L"riched20.dll"` in `dlls/msftedit/msftedit_main.c`.
pub const RICHEDIT_DLL_NAME: &str = "riched20.dll";

/// The Win32 calls `DllMain` makes. Module handles are `usize`, where `0` is `NULL`.
pub trait DllHost {
    /// `LoadLibraryW`. Returns the module handle, or `0` when the load fails.
    fn load_library_w(&mut self, name: &str) -> usize;

    /// `DisableThreadLibraryCalls`. The C code ignores the result.
    fn disable_thread_library_calls(&mut self, module: usize);

    /// `FreeLibrary`. The C code ignores the result, and a `0` handle is passed through unchanged.
    fn free_library(&mut self, module: usize);
}

/// The `static HMODULE richedit` that `DllMain` keeps between calls.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DllMainState {
    richedit: usize,
}

impl DllMainState {
    /// `DllMain`. Only process attach and process detach make calls, and the result is always `TRUE`.
    pub fn dll_main(&mut self, host: &mut impl DllHost, inst: usize, reason: u32) -> bool {
        match reason {
            DLL_PROCESS_ATTACH => {
                self.richedit = host.load_library_w(RICHEDIT_DLL_NAME);
                host.disable_thread_library_calls(inst);
            }
            DLL_PROCESS_DETACH => host.free_library(self.richedit),
            _ => {}
        }
        true
    }
}
