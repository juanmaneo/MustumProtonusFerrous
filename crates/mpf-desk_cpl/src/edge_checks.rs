//! Edge-case tests for the ported desk.cpl logic.
//!
//! These cover boundary rectangles, every branch of the layout code, and the
//! flag combinations. Tests that are not oracle-derived say so in a comment.

use super::*;

fn rc(left: i32, top: i32, right: i32, bottom: i32) -> Rect {
    Rect {
        left,
        top,
        right,
        bottom,
    }
}

fn mode_at(x: i32, y: i32, w: u32, h: u32) -> DevMode {
    DevMode {
        fields: DM_POSITION | DM_PELSWIDTH | DM_PELSHEIGHT,
        position: Point { x, y },
        pels_width: w,
        pels_height: h,
        ..DevMode::default()
    }
}

fn entry_at(x: i32, y: i32, w: u32, h: u32) -> Entry {
    let current = mode_at(x, y, w, h);
    Entry {
        current,
        pending: current,
    }
}

#[test]
fn rect_from_devmode_without_position_flag_ignores_position() {
    // Not oracle-derived: the C only sets the corners under DM_POSITION.
    let mode = DevMode {
        fields: DM_PELSWIDTH,
        position: Point { x: 50, y: 60 },
        pels_width: 10,
        pels_height: 20,
        ..DevMode::default()
    };
    assert_eq!(rect_from_devmode(&mode), rc(0, 0, 10, 0));
    assert_eq!(rect_from_devmode(&DevMode::default()), rc(0, 0, 0, 0));
}

#[test]
fn rect_from_devmode_sums_wrap_as_32_bit() {
    // Not oracle-derived: the C adds a DWORD to a LONG, so the sum wraps in 32 bits.
    let mode = DevMode {
        fields: DM_PELSWIDTH | DM_PELSHEIGHT,
        pels_width: u32::MAX,
        pels_height: 0x8000_0000,
        ..DevMode::default()
    };
    assert_eq!(rect_from_devmode(&mode), rc(0, 0, -1, i32::MIN));
}

#[test]
fn is_same_devmode_checks_each_compared_field() {
    // Not oracle-derived: each compared field differs on its own, and fields and position never count.
    let base = DevMode {
        fields: DM_POSITION,
        position: Point { x: 1, y: 2 },
        display_orientation: 0,
        bits_per_pel: 32,
        pels_width: 1920,
        pels_height: 1080,
        display_frequency: 60,
    };
    assert!(is_same_devmode(&base, &base));
    let others = [
        DevMode {
            display_orientation: 1,
            ..base
        },
        DevMode {
            bits_per_pel: 16,
            ..base
        },
        DevMode {
            pels_width: 1280,
            ..base
        },
        DevMode {
            pels_height: 1024,
            ..base
        },
        DevMode {
            display_frequency: 75,
            ..base
        },
    ];
    for other in others {
        assert!(!is_same_devmode(&base, &other));
    }
    let moved = DevMode {
        fields: 0,
        position: Point { x: 99, y: 99 },
        ..base
    };
    assert!(is_same_devmode(&base, &moved));
}

#[test]
fn is_empty_covers_zero_and_inverted_extents() {
    // Not oracle-derived: IsRectEmpty treats zero or negative width or height as empty.
    assert!(rc(5, 5, 5, 10).is_empty());
    assert!(rc(0, 10, 10, 5).is_empty());
    assert!(rc(10, 0, 0, 10).is_empty());
    assert!(!rc(0, 0, 1, 1).is_empty());
}

#[test]
fn intersect_rect_returns_overlap_or_none() {
    // Not oracle-derived: IntersectRect's FALSE cases and overlap result.
    assert_eq!(
        intersect_rect(rc(0, 0, 100, 100), rc(10, 20, 30, 40)),
        Some(rc(10, 20, 30, 40))
    );
    assert_eq!(
        intersect_rect(rc(0, 0, 50, 50), rc(25, 25, 75, 75)),
        Some(rc(25, 25, 50, 50))
    );
    // Touching edges do not overlap.
    assert_eq!(intersect_rect(rc(0, 0, 10, 10), rc(10, 0, 20, 10)), None);
    assert_eq!(intersect_rect(rc(10, 0, 20, 10), rc(0, 0, 10, 10)), None);
    assert_eq!(intersect_rect(rc(0, 0, 10, 10), rc(0, 10, 10, 20)), None);
    assert_eq!(intersect_rect(rc(0, 10, 10, 20), rc(0, 0, 10, 10)), None);
    // Disjoint on each side.
    assert_eq!(intersect_rect(rc(0, 0, 10, 10), rc(50, 0, 60, 10)), None);
    assert_eq!(intersect_rect(rc(0, 0, 10, 10), rc(0, 50, 10, 60)), None);
    // An empty input never intersects.
    assert_eq!(intersect_rect(rc(5, 5, 5, 10), rc(0, 0, 10, 10)), None);
    assert_eq!(intersect_rect(rc(0, 0, 10, 10), rc(0, 10, 10, 5)), None);
}

