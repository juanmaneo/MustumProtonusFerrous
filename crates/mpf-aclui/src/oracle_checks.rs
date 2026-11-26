//! Oracle-derived vectors for the ported aclui logic.
//!
//! Rows were printed by `target/canon/aclui/harness.c`. It compiles the verbatim Wine bodies of
//! `get_sid_from_ace`, `compute_access_masks` and `update_access_list` from
//! `dlls/aclui/aclui_main.c` with `gcc -fsanitize=address,undefined -g` and runs them on
//! fake DACLs. The run printed 21 rows: 4 `get_sid_from_ace` rows and 17 mask rows. ASan and
//! UBSan reported nothing, so no vector was excluded.

use super::*;

fn ace(ace_type: u8, sid: u32, mask: u32) -> Ace<u32> {
    Ace {
        ace_type,
        sid,
        mask,
    }
}

fn acc(mask: u32, flags: u32) -> AccessEntry {
    AccessEntry { mask, flags }
}

#[test]
fn oracle_sid_from_ace_rows() {
    // (ace_type, get_sid_from_ace(SidStart = 0x1234))
    let rows: &[(u8, Option<u32>)] = &[
        (0, Some(0x1234)),
        (0x1, Some(0x1234)),
        (0x2, None),
        (0x7f, None),
    ];
    for &(ace_type, want) in rows {
        assert_eq!(
            sid_from_ace(&ace(ace_type, 0x1234, 0)).copied(),
            want,
            "ace_type {ace_type:#x}"
        );
    }
}

#[test]
fn oracle_access_mask_rows() {
    // (dacl, sid, access, (allowed, denied), marks)
    type Row<'a> = (
        Option<&'a [Ace<u32>]>,
        u32,
        &'a [AccessEntry],
        (u32, u32),
        &'a [(bool, bool)],
    );
    let rows: &[Row<'_>] = &[
        // no_dacl_present_clear
        (
            None,
            1,
            &[acc(0x3, 0x20000), acc(0, 0x20000)],
            (0, 0),
            &[(false, false), (true, true)],
        ),
        // dacl_query_fails
        (None, 1, &[acc(0x1, 0x20000)], (0, 0), &[(false, false)]),
        // empty_dacl
        (
            Some(&[]),
            1,
            &[acc(0x1, 0x20000)],
            (0, 0),
            &[(false, false)],
        ),
        // single_allow_match
        (
            Some(&[ace(0, 1, 0x1)]),
            1,
            &[acc(0x1, 0x20000), acc(0x2, 0x20000)],
            (0x1, 0),
            &[(true, false), (false, false)],
        ),
        // single_deny_match
        (
            Some(&[ace(0x1, 1, 0x2)]),
            1,
            &[acc(0x1, 0x20000), acc(0x2, 0x20000)],
            (0, 0x2),
            &[(false, false), (false, true)],
        ),
        // allow_deny_overlap
        (
            Some(&[ace(0, 1, 0x7), ace(0x1, 1, 0x3)]),
            1,
            &[acc(0x3, 0x20000), acc(0x4, 0x20000), acc(0x7, 0x20000)],
            (0x7, 0x3),
            &[(true, true), (true, false), (true, false)],
        ),
        // allow_or_accumulates
        (
            Some(&[ace(0, 1, 0x10), ace(0, 1, 0x20), ace(0, 1, 0x10)]),
            1,
            &[acc(0x30, 0x20000), acc(0x10, 0x20000), acc(0x40, 0x20000)],
            (0x30, 0),
            &[(true, false), (true, false), (false, false)],
        ),
        // other_sids_ignored
        (
            Some(&[ace(0, 2, 0xff), ace(0x1, 3, 0xff)]),
            1,
            &[acc(0x1, 0x20000)],
            (0, 0),
            &[(false, false)],
        ),
        // unknown_type_ignored
        (
            Some(&[ace(0x5, 1, 0xff), ace(0, 1, 0x4)]),
            1,
            &[acc(0x4, 0x20000), acc(0x8, 0x20000)],
            (0x4, 0),
            &[(true, false), (false, false)],
        ),
        // deny_before_allow
        (
            Some(&[ace(0x1, 1, 0x1), ace(0, 1, 0x2)]),
            1,
            &[acc(0x1, 0x20000), acc(0x2, 0x20000), acc(0x3, 0x20000)],
            (0x2, 0x1),
            &[(false, true), (true, false), (false, false)],
        ),
        // zero_mask_aces
        (
            Some(&[ace(0, 1, 0), ace(0x1, 1, 0)]),
            1,
            &[acc(0, 0x20000)],
            (0, 0),
            &[(true, true)],
        ),
        // non_general_rows_skipped
        (
            Some(&[ace(0, 1, 0x5)]),
            1,
            &[acc(0x1, 0x10000), acc(0x4, 0x20000), acc(0x1, 0x40000)],
            (0x5, 0),
            &[(true, false)],
        ),
        // flags_with_general_bit
        (
            Some(&[ace(0, 1, 0x5)]),
            1,
            &[acc(0x5, 0x30000), acc(0x7, 0x80000), acc(0x1, 0)],
            (0x5, 0),
            &[(true, false)],
        ),
        // partial_bits_show_dash
        (
            Some(&[ace(0, 1, 0x5), ace(0x1, 1, 0x2)]),
            1,
            &[acc(0x7, 0x20000)],
            (0x5, 0x2),
            &[(false, false)],
        ),
        // empty_access_table
        (Some(&[ace(0, 1, 0x1)]), 1, &[], (0x1, 0), &[]),
        // sid_zero_target
        (
            Some(&[ace(0, 0, 0x8), ace(0x1, 0, 0x1)]),
            0,
            &[acc(0x8, 0x20000), acc(0x1, 0x20000)],
            (0x8, 0x1),
            &[(true, false), (false, true)],
        ),
        // mixed_sids
        (
            Some(&[
                ace(0x1, 9, 0xf),
                ace(0, 1, 0x2),
                ace(0x1, 1, 0x4),
                ace(0, 9, 0x1),
            ]),
            1,
            &[acc(0x6, 0x20000), acc(0x4, 0x20000)],
            (0x2, 0x4),
            &[(false, false), (false, true)],
        ),
    ];
    for (i, (dacl, sid, access, want_masks, want_marks)) in rows.iter().enumerate() {
        let masks = compute_access_masks(*dacl, sid);
        assert_eq!((masks.allowed, masks.denied), *want_masks, "row {i}");
        let marks: Vec<(bool, bool)> = access_rows(access, masks)
            .iter()
            .map(|row| (row.allowed, row.denied))
            .collect();
        assert_eq!(marks, *want_marks, "row {i}");
    }
}
