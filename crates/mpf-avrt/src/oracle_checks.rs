//! Oracle-derived test vectors for the ported avrt logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies from
//! `dlls/avrt/main.c` (lines 30-135) and `__wine_spec_unimplemented_stub` from
//! `dlls/ntdll/exception.c` (lines 504-516) with `gcc -fsanitize=address,undefined -g`
//! under `target/canon/avrt/`, using minimal Windows type shims. The harness printed
//! 25 rows. Row 15 (`set_max_a task1_kind=1 task2_kind=2`) is excluded: LeakSanitizer
//! reports that Wine's body leaks `task1W` on that return path. 24 rows remain.

use super::*;

/// Wide `"task"` fixture. The W variants only test whether the pointer is NULL.
const TASK_W: &[u16] = &[0x74, 0x61, 0x73, 0x6b, 0];

/// One MAX-variant row: `(task1, task2, has_index, handle, error)`.
type MaxRow<T> = (T, T, bool, usize, u32);

/// `(handle, last error)` as the C harness prints them. A failed call returns NULL
/// and sets the error; a successful call leaves the error at 0.
fn outcome(result: Result<usize, AvrtError>) -> (usize, u32) {
    match result {
        Ok(handle) => (handle, 0),
        Err(error) => (0, error.code()),
    }
}

/// The record that the oracle prints for one ordering-group export.
fn oracle_stub(function: &'static str) -> StubRaise {
    StubRaise {
        code: 0x8000_0100,
        flags: 0x01,
        function,
    }
}

#[test]
fn oracle_av_set_mm_thread_characteristics_w_rows() {
    // (name, has_index, handle, error)
    let rows: [(Option<&[u16]>, bool, usize, u32); 3] = [
        // row 01: name=0 index=0
        (None, false, 0, 1550),
        // row 02: name=1 index=0
        (Some(TASK_W), false, 0, 6),
        // row 03: name=1 index=1
        (Some(TASK_W), true, 0x1234_5678, 0),
    ];
    for (name, has_index, handle, error) in rows {
        assert_eq!(
            outcome(av_set_mm_thread_characteristics_w(name, has_index)),
            (handle, error)
        );
    }
}

#[test]
fn oracle_av_set_mm_max_thread_characteristics_w_rows() {
    let rows: [MaxRow<Option<&[u16]>>; 4] = [
        // row 04: task1=0 task2=0 index=0
        (None, None, false, 0, 1550),
        // row 05: task1=1 task2=0 index=0
        (Some(TASK_W), None, false, 0, 6),
        // row 06: task1=1 task2=0 index=1
        (Some(TASK_W), None, true, 0x1234_5678, 0),
        // row 07: task1=1 task2=1 index=0
        (Some(TASK_W), Some(TASK_W), false, 0, 1550),
    ];
    for (task1, task2, has_index, handle, error) in rows {
        assert_eq!(
            outcome(av_set_mm_max_thread_characteristics_w(
                task1, task2, has_index
            )),
            (handle, error)
        );
    }
}

#[test]
fn oracle_av_set_mm_thread_characteristics_a_rows() {
    // (name, has_index, handle, error)
    let rows: [(AnsiArg<'static>, bool, usize, u32); 4] = [
        // row 08: name_kind=0 index=1
        (AnsiArg::Null, true, 0, 1550),
        // row 09: name_kind=1 index=0
        (AnsiArg::Converted(TASK_W), false, 0, 6),
        // row 10: name_kind=1 index=1
        (AnsiArg::Converted(TASK_W), true, 0x1234_5678, 0),
        // row 11: name_kind=2 (strdupAW malloc fails) index=1
        (AnsiArg::OutOfMemory, true, 0, 14),
    ];
    for (name, has_index, handle, error) in rows {
        assert_eq!(
            outcome(av_set_mm_thread_characteristics_a(name, has_index)),
            (handle, error)
        );
    }
}

#[test]
fn oracle_av_set_mm_max_thread_characteristics_a_rows() {
    let rows: [MaxRow<AnsiArg<'static>>; 3] = [
        // row 12: task1_kind=0 task2_kind=0 index=1
        (AnsiArg::Null, AnsiArg::Null, true, 0, 1550),
        // row 13: task1_kind=1 task2_kind=1 index=1
        (
            AnsiArg::Converted(TASK_W),
            AnsiArg::Converted(TASK_W),
            true,
            0,
            1550,
        ),
        // row 14: task1_kind=2 task2_kind=0 index=1
        (AnsiArg::OutOfMemory, AnsiArg::Null, true, 0, 14),
    ];
    for (task1, task2, has_index, handle, error) in rows {
        assert_eq!(
            outcome(av_set_mm_max_thread_characteristics_a(
                task1, task2, has_index
            )),
            (handle, error)
        );
    }
}

#[test]
fn oracle_stub_return_values() {
    // row 16: query handle=0x12345678 -> ret=0
    assert!(!av_query_system_responsiveness(0x1234_5678));
    // row 17: revert handle=0x12345678 -> ret=1
    assert!(av_revert_mm_thread_characteristics(0x1234_5678));
    // row 18: priority handle=0x12345678 prio=-2 -> ret=1
    assert!(av_set_mm_thread_priority(0x1234_5678, -2));
}

#[test]
fn oracle_ordering_group_stub_exceptions() {
    // rows 19-25: each `@ stub` export raises EXCEPTION_WINE_STUB and names itself.
    assert_eq!(
        av_rt_create_thread_ordering_group(),
        oracle_stub("AvRtCreateThreadOrderingGroup")
    );
    assert_eq!(
        av_rt_create_thread_ordering_group_ex_a(),
        oracle_stub("AvRtCreateThreadOrderingGroupExA")
    );
    assert_eq!(
        av_rt_create_thread_ordering_group_ex_w(),
        oracle_stub("AvRtCreateThreadOrderingGroupExW")
    );
    assert_eq!(
        av_rt_delete_thread_ordering_group(),
        oracle_stub("AvRtDeleteThreadOrderingGroup")
    );
    assert_eq!(
        av_rt_join_thread_ordering_group(),
        oracle_stub("AvRtJoinThreadOrderingGroup")
    );
    assert_eq!(
        av_rt_leave_thread_ordering_group(),
        oracle_stub("AvRtLeaveThreadOrderingGroup")
    );
    assert_eq!(
        av_rt_wait_on_thread_ordering_group(),
        oracle_stub("AvRtWaitOnThreadOrderingGroup")
    );
}