#[test]
fn map_virtual_client_rect_scale_zero_collapses_to_client_centre() {
    // Not oracle-derived: scale 0 leaves only the client centre offset.
    let mapped = map_virtual_client_rect(
        rc(0, 0, 100, 100),
        rc(0, 0, 200, 200),
        rc(0, 0, 100, 100),
        0.0,
    );
    assert_eq!(mapped, rc(100, 100, 100, 100));
}

#[test]
fn map_virtual_client_rect_truncates_toward_zero() {
    // Not oracle-derived: float to LONG truncates toward zero, so -0.5 becomes 0.
    let mapped = map_virtual_client_rect(rc(-1, -1, 1, 1), rc(0, 0, 0, 0), rc(0, 0, 0, 0), 0.5);
    assert_eq!(mapped, rc(0, 0, 0, 0));
    // Not oracle-derived: -1.5 becomes -1 and 1.5 becomes 1.
    let mapped = map_virtual_client_rect(rc(-3, -3, 3, 3), rc(0, 0, 0, 0), rc(0, 0, 0, 0), 0.5);
    assert_eq!(mapped, rc(-1, -1, 1, 1));
}

#[test]
fn move_rect_without_other_entries_sets_requested_mode() {
    // Not oracle-derived: with no other entries, the requested position is kept.
    let mut entries = vec![entry_at(0, 0, 100, 100)];
    move_rect(&mut entries, 0, mode_at(5, 6, 100, 100));
    assert_eq!(entries[0].pending.position, Point { x: 5, y: 6 });
    assert_eq!(entries[0].current.position, Point { x: 0, y: 0 });
}

#[test]
fn move_rect_pushes_above_a_neighbour_when_moving_down_into_it() {
    // Not oracle-derived: the old rect's bottom edge is above the neighbour, so y becomes top - height.
    let mut entries = vec![entry_at(0, 500, 100, 100), entry_at(0, 0, 100, 100)];
    move_rect(&mut entries, 1, mode_at(0, 450, 100, 100));
    assert_eq!(entries[1].pending.position, Point { x: 0, y: 400 });
}

#[test]
fn snap_rect_snaps_to_right_neighbour() {
    // Not oracle-derived: the neighbour starts at x = 200, so the device snaps to x = 100.
    let mut entries = vec![entry_at(200, 0, 100, 100), entry_at(0, 0, 100, 100)];
    snap_rect(&mut entries, 1);
    assert_eq!(entries[1].current.position, Point { x: 100, y: 0 });
    assert_eq!(entries[1].pending, entries[1].current);
}

#[test]
fn snap_rect_snaps_vertically_to_neighbour_below() {
    // Not oracle-derived: the neighbour starts at y = 200, so the device snaps to y = 100.
    let mut entries = vec![entry_at(0, 200, 100, 100), entry_at(0, 0, 100, 100)];
    snap_rect(&mut entries, 1);
    assert_eq!(entries[1].current.position, Point { x: 0, y: 100 });
}

#[test]
fn snap_rect_snaps_vertically_to_neighbour_above() {
    // Not oracle-derived: the device starts below the neighbour, so it snaps up to y = 100.
    let mut entries = vec![entry_at(0, 0, 100, 100), entry_at(0, 300, 100, 100)];
    snap_rect(&mut entries, 1);
    assert_eq!(entries[1].current.position, Point { x: 0, y: 100 });
}

#[test]
fn option_char_test_follows_is_option_true_macro() {
    // Not oracle-derived: the macro compares the whole WCHAR with ASCII characters.
    for ch in *b"yYtT1" {
        assert!(is_option_true(u16::from(ch)));
    }
    for ch in *b"nN0 " {
        assert!(!is_option_true(u16::from(ch)));
    }
    assert!(!is_option_true(0));
    assert!(!is_option_true(0x179));
    assert!(!is_option_true(0x131));
}

#[test]
fn option_value_uses_default_only_without_a_value() {
    // Not oracle-derived: a present value overrides the default, and an empty string reads as false.
    assert!(option_value(None, true));
    assert!(!option_value(None, false));
    assert!(!option_value(Some(&[]), true));
    assert!(option_value(Some(&[u16::from(b'Y')]), false));
    assert!(!option_value(Some(&[u16::from(b'0')]), true));
}
