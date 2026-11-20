//! Axis value scaling from `dlls/dinput/joystick_hid.c`, with the `MulDiv` rounding it uses.

use crate::mul_div;

/// The `object_properties` fields that the value scaling reads.
#[derive(Clone, Copy, Debug, Default)]
pub struct ObjectProperties {
    pub bit_size: i32,
    pub logical_min: i32,
    pub logical_max: i32,
    pub range_min: i32,
    pub range_max: i32,
    pub deadzone: i32,
    pub saturation: i32,
}

/// The `hid_value_caps` fields that the logical and physical mapping reads.
#[derive(Clone, Copy, Debug)]
pub struct ValueRange {
    pub logical_min: i32,
    pub logical_max: i32,
    pub physical_min: i32,
    pub physical_max: i32,
}

/// `scale_to_logical_value`: maps a physical value into the logical range. A value
/// outside the physical range maps to -1. Without caps, or with both physical bounds
/// zero, the value is returned unchanged.
pub fn scale_to_logical_value(value: i32, caps: Option<ValueRange>) -> i32 {
    let Some(caps) = caps else {
        return value;
    };
    if caps.physical_min == 0 && caps.physical_max == 0 {
        return value;
    }
    // A zero-width physical range divides by zero in C; the port reports it as out of range.
    if !(caps.physical_min..=caps.physical_max).contains(&value)
        || caps.physical_min == caps.physical_max
    {
        return -1;
    }
    let value = i128::from(value);
    let log_min = i128::from(caps.logical_min);
    let log_max = i128::from(caps.logical_max);
    let phy_min = i128::from(caps.physical_min);
    let phy_max = i128::from(caps.physical_max);
    (log_min + (value - phy_min) * (log_max - log_min) / (phy_max - phy_min)) as i32
}

/// `clamp_to_physical_value`: clamps a value to the physical range. Without caps, the
/// value is returned unchanged.
pub fn clamp_to_physical_value(value: i32, caps: Option<ValueRange>) -> i32 {
    let Some(caps) = caps else {
        return value;
    };
    value.min(caps.physical_max).max(caps.physical_min)
}

/// `sign_extend`: reads a `bit_size` wide field as signed, when the logical range
/// starts below zero.
pub fn sign_extend(value: u32, properties: &ObjectProperties) -> i32 {
    if properties.bit_size <= 1 || properties.logical_min >= 0 {
        return value as i32;
    }
    let sign = 1i64 << (properties.bit_size - 1).min(63);
    let value = i64::from(value);
    (value - ((value & sign) << 1)) as i32
}

/// `scale_value`: maps a logical button or axis value to its physical range. A value
/// outside the logical range maps to -1.
pub fn scale_value(value: u32, properties: &ObjectProperties) -> i32 {
    let tmp = sign_extend(value, properties);
    let log_min = properties.logical_min;
    let log_max = properties.logical_max;
    if !(log_min..=log_max).contains(&tmp) {
        return -1;
    }
    let scaled = mul_div(
        i64::from(tmp) - i64::from(log_min),
        i64::from(properties.range_max) - i64::from(properties.range_min),
        i64::from(log_max) - i64::from(log_min),
    );
    (i64::from(properties.range_min) + i64::from(scaled)) as i32
}

/// `scale_axis_value`: maps a logical axis value to its physical range, with a
/// deadzone and a saturation point on each side of the centre.
pub fn scale_axis_value(value: u32, properties: &ObjectProperties) -> i32 {
    let mut tmp = i64::from(sign_extend(value, properties));
    let bit_max = (1i64 << properties.bit_size.clamp(0, 62)) - 1;
    let mut log_min = i64::from(properties.logical_min);
    let mut log_max = i64::from(properties.logical_max);
    let mut phy_min = i64::from(properties.range_min);
    let mut phy_max = i64::from(properties.range_max);
    let deadzone = i64::from(properties.deadzone);
    let saturation = i64::from(properties.saturation);

    // XInput HID gamepads report a bogus logical range; the bit range is used instead.
    if log_min == 0 && log_max == -1 {
        log_max = bit_max;
    }

    let phy_ctr = centre(phy_min, phy_max);
    let log_ctr = centre(log_min, log_max);
    tmp -= log_ctr;
    if tmp <= 0 {
        log_max = i64::from(mul_div(log_min - log_ctr, deadzone, 10000));
        log_min = i64::from(mul_div(log_min - log_ctr, saturation, 10000));
        phy_max = phy_ctr;
    } else {
        log_min = i64::from(mul_div(log_max - log_ctr, deadzone, 10000));
        log_max = i64::from(mul_div(log_max - log_ctr, saturation, 10000));
        phy_min = phy_ctr;
    }

    let scaled = if tmp <= log_min {
        phy_min
    } else if tmp >= log_max {
        phy_max
    } else {
        phy_min + i64::from(mul_div(tmp - log_min, phy_max - phy_min, log_max - log_min))
    };
    scaled as i32
}

/// The centre of a range as `scale_axis_value` takes it: `max >> 1` when `min` is zero,
/// otherwise the midpoint rounded half away from zero.
fn centre(min: i64, max: i64) -> i64 {
    if min == 0 {
        max >> 1
    } else {
        ((min + max) as f64 / 2.0).round() as i64
    }
}
