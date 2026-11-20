//! The server name check from Wine's `netapi32.c`.

use crate::wide::{at, BACKSLASH};
use crate::{ERROR_BAD_NETPATH, ERROR_INVALID_NAME, NERR_SUCCESS};

/// `NETAPI_ValidateServername`: checks a server name. `None` is accepted. An empty name gives
/// `ERROR_BAD_NETPATH`. A name that starts with one backslash, or with `\\` and nothing after it,
/// gives `ERROR_INVALID_NAME`. Any other name gives `NERR_SUCCESS`.
pub fn validate_servername(server_name: Option<&[u16]>) -> u32 {
    let Some(name) = server_name else {
        return NERR_SUCCESS;
    };
    match (at(name, 0), at(name, 1)) {
        (0, _) => ERROR_BAD_NETPATH,
        (BACKSLASH, second) if second != BACKSLASH || at(name, 2) == 0 => ERROR_INVALID_NAME,
        _ => NERR_SUCCESS,
    }
}
