/// `ERROR_INVALID_PARAMETER` from `include/winerror.h`, the code `SetupColorMatchingW` sets.
pub const ERROR_INVALID_PARAMETER: u32 = 87;

/// What `SetupColorMatchingW` leaves behind: its `BOOL` return value and the thread's last error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetupColorMatchingWResult {
    /// The `BOOL` return value. `false` is `FALSE`.
    pub ret: bool,
    /// The value `GetLastError` returns after the call.
    pub last_error: u32,
}

/// `SetupColorMatchingW` from `dlls/icmui/icmui.c`. For every `cms` the C body calls
/// `SetLastError(ERROR_INVALID_PARAMETER)` and returns `FALSE`, so the result is fixed.
pub fn setup_color_matching_w() -> SetupColorMatchingWResult {
    SetupColorMatchingWResult {
        ret: false,
        last_error: ERROR_INVALID_PARAMETER,
    }
}
