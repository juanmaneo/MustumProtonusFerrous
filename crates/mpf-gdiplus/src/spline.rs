use crate::geometry::PointF;
use crate::math::gdip_round;

/// `calc_curve_bezier` from `dlls/gdiplus/gdiplus.c`: the two control points of the segment
/// through `pts[1]`, with the tangent taken from `pts[0]` to `pts[2]`.
pub fn calc_curve_bezier(pts: &[PointF; 3], tension: f32) -> (PointF, PointF) {
    let xdiff = pts[2].x - pts[0].x;
    let ydiff = pts[2].y - pts[0].y;
    (
        PointF {
            x: pts[1].x - tension * xdiff,
            y: pts[1].y - tension * ydiff,
        },
        PointF {
            x: pts[1].x + tension * xdiff,
            y: pts[1].y + tension * ydiff,
        },
    )
}

/// `calc_curve_bezier_endp` from `dlls/gdiplus/gdiplus.c`: the control point at an endpoint,
/// along the line to the adjacent point, rounded to whole units.
pub fn calc_curve_bezier_endp(end: PointF, adj: PointF, tension: f32) -> PointF {
    PointF {
        x: gdip_round(tension * (adj.x - end.x) + end.x) as f32,
        y: gdip_round(tension * (adj.y - end.y) + end.y) as f32,
    }
}
