//! Edge-case tests for the ported fltmgr.sys logic.
//!
//! Boundary and branch tests. Expectations not derived from the oracle say so in
//! a comment; they come from the reference source's control flow or arithmetic.

use super::*;
use crate::security::{SECURITY_DESCRIPTOR_MIN_LENGTH, SECURITY_NULL_SID_AUTHORITY, SID_REVISION};

#[test]
fn sid_length_formula() {
    // Not oracle-derived: RtlLengthRequiredSid is (n - 1) * sizeof(DWORD) + sizeof(SID).
    assert_eq!(length_required_sid(1), 12);
    assert_eq!(length_required_sid(2), 16);
    assert_eq!(length_required_sid(3), 20);
}

#[test]
fn sid_length_wraps_like_dword() {
    // Not oracle-derived: (0 - 1) * 4 wraps to 0xFFFF_FFFC, and + 12 wraps to 8.
    assert_eq!(length_required_sid(0), 8);
}

#[test]
fn ace_size_is_header_mask_and_sid() {
    // Not oracle-derived: AceSize = 8 + RtlLengthSid for a three-sub-authority SID.
    let ace = AccessAllowedAce {
        mask: 0,
        sid: Sid {
            revision: SID_REVISION,
            authority: SECURITY_NULL_SID_AUTHORITY,
            sub_authority: vec![1, 2, 3],
        },
    };
    assert_eq!(ace.size(), 28);
}

#[test]
fn allocation_stops_at_first_refusal() {
    // Not oracle-derived: the builder makes no later allocation after a refusal.
    let mut calls = 0;
    let result = build_default_security_descriptor_with(0x1, |_| {
        calls += 1;
        calls < 2
    });
    assert_eq!(result, Err(STATUS_INSUFFICIENT_RESOURCES));
    assert_eq!(calls, 2);
}

#[test]
fn acl_size_covers_the_two_aces() {
    // Not oracle-derived: the ACL is allocated 20 bytes larger than the 40-byte
    // descriptor header, so AclSize is 60 while the ACEs use 8 + 24 + 20 = 52
    // bytes. The 8 surplus bytes are Wine's per-ACE SidStart over-count.
    let sd = build_default_security_descriptor(0x1).expect("success");
    let dacl = sd.dacl.expect("dacl");
    let used = 8 + dacl.aces.iter().map(AccessAllowedAce::size).sum::<u32>();
    assert_eq!(used, 52);
    assert!(u32::from(dacl.size) >= used);
    assert_eq!(
        sd.alloc_len,
        SECURITY_DESCRIPTOR_MIN_LENGTH + u32::from(dacl.size)
    );
}

#[test]
fn filter_handle_is_deadbeaf_not_deadbeef() {
    // Not oracle-derived: restates the reference constant so a typo fails loudly.
    assert_eq!(FLT_FILTER_HANDLE, 0xdead_beaf);
    assert_ne!(FLT_FILTER_HANDLE, 0xdead_beef);
}

#[test]
fn enumerate_volumes_overwrites_nonzero_count() {
    // Not oracle-derived: boundary on the non-null write path.
    let mut returned = u32::MAX;
    assert_eq!(
        flt_enumerate_volumes(Some(&mut returned)),
        STATUS_NOT_SUPPORTED
    );
    assert_eq!(returned, 0);
}

#[test]
fn void_exports_have_no_return_value() {
    // Not oracle-derived: FIXME-only bodies, so the call is the whole behaviour.
    flt_initialize_push_lock();
    flt_delete_push_lock();
    flt_acquire_push_lock_exclusive();
    flt_release_push_lock();
    flt_unregister_filter();
    flt_release_context();
    flt_release_file_name_information();
    flt_close_communication_port();
}
