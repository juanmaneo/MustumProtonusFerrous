//! Port of the portable logic in Wine's `dlls/desk.cpl/main.c`, with the
//! rectangle helpers it uses from `dlls/user32/uitools.c`.
//!
//! Ported:
//! - `IsRectEmpty`, `IntersectRect`, `OffsetRect`: [`Rect::is_empty`], [`intersect_rect`]
//! - `map_virtual_client_rect`: [`map_virtual_client_rect`]
//! - `rect_from_devmode`, `is_same_devmode`: [`rect_from_devmode`], [`is_same_devmode`]
//! - `device_entry_move_rect`, `device_entry_snap_rect`: [`move_rect`], [`snap_rect`]
//! - `IS_OPTION_TRUE` and the value decision of `get_option`: [`is_option_true`], [`option_value`]
//!
//! Skipped:
//! - `RegGetValueW` in `get_option`: registry access. The caller passes the value it read.
//! - `refresh_device_list`, `update_display_settings_list`, `clear_devices`: display enumeration and combo box messages.
//! - `handle_display_settings_change`, `handle_display_settings_apply`, `handle_emulate_modeset_change`: `ChangeDisplaySettings` and registry writes.
//! - `desktop_view_proc` (including the WM_PAINT fit scale and the drag scaling), `create_desktop_view`, `draw_monitor_rect`: GDI and window messages.
//! - `desktop_dialog_proc`, `property_sheet_callback`, `create_property_sheets`, `register_window_class`, `unregister_window_class`, `CPlApplet`, `DllMain`: dialogs, window classes and module lifecycle.

mod devmode;
mod layout;
mod option;

pub use devmode::{
    is_same_devmode, rect_from_devmode, DevMode, DM_PELSHEIGHT, DM_PELSWIDTH, DM_POSITION,
};
pub use layout::{move_rect, snap_rect, Entry};
pub use option::{is_option_true, option_value};

/// `POINT` (and `POINTL`), with `LONG` coordinates.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// `RECT`, with `LONG` edges.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    /// `IsRectEmpty`: zero or negative width or height.
    pub fn is_empty(&self) -> bool {
        self.left >= self.right || self.top >= self.bottom
    }

    /// `OffsetRect`.
    fn offset(self, dx: i32, dy: i32) -> Rect {
        Rect {
            left: self.left + dx,
            top: self.top + dy,
            right: self.right + dx,
            bottom: self.bottom + dy,
        }
    }
}

/// `IntersectRect`: the overlap of two rectangles, or `None` where Wine returns `FALSE`
/// (either input empty, or no overlap). Wine also writes the empty rectangle to `dest` on
/// failure; the callers in `desk.cpl` only test the result, so the port returns the overlap.
pub fn intersect_rect(src1: Rect, src2: Rect) -> Option<Rect> {
    if src1.is_empty()
        || src2.is_empty()
        || src1.left >= src2.right
        || src2.left >= src1.right
        || src1.top >= src2.bottom
        || src2.top >= src1.bottom
    {
        return None;
    }
    Some(Rect {
        left: src1.left.max(src2.left),
        top: src1.top.max(src2.top),
        right: src1.right.min(src2.right),
        bottom: src1.bottom.min(src2.bottom),
    })
}

/// `map_virtual_client_rect`: centres `rect` on the virtual desktop's centre, scales it by
/// `scale`, then centres it on the client area.
///
/// The C code converts each `LONG` edge to `float`, multiplies, and truncates back to `LONG`.
/// `as i32` does the same for values in range.
pub fn map_virtual_client_rect(
    rect: Rect,
    client_rect: Rect,
    virtual_rect: Rect,
    scale: f32,
) -> Rect {
    let mut rect = rect.offset(
        -(virtual_rect.left + virtual_rect.right) / 2,
        -(virtual_rect.top + virtual_rect.bottom) / 2,
    );
    rect.left = (rect.left as f32 * scale) as i32;
    rect.right = (rect.right as f32 * scale) as i32;
    rect.top = (rect.top as f32 * scale) as i32;
    rect.bottom = (rect.bottom as f32 * scale) as i32;
    rect.offset(
        (client_rect.left + client_rect.right) / 2,
        (client_rect.top + client_rect.bottom) / 2,
    )
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
