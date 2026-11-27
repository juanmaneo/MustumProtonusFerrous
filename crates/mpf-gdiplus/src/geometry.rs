/// `GpPointF`: a point with `REAL` coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointF {
    pub x: f32,
    pub y: f32,
}

/// `GpPoint`: a point with `INT` coordinates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// `GpRectF`: a rectangle with `REAL` coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RectF {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// `GpRect`: a rectangle with `INT` coordinates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

/// `set_rect` from `dlls/gdiplus/gdiplus_private.h`, which the C version writes through a pointer.
pub fn set_rect(x: f32, y: f32, width: f32, height: f32) -> RectF {
    RectF {
        x,
        y,
        width,
        height,
    }
}
