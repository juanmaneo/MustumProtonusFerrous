//! `HRESULT` values from `winerror.h` used by the ported logic.

/// A Windows `HRESULT`, kept as its raw unsigned bit pattern.
pub type HResult = u32;

pub const S_OK: HResult = 0x0000_0000;
pub const E_INVALIDARG: HResult = 0x8007_0057;
pub const E_NOINTERFACE: HResult = 0x8000_4002;
pub const E_NOTIMPL: HResult = 0x8000_4001;
pub const E_FAIL: HResult = 0x8000_4005;
pub const CLASS_E_CLASSNOTAVAILABLE: HResult = 0x8004_0111;

/// `SUCCEEDED(hr)`: true when the severity bit is clear.
pub const fn succeeded(hr: HResult) -> bool {
    (hr as i32) >= 0
}
