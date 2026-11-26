//! Edge-case tests for the ported gamingtcui logic.
//!
//! These cover the completion-routine branches, pass-through of arguments the reference never
//! inspects, and the export table. Expectations marked "reference-derived" come from the Wine
//! source and are not captured by the oracle run.

use super::*;

type PickerRoutine = fn(Hresult, Handle, HStringArray);
type ProfileRoutine = fn(Hresult, Handle);

fn list(ptr: Handle, count: usize) -> HStringArray {
    HStringArray { ptr, count }
}

/// C-style `PlayerPickerUICompletionRoutine`, called by `ShowPlayerPickerUI` in the test below.
fn picker_routine(rc: Hresult, ctx: Handle, selected: HStringArray) {
    assert_eq!((rc, ctx, selected), (S_OK, 0x40, list(0x30, 1)));
}

/// C-style `GameUICompletionRoutine`, called by `ShowProfileCardUI` in the test below.
fn profile_routine(rc: Hresult, ctx: Handle) {
    assert_eq!((rc, ctx), (S_OK, 0x6000));
}

#[test]
fn process_pending_game_ui_ignores_wait_flag() {
    // reference-derived: the body ignores wait_for_completion and returns S_OK.
    for wait in [0, 1, -1, i32::MIN, i32::MAX] {
        assert_eq!(process_pending_game_ui(wait), S_OK);
    }
}

#[test]
fn show_player_picker_ui_without_completion_returns_s_ok() {
    // reference-derived: `if (completion_routine)` is false, so only S_OK is returned.
    let rc = show_player_picker_ui(0, list(0, 0), list(0, 0), 0, 0, None::<PickerRoutine>, 0);
    assert_eq!(rc, S_OK);
}

#[test]
fn show_player_picker_ui_accepts_function_pointer_routine() {
    // reference-derived: the completion routine is a C function pointer. The routine asserts
    // the arguments, and it runs only if the reference's call is reproduced.
    let routine: PickerRoutine = picker_routine;
    let rc = show_player_picker_ui(1, list(0x20, 9), list(0x30, 1), 0, 0, Some(routine), 0x40);
    assert_eq!(rc, S_OK);
}

#[test]
fn show_player_picker_ui_passes_null_list_through() {
    // reference-derived: a NULL pointer with a nonzero count is passed to the completion
    // routine unchanged; the reference never dereferences it.
    let mut seen: Option<(Hresult, Handle, HStringArray)> = None;
    let rc = show_player_picker_ui(
        0,
        list(0, 0),
        list(0, 5),
        0,
        0,
        Some(|rc, ctx, selected| seen = Some((rc, ctx, selected))),
        0,
    );
    assert_eq!(rc, S_OK);
    assert_eq!(seen, Some((S_OK, 0, list(0, 5))));
}

#[test]
fn show_player_picker_ui_ignores_min_max_bounds() {
    // reference-derived: min and max are unused, so min > max still reaches the completion routine.
    let mut seen: Option<(Hresult, Handle, HStringArray)> = None;
    let rc = show_player_picker_ui(
        1,
        list(0x20, 9),
        list(0x30, 1),
        usize::MAX,
        0,
        Some(|rc, ctx, selected| seen = Some((rc, ctx, selected))),
        0x40,
    );
    assert_eq!(rc, S_OK);
    assert_eq!(seen, Some((S_OK, 0x40, list(0x30, 1))));
}

#[test]
fn show_profile_card_ui_without_completion_returns_s_ok() {
    // reference-derived: `if (completion_routine)` is false, so only S_OK is returned.
    assert_eq!(show_profile_card_ui(0, None::<ProfileRoutine>, 0), S_OK);
}

#[test]
fn show_profile_card_ui_accepts_function_pointer_routine() {
    // reference-derived: the completion routine is a C function pointer. The routine asserts
    // the arguments, and it runs only if the reference's call is reproduced.
    let routine: ProfileRoutine = profile_routine;
    assert_eq!(show_profile_card_ui(0x5000, Some(routine), 0x6000), S_OK);
}

#[test]
fn show_profile_card_ui_passes_null_context() {
    // reference-derived: context is passed through as-is, including NULL.
    let mut seen: Option<(Hresult, Handle)> = None;
    let rc = show_profile_card_ui(0, Some(|rc, ctx| seen = Some((rc, ctx))), 0);
    assert_eq!(rc, S_OK);
    assert_eq!(seen, Some((S_OK, 0)));
}

#[test]
fn unimplemented_stub_flags_and_count_do_not_depend_on_names() {
    // reference-derived: code, flags and argument count are fixed in the C body.
    assert_eq!(
        unimplemented_stub("", ""),
        StubException {
            code: EXCEPTION_WINE_STUB,
            flags: EXCEPTION_NONCONTINUABLE,
            number_parameters: 2,
            module: "",
            function: "",
        }
    );
}

#[test]
fn stub_exports_are_the_23_stub_names() {
    // reference-derived: the spec lists 26 exports; 3 have bodies and 23 are `@ stub`.
    assert_eq!(STUB_EXPORTS.len(), 23);
    for implemented in [
        "ProcessPendingGameUI",
        "ShowPlayerPickerUI",
        "ShowProfileCardUI",
    ] {
        assert!(!STUB_EXPORTS.contains(&implemented));
    }
    assert!(STUB_EXPORTS.contains(&"ShowPlayerPickerUIForUser"));
    assert!(STUB_EXPORTS.contains(&"ShowProfileCardUIForUser"));
    for (i, name) in STUB_EXPORTS.iter().enumerate() {
        assert!(!STUB_EXPORTS[i + 1..].contains(name), "duplicate {name}");
    }
}
