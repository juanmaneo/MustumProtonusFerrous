/// `E_FAIL` from `include/winerror.h`, as a `u32` bit pattern.
pub const E_FAIL: u32 = 0x8000_4005;

/// `PvkFreeCryptProv`. The C body only traces its arguments, so this does nothing.
pub fn pvk_free_crypt_prov() {}

/// `PvkGetCryptProv`. The C body only traces its arguments and returns `E_FAIL`.
pub fn pvk_get_crypt_prov() -> u32 {
    E_FAIL
}

/// `PvkPrivateKeyAcquireContextFromMemory`. The C body only traces its arguments and returns `FALSE`.
pub fn pvk_private_key_acquire_context_from_memory() -> bool {
    false
}
