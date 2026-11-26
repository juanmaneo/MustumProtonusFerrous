/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = mgmtapi.dll` in `dlls/mgmtapi/Makefile.in`.
const MGMTAPI_DLL_NAME: &str = "mgmtapi.dll";

/// The record that `__wine_spec_unimplemented_stub` raises.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`. The C function raises
/// the record in a loop and never returns, so the port returns the record instead.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        // The module and the function name.
        number_parameters: 2,
        module,
        function,
    }
}

/// `SnmpMgrClose`, exported as `@ stub` in `dlls/mgmtapi/mgmtapi.spec`.
pub fn snmp_mgr_close() -> StubException {
    wine_spec_unimplemented_stub(MGMTAPI_DLL_NAME, "SnmpMgrClose")
}

/// `SnmpMgrCtl`, exported as `@ stub` in `dlls/mgmtapi/mgmtapi.spec`.
pub fn snmp_mgr_ctl() -> StubException {
    wine_spec_unimplemented_stub(MGMTAPI_DLL_NAME, "SnmpMgrCtl")
}

/// `SnmpMgrGetTrap`, exported as `@ stub` in `dlls/mgmtapi/mgmtapi.spec`.
pub fn snmp_mgr_get_trap() -> StubException {
    wine_spec_unimplemented_stub(MGMTAPI_DLL_NAME, "SnmpMgrGetTrap")
}

/// `SnmpMgrGetTrapEx`, exported as `@ stub` in `dlls/mgmtapi/mgmtapi.spec`.
pub fn snmp_mgr_get_trap_ex() -> StubException {
    wine_spec_unimplemented_stub(MGMTAPI_DLL_NAME, "SnmpMgrGetTrapEx")
}

/// `SnmpMgrOidToStr`, exported as `@ stub` in `dlls/mgmtapi/mgmtapi.spec`.
pub fn snmp_mgr_oid_to_str() -> StubException {
    wine_spec_unimplemented_stub(MGMTAPI_DLL_NAME, "SnmpMgrOidToStr")
}

/// `SnmpMgrRequest`, exported as `@ stub` in `dlls/mgmtapi/mgmtapi.spec`.
pub fn snmp_mgr_request() -> StubException {
    wine_spec_unimplemented_stub(MGMTAPI_DLL_NAME, "SnmpMgrRequest")
}

/// `SnmpMgrStrToOid`, exported as `@ stub` in `dlls/mgmtapi/mgmtapi.spec`.
pub fn snmp_mgr_str_to_oid() -> StubException {
    wine_spec_unimplemented_stub(MGMTAPI_DLL_NAME, "SnmpMgrStrToOid")
}
