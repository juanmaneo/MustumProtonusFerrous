/// `S_OK` from `include/winerror.h`.
pub const S_OK: u32 = 0;

/// `E_NOTIMPL` from `include/winerror.h`, as a `u32` bit pattern.
pub const E_NOTIMPL: u32 = 0x8000_4001;

/// `SignerSign`. The C body only traces its arguments and returns `E_NOTIMPL`.
pub fn signer_sign() -> u32 {
    E_NOTIMPL
}

/// `SignerSignEx`. The C body only traces its arguments and returns `E_NOTIMPL`.
pub fn signer_sign_ex() -> u32 {
    E_NOTIMPL
}

/// `SignerFreeSignerContext`. Returns `S_OK` and does not touch the context.
pub fn signer_free_signer_context() -> u32 {
    S_OK
}
