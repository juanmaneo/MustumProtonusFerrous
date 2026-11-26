//! `HRESULT` values and the `FAILED` macro from `include/winerror.h`.

/// `HRESULT`, a signed 32-bit value, so that `FAILED` is a sign test.
pub type Hresult = i32;

pub const S_OK: Hresult = 0;
pub const S_FALSE: Hresult = 1;
pub const E_FAIL: Hresult = 0x8000_4005u32 as i32;
pub const E_NOTIMPL: Hresult = 0x8000_4001u32 as i32;
pub const E_NOINTERFACE: Hresult = 0x8000_4002u32 as i32;
pub const E_OUTOFMEMORY: Hresult = 0x8007_000Eu32 as i32;
pub const CLASS_E_NOAGGREGATION: Hresult = 0x8004_0110u32 as i32;
pub const CLASS_E_CLASSNOTAVAILABLE: Hresult = 0x8004_0111u32 as i32;

/// `FAILED`: true for every negative `HRESULT`.
pub fn failed(hr: Hresult) -> bool {
    hr < 0
}
