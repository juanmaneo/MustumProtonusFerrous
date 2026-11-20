//! PID effect direction conversion from `dlls/dinput/joystick_hid.c`.
//!
//! The three frames are Cartesian, polar and spherical. Angles are in hundredths of a
//! degree, and a unit vector is scaled by 10000. Each conversion runs through the
//! spherical frame when the two frames differ.

use std::f64::consts::PI;

/// `DIEFF_CARTESIAN`.
pub const DIEFF_CARTESIAN: u32 = 0x10;
/// `DIEFF_POLAR`.
pub const DIEFF_POLAR: u32 = 0x20;
/// `DIEFF_SPHERICAL`.
pub const DIEFF_SPHERICAL: u32 = 0x40;

const DIEFF_DIRECTION_MASK: u32 = DIEFF_CARTESIAN | DIEFF_POLAR | DIEFF_SPHERICAL;

enum Frame {
    Cartesian,
    Polar,
    Spherical,
}

fn frame(flags: u32) -> Option<Frame> {
    match flags & DIEFF_DIRECTION_MASK {
        DIEFF_CARTESIAN => Some(Frame::Cartesian),
        DIEFF_POLAR => Some(Frame::Polar),
        DIEFF_SPHERICAL => Some(Frame::Spherical),
        _ => None,
    }
}

/// Converts `axes` from the frame in `in_flags` to the frame in `out_flags`. Returns
/// `None` when either flags word has no direction frame bit, as C leaves the output
/// untouched in that case.
pub fn convert_directions(in_flags: u32, axes: &[i32], out_flags: u32) -> Option<Vec<i32>> {
    let from = frame(in_flags)?;
    let to = frame(out_flags)?;
    Some(match (from, to) {
        (Frame::Cartesian, Frame::Cartesian) | (Frame::Polar, Frame::Polar) => axes.to_vec(),
        (from, Frame::Spherical) => to_spherical(axes, from),
        (Frame::Spherical, Frame::Cartesian) => cartesian_from_spherical(axes),
        (Frame::Spherical, Frame::Polar) => polar_from_spherical(axes),
        (from, Frame::Cartesian) => cartesian_from_spherical(&to_spherical(axes, from)),
        (from, Frame::Polar) => polar_from_spherical(&to_spherical(axes, from)),
    })
}

fn to_spherical(axes: &[i32], from: Frame) -> Vec<i32> {
    match from {
        Frame::Cartesian => cartesian_to_spherical(axes),
        Frame::Polar => polar_to_spherical(axes),
        Frame::Spherical => spherical_to_spherical(axes),
    }
}

/// Adds a full turn to a negative angle, once. The C code does not wrap it again.
fn turn_if_negative(angle: i64) -> i32 {
    if angle < 0 {
        (angle + 36000) as i32
    } else {
        angle as i32
    }
}

fn cartesian_to_spherical(axes: &[i32]) -> Vec<i32> {
    let mut out = vec![0; axes.len()];
    for (i, &axis) in axes.iter().enumerate().skip(1) {
        let mut tmp = f64::from(axes[0]);
        for &prev in &axes[1..i] {
            tmp = (tmp * tmp + f64::from(prev) * f64::from(prev)).sqrt();
        }
        let angle = f64::from(axis).atan2(tmp) * 18000.0 / PI;
        out[i - 1] = turn_if_negative(angle as i64);
    }
    out
}

fn polar_to_spherical(axes: &[i32]) -> Vec<i32> {
    let mut out = vec![0; axes.len()];
    if let Some(&first) = axes.first() {
        out[0] = turn_if_negative(i64::from(first) % 36000 - 9000);
    }
    out
}

fn spherical_to_spherical(axes: &[i32]) -> Vec<i32> {
    let mut out: Vec<i32> = axes
        .iter()
        .map(|&angle| turn_if_negative(i64::from(angle) % 36000))
        .collect();
    if let Some(last) = out.last_mut() {
        *last = 0;
    }
    out
}

fn cartesian_from_spherical(angles: &[i32]) -> Vec<i32> {
    let count = angles.len();
    let mut out = vec![0; count];
    if let Some(first) = out.first_mut() {
        *first = 10000;
    }
    for (k, &angle) in angles.iter().enumerate() {
        let i = k + 1;
        let radians = f64::from(angle) * PI / 18000.0;
        let cos = (radians.cos() * 10000.0) as i64;
        for v in &mut out[..i] {
            *v = ((i64::from(*v) * cos) as f64 / 10000.0).round() as i32;
        }
        if i < count {
            out[i] = (radians.sin() * 10000.0) as i32;
        }
    }
    out
}

/// Always two angles. An empty input reads 0 for the first angle.
fn polar_from_spherical(angles: &[i32]) -> Vec<i32> {
    let first = angles.first().copied().unwrap_or(0);
    vec![turn_if_negative((i64::from(first) + 9000) % 36000), 0]
}
