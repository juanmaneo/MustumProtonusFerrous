//! Oracle-derived test vectors for the ported vcruntime140 logic.
//!
//! The vectors were printed by a harness that compiles the verbatim Wine C bodies of `misc.c`,
//! `init_thread.c`, `new.cpp` (with `g++`) and `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`)
//! with `-fsanitize=address,undefined -g`, under `target/canon/vcruntime140/`. The harness's
//! `RtlRaiseException` recorder longjmps out, because the stub body is a `for (;;)` loop.
//!
//! Rows: 17 stub, 4 uncaught, 2 telemetry, 10 init-thread and 12 operator-new rows plus 4 delete rows,
//! 49 in total. One printed row is excluded: `footer_wrap` (`++_Init_global_epoch` at `INT_MAX`),
//! which UBSan flags as signed integer overflow at `init_thread.c:78`. The port wraps there; that
//! is covered by edge_checks.rs and is not oracle-derived.

use super::*;
use std::cell::{Cell, RefCell};

const STUB_ROWS: &[(&str, u32, u32, u32, &str, &str)] = &[
    (
        "_NLG_Dispatch2",
        0x80000100,
        1,
        2,
        "vcruntime140.dll",
        "_NLG_Dispatch2",
    ),
    (
        "_NLG_Return",
        0x80000100,
        1,
        2,
        "vcruntime140.dll",
        "_NLG_Return",
    ),
    (
        "_NLG_Return2",
        0x80000100,
        1,
        2,
        "vcruntime140.dll",
        "_NLG_Return2",
    ),
    (
        "__BuildCatchObject",
        0x80000100,
        1,
        2,
        "vcruntime140.dll",
        "__BuildCatchObject",
    ),
    (
        "__BuildCatchObjectHelper",
        0x80000100,
        1,
        2,
        "vcruntime140.dll",
        "__BuildCatchObjectHelper",
    ),
    (
        "__C_specific_handler_noexcept",
        0x80000100,
        1,
        2,
        "vcruntime140.dll",
        "__C_specific_handler_noexcept",
    ),
    (
        "__FrameUnwindFilter",
        0x80000100,
        1,
        2,
        "vcruntime140.dll",
        "__FrameUnwindFilter",
    ),
    (
        "__GetPlatformExceptionInfo",
        0x80000100,
        1,
        2,
        "vcruntime140.dll",
        "__GetPlatformExceptionInfo",
    ),
    (
        "__NLG_Dispatch2",
        0x80000100,
        1,
        2,
        "vcruntime140.dll",
        "__NLG_Dispatch2",
    ),
    (
        "__NLG_Return2",
        0x80000100,
        1,
        2,
        "vcruntime140.dll",
        "__NLG_Return2",
    ),
    (
        "__TypeMatch",
        0x80000100,
        1,
        2,
        "vcruntime140.dll",
        "__TypeMatch",
    ),
    (
        "__report_gsfailure",
        0x80000100,
        1,
        2,
        "vcruntime140.dll",
        "__report_gsfailure",
    ),
    (
        "__vcrt_GetModuleFileNameW",
        0x80000100,
        1,
        2,
        "vcruntime140.dll",
        "__vcrt_GetModuleFileNameW",
    ),
    (
        "__vcrt_GetModuleHandleW",
        0x80000100,
        1,
        2,
        "vcruntime140.dll",
        "__vcrt_GetModuleHandleW",
    ),
    (
        "__vcrt_LoadLibraryExW",
        0x80000100,
        1,
        2,
        "vcruntime140.dll",
        "__vcrt_LoadLibraryExW",
    ),
    (
        "_is_exception_typeof",
        0x80000100,
        1,
        2,
        "vcruntime140.dll",
        "_is_exception_typeof",
    ),
    (
        "unexpected",
        0x80000100,
        1,
        2,
        "vcruntime140.dll",
        "unexpected",
    ),
];

const UNCAUGHT_ROWS: &[(i32, i32)] = &[(0, 0), (1, 1), (3, 3), (-2, -2)];

