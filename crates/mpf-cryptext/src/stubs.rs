//! `CryptExtAddPFX` and `CryptExtAddPFXW` from `dlls/cryptext/cryptext_main.c`. Both are stubs
//! (`FIXME` then `return E_NOTIMPL`) that ignore the file name.

/// `HRESULT` from `winerror.h`.
pub type HRESULT = i32;

/// `S_OK` from `winerror.h`.
pub const S_OK: HRESULT = 0;

/// `E_NOTIMPL` from `winerror.h`.
pub const E_NOTIMPL: HRESULT = 0x8000_4001_u32 as HRESULT;

/// `CryptExtAddPFX`.
pub fn crypt_ext_add_pfx(_filename: Option<&[u8]>) -> HRESULT {
    E_NOTIMPL
}

/// `CryptExtAddPFXW`.
pub fn crypt_ext_add_pfx_w(_filename: Option<&[u16]>) -> HRESULT {
    E_NOTIMPL
}
