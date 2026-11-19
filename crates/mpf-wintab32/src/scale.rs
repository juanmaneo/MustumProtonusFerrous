//! Axis scaling of `context.c`: `ScaleForContext`, the Y flip of `AddPacketToContextQueue`, and the
//! `MulDiv` they use (`kernelbase/main.c`). Values are tablet coordinates on input and context
//! coordinates on output.

/// One axis of a logical context: the input origin and extent, and the output origin and extent.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Axis {
    pub in_org: i32,
    pub in_ext: i32,
    pub out_org: i32,
    pub out_ext: i32,
}

/// `MulDiv(a, b, c)`: `a * b / c`, rounded by half the divisor. Returns -1 for a zero divisor or a
/// result outside ±`i32::MAX`.
pub fn mul_div(a: i32, b: i32, c: i32) -> i32 {
    if c == 0 {
        return -1;
    }
    let (a, c) = if c < 0 {
        (a.wrapping_neg(), c.wrapping_neg())
    } else {
        (a, c)
    };
    let product = i64::from(a) * i64::from(b);
    let half = i64::from(c / 2);
    let rounded = if (a < 0) == (b < 0) {
        product + half
    } else {
        product - half
    };
    let ret = rounded / i64::from(c);
    if ret > i64::from(i32::MAX) || ret < -i64::from(i32::MAX) {
        -1
    } else {
        ret as i32
    }
}

/// `ScaleForContext`: maps the tablet coordinate `input` onto the output range of `axis`. A
/// negative input extent is measured from the far end of the range.
pub fn scale_for_context(input: u32, axis: Axis) -> u32 {
    let offset = input.wrapping_sub(axis.in_org as u32) as i32;
    let in_ext = axis.in_ext.wrapping_abs();
    let out_ext = axis.out_ext.wrapping_abs();
    let scaled = if (axis.in_ext > 0 && axis.out_ext > 0) || (axis.in_ext < 0 && axis.out_ext < 0) {
        mul_div(offset, out_ext, in_ext)
    } else {
        mul_div(in_ext.wrapping_sub(offset), out_ext, in_ext)
    };
    scaled.wrapping_add(axis.out_org) as u32
}

/// Maps a packet position as `AddPacketToContextQueue` does: scales both axes, then flips Y when
/// the output Y extent is set.
pub fn map_point(x_axis: Axis, y_axis: Axis, x: u32, y: u32) -> (u32, u32) {
    let scaled_y = scale_for_context(y, y_axis);
    (
        scale_for_context(x, x_axis),
        flip_y(scaled_y, y_axis.out_ext),
    )
}

fn flip_y(y: u32, out_ext: i32) -> u32 {
    if out_ext > 0 {
        (out_ext as u32).wrapping_sub(y)
    } else if out_ext < 0 {
        out_ext.wrapping_add(y as i32).wrapping_abs() as u32
    } else {
        y
    }
}
