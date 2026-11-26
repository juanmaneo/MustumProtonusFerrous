//! Edge-case tests for the ported rstrtmgr logic.
//!
//! These cover the out-parameter boundaries, the fixed session handle, the stub records and the
//! export table. Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/winternl.h and include/winnt.h.
    assert_eq!(ERROR_SUCCESS, 0);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn rm_get_list_zeroes_whatever_the_caller_stored() {
    // Not oracle-derived: the oracle stored only 0xffffffff, 0xfffffffe, 5 and 9. The C body
    // writes 0 whatever the old value was, so the boundary values also come out as 0.
    for start in [0, 1, u32::MAX] {
        let mut needed = start;
        let mut info = start;
        assert_eq!(
            rm_get_list(Some(&mut needed), Some(&mut info)),
            ERROR_SUCCESS
        );
        assert_eq!((needed, info), (0, 0));
    }
}

#[test]
fn rm_start_session_overwrites_any_previous_handle() {
    // Not oracle-derived: the oracle stored only 0x11111111. The C body overwrites whatever the
    // caller stored, including 0 and u32::MAX.
    for start in [0, u32::MAX] {
        let mut handle = start;
        assert_eq!(rm_start_session(Some(&mut handle)), ERROR_SUCCESS);
        assert_eq!(handle, 0xdead_beef);
    }
}

#[test]
fn wine_spec_unimplemented_stub_passes_names_through() {
    // Not oracle-derived: the harness only passed the fixed names of rstrtmgr.spec. The code
    // and flags are the same for any names.
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
fn every_stub_helper_names_a_stub_export() {
    // Not oracle-derived: each helper's function name must be a `@ stub` line of rstrtmgr.spec.
    for stub in [
        rm_cancel_current_task(),
        rm_get_filter_list(),
        rm_join_session(),
        rm_reserve_heap(),
    ] {
        assert_eq!(stub.module, "rstrtmgr.dll");
        assert_eq!(export_kind(stub.function), Some(ExportKind::Stub));
    }
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the twelve lines of dlls/rstrtmgr/rstrtmgr.spec, in order.
    assert_eq!(
        EXPORTS,
        [
            ("RmAddFilter", ExportKind::Stdcall),
            ("RmCancelCurrentTask", ExportKind::Stub),
            ("RmEndSession", ExportKind::Stdcall),
            ("RmGetFilterList", ExportKind::Stub),
            ("RmGetList", ExportKind::Stdcall),
            ("RmJoinSession", ExportKind::Stub),
            ("RmRegisterResources", ExportKind::Stdcall),
            ("RmRemoveFilter", ExportKind::Stdcall),
            ("RmReserveHeap", ExportKind::Stub),
            ("RmRestart", ExportKind::Stdcall),
            ("RmShutdown", ExportKind::Stdcall),
            ("RmStartSession", ExportKind::Stdcall),
        ]
    );
}

#[test]
fn export_kind_is_case_sensitive_and_rejects_unknown_names() {
    // Not oracle-derived: lookups against the spec table.
    assert_eq!(export_kind("RmGetList"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("RmReserveHeap"), Some(ExportKind::Stub));
    assert_eq!(export_kind("rmgetlist"), None);
    assert_eq!(export_kind("RmNotAnExport"), None);
    assert_eq!(export_kind(""), None);
}
