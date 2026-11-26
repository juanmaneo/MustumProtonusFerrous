//! Edge-case tests for the ported loadperf logic.
//!
//! These cover the NULL pass-through, the allocation-failure returns, the conversion order,
//! and the export and stub tables. Not every expectation is oracle-derived; the ones that aren't
//! say so in a comment.

use super::*;
use std::cell::{Cell, RefCell};

fn widen(s: &[u8]) -> Option<Vec<u16>> {
    Some(s.iter().map(|&b| u16::from(b)).collect())
}

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/winternl.h and include/winnt.h.
    assert_eq!(ERROR_SUCCESS, 0);
    assert_eq!(ERROR_OUTOFMEMORY, 14);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn null_string_is_never_converted() {
    // Not oracle-derived: the C wrappers skip strdupAW for a NULL argument (`cmdline && ...`).
    // The same counting converter is then given a non-NULL string, so it is also exercised.
    let calls = Cell::new(0);
    let convert = |s: &[u8]| {
        calls.set(calls.get() + 1);
        widen(s)
    };
    assert_eq!(strdup_aw(None, convert), Ok(None));
    assert_eq!(calls.get(), 0);
    assert_eq!(strdup_aw(Some(&b"x"[..]), convert), Ok(Some(vec![0x78])));
    assert_eq!(calls.get(), 1);
}

#[test]
fn empty_string_is_converted_not_treated_as_null() {
    // Not oracle-derived: an empty string is not NULL, so it is converted. The C code allocates
    // room for the terminating NUL alone.
    assert_eq!(strdup_aw(Some(&b""[..]), widen), Ok(Some(Vec::new())));
}

#[test]
fn failed_conversion_is_out_of_memory() {
    // Not oracle-derived: strdupAW's NULL return for a non-NULL string, which the wrappers map
    // to ERROR_OUTOFMEMORY.
    assert_eq!(strdup_aw(Some(&b"x"[..]), |_| None), Err(ERROR_OUTOFMEMORY));
}

#[test]
fn install_perf_dll_a_converts_computer_before_ini() {
    // Not oracle-derived: the order of the two conversions. The C wrapper converts `computer`
    // first, then `ini`.
    let seen = RefCell::new(Vec::new());
    let convert = |s: &[u8]| {
        seen.borrow_mut().push(s.to_vec());
        widen(s)
    };
    let ret = install_perf_dll_a(Some(&b"computer"[..]), Some(&b"ini"[..]), 0, convert);
    assert_eq!(ret, ERROR_SUCCESS);
    assert_eq!(*seen.borrow(), [b"computer".to_vec(), b"ini".to_vec()]);
}

#[test]
fn install_perf_dll_a_stops_at_the_first_failed_conversion() {
    // Not oracle-derived: once `computer` fails, `ini` is never converted.
    let calls = Cell::new(0);
    let convert = |_: &[u8]| {
        calls.set(calls.get() + 1);
        None
    };
    let ret = install_perf_dll_a(Some(&b"computer"[..]), Some(&b"ini"[..]), 0, convert);
    assert_eq!(ret, ERROR_OUTOFMEMORY);
    assert_eq!(calls.get(), 1);
}

#[test]
fn flags_are_not_read() {
    // Not oracle-derived: the C bodies never read `flags`, so the extreme value gives the same
    // return as zero.
    assert_eq!(install_perf_dll_w(None, None, usize::MAX), ERROR_SUCCESS);
    assert_eq!(
        install_perf_dll_a(None, None, usize::MAX, widen),
        ERROR_SUCCESS
    );
}

#[test]
fn quiet_is_not_read() {
    // Not oracle-derived: `quiet` only feeds the FIXME trace, so both values return the same.
    assert_eq!(load_perf_counter_text_strings_w(None, true), ERROR_SUCCESS);
    assert_eq!(load_perf_counter_text_strings_w(None, false), ERROR_SUCCESS);
    assert_eq!(
        unload_perf_counter_text_strings_w(None, true),
        ERROR_SUCCESS
    );
    assert_eq!(
        unload_perf_counter_text_strings_w(None, false),
        ERROR_SUCCESS
    );
}

#[test]
fn export_table_matches_the_spec() {
    // Not oracle-derived: counts from dlls/loadperf/loadperf.spec (14 exports: 8 stubs and 6
    // stdcall functions).
    assert_eq!(EXPORTS.len(), 14);
    let stubs = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .count();
    assert_eq!(stubs, 8);
    assert_eq!(EXPORTS.len() - stubs, 6);
}

#[test]
fn export_kind_is_case_sensitive_and_total() {
    // Not oracle-derived: lookups of spec names, a case-mismatched name and a missing name.
    assert_eq!(export_kind("InstallPerfDllA"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("SetServiceAsTrustedW"), Some(ExportKind::Stub));
    assert_eq!(export_kind("installperfdlla"), None);
    assert_eq!(export_kind(""), None);
}

#[test]
fn stub_helpers_are_the_spec_stub_exports() {
    // Not oracle-derived: every stub helper names an export whose kind is Stub, and there are
    // no other Stub entries in the table.
    let helpers = [
        backup_perf_registry_to_file_w().function,
        load_mof_from_installed_service_a().function,
        load_mof_from_installed_service_w().function,
        restore_perf_registry_from_file_w().function,
        set_service_as_trusted_a().function,
        set_service_as_trusted_w().function,
        update_perf_name_files_a().function,
        update_perf_name_files_w().function,
    ];
    for name in helpers {
        assert_eq!(export_kind(name), Some(ExportKind::Stub), "{name}");
    }
    let stubs = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .count();
    assert_eq!(stubs, helpers.len());
}
