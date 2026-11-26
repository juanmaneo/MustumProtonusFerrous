//! Port of the portable logic in Wine's `dlls/joy.cpl/dinput.c` and `dlls/joy.cpl/xinput.c`,
//! plus `MulDiv` from `dlls/kernel32/kernel_main.c`, which the XInput drawing uses for scaling.
//!
//! Ported:
//! - `MulDiv`: [`mul_div`]
//! - `OffsetRect` and the rectangle and point arithmetic the views use: [`Rect::offset`], [`Point::offset`]
//! - `dinput.c` drawings: [`dinput::draw_axis_view`], [`dinput::draw_pov_view`],
//!   [`dinput::draw_button_view`] (shape choice only)
//! - `dinput.c` panel layouts (`paint_axes_view`, `paint_povs_view`, `paint_buttons_view`):
//!   [`dinput::axes_view_rects`], [`dinput::povs_view_rects`], [`dinput::buttons_view_cells`]
//! - `xinput.c` drawings: [`xinput::draw_axis_view`], [`xinput::draw_trigger_view`]
//! - `xinput.c` gamepad panel layout (`paint_gamepad_view`): [`xinput::gamepad_view_layout`]
//!
//! Skipped:
//! - GDI colour, brush, pen, font and text calls (`SetDCBrushColor`, `SelectObject`,
//!   `DrawTextW`, `SetBkMode`, ...), `FillRect` backgrounds, and `BeginPaint`/`EndPaint`.
//!   The crate returns the shapes and cells; it does not draw them.
//! - The colour-only decisions: the `set` flag and `dx || dy` choose highlight colours in
//!   the C code, so the ported functions do not take or return them (except the button
//!   `set` byte carried by [`dinput::ButtonCell`], which the caller can use for colour).
//! - `xinput.c` `draw_button_view`: it draws Marlett glyphs, bars and text. Its cell
//!   positions and labels are in [`xinput::gamepad_view_layout`].
//! - DirectInput effects and devices, and the input threads in `dinput.c`, `xinput.c` and
//!   `wginput.c`.
//! - Windows.Gaming.Input (`wginput.c`): COM interfaces and device enumeration.
//! - `main.c`: `DllMain`, `CPlApplet`, the registry options, the joystick list dialog and
//!   the window classes.
//! - Window procedures and dialog procedures (`test_*_window_proc`, `test_*_dialog_proc`,
//!   `create_*_view`, `update_*_view`).

pub mod dinput;
pub mod xinput;

/// `POINT`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Moves the point by `(dx, dy)`. Wraps like the C `LONG` arithmetic it replaces.
    pub const fn offset(self, dx: i32, dy: i32) -> Self {
        Self::new(self.x.wrapping_add(dx), self.y.wrapping_add(dy))
    }
}

/// `RECT`. The accessors and [`Rect::offset`] wrap on overflow.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    pub const fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    /// `OffsetRect`.
    pub const fn offset(self, dx: i32, dy: i32) -> Self {
        Self::new(
            self.left.wrapping_add(dx),
            self.top.wrapping_add(dy),
            self.right.wrapping_add(dx),
            self.bottom.wrapping_add(dy),
        )
    }

    /// `rect.right - rect.left`.
    pub const fn width(self) -> i32 {
        self.right.wrapping_sub(self.left)
    }

    /// `rect.bottom - rect.top`.
    pub const fn height(self) -> i32 {
        self.bottom.wrapping_sub(self.top)
    }

    /// `(rect.left + rect.right) / 2`.
    pub const fn centre_x(self) -> i32 {
        self.left.wrapping_add(self.right) / 2
    }

    /// `(rect.top + rect.bottom) / 2`.
    pub const fn centre_y(self) -> i32 {
        self.top.wrapping_add(self.bottom) / 2
    }
}

/// One drawing primitive, in the order the C code issues it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Shape {
    /// `Ellipse` inside the bounding box.
    Ellipse(Rect),
    /// `Rectangle` with the given bounding box.
    Rectangle(Rect),
    /// `RoundRect` with the given bounding box. The C corner radius (5) is not modelled.
    RoundRect(Rect),
    /// `MoveToEx(from)` followed by `LineTo(to)`.
    Line { from: Point, to: Point },
    /// `Polygon` with three vertices.
    Polygon([Point; 3]),
}

/// `MulDiv` from `dlls/kernel32/kernel_main.c`: `multiplicand * multiplier / divisor`, rounded
/// by adding or subtracting half the divisor before truncating, so ties round away from zero.
/// Returns -1 for a zero divisor or a result outside `-i32::MAX..=i32::MAX`.
///
/// The C code negates a negative divisor, and the multiplicand with it, which overflows for
/// `i32::MIN`. Here both negations wrap, so those inputs are defined instead of undefined.
pub fn mul_div(multiplicand: i32, multiplier: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        return -1;
    }
    let (multiplicand, divisor) = if divisor < 0 {
        (multiplicand.wrapping_neg(), divisor.wrapping_neg())
    } else {
        (multiplicand, divisor)
    };
    let product = i64::from(multiplicand) * i64::from(multiplier);
    let half = i64::from(divisor / 2);
    let divisor = i64::from(divisor);
    let quotient = if (multiplicand < 0) == (multiplier < 0) {
        (product + half) / divisor
    } else {
        (product - half) / divisor
    };
    if quotient.abs() > i64::from(i32::MAX) {
        -1
    } else {
        quotient as i32
    }
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
