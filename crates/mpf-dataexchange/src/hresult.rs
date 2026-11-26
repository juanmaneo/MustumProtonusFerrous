//! `HRESULT` values and `HRESULT_FROM_WIN32` from Wine's `include/winerror.h`.

/// `HRESULT` is a 32-bit signed value; failure codes have the severity bit set.
pub type Hresult = i32;

pub const S_OK: Hresult = 0x0000_0000;
pub const E_NOTIMPL: Hresult = 0x8000_4001u32 as i32;
pub const E_NOINTERFACE: Hresult = 0x8000_4002u32 as i32;
pub const E_ACCESSDENIED: Hresult = 0x8007_0005u32 as i32;
pub const E_OUTOFMEMORY: Hresult = 0x8007_000Eu32 as i32;
pub const CLASS_E_CLASSNOTAVAILABLE: Hresult = 0x8004_0111u32 as i32;

/// `ERROR_INVALID_WINDOW_HANDLE` from `winerror.h`.
pub const ERROR_INVALID_WINDOW_HANDLE: u32 = 1400;

const FACILITY_WIN32: u32 = 7;

/// `HRESULT_FROM_WIN32`. Positive codes keep their low 16 bits and get the WIN32 facility
/// and the severity bit. Zero and values that are negative as an `HRESULT` pass through.
pub fn hresult_from_win32(x: u32) -> Hresult {
    let hr = x as Hresult;
    if hr > 0 {
        ((x & 0x0000_FFFF) | (FACILITY_WIN32 << 16) | 0x8000_0000) as Hresult
    } else {
        hr
    }
}
