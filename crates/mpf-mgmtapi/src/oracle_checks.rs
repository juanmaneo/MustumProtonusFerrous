//! Oracle-derived test vectors for the ported mgmtapi logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `SnmpMgrOpen` and
//! `SnmpMgrTrapListen` (`dlls/mgmtapi/mgmtapi.c`) and `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/mgmtapi/`, and running the harness. The 16 printed rows were all clean under
//! ASan and UBSan, so none is excluded.

use super::*;

fn raised(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: 0x8000_0100,
        flags: 0x01,
        number_parameters: 2,
        module,
        function,
    }
}

#[test]
fn oracle_snmp_mgr_open_rows() {
    // row 01 SnmpMgrOpen("public-host", "public", 1000, 3) -> ret=NULL
    assert!(snmp_mgr_open().is_none());
    // row 02 SnmpMgrOpen(NULL, NULL, 0, 0) -> ret=NULL
    assert!(snmp_mgr_open().is_none());
    // row 03 SnmpMgrOpen("", "", -1, -1) -> ret=NULL
    assert!(snmp_mgr_open().is_none());
    // row 04 SnmpMgrOpen("h", NULL, INT_MAX, INT_MIN) -> ret=NULL
    assert!(snmp_mgr_open().is_none());
    // row 05 SnmpMgrOpen(NULL, "private", 5, 0) -> ret=NULL
    assert!(snmp_mgr_open().is_none());
}

#[test]
fn oracle_snmp_mgr_trap_listen_rows() {
    // row 06 SnmpMgrTrapListen(available=NULL, preset_error=0) -> ret=0 last_error=120 slot=unchanged
    assert_eq!(snmp_mgr_trap_listen(), Err(120));
    // row 07 SnmpMgrTrapListen(available=&slot, preset_error=0) -> ret=0 last_error=120 slot=unchanged
    assert_eq!(snmp_mgr_trap_listen(), Err(120));
    // row 08 SnmpMgrTrapListen(available=NULL, preset_error=120) -> ret=0 last_error=120 slot=unchanged
    assert_eq!(snmp_mgr_trap_listen(), Err(120));
    // row 09 SnmpMgrTrapListen(available=&slot, preset_error=999) -> ret=0 last_error=120 slot=unchanged
    assert_eq!(snmp_mgr_trap_listen(), Err(120));
}

#[test]
fn oracle_stub_rows() {
    // row 10 SnmpMgrClose -> raise code=0x80000100 flags=0x1 nargs=2 p0=mgmtapi.dll p1=SnmpMgrClose
    assert_eq!(snmp_mgr_close(), raised("mgmtapi.dll", "SnmpMgrClose"));
    // row 11 SnmpMgrCtl -> raise code=0x80000100 flags=0x1 nargs=2 p0=mgmtapi.dll p1=SnmpMgrCtl
    assert_eq!(snmp_mgr_ctl(), raised("mgmtapi.dll", "SnmpMgrCtl"));
    // row 12 SnmpMgrGetTrap -> raise code=0x80000100 flags=0x1 nargs=2 p0=mgmtapi.dll p1=SnmpMgrGetTrap
    assert_eq!(snmp_mgr_get_trap(), raised("mgmtapi.dll", "SnmpMgrGetTrap"));
    // row 13 SnmpMgrGetTrapEx -> raise code=0x80000100 flags=0x1 nargs=2 p0=mgmtapi.dll p1=SnmpMgrGetTrapEx
    assert_eq!(
        snmp_mgr_get_trap_ex(),
        raised("mgmtapi.dll", "SnmpMgrGetTrapEx")
    );
    // row 14 SnmpMgrOidToStr -> raise code=0x80000100 flags=0x1 nargs=2 p0=mgmtapi.dll p1=SnmpMgrOidToStr
    assert_eq!(
        snmp_mgr_oid_to_str(),
        raised("mgmtapi.dll", "SnmpMgrOidToStr")
    );
    // row 15 SnmpMgrRequest -> raise code=0x80000100 flags=0x1 nargs=2 p0=mgmtapi.dll p1=SnmpMgrRequest
    assert_eq!(snmp_mgr_request(), raised("mgmtapi.dll", "SnmpMgrRequest"));
    // row 16 SnmpMgrStrToOid -> raise code=0x80000100 flags=0x1 nargs=2 p0=mgmtapi.dll p1=SnmpMgrStrToOid
    assert_eq!(
        snmp_mgr_str_to_oid(),
        raised("mgmtapi.dll", "SnmpMgrStrToOid")
    );
}
