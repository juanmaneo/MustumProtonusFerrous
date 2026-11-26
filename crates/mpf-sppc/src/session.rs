//! `SLOpen` and `SLClose` from `dlls/sppc/sppc.c`.

/// `HRESULT`, as the bit pattern of the C value.
pub type Hresult = u32;

/// `S_OK` from `include/winerror.h`.
pub const S_OK: Hresult = 0;

/// `E_INVALIDARG` from `include/winerror.h`.
pub const E_INVALIDARG: Hresult = 0x8007_0057;

/// `HSLC`, the licensing handle. `PVOID` in C.
pub type Hslc = usize;

/// The handle value `SLOpen` stores, `(HSLC)0xdeadbeef` in `dlls/sppc/sppc.c`.
pub const SLC_HANDLE: Hslc = 0xdead_beef;

/// `SLOpen`. A NULL `handle` is `E_INVALIDARG`. Otherwise the handle is overwritten with
/// [`SLC_HANDLE`] and `S_OK` is returned.
pub fn sl_open(handle: Option<&mut Hslc>) -> Hresult {
    match handle {
        None => E_INVALIDARG,
        Some(handle) => {
            *handle = SLC_HANDLE;
            S_OK
        }
    }
}

/// `SLClose`. The handle is not checked, so the result is always `S_OK`.
pub fn sl_close(_handle: Hslc) -> Hresult {
    S_OK
}
