//! `HRESULT` values from Wine's `include/winerror.h`.

/// `HRESULT` is a 32-bit signed value; failure codes have the severity bit set.
pub type HRESULT = i32;

pub const S_OK: HRESULT = 0x0000_0000;
pub const E_NOINTERFACE: HRESULT = 0x8000_4002_u32 as i32;
pub const E_NOTIMPL: HRESULT = 0x8000_4001_u32 as i32;
pub const E_OUTOFMEMORY: HRESULT = 0x8007_000E_u32 as i32;
pub const E_INVALIDARG: HRESULT = 0x8007_0057_u32 as i32;
pub const CLASS_E_CLASSNOTAVAILABLE: HRESULT = 0x8004_0111_u32 as i32;

/// `SUCCEEDED`: true when the severity bit is clear.
pub fn succeeded(hr: HRESULT) -> bool {
    hr >= 0
}
