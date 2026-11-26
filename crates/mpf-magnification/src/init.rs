/// `ERROR_CALL_NOT_IMPLEMENTED` from `include/winerror.h`.
pub const ERROR_CALL_NOT_IMPLEMENTED: u32 = 120;

/// `MagInitialize`: `SetLastError(ERROR_CALL_NOT_IMPLEMENTED)`, then `FALSE`. `last_error` is
/// the thread's last-error slot that `SetLastError` writes.
pub fn mag_initialize(last_error: &mut u32) -> bool {
    *last_error = ERROR_CALL_NOT_IMPLEMENTED;
    false
}

/// `MagUninitialize`: `TRUE` with no other effect.
pub fn mag_uninitialize() -> bool {
    true
}
