//! Edge-case tests for the ported avrt logic.
//!
//! These cover the argument-check order, the OutOfMemory branches of the task2
//! conversion, and the stub return values at boundary handles and priorities.
//! Each expectation that is not oracle-derived is marked as such.

use super::*;

const TASK_W: &[u16] = &[0x74, 0x61, 0x73, 0x6b, 0];
const EMPTY_W: &[u16] = &[];

#[test]
fn stub_handle_is_the_fixed_value() {
    // Oracle-derived: rows 03, 06, 10 return 0x12345678.
    assert_eq!(AVRT_STUB_HANDLE, 0x1234_5678);
}

#[test]
fn error_codes_match_winerror_h() {
    // Not oracle-derived: the values come from include/winerror.h. The oracle rows
    // print the same codes (6, 14, 1550) through the shim constants.
    assert_eq!(AvrtError::InvalidHandle.code(), 6);
    assert_eq!(AvrtError::OutOfMemory.code(), 14);
    assert_eq!(AvrtError::InvalidTaskName.code(), 1550);
}

#[test]
fn w_name_check_runs_before_index_check() {
    // Not oracle-derived: the oracle row for name=0 index=1 was dropped. Follows from
    // the check order in main.c.
    assert_eq!(
        av_set_mm_thread_characteristics_w(None, true),
        Err(AvrtError::InvalidTaskName)
    );
}

#[test]
fn w_empty_non_null_name_is_accepted() {
    // Not oracle-derived: main.c tests `!name`, so an empty non-NULL string passes.
    assert_eq!(
        av_set_mm_thread_characteristics_w(Some(EMPTY_W), true),
        Ok(AVRT_STUB_HANDLE)
    );
}

#[test]
fn max_w_missing_first_task_with_second_present() {
    // Not oracle-derived: the oracle row for task1=0 task2=1 was dropped.
    assert_eq!(
        av_set_mm_max_thread_characteristics_w(None, Some(TASK_W), true),
        Err(AvrtError::InvalidTaskName)
    );
}

#[test]
fn max_w_second_task_present_without_index_is_invalid_task_name() {
    // Not oracle-derived: the oracle row for task1=1 task2=1 index=1 was dropped.
    // main.c checks the task names before the index.
    assert_eq!(
        av_set_mm_max_thread_characteristics_w(Some(TASK_W), Some(TASK_W), true),
        Err(AvrtError::InvalidTaskName)
    );
}

#[test]
fn a_out_of_memory_precedes_missing_index() {
    // Not oracle-derived: the conversion runs before the W call, so OutOfMemory wins
    // over a missing index.
    assert_eq!(
        av_set_mm_thread_characteristics_a(AnsiArg::OutOfMemory, false),
        Err(AvrtError::OutOfMemory)
    );
}

#[test]
fn max_a_null_first_task_with_converted_second_task() {
    // Not oracle-derived: the oracle row for (Null, Converted) was not captured.
    assert_eq!(
        av_set_mm_max_thread_characteristics_a(AnsiArg::Null, AnsiArg::Converted(TASK_W), true),
        Err(AvrtError::InvalidTaskName)
    );
}

#[test]
fn max_a_second_task_out_of_memory_after_first_converted() {
    // Not oracle-derived: this input is oracle row 15, which is excluded because
    // LeakSanitizer flags Wine's task1W leak on this return path. The return value
    // follows from the same control flow.
    assert_eq!(
        av_set_mm_max_thread_characteristics_a(
            AnsiArg::Converted(TASK_W),
            AnsiArg::OutOfMemory,
            true
        ),
        Err(AvrtError::OutOfMemory)
    );
}

#[test]
fn max_a_second_task_out_of_memory_with_first_null() {
    // Not oracle-derived: task2 is still converted when task1 is NULL, so its
    // OutOfMemory is reported.
    assert_eq!(
        av_set_mm_max_thread_characteristics_a(AnsiArg::Null, AnsiArg::OutOfMemory, false),
        Err(AvrtError::OutOfMemory)
    );
}

#[test]
fn max_a_out_of_memory_precedes_missing_index() {
    // Not oracle-derived: conversion errors are reported before the index check.
    assert_eq!(
        av_set_mm_max_thread_characteristics_a(AnsiArg::OutOfMemory, AnsiArg::Null, false),
        Err(AvrtError::OutOfMemory)
    );
}

#[test]
fn max_a_converted_first_task_without_index_is_invalid_handle() {
    // Not oracle-derived: the A variant forwards a converted first task to the W check.
    assert_eq!(
        av_set_mm_max_thread_characteristics_a(AnsiArg::Converted(TASK_W), AnsiArg::Null, false),
        Err(AvrtError::InvalidHandle)
    );
}

#[test]
fn stubs_ignore_the_handle_value() {
    // Not oracle-derived: the C stubs never read AvrtHandle, so boundary handles give
    // the same results as the oracle handle.
    for handle in [0, 1, usize::MAX] {
        assert!(!av_query_system_responsiveness(handle));
        assert!(av_revert_mm_thread_characteristics(handle));
        assert!(av_set_mm_thread_priority(handle, 0));
    }
}

#[test]
fn priority_extremes_return_true() {
    // Not oracle-derived: the priority is never read, so i32 extremes outside the
    // AVRT_PRIORITY range also return TRUE.
    for priority in [i32::MIN, i32::MAX] {
        assert!(av_set_mm_thread_priority(AVRT_STUB_HANDLE, priority));
    }
}

#[test]
fn unimplemented_stub_carries_its_function_name() {
    // Not oracle-derived: the generic record for a name the oracle did not capture.
    assert_eq!(
        unimplemented_stub("AvRtExample"),
        StubRaise {
            code: 0x8000_0100,
            flags: 0x01,
            function: "AvRtExample",
        }
    );
}
