//! Edge cases of the port that the Wine oracle rows do not reach.
//!
//! These cover the guards and bounds of `update_time_remaining`, `SetLine`, `SetCancelMsg`,
//! `SetProgress64`, `update_dialog` and `ACLMulti`. Each expectation follows from the verbatim C
//! in `progressdlg.c` and `aclmulti.c` at commit e2aa6cd75ae60acd90ff384feee171ce994f0f6d.

use super::*;
use crate::oracle_checks::{acl, enumerator, Acl, Enum};

fn estimate(remaining: u32, unit: usize, remainder: u32, long_form: bool) -> Option<TimeRemaining> {
    Some(TimeRemaining {
        remaining,
        unit,
        remainder,
        long_form,
    })
}

#[test]
fn time_remaining_needs_a_start_time_progress_and_a_total() {
    assert_eq!(time_remaining(0, 5000, 1, 2), None);
    assert_eq!(time_remaining(1000, 5000, 0, 2), None);
    assert_eq!(time_remaining(1000, 5000, 1, 0), None);
}

#[test]
fn time_remaining_zero_elapsed_is_zero_seconds() {
    assert_eq!(time_remaining(5000, 5000, 1, 2), estimate(0, 0, 0, false));
}

#[test]
fn time_remaining_under_a_minute_stays_in_seconds() {
    assert_eq!(time_remaining(1000, 46000, 1, 2), estimate(45, 0, 0, false));
}

#[test]
fn time_remaining_exactly_one_minute_has_no_seconds_left() {
    assert_eq!(time_remaining(1000, 61000, 1, 2), estimate(1, 1, 0, false));
}

#[test]
fn time_remaining_119_seconds_is_one_minute_and_59_seconds() {
    assert_eq!(time_remaining(1000, 120000, 1, 2), estimate(1, 1, 59, true));
}

#[test]
fn time_remaining_two_hours_is_capped_at_the_hours_unit() {
    assert_eq!(
        time_remaining(1000, 7_201_000, 1, 2),
        estimate(2, 2, 0, false)
    );
}

#[test]
fn time_remaining_past_the_hours_unit_keeps_counting_hours() {
    assert_eq!(
        time_remaining(1000, 216_001_000, 1, 2),
        estimate(60, 2, 0, false)
    );
}

#[test]
fn time_remaining_wraps_a_clock_that_went_backwards() {
    assert_eq!(time_remaining(10, 5, 1, 1), estimate(0, 0, 0, false));
}

#[test]
fn scale_progress_zero_is_zero() {
    assert_eq!(scale_progress(0, 0), ProgressBar { range: 0, pos: 0 });
}

#[test]
fn scale_progress_keeps_a_total_that_fits_in_32_bits() {
    assert_eq!(
        scale_progress(3, u64::from(u32::MAX)),
        ProgressBar {
            range: u32::MAX,
            pos: 3,
        }
    );
}

#[test]
fn line_slot_maps_one_to_three_and_folds_the_rest_to_zero() {
    assert_eq!(line_slot(0), 0);
    assert_eq!(line_slot(1), 0);
    assert_eq!(line_slot(2), 1);
    assert_eq!(line_slot(3), 2);
    assert_eq!(line_slot(4), 0);
    assert_eq!(line_slot(u32::MAX), 0);
}

#[test]
fn update_flags_and_hresults_match_the_wine_headers() {
    assert_eq!(UPDATE_PROGRESS, 0x1);
    assert_eq!(UPDATE_TITLE, 0x2);
    assert_eq!(UPDATE_LINE1, 0x4);
    assert_eq!(UPDATE_LINE2, 0x8);
    assert_eq!(UPDATE_LINE3, 0x20);
    assert_eq!(S_OK, 0);
    assert_eq!(S_FALSE, 1);
    assert_eq!(E_FAIL, 0x8000_4005);
    assert_eq!(E_NOTIMPL, 0x8000_4001);
    assert_eq!(E_OUTOFMEMORY, 0x8007_000E);
}

#[test]
fn set_progress_without_autotime_gives_no_estimate() {
    let mut dialog = Dialog {
        start_time: 1000,
        ..Dialog::default()
    };
    assert_eq!(dialog.set_progress(1, 2, 121000), None);
    assert_eq!(
        (dialog.completed, dialog.total, dialog.update),
        (1, 2, UPDATE_PROGRESS)
    );
}

#[test]
fn set_progress_with_autotime_but_no_start_time_gives_no_estimate() {
    let mut dialog = Dialog {
        autotime: true,
        ..Dialog::default()
    };
    assert_eq!(dialog.set_progress(1, 2, 121000), None);
    assert_eq!(dialog.update, UPDATE_PROGRESS);
}

#[test]
fn set_cancel_msg_before_cancel_stores_text_without_a_message() {
    let mut dialog = Dialog::default();
    assert!(!dialog.set_cancel_msg(Some("Cancelled")));
    assert_eq!(
        (dialog.cancel_msg.as_str(), dialog.update),
        ("Cancelled", 0x10)
    );
    assert!(!dialog.set_cancel_msg(None));
    assert_eq!(dialog.cancel_msg, "");
}

#[test]
fn paint_of_a_clean_dialog_is_empty() {
    assert_eq!(Dialog::default().paint(), Paint::default());
}

#[test]
fn paint_shows_the_title_only_when_flagged() {
    let mut dialog = Dialog {
        title: "Copying".to_string(),
        ..Dialog::default()
    };
    assert_eq!(dialog.paint().title, None);
    dialog.update = UPDATE_TITLE;
    assert_eq!(dialog.paint().title, Some("Copying"));
}

#[test]
fn empty_list_yields_nothing_and_expands_ok() {
    let mut list = AclMulti::new(Vec::<Member<Enum, Acl>>::new());
    assert_eq!(list.next_string(), Ok(None));
    assert_eq!(list.expand("x"), S_OK);
    list.reset();
    assert_eq!(list.current(), 0);
}

#[test]
fn next_skips_members_that_have_no_enumerator() {
    let mut list = AclMulti::new(vec![acl(S_OK), enumerator(4, 1, S_FALSE)]);
    assert_eq!(list.next_string(), Ok(Some(4)));
    assert_eq!(list.current(), 1);
}
