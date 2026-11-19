/// A Win32 `RECT`: the left and top edges are inside the rectangle, the right and bottom
/// edges are outside it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

/// `IsRectEmpty`: a rectangle with no width or no height.
pub fn is_rect_empty(rect: &Rect) -> bool {
    rect.left >= rect.right || rect.top >= rect.bottom
}

/// `intersect_rect`: the overlap of two rectangles, or `None` when either is empty or they do
/// not overlap. Rectangles that only touch along an edge do not overlap.
pub fn intersect_rect(a: &Rect, b: &Rect) -> Option<Rect> {
    if is_rect_empty(a)
        || is_rect_empty(b)
        || a.left >= b.right
        || b.left >= a.right
        || a.top >= b.bottom
        || b.top >= a.bottom
    {
        return None;
    }

    Some(Rect {
        left: a.left.max(b.left),
        top: a.top.max(b.top),
        right: a.right.min(b.right),
        bottom: a.bottom.min(b.bottom),
    })
}
