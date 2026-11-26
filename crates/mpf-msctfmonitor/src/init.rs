//! `InitLocalMsCtfMonitor` from `dlls/msctfmonitor/main.c`.

use crate::dll::Hresult;

/// `E_FAIL` from `include/winerror.h`.
pub const E_FAIL: Hresult = 0x8000_4005u32 as i32;

/// `InitLocalMsCtfMonitor(DWORD flags)`: the C body traces `flags` as a stub and returns `E_FAIL`.
pub fn init_local_ms_ctf_monitor(_flags: u32) -> Hresult {
    E_FAIL
}
