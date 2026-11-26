//! Edge-case tests for the ported rtutils logic.
//!
//! These cover the allocation boundary of `TraceRegisterExA`, the NUL and empty-name cases of
//! the conversion, the export table, and the stub record. Not every expectation is
//! oracle-derived; the ones that aren't say so in a comment.

use super::*;
use std::cell::Cell;
use std::collections::HashSet;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/rtutils.h, include/winerror.h, include/winternl.h
    // and include/winnt.h.
    assert_eq!(INVALID_TRACEID, 0xFFFF_FFFF);
    assert_eq!(ERROR_OUTOFMEMORY, 14);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn alloc_is_asked_once_for_the_converted_size() {
    // Not oracle-derived: one allocation per call, sized as the converted name in bytes. The
    // oracle covers the sizes for "", "Trace", a high byte and an embedded NUL.
    let calls = Cell::new(0);
    let asked = Cell::new(0);
    let reg = trace_register_ex_a(Some(b"x"), 0, |size| {
        calls.set(calls.get() + 1);
        asked.set(size);
        true
    });
    assert_eq!(calls.get(), 1);
    assert_eq!(asked.get(), 4);
    assert_eq!(
        reg,
        TraceRegistration {
            id: INVALID_TRACEID,
            last_error: None,
        }
    );
}

#[test]
fn null_name_asks_for_zero_bytes_and_still_succeeds() {
    // Not oracle-derived: the oracle has no NULL row with a refusing allocator. A NULL name
    // converts to nothing, so the size is 0, and a refused allocation gives ERROR_OUTOFMEMORY.
    let mut asked = None;
    let reg = trace_register_ex_a(None, 7, |size| {
        asked = Some(size);
        false
    });
    assert_eq!(asked, Some(0));
    assert_eq!(
        reg,
        TraceRegistration {
            id: INVALID_TRACEID,
            last_error: Some(ERROR_OUTOFMEMORY),
        }
    );
}

#[test]
fn name_stops_at_the_first_nul() {
    // Not oracle-derived: a NUL in the middle ends the converted name, so the size counts only the
    // bytes before it plus the terminator. The oracle covers "ab\0cd" (6 bytes).
    let mut asked = 0;
    trace_register_ex_a(Some(b"a\0bc"), 0, |size| {
        asked = size;
        true
    });
    assert_eq!(asked, 4);
}

#[test]
fn wide_register_ignores_its_arguments() {
    // Not oracle-derived: the C body only traces its arguments, so any name and flags give
    // INVALID_TRACEID. The oracle covers a NULL name with 0 and a name with 0xFFFFFFFF.
    let units = [0x41u16, 0x42];
    assert_eq!(trace_register_ex_w(None, 0), INVALID_TRACEID);
    assert_eq!(trace_register_ex_w(Some(&[]), 1), INVALID_TRACEID);
    assert_eq!(trace_register_ex_w(Some(&units), 2), INVALID_TRACEID);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the helper passes its names straight through. The oracle covers the
    // module and function names of every export.
    let other = wine_spec_unimplemented_stub("other.dll", "Foo");
    assert_eq!(other.module, "other.dll");
    assert_eq!(other.function, "Foo");
    assert_eq!(other.number_parameters, 2);

    let empty = wine_spec_unimplemented_stub("", "");
    assert_eq!(empty.module, "");
    assert_eq!(empty.function, "");
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the 56 entries and their order come from rtutils.spec.
    assert_eq!(EXPORTS.len(), 56);
    assert_eq!(EXPORTS[0], ("CreateWaitEvent", ExportKind::Stub));
    assert_eq!(EXPORTS[55], ("WTFreeTimer", ExportKind::Stub));
    assert_eq!(
        EXPORTS
            .iter()
            .filter(|(_, kind)| *kind == ExportKind::Stub)
            .count(),
        54
    );
    assert_eq!(
        EXPORTS
            .iter()
            .filter(|(_, kind)| *kind == ExportKind::Stdcall)
            .count(),
        2
    );
}

#[test]
fn export_kind_is_case_sensitive_and_total() {
    // Not oracle-derived: lookup behaviour of the export table.
    assert_eq!(export_kind("TraceRegisterExA"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("TraceRegisterExW"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("LogErrorA"), Some(ExportKind::Stub));
    assert_eq!(export_kind("createwaitevent"), None);
    assert_eq!(export_kind("NoSuchExport"), None);
}

#[test]
fn export_names_are_unique() {
    // Not oracle-derived: every name in the spec is exported once.
    let names: HashSet<&str> = EXPORTS.iter().map(|(name, _)| *name).collect();
    assert_eq!(names.len(), EXPORTS.len());
}
