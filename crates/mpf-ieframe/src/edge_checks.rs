//! Edge cases of the port that the Wine oracle rows do not reach.
//!
//! The constants follow `include/objbase.h`, which Wine's `intshcut.c` includes. The loops
//! cover every access nibble, with and without high bits and the share bits, so each path of
//! `open_mode` runs for every nibble value.

use super::*;

#[test]
fn stgm_constants_match_the_wine_headers() {
    assert_eq!(STGM_WRITE, 0x1);
    assert_eq!(STGM_READWRITE, 0x2);
    assert_eq!(STGM_SHARE_EXCLUSIVE, 0x10);
    assert_eq!(STGM_ACCESS_MASK, 0xf);
}

#[test]
fn every_access_nibble_is_rewritten_only_when_it_is_write() {
    for access in 0..=0xfu32 {
        for high in [0x0, 0x2000_0000, 0xf000_0000] {
            let grf_mode = high | access;
            let expected_access = if access == STGM_WRITE {
                STGM_READWRITE
            } else {
                access
            };
            assert_eq!(
                open_mode(grf_mode),
                high | expected_access | STGM_SHARE_EXCLUSIVE,
                "grfMode {grf_mode:#010x}"
            );
        }
    }
}

#[test]
fn the_share_flag_is_always_set() {
    for grf_mode in [0, 0x1, 0x10, 0x11, 0xffff_ffff, 0x8000_0000] {
        assert_ne!(open_mode(grf_mode) & STGM_SHARE_EXCLUSIVE, 0);
    }
}

#[test]
fn the_rewritten_access_is_never_write_only() {
    for access in 0..=0xfu32 {
        assert_ne!(open_mode(access) & STGM_ACCESS_MASK, STGM_WRITE);
    }
}

#[test]
fn open_mode_is_idempotent() {
    for grf_mode in [0, 0x1, 0x2, 0x11, 0x13, 0x8000_0001, 0xffff_fff1] {
        let once = open_mode(grf_mode);
        assert_eq!(open_mode(once), once, "grfMode {grf_mode:#010x}");
    }
}

#[test]
fn bits_outside_the_access_mask_are_kept_for_a_write_open() {
    assert_eq!(open_mode(0xf000_0001), 0xf000_0012);
    assert_eq!(open_mode(0x0000_0101), 0x0000_0112);
}
