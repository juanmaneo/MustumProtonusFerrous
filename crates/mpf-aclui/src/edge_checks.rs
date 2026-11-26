//! Edge-case tests for the ported aclui logic.
//!
//! These cover boundary values and the branches of the three ported functions. Expectations
//! that the oracle run does not produce are marked "Not oracle-derived" in a comment.

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
fn sid_from_ace_only_allowed_and_denied_carry_a_sid() {
    // Not oracle-derived: the oracle run covers types 0x0, 0x1, 0x2 and 0x7f. These types
    // reach the `default:` branch of get_sid_from_ace.
    for ace_type in [0x2, 0x3, 0x5, 0x6, 0x7f, 0x80, 0xff] {
        assert_eq!(
            sid_from_ace(&ace(ace_type, 7, 0)),
            None,
            "ace_type {ace_type:#x}"
        );
    }
}

#[test]
fn sid_from_ace_keeps_sid_identity_at_boundaries() {
    // Not oracle-derived: boundary SID values (0, 1 and u32::MAX).
    for sid in [0, 1, u32::MAX] {
        assert_eq!(
            sid_from_ace(&ace(ACCESS_ALLOWED_ACE_TYPE, sid, 0)),
            Some(&sid)
        );
        assert_eq!(
            sid_from_ace(&ace(ACCESS_DENIED_ACE_TYPE, sid, u32::MAX)),
            Some(&sid)
        );
    }
}

#[test]
fn compute_without_dacl_is_zero_for_any_sid() {
    // Not oracle-derived: the oracle run queries only SID 1 with a missing DACL.
    for sid in [0, u32::MAX] {
        assert_eq!(compute_access_masks(None, &sid), AccessMasks::default());
        assert_eq!(
            compute_access_masks(Some(&[]), &sid),
            AccessMasks::default()
        );
    }
}

#[test]
fn compute_ors_full_width_masks() {
    // Not oracle-derived: 32-bit boundary masks. Bit 31 and the low 31 bits each come from
    // their own ACE, so the OR must keep every bit.
    let dacl = [
        ace(ACCESS_ALLOWED_ACE_TYPE, 5, 0x8000_0000),
        ace(ACCESS_ALLOWED_ACE_TYPE, 5, 0x7fff_ffff),
        ace(ACCESS_DENIED_ACE_TYPE, 5, 0x8000_0000),
        ace(ACCESS_DENIED_ACE_TYPE, 5, 0x7fff_ffff),
    ];
    assert_eq!(
        compute_access_masks(Some(&dacl), &5),
        AccessMasks {
            allowed: u32::MAX,
            denied: u32::MAX,
        }
    );
}

#[test]
fn access_rows_general_bit_is_the_only_filter() {
    // Not oracle-derived: flag boundaries. SI_ACCESS_GENERAL - 1 is the bit just below the
    // general bit, so it is excluded. The general bit alone, with other bits set, and
    // u32::MAX are included.
    let access = [
        acc(1, 0),
        acc(1, SI_ACCESS_GENERAL - 1),
        acc(1, SI_ACCESS_GENERAL),
        acc(1, SI_ACCESS_GENERAL | 0x1_0000),
        acc(1, u32::MAX),
    ];
    let masks = AccessMasks {
        allowed: 1,
        denied: 0,
    };
    assert_eq!(
        access_rows(&access, masks),
        vec![
            AccessRow {
                allowed: true,
                denied: false,
            };
            3
        ]
    );
}

#[test]
fn access_rows_empty_when_no_entry_is_general() {
    // Not oracle-derived: every entry is filtered out, so the listview gets no rows.
    let access = [acc(0x1, 0x1_0000), acc(0x2, 0x4_0000)];
    let masks = AccessMasks {
        allowed: u32::MAX,
        denied: u32::MAX,
    };
    assert!(access_rows(&access, masks).is_empty());
}

#[test]
fn access_rows_need_every_bit_of_the_mask() {
    // Not oracle-derived: one missing bit at the top of the mask shows "-". The oracle
    // run's masks all fit in 8 bits.
    let access = [acc(0x8000_0001, SI_ACCESS_GENERAL)];
    let masks = AccessMasks {
        allowed: 0x8000_0000,
        denied: u32::MAX,
    };
    assert_eq!(
        access_rows(&access, masks),
        vec![AccessRow {
            allowed: false,
            denied: true,
        }]
    );
}
