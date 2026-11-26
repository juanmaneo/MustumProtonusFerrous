//! Return values of the fltlib filter-manager exports that Wine implements as stubs.

use crate::hresult::{
    hresult_from_win32, ERROR_INVALID_PARAMETER, ERROR_NOT_SUPPORTED, ERROR_NO_MORE_ITEMS,
    E_NOTIMPL, INVALID_HANDLE_VALUE, S_OK,
};

/// `FilterConnectCommunicationPort`. Returns `(hresult, port)`: the stub always stores
/// `INVALID_HANDLE_VALUE` in `*hPort` and returns `E_NOTIMPL`.
pub fn filter_connect_communication_port() -> (i32, isize) {
    (E_NOTIMPL, INVALID_HANDLE_VALUE)
}

/// `FilterFindFirst`: always `HRESULT_FROM_WIN32(ERROR_NO_MORE_ITEMS)`.
pub fn filter_find_first() -> i32 {
    hresult_from_win32(ERROR_NO_MORE_ITEMS)
}

/// `FilterFindClose`: always `S_OK`, whatever the handle.
pub fn filter_find_close() -> i32 {
    S_OK
}

/// `FilterLoad`: always `HRESULT_FROM_WIN32(ERROR_NOT_SUPPORTED)`.
pub fn filter_load() -> i32 {
    hresult_from_win32(ERROR_NOT_SUPPORTED)
}

/// `FilterUnload`: `HRESULT_FROM_WIN32(ERROR_INVALID_PARAMETER)` for a NULL name, else `S_OK`.
pub fn filter_unload(filtername: Option<&[u16]>) -> i32 {
    if filtername.is_none() {
        hresult_from_win32(ERROR_INVALID_PARAMETER)
    } else {
        S_OK
    }
}
