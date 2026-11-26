use super::*;

#[test]
fn export_table_has_every_spec_export_in_order() {
    let names: Vec<&str> = EXPORTS.iter().map(|(name, _)| *name).collect();
    assert_eq!(names.len(), 19);
    assert_eq!(names[0], "VDMBreakThread");
    assert_eq!(names[18], "VDMTerminateTaskWOW");
}

#[test]
fn export_kind_finds_stdcall_and_stub_names() {
    assert_eq!(export_kind("VDMEnumProcessWOW"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("VDMEnumTaskWOW"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("VDMKillWOW"), Some(ExportKind::Stub));
}

#[test]
fn export_kind_rejects_unknown_and_case_mismatched_names() {
    // Not oracle-derived: the spec exports no such name, and names are case-sensitive.
    assert_eq!(export_kind(""), None);
    assert_eq!(export_kind("vdmkillwow"), None);
    assert_eq!(export_kind("VDMKillWOW2"), None);
}

#[test]
fn stub_exports_in_table_are_the_seventeen_stub_helpers() {
    // Not oracle-derived: counts the `@ stub` lines in vdmdbg.spec (17 of 19 exports).
    let stubs = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .count();
    assert_eq!(stubs, 17);
}

#[test]
fn stub_record_passes_module_and_function_through() {
    // Not oracle-derived: the C function stores the two pointers verbatim, so any strings round-trip.
    let record = wine_spec_unimplemented_stub("other.dll", "Name");
    assert_eq!(record.module, "other.dll");
    assert_eq!(record.function, "Name");
    assert_eq!(record.code, EXCEPTION_WINE_STUB);
    assert_eq!(record.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(record.number_parameters, 2);
}

#[test]
fn stub_constants_match_the_wine_headers() {
    // Not oracle-derived: the values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x1);
}
