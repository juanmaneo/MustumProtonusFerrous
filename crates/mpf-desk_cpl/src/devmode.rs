//! Port of `rect_from_devmode` and `is_same_devmode` from `dlls/desk.cpl/main.c`.

use crate::{Point, Rect};

/// `DM_POSITION` from `wingdi.h`.
pub const DM_POSITION: u32 = 0x0000_0020;
/// `DM_PELSWIDTH` from `wingdi.h`.
pub const DM_PELSWIDTH: u32 = 0x0008_0000;
/// `DM_PELSHEIGHT` from `wingdi.h`.
pub const DM_PELSHEIGHT: u32 = 0x0010_0000;

/// The `DEVMODEW` fields that the `desk.cpl` display logic reads.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DevMode {
    pub fields: u32,
    pub position: Point,
    pub display_orientation: u32,
    pub bits_per_pel: u32,
    pub pels_width: u32,
    pub pels_height: u32,
    pub display_frequency: u32,
}

/// `rect_from_devmode`: a flagged position sets both corners, then the flagged sizes are added
/// to the right and bottom edges. Unflagged parts stay zero.
pub fn rect_from_devmode(mode: &DevMode) -> Rect {
    let mut rect = Rect::default();
    if (mode.fields & DM_POSITION) != 0 {
        rect = Rect {
            left: mode.position.x,
            top: mode.position.y,
            right: mode.position.x,
            bottom: mode.position.y,
        };
    }
    // The C adds an unsigned DWORD to a LONG, so the sum wraps in 32 bits.
    if (mode.fields & DM_PELSWIDTH) != 0 {
        rect.right = rect.right.wrapping_add(mode.pels_width as i32);
    }
    if (mode.fields & DM_PELSHEIGHT) != 0 {
        rect.bottom = rect.bottom.wrapping_add(mode.pels_height as i32);
    }
    rect
}

/// `is_same_devmode`: compares orientation, depth, size and frequency. `fields` and the
/// position are ignored.
pub fn is_same_devmode(a: &DevMode, b: &DevMode) -> bool {
    a.display_orientation == b.display_orientation
        && a.bits_per_pel == b.bits_per_pel
        && a.pels_width == b.pels_width
        && a.pels_height == b.pels_height
        && a.display_frequency == b.display_frequency
}
