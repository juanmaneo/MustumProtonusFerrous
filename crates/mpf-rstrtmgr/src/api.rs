/// `ERROR_SUCCESS` from `include/winerror.h`, returned by the explicit functions in `dlls/rstrtmgr/main.c`.
pub const ERROR_SUCCESS: u32 = 0;

/// The handle `RmStartSession` writes, `0xdeadbeef` in `dlls/rstrtmgr/main.c`.
const FIXED_SESSION_HANDLE: u32 = 0xdead_beef;

/// `RmGetList`. Each out-parameter that is not NULL is set to 0.
pub fn rm_get_list(proc_info_needed: Option<&mut u32>, proc_info: Option<&mut u32>) -> u32 {
    if let Some(needed) = proc_info_needed {
        *needed = 0;
    }
    if let Some(info) = proc_info {
        *info = 0;
    }
    ERROR_SUCCESS
}

/// `RmStartSession`. A session handle that is not NULL receives [`FIXED_SESSION_HANDLE`].
pub fn rm_start_session(session_handle: Option<&mut u32>) -> u32 {
    if let Some(handle) = session_handle {
        *handle = FIXED_SESSION_HANDLE;
    }
    ERROR_SUCCESS
}

/// `RmRegisterResources`, which returns `ERROR_SUCCESS` without reading its arguments.
pub fn rm_register_resources() -> u32 {
    ERROR_SUCCESS
}

/// `RmEndSession`, which returns `ERROR_SUCCESS` without reading its handle.
pub fn rm_end_session() -> u32 {
    ERROR_SUCCESS
}

/// `RmRestart`, which returns `ERROR_SUCCESS` without reading its arguments.
pub fn rm_restart() -> u32 {
    ERROR_SUCCESS
}

/// `RmShutdown`, which returns `ERROR_SUCCESS` without reading its arguments.
pub fn rm_shutdown() -> u32 {
    ERROR_SUCCESS
}

/// `RmAddFilter`, which returns `ERROR_SUCCESS` without reading its arguments.
pub fn rm_add_filter() -> u32 {
    ERROR_SUCCESS
}

/// `RmRemoveFilter`, which returns `ERROR_SUCCESS` without reading its arguments.
pub fn rm_remove_filter() -> u32 {
    ERROR_SUCCESS
}
