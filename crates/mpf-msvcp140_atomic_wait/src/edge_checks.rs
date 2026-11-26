//! Edge-case tests for the ported msvcp140_atomic_wait logic.
//!
//! These cover the WCHAR-to-char name copy, the shared mutex reference counts, the enumerator
//! failure paths, the bulk submit loop and the export and stub tables. Not every expectation is
//! oracle-derived. The ones that are not say so in a comment.

use super::*;
use std::cell::Cell;

const STUB_EXPORTS: [&str; 11] = [
    "__std_atomic_compare_exchange_128",
    "__std_atomic_get_mutex",
    "__std_atomic_has_cmpxchg16b",
    "__std_atomic_notify_all_indirect",
    "__std_atomic_notify_one_indirect",
    "__std_atomic_set_api_level",
    "__std_atomic_wait_get_deadline",
    "__std_atomic_wait_get_remaining_timeout",
    "__std_atomic_wait_indirect",
    "__std_tzdb_delete_sys_info",
    "__std_tzdb_get_sys_info",
];

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h, and the enum
    // values from `enum tzdb_error` in dlls/msvcp140_atomic_wait/main.c.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    assert_eq!(TzdbError::Success as u32, 0);
    assert_eq!(TzdbError::Win as u32, 1);
    assert_eq!(TzdbError::Icu as u32, 2);
}

#[test]
fn stub_record_takes_any_names() {
    // Not oracle-derived: the harness only passed the 11 export names of the spec. The record
    // shape is the same for any module and function name.
    assert_eq!(
        wine_spec_unimplemented_stub("other.dll", "Ordinal42"),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "other.dll",
            function: "Ordinal42",
        }
    );
}

#[test]
fn export_table_counts_match_spec() {
    // Not oracle-derived: 32 export lines in msvcp140_atomic_wait.spec, 11 of them `@ stub`.
    assert_eq!(EXPORTS.len(), 32);
    let stubs = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .count();
    let stdcall = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stdcall)
        .count();
    assert_eq!(stubs, 11);
    assert_eq!(stdcall, 21);
}

#[test]
fn every_stub_helper_name_is_a_stub_export() {
    // Not oracle-derived: checks that the helper names used in the oracle rows are `@ stub`
    // exports of the spec.
    for name in STUB_EXPORTS {
        assert_eq!(export_kind(name), Some(ExportKind::Stub), "{name}");
    }
}

#[test]
fn export_kind_is_case_sensitive_and_exact() {
    // Not oracle-derived: lookup rules of `export_kind`.
    assert_eq!(
        export_kind("__std_atomic_wait_direct"),
        Some(ExportKind::Stdcall)
    );
    assert_eq!(export_kind("__STD_atomic_wait_direct"), None);
    assert_eq!(export_kind("std_atomic_wait_direct"), None);
    assert_eq!(export_kind("__std_no_such_export"), None);
}

#[test]
fn copy_keeps_low_byte_and_continues() {
    // Not oracle-derived: U+0141 stores its low byte 0x41, and the copy goes on past it.
    let zone = std_tzdb_get_current_zone(&|| Some(vec![0x0141, 0x0041, 0]));
    assert_eq!(
        zone,
        TzdbCurrentZone {
            error: TzdbError::Success,
            name: Some(vec![b'A', b'A', 0]),
        }
    );
}

#[test]
fn zero_low_byte_on_first_char_gives_empty_name() {
    // Not oracle-derived: the first stored byte is zero, so the copy stops at once. The buffer
    // is still wcslen + 1 bytes long.
    let zone = std_tzdb_get_current_zone(&|| Some(vec![0x0200, 0x0041, 0]));
    assert_eq!(zone.name, Some(vec![0, 0, 0]));
}

#[test]
fn empty_standard_name_is_one_zero_byte() {
    // Not oracle-derived: wcslen is 0, so the buffer is 1 byte and holds only the terminator.
    let zone = std_tzdb_get_current_zone(&|| Some(vec![0]));
    assert_eq!(zone.name, Some(vec![0]));
}

