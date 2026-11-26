//! `HRESULT` values and the `SUCCEEDED` test from `include/winerror.h`.

/// An `HRESULT`: a 32-bit signed value, negative for failure.
pub type Hresult = i32;

pub const S_OK: Hresult = 0;
pub const S_FALSE: Hresult = 1;
pub const E_FAIL: Hresult = 0x8000_4005_u32 as i32;
pub const E_NOINTERFACE: Hresult = 0x8000_4002_u32 as i32;
pub const E_NOTIMPL: Hresult = 0x8000_4001_u32 as i32;
pub const CLASS_E_CLASSNOTAVAILABLE: Hresult = 0x8004_0111_u32 as i32;

/// `SUCCEEDED`: any non-negative `HRESULT` succeeds.
pub fn succeeded(hr: Hresult) -> bool {
    hr >= 0
}
