//! `HRESULT` values from Wine's `include/winerror.h` and `include/msident.idl`.

/// `HRESULT` is a 32-bit signed value; failure codes have the severity bit set.
pub type HRESULT = i32;

pub const S_OK: HRESULT = 0x0000_0000;
pub const S_FALSE: HRESULT = 0x0000_0001;
pub const E_NOINTERFACE: HRESULT = 0x8000_4002_u32 as i32;
pub const E_NOTIMPL: HRESULT = 0x8000_4001_u32 as i32;
pub const E_OUTOFMEMORY: HRESULT = 0x8007_000E_u32 as i32;
pub const CLASS_E_CLASSNOTAVAILABLE: HRESULT = 0x8004_0111_u32 as i32;
/// `E_USER_CANCELLED` from `include/msident.idl`.
pub const E_USER_CANCELLED: HRESULT = 0x8000_7112_u32 as i32;

/// `SUCCEEDED`: true when the severity bit is clear.
pub fn succeeded(hr: HRESULT) -> bool {
    hr >= 0
}
