//! `HRESULT` values used by the ported `explorerframe` functions.

/// A 32-bit `HRESULT`, stored as its bit pattern.
pub type Hresult = u32;

pub const S_OK: Hresult = 0x0000_0000;
pub const S_FALSE: Hresult = 0x0000_0001;
pub const E_NOINTERFACE: Hresult = 0x8000_4002;
pub const E_NOTIMPL: Hresult = 0x8000_4001;
pub const E_FAIL: Hresult = 0x8000_4005;
pub const E_INVALIDARG: Hresult = 0x8007_0057;
pub const CLASS_E_NOAGGREGATION: Hresult = 0x8004_0110;
pub const CLASS_E_CLASSNOTAVAILABLE: Hresult = 0x8004_0111;

/// `SUCCEEDED(hr)`: the severity bit is clear.
pub fn succeeded(hr: Hresult) -> bool {
    hr & 0x8000_0000 == 0
}
