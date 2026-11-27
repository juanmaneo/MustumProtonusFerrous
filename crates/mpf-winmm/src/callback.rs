//! `WINMM_CheckCallback` from `dlls/winmm/winmm.c`, which validates the callback of an open call.

use crate::error::{
    MMSYSERR_INVALFLAG, MMSYSERR_INVALPARAM, MMSYSERR_NOERROR, MMSYSERR_NOTSUPPORTED,
};

/// `CALLBACK_TYPEMASK`, from `include/mmsystem.h`.
pub const CALLBACK_TYPEMASK: u32 = 0x0007_0000;
pub const CALLBACK_NULL: u32 = 0x0000_0000;
pub const CALLBACK_WINDOW: u32 = 0x0001_0000;
pub const CALLBACK_THREAD: u32 = 0x0002_0000;
pub const CALLBACK_FUNCTION: u32 = 0x0003_0000;
pub const CALLBACK_EVENT: u32 = 0x0005_0000;

/// `WINMM_CheckCallback`. `dw_callback` is the `DWORD_PTR` callback and `is_window` stands in for
/// `IsWindow`, which the C body calls only for a non-zero window handle.
pub fn check_callback(
    dw_callback: usize,
    fdw_open: u32,
    mixer: bool,
    is_window: impl FnOnce(usize) -> bool,
) -> u32 {
    match fdw_open & CALLBACK_TYPEMASK {
        CALLBACK_NULL => MMSYSERR_NOERROR,
        CALLBACK_WINDOW if dw_callback != 0 && !is_window(dw_callback) => MMSYSERR_INVALPARAM,
        CALLBACK_FUNCTION if mixer => MMSYSERR_INVALFLAG,
        CALLBACK_THREAD | CALLBACK_EVENT if mixer => MMSYSERR_NOTSUPPORTED,
        _ => MMSYSERR_NOERROR,
    }
}
