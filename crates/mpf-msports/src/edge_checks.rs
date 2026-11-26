//! Boundary and branch tests. Not every expectation is oracle-derived; the ones that aren't say so
//! in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/winbase.h, include/winternl.h
    // and include/winnt.h.
    assert_eq!(ERROR_ACCESS_DENIED, 5);
    assert_eq!(ERROR_INVALID_PARAMETER, 87);
    assert_eq!(INVALID_HANDLE_VALUE, -1);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn com_db_open_overwrites_any_prior_value() {
    // Not oracle-derived: the oracle used 0, 0x1234 and -1. The C store does not read the old
    // value, so the extremes of the handle type give the same result.
    for prior in [isize::MIN, isize::MAX, 0, 1] {
        let mut h = prior;
        assert_eq!(com_db_open(&mut h), ERROR_ACCESS_DENIED);
        assert_eq!(h, INVALID_HANDLE_VALUE);
    }
}

#[test]
fn com_db_close_result_does_not_depend_on_calls() {
    // Not oracle-derived: the C body has no state, so repeated calls give the same value.
    assert_eq!(com_db_close(), ERROR_INVALID_PARAMETER);
    assert_eq!(com_db_close(), ERROR_INVALID_PARAMETER);
}

#[test]
fn export_kind_is_case_sensitive_and_exact() {
    // Not oracle-derived: the names that the spec does not export, including near misses.
    assert_eq!(export_kind("comdbopen"), None);
    assert_eq!(export_kind("COMDBOPEN"), None);
    assert_eq!(export_kind("ComDBOpen "), None);
    assert_eq!(export_kind(" ComDBOpen"), None);
    assert_eq!(export_kind("ComDB"), None);
    assert_eq!(export_kind(""), None);
    assert_eq!(export_kind("fontsub.dll"), None);
    assert_eq!(export_kind("ComDBOpen"), Some(ExportKind::Stdcall));
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the eleven lines of dlls/msports/msports.spec, in order.
    assert_eq!(
        EXPORTS,
        [
            ("ComDBClaimNextFreePort", ExportKind::Stub),
            ("ComDBClaimPort", ExportKind::Stub),
            ("ComDBClose", ExportKind::Stdcall),
            ("ComDBGetCurrentPortUsage", ExportKind::Stub),
            ("ComDBOpen", ExportKind::Stdcall),
            ("ComDBReleasePort", ExportKind::Stub),
            ("ComDBResizeDatabase", ExportKind::Stub),
            ("ParallelPortPropPageProvider", ExportKind::Stub),
            ("PortsClassInstaller", ExportKind::Stub),
            ("SerialDisplayAdvancedSettings", ExportKind::Stub),
            ("SerialPortPropPageProvider", ExportKind::Stub),
        ]
    );
    assert_eq!(export_kind("ComDBClose"), Some(ExportKind::Stdcall));
    assert_eq!(
        export_kind("SerialPortPropPageProvider"),
        Some(ExportKind::Stub)
    );
}

#[test]
fn every_stub_export_has_a_helper_with_its_name() {
    // Not oracle-derived: pairs each `@ stub` name from the spec with its helper, so a missing
    // or misnamed helper fails here even if the oracle rows still pass.
    type StubFn = fn() -> StubException;
    let helpers: [(StubFn, &str); 9] = [
        (com_db_claim_next_free_port, "ComDBClaimNextFreePort"),
        (com_db_claim_port, "ComDBClaimPort"),
        (com_db_get_current_port_usage, "ComDBGetCurrentPortUsage"),
        (com_db_release_port, "ComDBReleasePort"),
        (com_db_resize_database, "ComDBResizeDatabase"),
        (
            parallel_port_prop_page_provider,
            "ParallelPortPropPageProvider",
        ),
        (ports_class_installer, "PortsClassInstaller"),
        (
            serial_display_advanced_settings,
            "SerialDisplayAdvancedSettings",
        ),
        (serial_port_prop_page_provider, "SerialPortPropPageProvider"),
    ];
    let stubs: Vec<&str> = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .map(|(name, _)| *name)
        .collect();
    assert_eq!(stubs.len(), helpers.len());
    for (helper, name) in helpers {
        assert!(stubs.contains(&name), "{name} is not a @ stub export");
        let raised = helper();
        assert_eq!(raised.function, name);
        assert_eq!(raised.module, "msports.dll");
    }
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed names of msports.spec. The code and
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
fn kinds_format_for_diagnostics() {
    // Not oracle-derived: Debug output is only reached when an assertion fails, so the test
    // formats each variant directly.
    assert_eq!(format!("{:?}", ExportKind::Stdcall), "Stdcall");
    assert_eq!(format!("{:?}", ExportKind::Stub), "Stub");
    assert!(format!("{:?}", wine_spec_unimplemented_stub("a", "b")).contains("StubException"));
}
