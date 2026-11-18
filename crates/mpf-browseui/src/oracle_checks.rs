//! Checks the port against the rows of the Wine `dlls/browseui` oracle.
//!
//! The rows come from `target/canon/browseui/oracle.txt`, which compiles the verbatim
//! `progressdlg.c` and `aclmulti.c` functions of commit e2aa6cd75ae60acd90ff384feee171ce994f0f6d
//! under ASan and UBSan, driven by `driver.c`. Each test is one output row. Row 8 is left out: its
//! estimate is under a minute, so the C code reads `timeMsg[-1]`. The default sanitizer flags do
//! not report that read, and `-fsanitize=bounds-strict` does.
//!
//! The oracle's `FormatMessageW` stub prints `F<long>(<count>,<unit>,<remainder>,<unit - 1>)`,
//! which [`render_remaining`] reproduces.

use super::*;

/// A fake `IEnumString`: yields `count` items with value `id`, then returns `tail`. Counts resets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Enum {
    id: u32,
    count: u32,
    pos: u32,
    resets: u32,
    tail: u32,
}

impl StringEnum for Enum {
    type Item = u32;

    fn fetch(&mut self) -> Fetch<u32> {
        if self.pos < self.count {
            self.pos += 1;
            return Fetch::Item(self.id);
        }
        match self.tail {
            S_FALSE => Fetch::Done,
            hr => Fetch::Failed(hr),
        }
    }

    fn reset(&mut self) {
        self.pos = 0;
        self.resets += 1;
    }
}

/// A fake `IACList`: `expand` returns `result` and counts calls.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Acl {
    result: u32,
    calls: u32,
}

impl AclExpand for Acl {
    fn expand(&mut self, _pattern: &str) -> u32 {
        self.calls += 1;
        self.result
    }
}

pub(crate) fn enumerator(id: u32, count: u32, tail: u32) -> Member<Enum, Acl> {
    Member {
        enumerator: Some(Enum {
            id,
            count,
            pos: 0,
            resets: 0,
            tail,
        }),
        acl: None,
    }
}

pub(crate) fn acl(result: u32) -> Member<Enum, Acl> {
    Member {
        enumerator: None,
        acl: Some(Acl { result, calls: 0 }),
    }
}

pub(crate) fn none() -> Member<Enum, Acl> {
    Member {
        enumerator: None,
        acl: None,
    }
}

pub(crate) fn resets(list: &AclMulti<Enum, Acl>, index: usize) -> u32 {
    list.members()[index]
        .enumerator
        .as_ref()
        .map_or(0, |e| e.resets)
}

pub(crate) fn acl_calls(list: &AclMulti<Enum, Acl>, index: usize) -> u32 {
    list.members()[index].acl.as_ref().map_or(0, |a| a.calls)
}

/// The oracle's `FormatMessageW` stub. `unit - 1` is `timeMsg[i - 1]`, which exists only when
/// `unit > 0`.
fn render_remaining(time: TimeRemaining) -> String {
    format!(
        "F{}({},{},{},{})",
        u8::from(time.long_form),
        time.remaining,
        time.unit,
        time.remainder,
        time.unit - 1
    )
}

/// `update_time_remaining` through the oracle's formatting: the update flags and line 3 afterwards.
fn time_row(start: u64, now: u64, completed: u64, total: u64) -> (u32, String) {
    let mut dialog = Dialog::default();
    let time = time_remaining(start, now, completed, total).expect("estimate");
    dialog.set_remaining_line(&render_remaining(time));
    (dialog.update, dialog.lines[2].clone())
}

#[test]
fn oracle_time_row_1_two_minutes_at_half() {
    assert_eq!(
        time_row(1000, 121000, 1, 2),
        (UPDATE_LINE3, "F0(2,1,0,0)".to_string())
    );
}

#[test]
fn oracle_time_row_2_one_minute_one_second_is_long_form() {
    assert_eq!(
        time_row(1000, 62000, 1, 2),
        (UPDATE_LINE3, "F1(1,1,1,0)".to_string())
    );
}

