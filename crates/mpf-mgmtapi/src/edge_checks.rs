//! Edge-case tests for the ported mgmtapi logic.
//!
//! These cover the constants, the stub record, the stub helpers against the export table, and
//! the lookup behaviour of the export table. Not every expectation is oracle-derived; the ones
//! that aren't say so in a comment.

use super::*;

type StubHelper = fn() -> StubException;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/winternl.h and include/winnt.h.
    assert_eq!(ERROR_CALL_NOT_IMPLEMENTED, 120);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed names of mgmtapi.spec. The code and
    // flags are the same for any names.
    let other = wine_spec_unimplemented_stub("other.dll", "Foo");
    assert_eq!(other.module, "other.dll");
    assert_eq!(other.function, "Foo");
    assert_eq!(other.code, EXCEPTION_WINE_STUB);
    assert_eq!(other.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(other.number_parameters, 2);

    let empty = wine_spec_unimplemented_stub("", "");
    assert_eq!(empty.module, "");
    assert_eq!(empty.function, "");
}

#[test]
fn stub_helpers_match_the_stub_exports_in_the_spec() {
    // Not oracle-derived: each helper must raise the generic record for its own export name, and
    // that name must be a `@ stub` line in the export table.
    let helpers: [(StubHelper, &str); 7] = [
        (snmp_mgr_close, "SnmpMgrClose"),
        (snmp_mgr_ctl, "SnmpMgrCtl"),
        (snmp_mgr_get_trap, "SnmpMgrGetTrap"),
        (snmp_mgr_get_trap_ex, "SnmpMgrGetTrapEx"),
        (snmp_mgr_oid_to_str, "SnmpMgrOidToStr"),
        (snmp_mgr_request, "SnmpMgrRequest"),
        (snmp_mgr_str_to_oid, "SnmpMgrStrToOid"),
    ];
    for (helper, name) in helpers {
        assert_eq!(helper(), wine_spec_unimplemented_stub("mgmtapi.dll", name));
        assert_eq!(export_kind(name), Some(ExportKind::Stub));
    }
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the nine lines of dlls/mgmtapi/mgmtapi.spec, in order.
    assert_eq!(
        EXPORTS,
        [
            ("SnmpMgrClose", ExportKind::Stub),
            ("SnmpMgrCtl", ExportKind::Stub),
            ("SnmpMgrGetTrap", ExportKind::Stub),
            ("SnmpMgrGetTrapEx", ExportKind::Stub),
            ("SnmpMgrOidToStr", ExportKind::Stub),
            ("SnmpMgrOpen", ExportKind::Stdcall),
            ("SnmpMgrRequest", ExportKind::Stub),
            ("SnmpMgrStrToOid", ExportKind::Stub),
            ("SnmpMgrTrapListen", ExportKind::Stdcall),
        ]
    );
}

#[test]
fn export_kind_is_case_sensitive_and_rejects_unknown_names() {
    // Not oracle-derived: lookup behaviour of export_kind.
    assert_eq!(export_kind("SnmpMgrOpen"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("SnmpMgrTrapListen"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("SnmpMgrCtl"), Some(ExportKind::Stub));
    assert_eq!(export_kind("snmpmgropen"), None);
    assert_eq!(export_kind(""), None);
}
