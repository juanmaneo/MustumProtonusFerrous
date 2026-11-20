//! Port of the portable logic in Wine's `dlls/capi2032/cap20wxx.c`.
//!
//! `capi2032.spec` exports 11 functions. Only `CAPI_GET_MANUFACTURER` has
//! logic in `cap20wxx.c`; the rest are thin marshals over the external
//! `capi20_*` C library calls, which need the OS and the ISDN hardware.
//! `CAPI_MANUFACTURER` is a stub that returns `0x1109`.
//!
//! Ported (1 of 11):
//! - `CAPI_GET_MANUFACTURER`: [`normalize_manufacturer`]; the surrounding
//!   `capi20_get_manufacturer` call and the returned `fret` are not portable.
//!
//! Skipped (10 of 11):
//! - `CAPI_REGISTER`, `CAPI_RELEASE`, `CAPI_PUT_MESSAGE`, `CAPI_GET_MESSAGE`,
//!   `CAPI_WAIT_FOR_SIGNAL`, `CAPI_GET_VERSION`, `CAPI_GET_SERIAL_NUMBER`,
//!   `CAPI_GET_PROFILE`, `CAPI_INSTALLED`: all marshal parameters and call the
//!   external `capi20_*` C library, which needs the OS and the hardware.
//! - `CAPI_MANUFACTURER`: stub in Wine, returns `0x1109`, nothing to port.

/// The CAPI manufacturer string that Wine substitutes for "AVM".
pub const AVM_MANUFACTURER: &[u8] = b"AVM-GmbH";
/// The prefix Wine checks for in the manufacturer buffer.
pub const AVM_PREFIX: &[u8] = b"AVM";

/// If `buffer` starts with `b"AVM"`, replaces it with `b"AVM-GmbH\0"`.
///
/// Returns `true` if a replacement was made, `false` otherwise. Does nothing
/// when the buffer is too short to hold the replacement, which is a Rust-side
/// guard: the C code relies on the caller's buffer being large enough.
pub fn normalize_manufacturer(buffer: &mut [u8]) -> bool {
    const REPLACEMENT: &[u8] = b"AVM-GmbH\0";

    if buffer.len() < REPLACEMENT.len() || !buffer.starts_with(AVM_PREFIX) {
        return false;
    }
    buffer[..REPLACEMENT.len()].copy_from_slice(REPLACEMENT);
    true
}

#[cfg(test)]
pub mod edge_checks;
#[cfg(test)]
pub mod oracle_checks;
