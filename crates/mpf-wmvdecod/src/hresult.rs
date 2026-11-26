//! `HRESULT` values from `include/winerror.h`, and the `FAILED` test.

/// `HRESULT`.
pub type Hresult = i32;

pub const S_OK: Hresult = 0;
pub const S_FALSE: Hresult = 1;
pub const E_FAIL: Hresult = 0x8000_4005u32 as i32;
pub const E_NOINTERFACE: Hresult = 0x8000_4002u32 as i32;
pub const E_OUTOFMEMORY: Hresult = 0x8007_000Eu32 as i32;
pub const REGDB_E_CLASSNOTREG: Hresult = 0x8004_0154u32 as i32;
pub const CLASS_E_CLASSNOTAVAILABLE: Hresult = 0x8004_0111u32 as i32;

/// `FAILED`: a negative HRESULT fails, so `S_FALSE` does not.
pub const fn failed(hr: Hresult) -> bool {
    hr < 0
}
