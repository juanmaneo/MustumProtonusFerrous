/// `ERROR_MR_MID_NOT_FOUND` from `include/winerror.h`.
pub const ERROR_MR_MID_NOT_FOUND: u32 = 317;

/// `ERROR_NOT_SUPPORTED` from `include/winerror.h`.
pub const ERROR_NOT_SUPPORTED: u32 = 50;

/// `MprAdminGetErrorString`, `@ stdcall` export.
///
/// The C body always takes the failure path, so the documented success path is not reached.
/// It never allocates the LocalAlloc buffer, so the out-pointer is always None on return.
pub fn mpr_admin_get_error_string(_mprerror: u32, localstr: &mut Option<Vec<u16>>) -> u32 {
    *localstr = None;
    ERROR_MR_MID_NOT_FOUND
}

/// `MprAdminIsServiceRunning`, `@ stdcall` export. Always FALSE.
pub fn mpr_admin_is_service_running(_server: Option<&[u16]>) -> bool {
    false
}

/// `MprConfigServerConnect`, `@ stdcall` export. The HANDLE is modelled as usize, with NULL
/// as None. The C body clears the out-pointer and fails with ERROR_NOT_SUPPORTED.
pub fn mpr_config_server_connect(
    _server_name: Option<&[u16]>,
    hmprconfig: &mut Option<usize>,
) -> u32 {
    *hmprconfig = None;
    ERROR_NOT_SUPPORTED
}
