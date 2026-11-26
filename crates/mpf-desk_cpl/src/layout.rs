//! Port of `device_entry_move_rect` and `device_entry_snap_rect` from `dlls/desk.cpl/main.c`.
//!
//! The C code walks the global `devices` list. Here `entries` is that list in order, and
//! `device` is the index of the entry being placed.

use crate::devmode::{rect_from_devmode, DevMode};
use crate::{intersect_rect, Point, Rect};

/// `LONG_MAX`: the "not set" marker for snap offsets and differences.
const LONG_MAX: i32 = i32::MAX;

/// The `current` and `pending` modes of one `struct device_entry`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Entry {
    pub current: DevMode,
    pub pending: DevMode,
}

/// `device_entry_move_rect`: sets the pending mode of `entries[device]` to `mode`, moved
/// clear of the other entries' current rectangles. If the adjusted position still overlaps
/// one of them, the original pending mode is kept.
pub fn move_rect(entries: &mut [Entry], device: usize, mode: DevMode) {
    let pending = entries[device].pending;
    let mut mode = mode;
    let old_rect = rect_from_devmode(&pending);
    let mut new_rect = rect_from_devmode(&mode);

    // adjust the position to avoid any overlapping
    for (index, entry) in entries.iter().enumerate() {
        let other = rect_from_devmode(&entry.current);
        if index == device {
            continue;
        }
        if intersect_rect(other, new_rect).is_none() {
            continue;
        }
        if old_rect.left >= other.right {
            mode.position.x = other.right;
        }
        if old_rect.right <= other.left {
            mode.position.x = other.left.wrapping_sub(mode.pels_width as i32);
        }
        if old_rect.top >= other.bottom {
            mode.position.y = other.bottom;
        }
        if old_rect.bottom <= other.top {
            mode.position.y = other.top.wrapping_sub(mode.pels_height as i32);
        }
        new_rect = rect_from_devmode(&mode);
    }

    // if our adjustments caused more intersection, keep the original position
    for (index, entry) in entries.iter().enumerate() {
        let other = rect_from_devmode(&entry.current);
        if index == device {
            continue;
        }
        if intersect_rect(other, new_rect).is_none() {
            continue;
        }
        mode = pending;
    }

    entries[device].pending = mode;
}

/// `device_entry_snap_rect`: snaps the pending mode of `entries[device]` to the nearest
/// neighbour edge, then makes that mode both the current and the pending mode.
pub fn snap_rect(entries: &mut [Entry], device: usize) {
    let mut offset = Point {
        x: LONG_MAX,
        y: LONG_MAX,
    };
    let mut nearest = Point {
        x: LONG_MAX,
        y: LONG_MAX,
    };
    let pending = entries[device].pending;
    let mut mode = pending;
    let new_rect = rect_from_devmode(&mode);

    // snap the position to the nearest rectangles
    for (index, entry) in entries.iter().enumerate() {
        let other = rect_from_devmode(&entry.current);
        let mut diff = Point {
            x: LONG_MAX,
            y: LONG_MAX,
        };
        if index == device {
            continue;
        }

        if new_rect.left >= other.right {
            diff.x = other.right - new_rect.left;
        }
        if new_rect.right <= other.left {
            diff.x = other.left - new_rect.right;
        }
        if new_rect.top >= other.bottom {
            diff.y = other.bottom - new_rect.top;
        }
        if new_rect.bottom <= other.top {
            diff.y = other.top - new_rect.bottom;
        }
        if nearest.x.abs() > diff.x.abs() && nearest.y.abs() > diff.y.abs() {
            nearest = diff;
        }

        let strip = Rect {
            left: i32::MIN,
            top: new_rect.top,
            right: i32::MAX,
            bottom: new_rect.bottom,
        };
        if intersect_rect(strip, other).is_some() {
            diff.y = LONG_MAX;
            // The C compares `abs(offset.y)` with `abs(diff.x)` here; kept as written.
            if offset.x.abs() > diff.x.abs() && offset.y.abs() > diff.x.abs() {
                offset = diff;
            }
        }

        let strip = Rect {
            left: new_rect.left,
            top: i32::MIN,
            right: new_rect.right,
            bottom: i32::MAX,
        };
        if intersect_rect(strip, other).is_some() {
            diff.x = LONG_MAX;
            if offset.y.abs() > diff.y.abs() && offset.x.abs() > diff.y.abs() {
                offset = diff;
            }
        }
    }

    if offset.x == LONG_MAX && offset.y == LONG_MAX {
        offset = nearest;
    }
    if offset.x != LONG_MAX {
        mode.position.x += offset.x;
    }
    if offset.y != LONG_MAX {
        mode.position.y += offset.y;
    }

    // if our adjustments caused more intersection, keep the original position
    for (index, entry) in entries.iter().enumerate() {
        let other = rect_from_devmode(&entry.current);
        if index == device {
            continue;
        }
        if intersect_rect(other, new_rect).is_none() {
            continue;
        }
        mode = pending;
    }

    entries[device].current = mode;
    entries[device].pending = mode;
}
