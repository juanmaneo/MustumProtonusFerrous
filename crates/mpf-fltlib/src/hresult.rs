//! `HRESULT` helpers and the constants fltlib uses (`winerror.h`, `winbase.h`).

pub const FACILITY_WIN32: u32 = 7;
pub const S_OK: i32 = 0;
pub const E_NOTIMPL: i32 = 0x8000_4001_u32 as i32;
pub const ERROR_NOT_SUPPORTED: u32 = 50;
pub const ERROR_INVALID_PARAMETER: u32 = 87;
pub const ERROR_NO_MORE_ITEMS: u32 = 259;
/// `(HANDLE)~(ULONG_PTR)0`.
pub const INVALID_HANDLE_VALUE: isize = -1;

/// `HRESULT_FROM_WIN32`. A value that is not positive as an `HRESULT` passes through unchanged;
/// any other value is wrapped into the WIN32 facility with the severity bit set.
pub fn hresult_from_win32(x: u32) -> i32 {
    let h = x as i32;
    if h > 0 {
        ((x & 0x0000_FFFF) | (FACILITY_WIN32 << 16) | 0x8000_0000) as i32
    } else {
        h
    }
}
