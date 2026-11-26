//! Edge-case tests for the ported vcruntime140 logic.
//!
//! These cover the export table, the stub dispatch, the init-thread state machine and the
//! allocation rules. Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h, and the host pointer size.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    assert_eq!(POINTER_SIZE, std::mem::size_of::<usize>());
}

#[test]
fn export_table_matches_spec_counts() {
    // Not oracle-derived: counts of the entries in dlls/vcruntime140/vcruntime140.spec.
    let count = |want: ExportKind| EXPORTS.iter().filter(|(_, kind)| *kind == want).count();
    assert_eq!(EXPORTS.len(), 88);
    assert_eq!(count(ExportKind::Stub), 17);
    assert_eq!(count(ExportKind::Native), 4);
    assert_eq!(count(ExportKind::Forward), 67);
}

#[test]
fn export_kind_lookup() {
    // Not oracle-derived: spec lookups, including the names that are not exported.
    assert_eq!(export_kind("unexpected"), Some(ExportKind::Stub));
    assert_eq!(
        export_kind("__uncaught_exceptions"),
        Some(ExportKind::Native)
    );
    assert_eq!(export_kind("memcpy"), Some(ExportKind::Forward));
    assert_eq!(export_kind("_Init_thread_header"), None);
    assert_eq!(export_kind("Unexpected"), None);
}

#[test]
fn stub_dispatch_matches_export_kinds() {
    // Not oracle-derived: every `@ stub` name has a stub record, and no other name does.
    for &(name, kind) in EXPORTS.iter() {
        assert_eq!(
            stub_for_export(name).is_some(),
            kind == ExportKind::Stub,
            "{name}"
        );
    }
    assert_eq!(stub_for_export("not_an_export"), None);
}

#[test]
fn generic_stub_record_fields() {
    // Not oracle-derived: the generic record for an arbitrary module and function, as used by
    // the per-export helpers.
    let record = wine_spec_unimplemented_stub("other.dll", "Other");
    assert_eq!(record.code, EXCEPTION_WINE_STUB);
    assert_eq!(record.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(record.number_parameters, 2);
    assert_eq!(record.module, "other.dll");
    assert_eq!(record.function, "Other");
}

#[test]
fn init_thread_default_epochs_are_int_min() {
    // Not oracle-derived: the initial value of both counters, which the oracle's global variables
    // also start at, checked here without running the C code.
    let state = InitThreadState::default();
    assert_eq!(state.global_epoch, i32::MIN);
    assert_eq!(state.thread_epoch, i32::MIN);
}

#[test]
fn init_thread_wait_leaves_state_unchanged() {
    // Not oracle-derived: the wait case returns before it touches anything. The oracle row for a
    // header on -1 only records the state after the waiter has been woken.
    let mut state = InitThreadState {
        global_epoch: 7,
        thread_epoch: 3,
    };
    let mut once = -1;
    assert_eq!(state.header(&mut once), HeaderOutcome::Wait);
    assert_eq!(once, -1);
    assert_eq!((state.global_epoch, state.thread_epoch), (7, 3));
}

#[test]
fn init_thread_footer_wraps_the_global_epoch() {
    // Not oracle-derived: the C `++_Init_global_epoch` at INT_MAX is undefined behaviour, and the
    // oracle row for it is excluded (UBSan). The port wraps, so the value goes to INT_MIN.
    let mut state = InitThreadState {
        global_epoch: i32::MAX,
        thread_epoch: 0,
    };
    let mut once = -1;
    state.footer(&mut once);
    assert_eq!(state.global_epoch, i32::MIN);
    assert_eq!(state.thread_epoch, i32::MIN);
    assert_eq!(once, i32::MIN);
}

#[test]
fn size_and_alignment_normalization() {
    // Not oracle-derived: the boundary at the pointer size. The oracle rows cover alignments 0, 4,
    // 16 and 64; the value exactly at POINTER_SIZE is checked here.
    assert_eq!(normalize_size(0), 1);
    assert_eq!(normalize_size(1), 1);
    assert_eq!(normalize_size(5), 5);
    assert_eq!(normalize_alignment(0), POINTER_SIZE);
    assert_eq!(normalize_alignment(POINTER_SIZE), POINTER_SIZE);
    assert_eq!(normalize_alignment(POINTER_SIZE * 2), POINTER_SIZE * 2);
}
