//! `DllCanUnloadNow` for `graphicscapture.dll`. The DLL does not define it, so the default from
//! `libs/winecrt0/dll_canunload.c` applies: `tools/makedep.c` links `winecrt0` into every DLL that
//! does not opt out of the default libraries.

/// `S_FALSE` from `include/winerror.h`.
pub const S_FALSE: u32 = 1;

/// `DllCanUnloadNow` from `libs/winecrt0/dll_canunload.c`. It always returns `S_FALSE`, so the DLL
/// is never reported as unloadable.
pub fn dll_can_unload_now() -> u32 {
    S_FALSE
}
