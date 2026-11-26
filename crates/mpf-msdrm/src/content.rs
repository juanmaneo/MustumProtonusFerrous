/// `S_OK` from `include/winerror.h`.
pub const S_OK: i32 = 0;

/// `DRMRegisterContent(BOOL value)` from `dlls/msdrm/main.c`. The C body only traces `value`
/// and returns `S_OK`, so the port ignores the argument.
pub fn drm_register_content(_value: i32) -> i32 {
    S_OK
}
