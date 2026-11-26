//! HID axis scaling from `sign_extend` and `scale_value` in Wine's `dlls/winexinput.sys/main.c`,
//! with the `MulDiv` helper from `dlls/kernelbase/main.c`.

/// The fields of `HIDP_VALUE_CAPS` that the scaling reads.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ValueCaps {
    pub bit_size: u16,
    pub logical_min: i32,
    pub logical_max: i32,
}

/// Port of `sign_extend`. Bit sizes outside `1..=32` make the C shift undefined, so the value
/// is returned unchanged for them. A size of 32 is the plain 32-bit two's complement.
pub fn sign_extend(value: u32, caps: ValueCaps) -> i32 {
    let sign = 1u32
        .checked_shl(u32::from(caps.bit_size).wrapping_sub(1))
        .unwrap_or(0);
    if sign <= 1 || caps.logical_min >= 0 {
        return value as i32;
    }
    value.wrapping_sub((value & sign) << 1) as i32
}

/// Port of `scale_value`: maps a raw value from the logical range onto `min..=max`. Values
/// outside the logical range, or a logical range that is inverted, map to 0. A zero-width
/// logical range goes through `mul_div` with a zero divisor, so it yields `min - 1`, as in C.
pub fn scale_value(value: u32, caps: ValueCaps, min: i32, max: i32) -> i32 {
    let tmp = sign_extend(value, caps);
    if caps.logical_min > caps.logical_max {
        return 0;
    }
    if caps.logical_min > tmp || caps.logical_max < tmp {
        return 0;
    }
    min.wrapping_add(mul_div(
        tmp.wrapping_sub(caps.logical_min),
        max.wrapping_sub(min),
        caps.logical_max.wrapping_sub(caps.logical_min),
    ))
}

/// Port of kernelbase `MulDiv`: `a * b / c`, rounded half away from zero. Returns -1 for a
/// zero divisor or a result outside `-2147483647..=2147483647`.
pub fn mul_div(a: i32, b: i32, c: i32) -> i32 {
    if c == 0 {
        return -1;
    }
    let (mut a, mut c) = (i64::from(a), i64::from(c));
    let b = i64::from(b);
    if c < 0 {
        a = -a;
        c = -c;
    }
    let ret = if (a < 0 && b < 0) || (a >= 0 && b >= 0) {
        (a * b + c / 2) / c
    } else {
        (a * b - c / 2) / c
    };
    if !(-2_147_483_647..=2_147_483_647).contains(&ret) {
        return -1;
    }
    ret as i32
}
