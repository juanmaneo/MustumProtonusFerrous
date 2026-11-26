//! Edge-case tests for the ported riched32 logic.
//!
//! These cover the reason values that the C switch does not name, the `lpvReserved` boundary,
//! the module, window procedure and cursor values that the C code passes through unchanged, and
//! the `LoadCursorA` failure that the C code ignores. Not every expectation is oracle-derived; the
//! ones that aren't say so in a comment.

use super::*;
use crate::oracle_checks::{printed_class, Call, Recorder, HINST, WND_PROC};

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winnt.h, include/winuser.rh, include/winuser.h and
    // include/richedit.h.
    assert_eq!(DLL_PROCESS_DETACH, 0);
    assert_eq!(DLL_PROCESS_ATTACH, 1);
    assert_eq!(DLL_THREAD_ATTACH, 2);
    assert_eq!(DLL_THREAD_DETACH, 3);
    assert_eq!(CS_VREDRAW, 0x1);
    assert_eq!(CS_HREDRAW, 0x2);
    assert_eq!(CS_DBLCLKS, 0x8);
    assert_eq!(CS_GLOBALCLASS, 0x4000);
    assert_eq!(COLOR_WINDOW, 5);
    assert_eq!(IDC_ARROW, 32512);
    assert_eq!(RICHEDIT_CLASS10A, "RICHEDIT");
}

#[test]
fn reasons_without_a_case_make_no_calls() {
    // Partly oracle-derived: the harness covers 2, 3, 4 and u32::MAX. The value 0x8000_0000 is
    // not oracle-derived. The C switch has no case for any of these, so each returns TRUE with
    // no OS call.
    for reason in [
        DLL_THREAD_ATTACH,
        DLL_THREAD_DETACH,
        4,
        0x8000_0000,
        u32::MAX,
    ] {
        let mut rec = Recorder::default();
        assert!(dll_main(&mut rec, HINST, reason, 0, WND_PROC));
        assert!(rec.calls.is_empty());
    }
}

#[test]
fn lpv_reserved_is_only_checked_on_detach() {
    // Not oracle-derived for usize::MAX: any non-NULL lpvReserved skips the unregister, and the
    // attach path ignores lpvReserved.
    let mut rec = Recorder::default();
    assert!(dll_main(
        &mut rec,
        HINST,
        DLL_PROCESS_DETACH,
        usize::MAX,
        WND_PROC
    ));
    assert!(rec.calls.is_empty());

    let mut rec = Recorder::default();
    assert!(dll_main(
        &mut rec,
        HINST,
        DLL_PROCESS_ATTACH,
        usize::MAX,
        WND_PROC
    ));
    assert_eq!(
        rec.calls,
        vec![
            Call::DisableThreadLibraryCalls(HINST),
            Call::LoadCursorA(0, IDC_ARROW),
            Call::RegisterClassA(printed_class()),
        ]
    );
}

#[test]
fn zero_module_and_window_procedure_pass_through() {
    // Not oracle-derived: the C code neither checks hinstDLL nor the window procedure address
    // before it calls DisableThreadLibraryCalls and RegisterClassA. A NULL module and a NULL
    // window procedure are stored as given.
    let mut rec = Recorder::default();
    assert!(dll_main(&mut rec, 0, DLL_PROCESS_ATTACH, 0, 0));
    assert_eq!(
        rec.calls,
        vec![
            Call::DisableThreadLibraryCalls(0),
            Call::LoadCursorA(0, IDC_ARROW),
            Call::RegisterClassA(WndClassA {
                lpfn_wnd_proc: 0,
                ..printed_class()
            }),
        ]
    );
}

#[test]
fn failed_load_cursor_still_registers_the_class() {
    // Not oracle-derived: the harness's LoadCursorA never fails. RICHED32_Register stores the
    // cursor handle without checking it, still calls RegisterClassA, and returns TRUE.
    let mut rec = Recorder {
        cursor: 0,
        ..Recorder::default()
    };
    assert!(register_class(&mut rec, WND_PROC));
    assert_eq!(
        rec.calls,
        vec![
            Call::LoadCursorA(0, IDC_ARROW),
            Call::RegisterClassA(WndClassA {
                h_cursor: 0,
                ..printed_class()
            }),
        ]
    );
}