#[test]
fn name_without_terminator_gets_a_zero_last_byte() {
    // Not oracle-derived: the C code would read past such an array. The port stops at the slice
    // end and keeps the extra byte zero.
    let zone = std_tzdb_get_current_zone(&|| Some(vec![0x0041, 0x0042]));
    assert_eq!(zone.name, Some(vec![b'A', b'B', 0]));
}

#[test]
fn second_pass_failure_stops_names_and_links_keep_first_pass_count() {
    // Not oracle-derived: the harness enumerator is deterministic, so the second pass never fails.
    // The first pass sees 3 zones (calls 0 to 2) and stops at call 3. The second pass copies index
    // 0 (call 4) and then fails at index 1 (call 5). `links` keeps the 3 slots from the first pass.
    let calls = Cell::new(0);
    let zones = std_tzdb_get_time_zones(&|_| {
        let n = calls.get();
        calls.set(n + 1);
        match n {
            0..=2 | 4 => Some(vec![0x0041, 0]),
            _ => None,
        }
    });
    assert_eq!(zones.names, vec![b"A\0".to_vec()]);
    assert_eq!(zones.links, vec![None; 3]);
}

#[test]
fn leap_seconds_ignore_prev_size() {
    // Not oracle-derived: the oracle passed one prev_size. The C body never reads it.
    assert_eq!(std_tzdb_get_leap_seconds(0), (None, 0));
    assert_eq!(std_tzdb_get_leap_seconds(usize::MAX), (None, 0));
}

#[test]
fn bulk_submit_runs_once_per_count() {
    // Not oracle-derived: counts 1 and 5 are not in the oracle, which used 0 and 3.
    for count in [1usize, 5] {
        let mut submits = Vec::new();
        std_bulk_submit_threadpool_work(0x7000usize, count, |w| submits.push(w));
        assert_eq!(submits, vec![0x7000usize; count]);
    }
}

#[test]
fn null_instance_is_an_ordinary_key() {
    // Not oracle-derived: the C code matches a NULL instance the same way as any other pointer.
    let mut list = SharedMutexList::default();
    assert_eq!(std_acquire_shared_mutex_for_instance(&mut list, 0), 0);
    assert_eq!(std_acquire_shared_mutex_for_instance(&mut list, 0), 0);
    assert_eq!(list.ref_count(0), Some(2));
    std_release_shared_mutex_for_instance(&mut list, 0);
    assert_eq!(list.ref_count(0), Some(1));
}

#[test]
fn distinct_instances_keep_separate_counts() {
    // Not oracle-derived: the oracle used 0x1000 and 0x2000 only.
    let mut list = SharedMutexList::default();
    std_acquire_shared_mutex_for_instance(&mut list, 0x1000);
    std_acquire_shared_mutex_for_instance(&mut list, 0x2000);
    std_acquire_shared_mutex_for_instance(&mut list, 0x2000);
    std_release_shared_mutex_for_instance(&mut list, 0x1000);
    assert_eq!(list.ref_count(0x1000), None);
    assert_eq!(list.ref_count(0x2000), Some(2));
    assert_eq!(list.live_count(), 1);
}

#[test]
fn last_release_removes_element_and_reacquire_starts_at_one() {
    // Not oracle-derived: the oracle never reacquired an instance after its last release.
    let mut list = SharedMutexList::default();
    std_acquire_shared_mutex_for_instance(&mut list, 0x1000);
    std_acquire_shared_mutex_for_instance(&mut list, 0x1000);
    std_release_shared_mutex_for_instance(&mut list, 0x1000);
    std_release_shared_mutex_for_instance(&mut list, 0x1000);
    assert_eq!(list.ref_count(0x1000), None);
    assert_eq!(list.live_count(), 0);
    std_acquire_shared_mutex_for_instance(&mut list, 0x1000);
    assert_eq!(list.ref_count(0x1000), Some(1));
}
