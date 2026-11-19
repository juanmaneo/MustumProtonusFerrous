//! `MCIERR_*` codes from `mmsystem.h`, the Win32 error values that `mcicda.c` tests, and the
//! `MCICDA_GetError` mapping from the last Win32 error to an `MCIERR_*` code.

pub const MCIERR_INVALID_DEVICE_ID: u32 = 257;
pub const MCIERR_HARDWARE: u32 = 262;
pub const MCIERR_CANNOT_LOAD_DRIVER: u32 = 266;
pub const MCIERR_PARAM_OVERFLOW: u32 = 268;
pub const MCIERR_DRIVER_INTERNAL: u32 = 272;
pub const MCIERR_MISSING_PARAMETER: u32 = 273;
pub const MCIERR_UNSUPPORTED_FUNCTION: u32 = 274;
pub const MCIERR_DEVICE_NOT_READY: u32 = 276;
pub const MCIERR_OUTOFRANGE: u32 = 282;
pub const MCIERR_FLAGS_NOT_COMPATIBLE: u32 = 284;
pub const MCIERR_MUST_USE_SHAREABLE: u32 = 291;
pub const MCIERR_BAD_TIME_FORMAT: u32 = 293;
pub const MCIERR_INVALID_FILE: u32 = 296;
pub const MCIERR_NULL_PARAMETER_BLOCK: u32 = 297;
pub const MCIERR_NO_IDENTITY: u32 = 350;

pub const ERROR_NOT_READY: u32 = 21;
pub const ERROR_NOT_SUPPORTED: u32 = 50;
pub const ERROR_IO_DEVICE: u32 = 1117;

/// `MCICDA_GetError`: maps the Win32 error left by a failed device call to an `MCIERR_*` code.
pub fn device_error(last_error: u32) -> u32 {
    match last_error {
        ERROR_NOT_READY => MCIERR_DEVICE_NOT_READY,
        ERROR_NOT_SUPPORTED | ERROR_IO_DEVICE => MCIERR_HARDWARE,
        _ => MCIERR_DRIVER_INTERNAL,
    }
}
