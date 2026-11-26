//! The explicit functions in `dlls/wkscli/wkscli.c` that return a fixed status and never read
//! their arguments.

/// `NERR_Success` from `include/lmerr.h`.
pub const NERR_SUCCESS: u32 = 0;

/// `ERROR_INVALID_PARAMETER` from `include/winerror.h`.
pub const ERROR_INVALID_PARAMETER: u32 = 87;

/// `ERROR_NOT_SUPPORTED` from `include/winerror.h`.
pub const ERROR_NOT_SUPPORTED: u32 = 50;

/// `NetUseAdd` from `dlls/wkscli/wkscli.c`: reports success.
pub fn net_use_add() -> u32 {
    NERR_SUCCESS
}

/// `NetUseDel` from `dlls/wkscli/wkscli.c`: reports success.
pub fn net_use_del() -> u32 {
    NERR_SUCCESS
}

/// `NetUseEnum` from `dlls/wkscli/wkscli.c`: reports `ERROR_NOT_SUPPORTED`.
pub fn net_use_enum() -> u32 {
    ERROR_NOT_SUPPORTED
}

/// `NetUseGetInfo` from `dlls/wkscli/wkscli.c`: reports `ERROR_NOT_SUPPORTED`.
pub fn net_use_get_info() -> u32 {
    ERROR_NOT_SUPPORTED
}

/// `NetWkstaTransportEnum` from `dlls/wkscli/wkscli.c`: reports `ERROR_NOT_SUPPORTED`.
pub fn net_wksta_transport_enum() -> u32 {
    ERROR_NOT_SUPPORTED
}
