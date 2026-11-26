//! `copyOperStatus` and the interface status values from `ipifcons.h`.

pub const MIB_IF_ADMIN_STATUS_UP: u32 = 1;
pub const MIB_IF_ADMIN_STATUS_DOWN: u32 = 2;
pub const MIB_IF_ADMIN_STATUS_TESTING: u32 = 3;

pub const MIB_IF_OPER_STATUS_NON_OPERATIONAL: u32 = 0;
pub const MIB_IF_OPER_STATUS_UNREACHABLE: u32 = 1;
pub const MIB_IF_OPER_STATUS_DISCONNECTED: u32 = 2;
pub const MIB_IF_OPER_STATUS_CONNECTING: u32 = 3;
pub const MIB_IF_OPER_STATUS_CONNECTED: u32 = 4;
pub const MIB_IF_OPER_STATUS_OPERATIONAL: u32 = 5;

/// `copyOperStatus`: maps an IP Helper operational status to the MIB-2 `ifAdminStatus`
/// value. Only OPERATIONAL is UP, CONNECTING and CONNECTED are TESTING, and everything
/// else is DOWN.
pub fn oper_status_to_admin_status(oper: u32) -> u32 {
    match oper {
        MIB_IF_OPER_STATUS_OPERATIONAL => MIB_IF_ADMIN_STATUS_UP,
        MIB_IF_OPER_STATUS_CONNECTING | MIB_IF_OPER_STATUS_CONNECTED => MIB_IF_ADMIN_STATUS_TESTING,
        _ => MIB_IF_ADMIN_STATUS_DOWN,
    }
}
