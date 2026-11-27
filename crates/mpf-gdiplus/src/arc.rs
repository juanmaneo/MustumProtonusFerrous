use std::f64::consts::{FRAC_PI_2, PI};

use crate::geometry::{PointF, RectF};
use crate::math::{deg2rad, gdip_round, gdiplus_atan2};

/// `MAX_ARC_PTS` from `dlls/gdiplus/gdiplus_private.h`: the number of points one arc can produce.
pub const MAX_ARC_PTS: usize = 13;

/// `add_arc_part` from `dlls/gdiplus/gdiplus.c`: writes the Bezier points of the arc from `start`
/// to `end` (radians) into `pt[0..4]`, with `pt[0]` written only when `write_first` is set.
fn add_arc_part(pt: &mut [PointF], bounds: RectF, start: f32, end: f32, write_first: bool) {
    let rad_x = bounds.width / 2.0;
    let rad_y = bounds.height / 2.0;
    let center_x = bounds.x + rad_x;
    let center_y = bounds.y + rad_y;

    let cos_start = f64::from(start).cos() as f32;
    let cos_end = f64::from(end).cos() as f32;
    let sin_start = f64::from(start).sin() as f32;
    let sin_end = f64::from(end).sin() as f32;

    let half = f64::from((end - start) / 2.0);
    let a = (4.0 / 3.0 * (1.0 - half.cos()) / half.sin()) as f32;

    if write_first {
        pt[0] = PointF {
            x: cos_start,
            y: sin_start,
        };
    }
    pt[1] = PointF {
        x: cos_start - a * sin_start,
        y: sin_start + a * cos_start,
    };
    pt[3] = PointF {
        x: cos_end,
        y: sin_end,
    };
    pt[2] = PointF {
        x: cos_end + a * sin_end,
        y: sin_end - a * cos_end,
    };

    let first = usize::from(!write_first);
    for p in &mut pt[first..4] {
        p.x = p.x * rad_x + center_x;
        p.y = p.y * rad_y + center_y;
    }
}

/// `unstretch_angle` from `dlls/gdiplus/gdiplus.c`: the angle on the unit circle that maps to
/// `angle` on the ellipse with diameters `dia_x` and `dia_y`.
fn unstretch_angle(angle: f32, dia_x: f32, dia_y: f32) -> f32 {
    let rad = f64::from(angle);
    if rad.cos().abs() < 0.00001 || rad.sin().abs() < 0.00001 {
        return angle;
    }

    let mut stretched = gdiplus_atan2(
        (rad.sin() / f64::from(dia_y).abs()) as f32,
        (rad.cos() / f64::from(dia_x).abs()) as f32,
    );
    let revs_off = gdip_round((rad / (2.0 * PI)) as f32)
        .wrapping_sub(gdip_round((f64::from(stretched) / (2.0 * PI)) as f32));
    stretched = (f64::from(stretched) + f64::from(revs_off as f32) * PI * 2.0) as f32;
    stretched
}

/// `arc2polybezier` from `dlls/gdiplus/gdiplus.c`. The points of the arc are written to `points`
/// when it is given. The return value is the number of points the arc needs.
pub fn arc2polybezier(
    points: Option<&mut [PointF; MAX_ARC_PTS]>,
    bounds: RectF,
    start_angle: f32,
    sweep_angle: f32,
) -> i32 {
    let mut points = points;
    let mut end = deg2rad(start_angle + sweep_angle);
    let mut start = deg2rad(start_angle);

    if bounds.width != bounds.height {
        start = unstretch_angle(start, bounds.width, bounds.height);
        end = unstretch_angle(end, bounds.width, bounds.height);
    }

    let mut i = 0;
    while i < MAX_ARC_PTS - 1 {
        let partial_end = if sweep_angle > 0.0 {
            if start >= end {
                break;
            }
            let a = f64::from(start) + FRAC_PI_2;
            let b = f64::from(end);
            (if a < b { a } else { b }) as f32
        } else {
            if start <= end {
                break;
            }
            let a = f64::from(start) - FRAC_PI_2;
            let b = f64::from(end);
            (if a > b { a } else { b }) as f32
        };

        if let Some(pts) = points.as_deref_mut() {
            add_arc_part(&mut pts[i..i + 4], bounds, start, partial_end, i == 0);
        }

        start = partial_end;
        i += 3;
    }

    if i == 0 {
        0
    } else {
        (i + 1) as i32
    }
}
