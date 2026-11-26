//! HRESULT values from `winerror.h` and the `SUCCEEDED` test.

/// `HRESULT`, a 32-bit `LONG`.
pub type Hresult = i32;

pub const S_OK: Hresult = 0;
pub const S_FALSE: Hresult = 1;
pub const E_FAIL: Hresult = 0x8000_4005u32 as i32;
pub const E_NOTIMPL: Hresult = 0x8000_4001u32 as i32;
pub const E_NOINTERFACE: Hresult = 0x8000_4002u32 as i32;
pub const E_OUTOFMEMORY: Hresult = 0x8007_000Eu32 as i32;

/// `SUCCEEDED(hr)`: any non-negative HRESULT succeeds.
pub fn succeeded(hr: Hresult) -> bool {
    hr >= 0
}
