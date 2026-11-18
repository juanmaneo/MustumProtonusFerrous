//! Headless subset of Wine's dlls/wtsapi32/wtsapi32.c.
//!
//! Ported:
//! - WTSEnumerateProcessesExW and WTSEnumerateProcessesW: the argument checks (check_enumerate_processes_ex_w, check_enumerate_processes_w), the walk of the NT process chain (process_list::parse_process_list) and the output size (process_list::process_info_size).
//! - WTSQuerySessionInformationA: the argument check (check_query_session_information) and the class routing (session_info_route_a).
//!
//! Skipped (each is a FIXME stub, or needs the OS, a session, a token or a handle):
//! - The NtQuerySystemInformation buffer growth loop, OpenProcess, the token and SID copy, and the per-process user SID.
//! - WTSEnumerateProcessesA and WTSEnumerateProcessesExA, and the other server and session exports (WTSOpenServer*, WTSLogoffSession, WTSConnectSession*, and the rest): FIXME stubs.
//! - The WTSQuerySessionInformationW call, which goes to the session service, and the WCHAR to code page conversion.
//! - WTSFreeMemory, which is free.

pub mod process_list;

/// ERROR_INVALID_PARAMETER.
pub const ERROR_INVALID_PARAMETER: u32 = 87;
/// ERROR_CALL_NOT_IMPLEMENTED.
pub const ERROR_CALL_NOT_IMPLEMENTED: u32 = 120;
/// ERROR_INVALID_USER_BUFFER.
pub const ERROR_INVALID_USER_BUFFER: u32 = 1784;

/// WTS_INFO_CLASS values that WTSQuerySessionInformationA routes by value.
const WTS_CONNECT_STATE: i32 = 8;
const WTS_CLIENT_PROTOCOL_TYPE: i32 = 16;
const WTS_SESSION_INFO: i32 = 24;

/// How WTSQuerySessionInformationA produces its result for a class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionInfoRouteA {
    /// WTSConnectState and WTSClientProtocolType pass the caller's pointers through to the W query.
    Passthrough,
    /// WTSSessionInfo converts a WTSINFOW into a WTSINFOA, field by field.
    SessionInfo,
    /// Any other class converts the W result string into the ANSI buffer.
    Convert,
}

/// WTSEnumerateProcessesW rejects a non-zero reserved value and any version other than 1.
pub fn check_enumerate_processes_w(reserved: u32, version: u32) -> Result<(), u32> {
    if reserved != 0 || version != 1 {
        return Err(ERROR_INVALID_PARAMETER);
    }
    Ok(())
}

/// WTSEnumerateProcessesExW rejects missing output pointers first, then any non-zero level.
pub fn check_enumerate_processes_ex_w(
    has_info: bool,
    has_count: bool,
    level: u32,
) -> Result<(), u32> {
    if !has_info || !has_count {
        return Err(ERROR_INVALID_PARAMETER);
    }
    if level != 0 {
        return Err(ERROR_CALL_NOT_IMPLEMENTED);
    }
    Ok(())
}

/// WTSQuerySessionInformationA rejects a missing buffer or count pointer.
pub fn check_query_session_information(has_buffer: bool, has_count: bool) -> Result<(), u32> {
    if !has_buffer || !has_count {
        return Err(ERROR_INVALID_USER_BUFFER);
    }
    Ok(())
}

/// The route WTSQuerySessionInformationA takes for a WTS_INFO_CLASS value.
pub fn session_info_route_a(class: i32) -> SessionInfoRouteA {
    match class {
        WTS_CONNECT_STATE | WTS_CLIENT_PROTOCOL_TYPE => SessionInfoRouteA::Passthrough,
        WTS_SESSION_INFO => SessionInfoRouteA::SessionInfo,
        _ => SessionInfoRouteA::Convert,
    }
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
