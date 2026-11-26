/// `ERROR_CALL_NOT_IMPLEMENTED` from `include/winerror.h`.
pub const ERROR_CALL_NOT_IMPLEMENTED: u32 = 120;

/// `LPSNMP_MGR_SESSION` from `include/mgmtapi.h`, a `void *`. Wine never creates a session, so
/// the type has no values and a `None` return is the only possible result.
pub enum SnmpMgrSession {}

/// `SnmpMgrOpen` from `dlls/mgmtapi/mgmtapi.c`: always returns NULL.
pub fn snmp_mgr_open() -> Option<SnmpMgrSession> {
    None
}

/// `SnmpMgrTrapListen` from `dlls/mgmtapi/mgmtapi.c`. The BOOL result is FALSE in every case,
/// and `Err` carries the value the C body passes to `SetLastError`.
pub fn snmp_mgr_trap_listen() -> Result<(), u32> {
    Err(ERROR_CALL_NOT_IMPLEMENTED)
}
