/// `ERROR_SUCCESS` from `winerror.h`.
pub const ERROR_SUCCESS: u32 = 0;

/// `ERROR_UNSUPPORTED_TYPE` from `winerror.h`.
pub const ERROR_UNSUPPORTED_TYPE: u32 = 1630;

/// `FALSE` from `windef.h`.
pub const FALSE: i32 = 0;

/// `if (reboot) *reboot = FALSE;`: a null out-parameter is left alone.
pub fn clear_reboot(reboot: Option<&mut i32>) {
    if let Some(reboot) = reboot {
        *reboot = FALSE;
    }
}
