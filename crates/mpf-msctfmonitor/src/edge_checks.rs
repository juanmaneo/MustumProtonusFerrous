//! Edge-case tests for the ported msctfmonitor logic.
//!
//! These cover the constant values, the flags boundary of `InitLocalMsCtfMonitor`, the stub
//! record's pass-through of names, and the export table. Not every expectation is
//! oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/winternl.h and include/winnt.h.
    assert_eq!(E_FAIL as u32, 0x8000_4005);
    assert_eq!(S_OK, 0);
    assert_eq!(S_FALSE, 1);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn init_returns_e_fail_for_every_single_bit_flags_value() {
    // Not oracle-derived: the C body never branches on `flags`, so each single-bit value
    // (beyond the eight oracle rows) is expected to return E_FAIL.
    for bit in 0..32 {
        assert_eq!(init_local_ms_ctf_monitor(1 << bit), E_FAIL, "bit {bit}");
    }
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the C function stores the two pointers in the record unchanged, so
    // empty and long names are expected to pass through as they are.
    let e = wine_spec_unimplemented_stub("", "");
    assert_eq!(e.module, "");
    assert_eq!(e.function, "");
    assert_eq!(e.code, EXCEPTION_WINE_STUB);
    assert_eq!(e.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(e.number_parameters, 2);

    let long: &'static str = Box::leak("x".repeat(300).into_boxed_str());
    let e = wine_spec_unimplemented_stub("msctfmonitor.dll", long);
    assert_eq!(e.module, "msctfmonitor.dll");
    assert_eq!(e.function.len(), 300);
}

#[test]
fn export_table_matches_spec_order() {
    // The names and their order come from dlls/msctfmonitor/msctfmonitor.spec.
    let names: Vec<&str> = EXPORTS.iter().map(|(name, _)| *name).collect();
    assert_eq!(
        names,
        [
            "DllCanUnloadNow",
            "DllGetClassObject",
            "DllRegisterServer",
            "DllUnregisterServer",
            "DoMsCtfMonitor",
            "InitLocalMsCtfMonitor",
            "UnInitLocalMsCtfMonitor",
        ]
    );
    assert_eq!(
        export_kind("InitLocalMsCtfMonitor"),
        Some(ExportKind::Implemented)
    );
    assert_eq!(
        export_kind("DllCanUnloadNow"),
        Some(ExportKind::WinecrtDefault)
    );
    assert_eq!(
        export_kind("DllRegisterServer"),
        Some(ExportKind::WinecrtDefault)
    );
    assert_eq!(
        export_kind("DllUnregisterServer"),
        Some(ExportKind::WinecrtDefault)
    );
    assert_eq!(export_kind("DllGetClassObject"), Some(ExportKind::Stub));
    assert_eq!(export_kind("DoMsCtfMonitor"), Some(ExportKind::Stub));
    assert_eq!(
        export_kind("UnInitLocalMsCtfMonitor"),
        Some(ExportKind::Stub)
    );
}

#[test]
fn unknown_and_case_mismatched_names_are_not_exports() {
    // Not oracle-derived: the spec file is the only source of export names, and lookups are
    // case-sensitive.
    assert_eq!(export_kind(""), None);
    assert_eq!(export_kind("dllcanunloadnow"), None);
    assert_eq!(export_kind("DllGetClassObjectX"), None);
    assert_eq!(export_kind("DllMain"), None);
}
