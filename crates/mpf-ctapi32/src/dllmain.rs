//! Port of the `DllMain` reason switch in Wine's `dlls/ctapi32/ctapi32.c`.

use crate::unixlib::{NtStatus, STATUS_SUCCESS};

pub const DLL_PROCESS_DETACH: u32 = 0;
pub const DLL_PROCESS_ATTACH: u32 = 1;

/// Port of `DllMain`. `init_unix_call` stands for `__wine_init_unix_call`,
/// `load_functions` for the library load, and `detach` for the unix `detach` call.
/// `lpv_reserved` is whether `lpvReserved` is non-NULL.
pub fn dll_main(
    reason: u32,
    lpv_reserved: bool,
    init_unix_call: impl FnOnce() -> NtStatus,
    load_functions: impl FnOnce() -> bool,
    detach: impl FnOnce() -> NtStatus,
) -> bool {
    match reason {
        DLL_PROCESS_ATTACH => init_unix_call() == STATUS_SUCCESS && load_functions(),
        DLL_PROCESS_DETACH => {
            if !lpv_reserved {
                detach();
            }
            true
        }
        _ => true,
    }
}