#[test]
fn oracle_time_row_3_one_hour_exactly() {
    assert_eq!(
        time_row(1000, 3601000, 1, 2),
        (UPDATE_LINE3, "F0(1,2,0,1)".to_string())
    );
}

#[test]
fn oracle_time_row_4_one_hour_one_minute_one_second_is_long_form() {
    assert_eq!(
        time_row(1000, 3662000, 1, 2),
        (UPDATE_LINE3, "F1(1,2,1,1)".to_string())
    );
}

#[test]
fn oracle_time_row_5_three_of_ten_after_210_seconds() {
    assert_eq!(
        time_row(1000, 211000, 3, 10),
        (UPDATE_LINE3, "F0(8,1,10,0)".to_string())
    );
}

#[test]
fn oracle_time_row_6_huge_total_wraps_then_truncates_to_32_bits() {
    assert_eq!(
        time_row(1, 1 + (1u64 << 40), 1, 1u64 << 30),
        (UPDATE_LINE3, "F0(47721,2,51,1)".to_string())
    );
}

#[test]
fn oracle_time_row_7_rate_above_one_wraps_negative_elapsed() {
    assert_eq!(
        time_row(1, 120001, 2, 1),
        (UPDATE_LINE3, "F0(353141,2,44,1)".to_string())
    );
}

#[test]
fn oracle_progress_row_20_small_total_is_not_scaled() {
    assert_eq!(scale_progress(5, 10), ProgressBar { range: 10, pos: 5 });
}

#[test]
fn oracle_progress_row_21_one_halving_past_32_bits() {
    assert_eq!(
        scale_progress(0x1_0000_0004, 0x1_0000_0008),
        ProgressBar {
            range: 2_147_483_652,
            pos: 2_147_483_650,
        }
    );
}

#[test]
fn oracle_progress_row_22_nine_halvings_to_fit() {
    assert_eq!(
        scale_progress((1u64 << 40) + 1536, 1u64 << 40),
        ProgressBar {
            range: 1 << 31,
            pos: (1 << 31) + 3,
        }
    );
}

#[test]
fn oracle_progress_row_23_total_max_halves_to_32_bits() {
    assert_eq!(
        scale_progress(7, u64::MAX),
        ProgressBar {
            range: u32::MAX,
            pos: 0,
        }
    );
}

#[test]
fn oracle_line_row_30_three_sets_flag_line_three_as_0x10() {
    let mut dialog = Dialog::default();
    assert!(dialog.set_line(1, Some("Alpha")));
    assert!(dialog.set_line(2, Some("Beta")));
    assert!(dialog.set_line(3, Some("Gamma")));
    // SetLine(3) sets UPDATE_LINE1 << 2 (0x10), not UPDATE_LINE3 (0x20). See the crate docs.
    assert_eq!(dialog.update, 0x1c);
    assert_eq!(dialog.paint().lines, [Some("Alpha"), Some("Beta"), None]);
}

#[test]
fn oracle_line_row_31_numbers_outside_one_to_three_use_line_one() {
    let mut dialog = Dialog::default();
    dialog.set_line(0, Some("Zero"));
    dialog.set_line(4, Some("Four"));
    assert_eq!(dialog.update, 0x4);
    assert_eq!(dialog.paint().lines, [Some("Four"), None, None]);
}

#[test]
fn oracle_line_row_32_cancelled_dialog_paints_empty_line_without_message() {
    let mut dialog = Dialog {
        is_cancelled: true,
        ..Dialog::default()
    };
    assert!(!dialog.set_line(1, Some("Alpha")));
    assert_eq!(dialog.update, 0x4);
    assert_eq!(dialog.paint().lines, [Some(""), None, None]);
}

#[test]
fn oracle_cancel_row_33_cancel_message_paints_on_line_three() {
    let mut dialog = Dialog {
        is_cancelled: true,
        ..Dialog::default()
    };
    assert!(dialog.set_cancel_msg(Some("Cancelled")));
    assert_eq!(dialog.update, 0x10);
    assert_eq!(dialog.paint().lines, [None, None, None]);
    dialog.update |= UPDATE_LINE3;
    assert_eq!(dialog.paint().lines, [None, None, Some("Cancelled")]);
}

