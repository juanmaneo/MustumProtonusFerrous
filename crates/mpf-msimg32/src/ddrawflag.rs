/// `ERROR_CALL_NOT_IMPLEMENTED` from `include/winerror.h`.
pub const ERROR_CALL_NOT_IMPLEMENTED: u32 = 120;

/// `vSetDdrawflag` from `dlls/msimg32/msimg32_main.c`. The C body only calls `SetLastError` with
/// `ERROR_CALL_NOT_IMPLEMENTED`, so the port takes the thread's last-error slot and overwrites it.
pub fn v_set_ddrawflag(last_error: &mut u32) {
    *last_error = ERROR_CALL_NOT_IMPLEMENTED;
}
