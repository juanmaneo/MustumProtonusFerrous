//! The explicit `dlls/sfc_os/sfc_os.c` functions whose results are fixed.

use crate::{
    Outcome, ERROR_CALL_NOT_IMPLEMENTED, ERROR_FILE_NOT_FOUND, ERROR_INVALID_HANDLE,
    ERROR_NO_MORE_FILES,
};

/// `SfcGetNextProtectedFile`: no protected file is ever listed.
pub fn sfc_get_next_protected_file() -> Outcome<bool> {
    Outcome::with_error(false, ERROR_NO_MORE_FILES)
}

/// `SfcIsFileProtected`: every file is reported as not protected, which is the only result Wine gives.
pub fn sfc_is_file_protected() -> Outcome<bool> {
    Outcome::with_error(false, ERROR_FILE_NOT_FOUND)
}

/// `SfcIsKeyProtected`. `h_key_present` is whether `hKey` is non-NULL. A NULL key fails with
/// `ERROR_INVALID_HANDLE`. Any other key is reported as not protected.
pub fn sfc_is_key_protected(h_key_present: bool) -> Outcome<bool> {
    let code = if h_key_present {
        ERROR_FILE_NOT_FOUND
    } else {
        ERROR_INVALID_HANDLE
    };
    Outcome::with_error(false, code)
}

/// `SfcConnectToServer`: always returns 0 and does not touch the last error.
pub fn sfc_connect_to_server() -> u32 {
    0
}

/// `SfpVerifyFile`: not implemented.
pub fn sfp_verify_file() -> Outcome<bool> {
    Outcome::with_error(false, ERROR_CALL_NOT_IMPLEMENTED)
}