#[test]
fn oracle_line_row_34_null_text_is_empty() {
    let mut dialog = Dialog::default();
    assert!(dialog.set_line(2, None));
    assert_eq!(dialog.update, 0x8);
    assert_eq!(dialog.paint().lines, [None, Some(""), None]);
}

#[test]
fn oracle_progress_row_35_without_autotime_paints_the_bar() {
    let mut dialog = Dialog::default();
    assert_eq!(dialog.set_progress(5, 10, 0), None);
    assert_eq!(dialog.update, 0x1);
    assert_eq!(
        dialog.paint(),
        Paint {
            progress: Some(ProgressBar { range: 10, pos: 5 }),
            ..Paint::default()
        }
    );
}

#[test]
fn oracle_progress_row_36_autotime_sets_line_three() {
    let mut dialog = Dialog {
        autotime: true,
        start_time: 1000,
        ..Dialog::default()
    };
    let time = dialog.set_progress(1, 2, 121000).expect("estimate");
    dialog.set_remaining_line(&render_remaining(time));
    assert_eq!(dialog.update, 0x21);
    assert_eq!(
        dialog.paint(),
        Paint {
            lines: [None, None, Some("F0(2,1,0,0)")],
            progress: Some(ProgressBar { range: 2, pos: 1 }),
            ..Paint::default()
        }
    );
}

#[test]
fn oracle_acl_row_40_walks_members_in_order_then_stops() {
    let mut list = AclMulti::new(vec![
        enumerator(0, 2, S_FALSE),
        none(),
        enumerator(2, 1, S_FALSE),
    ]);
    assert_eq!(list.next_string(), Ok(Some(0)));
    assert_eq!(list.next_string(), Ok(Some(0)));
    assert_eq!(list.next_string(), Ok(Some(2)));
    assert_eq!(list.current(), 2);
    assert_eq!(list.next_string(), Ok(None));
    assert_eq!(list.current(), 3);
}

#[test]
fn oracle_acl_row_41_failure_stays_current_and_reset_restarts() {
    let mut list = AclMulti::new(vec![enumerator(0, 1, E_FAIL)]);
    assert_eq!(list.next_string(), Ok(Some(0)));
    assert_eq!(list.next_string(), Err(E_FAIL));
    assert_eq!(list.current(), 0);
    list.reset();
    assert_eq!(list.next_string(), Ok(Some(0)));
}

#[test]
fn oracle_acl_row_42_reset_resets_enumerators_and_skips_acl_only() {
    let mut list = AclMulti::new(vec![
        enumerator(0, 1, S_FALSE),
        acl(0),
        enumerator(2, 1, S_FALSE),
    ]);
    assert_eq!(list.next_string(), Ok(Some(0)));
    assert_eq!(list.next_string(), Ok(Some(2)));
    list.reset();
    assert_eq!(
        (
            resets(&list, 0),
            resets(&list, 2),
            list.current(),
            acl_calls(&list, 1)
        ),
        (1, 1, 0, 0)
    );
}

#[test]
fn oracle_acl_row_43_expand_returns_the_last_result() {
    let mut list = AclMulti::new(vec![acl(S_OK), none(), acl(E_NOTIMPL)]);
    assert_eq!(list.expand("x"), E_NOTIMPL);
    assert_eq!((acl_calls(&list, 0), acl_calls(&list, 2)), (1, 1));
}

#[test]
fn oracle_acl_row_44_expand_keeps_going_after_a_failure() {
    let mut list = AclMulti::new(vec![
        acl(E_OUTOFMEMORY),
        acl(S_FALSE),
        enumerator(2, 0, S_FALSE),
    ]);
    assert_eq!(list.expand("x"), S_FALSE);
    assert_eq!((acl_calls(&list, 0), acl_calls(&list, 1)), (1, 1));
}

#[test]
fn oracle_acl_row_45_expand_without_sources_is_ok() {
    let mut list = AclMulti::new(vec![none(), none()]);
    assert_eq!(list.expand("x"), S_OK);
    assert_eq!(list.next_string(), Ok(None));
    assert_eq!(list.current(), 2);
}
