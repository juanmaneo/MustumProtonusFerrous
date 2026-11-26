//! HRESULT values from `winerror.h` and `olectl.h`.

/// `HRESULT`, a signed 32-bit status code.
pub type HResult = i32;

pub const S_OK: HResult = 0;
pub const E_NOTIMPL: HResult = 0x8000_4001_u32 as HResult;
pub const E_NOINTERFACE: HResult = 0x8000_4002_u32 as HResult;
pub const E_POINTER: HResult = 0x8000_4003_u32 as HResult;
pub const E_FAIL: HResult = 0x8000_4005_u32 as HResult;
pub const DV_E_DVASPECT: HResult = 0x8004_006B_u32 as HResult;
pub const CLASS_E_NOAGGREGATION: HResult = 0x8004_0110_u32 as HResult;
pub const CLASS_E_CLASSNOTAVAILABLE: HResult = 0x8004_0111_u32 as HResult;
pub const CONNECT_E_NOCONNECTION: HResult = 0x8004_0200_u32 as HResult;

/// `SUCCEEDED(hr)`.
pub fn is_success(hr: HResult) -> bool {
    hr >= 0
}