const TELEMETRY_ROWS: &[&str] = &[
    "__telemetry_main_invoke_trigger",
    "__telemetry_main_return_trigger",
];

type InitRow = (&'static str, i32, i32, i32, i32, i32, i32, u32);

const INIT_ROWS: &[InitRow] = &[
    (
        "header",
        0,
        -2147483648,
        -2147483648,
        -1,
        -2147483648,
        -2147483648,
        0,
    ),
    ("header", 5, 7, 0, 5, 7, 7, 0),
    ("header", -1, 7, 0, 9, 7, 7, 1),
    ("header", 1, -2147483648, 3, 1, -2147483648, -2147483648, 0),
    ("header", -7, 12, 0, -7, 12, 12, 0),
    (
        "footer",
        -1,
        -2147483648,
        -2147483648,
        -2147483647,
        -2147483647,
        -2147483647,
        0,
    ),
    ("footer", 0, 41, 0, 42, 42, 42, 0),
    ("footer", -1, 99, 5, 100, 100, 100, 0),
    ("abort", -1, 41, 3, 0, 41, 3, 0),
    ("abort", 5, 41, 3, 0, 41, 3, 0),
];

type NewRow = (
    &'static str,
    usize,
    usize,
    u32,
    u32,
    bool,
    &'static [(usize, usize)],
    &'static [usize],
);

const NEW_ROWS: &[NewRow] = &[
    ("operator_new", 0, 0, 0, 0, true, &[(1, 0)], &[]),
    ("operator_new", 24, 0, 0, 0, true, &[(24, 0)], &[]),
    (
        "operator_new",
        24,
        0,
        2,
        5,
        true,
        &[(24, 0), (24, 0), (24, 0)],
        &[24, 24],
    ),
    (
        "operator_new",
        24,
        0,
        99,
        2,
        false,
        &[(24, 0), (24, 0), (24, 0)],
        &[24, 24, 24],
    ),
    ("operator_new", 0, 0, 99, 0, false, &[(1, 0)], &[1]),
    (
        "operator_new_nothrow",
        24,
        0,
        99,
        1,
        false,
        &[(24, 0), (24, 0)],
        &[24, 24],
    ),
    ("operator_new_nothrow", 0, 0, 0, 0, true, &[(1, 0)], &[]),
    ("operator_new_aligned", 0, 0, 0, 0, true, &[(1, 8)], &[]),
    ("operator_new_aligned", 40, 64, 0, 0, true, &[(40, 64)], &[]),
    ("operator_new_aligned", 40, 4, 0, 0, true, &[(40, 8)], &[]),
    (
        "operator_new_aligned_nothrow",
        40,
        16,
        99,
        0,
        false,
        &[(40, 16)],
        &[40],
    ),
    (
        "operator_new_aligned",
        40,
        16,
        1,
        3,
        true,
        &[(40, 16), (40, 16)],
        &[40],
    ),
];

const DELETE_ROWS: &[(&str, bool, u32, u32)] = &[
    ("operator_delete", false, 0, 0),
    ("operator_delete", true, 1, 0),
    ("operator_delete_aligned", false, 0, 0),
    ("operator_delete_aligned", true, 0, 1),
];

/// What `operator new` asked for, and the `_callnewh` sizes, for one scripted run.
#[derive(Default)]
struct Script {
    fails_left: u32,
    retries_left: u32,
    allocs: Vec<(usize, usize)>,
    callnewh_sizes: Vec<usize>,
}

impl Script {
    fn malloc(&mut self, size: usize, alignment: usize) -> Option<usize> {
        self.allocs.push((size, alignment));
        if self.fails_left > 0 {
            self.fails_left -= 1;
            None
        } else {
            Some(0x1000)
        }
    }

    fn callnewh(&mut self, size: usize) -> bool {
        self.callnewh_sizes.push(size);
        if self.retries_left > 0 {
            self.retries_left -= 1;
            true
        } else {
            false
        }
    }
}

#[test]
fn oracle_stub_rows() {
    for &(name, code, flags, number_parameters, module, function) in STUB_ROWS {
        let got = stub_for_export(name).expect(name);
        assert_eq!(
            got,
            StubException {
                code,
                flags,
                number_parameters,
                module,
                function,
            },
            "{name}"
        );
    }
}

#[test]
fn oracle_uncaught_exceptions_rows() {
    for &(value, out) in UNCAUGHT_ROWS {
        assert_eq!(__uncaught_exceptions(&value), out, "{value}");
    }
}

#[test]
fn oracle_telemetry_rows_return() {
    for &name in TELEMETRY_ROWS {
        assert!(name.starts_with("__telemetry_main_"), "{name}");
    }
    // Both triggers are no-ops in the C code too; the row only records that they return.
    __telemetry_main_invoke_trigger(0x1000);
    __telemetry_main_return_trigger(0x1000);
}

#[test]
fn oracle_init_thread_rows() {
    for &(op, once, global, thread, once_after, global_after, thread_after, waits) in INIT_ROWS {
        let mut state = InitThreadState {
            global_epoch: global,
            thread_epoch: thread,
        };
        let mut once_var = once;
        let mut seen_waits = 0u32;
        match op {
            "header" => {
                if state.header(&mut once_var) == HeaderOutcome::Wait {
                    seen_waits += 1;
                    // The harness's SleepConditionVariableSRW shim stores 9 here, the value another
                    // thread's footer leaves behind, so the retried header sees a completed epoch.
                    once_var = 9;
                    assert_eq!(state.header(&mut once_var), HeaderOutcome::Done, "{op}");
                }
            }
            "footer" => state.footer(&mut once_var),
            // The only other op in INIT_ROWS is "abort".
            _ => state.abort(&mut once_var),
        }
        assert_eq!(
            (once_var, state.global_epoch, state.thread_epoch, seen_waits),
            (once_after, global_after, thread_after, waits),
            "{op} {once} {global} {thread}"
        );
    }
}

#[test]
fn oracle_new_rows() {
    for &(op, size, alignment, fails, retries, ok, allocs, callnewh) in NEW_ROWS {
        let script = RefCell::new(Script {
            fails_left: fails,
            retries_left: retries,
            ..Default::default()
        });
        let got_ok = match op {
            "operator_new" => operator_new(
                size,
                |s| script.borrow_mut().malloc(s, 0),
                |s| script.borrow_mut().callnewh(s),
            )
            .is_ok(),
            "operator_new_nothrow" => operator_new_nothrow(
                size,
                |s| script.borrow_mut().malloc(s, 0),
                |s| script.borrow_mut().callnewh(s),
            )
            .is_some(),
            "operator_new_aligned" => operator_new_aligned(
                size,
                alignment,
                |s, a| script.borrow_mut().malloc(s, a),
                |s| script.borrow_mut().callnewh(s),
            )
            .is_ok(),
            _ => operator_new_aligned_nothrow(
                size,
                alignment,
                |s, a| script.borrow_mut().malloc(s, a),
                |s| script.borrow_mut().callnewh(s),
            )
            .is_some(),
        };
        let script = script.into_inner();
        assert_eq!(got_ok, ok, "{op} {size} {alignment}");
        assert_eq!(script.allocs, allocs, "{op} {size} {alignment}");
        assert_eq!(script.callnewh_sizes, callnewh, "{op} {size} {alignment}");
    }
}

#[test]
fn oracle_delete_rows() {
    for &(op, has_ptr, frees, aligned_frees) in DELETE_ROWS {
        let freed = Cell::new(0u32);
        let aligned_freed = Cell::new(0u32);
        let ptr = has_ptr.then_some(0x1000usize);
        match op {
            "operator_delete" => operator_delete(ptr, |_| freed.set(freed.get() + 1)),
            _ => operator_delete_aligned(ptr, |_| aligned_freed.set(aligned_freed.get() + 1)),
        }
        assert_eq!(
            (freed.get(), aligned_freed.get()),
            (frees, aligned_frees),
            "{op} {has_ptr}"
        );
    }
}
