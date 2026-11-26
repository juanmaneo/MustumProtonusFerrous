//! `HRESULT` values from Wine's `include/winerror.h` and the `FAILED` test.

/// An `HRESULT`: a 32-bit value whose severity bit (the top bit) marks a failure.
pub type HResult = u32;

pub const S_OK: HResult = 0x0000_0000;
pub const S_FALSE: HResult = 0x0000_0001;
pub const E_FAIL: HResult = 0x8000_4005;
pub const E_ACCESSDENIED: HResult = 0x8007_0005;
pub const E_OUTOFMEMORY: HResult = 0x8007_000e;
pub const CLASS_E_CLASSNOTAVAILABLE: HResult = 0x8004_0111;

/// The `FAILED` macro: true when the severity bit is set.
pub fn failed(hr: HResult) -> bool {
    (hr as i32) < 0
}

/// Passes a successful `HRESULT` through as `Ok` and turns a failed one into `Err`, which is the
/// `if (FAILED(hr = ...)) return hr;` step that msvdsp.c repeats.
pub fn checked(hr: HResult) -> Result<(), HResult> {
    if failed(hr) {
        Err(hr)
    } else {
        Ok(())
    }
}
