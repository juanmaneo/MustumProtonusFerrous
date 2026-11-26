//! Report translation from `check_value_caps` and `translate_report_to_xinput_state` in Wine's
//! `dlls/winexinput.sys/main.c`. The `HidP_*` calls that decode the raw report are not ported;
//! the caller passes the decoded usage values.

use crate::scale::{scale_value, ValueCaps};

const HID_USAGE_GENERIC_X: u16 = 0x30;
const HID_USAGE_GENERIC_Y: u16 = 0x31;
const HID_USAGE_GENERIC_Z: u16 = 0x32;
const HID_USAGE_GENERIC_RX: u16 = 0x33;
const HID_USAGE_GENERIC_RY: u16 = 0x34;
const HID_USAGE_GENERIC_RZ: u16 = 0x35;

/// The value caps of the six axes, as stored in `struct func_device`. `lt_caps` is the Z axis
/// and `rt_caps` is the RZ axis.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DeviceCaps {
    pub lx_caps: ValueCaps,
    pub ly_caps: ValueCaps,
    pub lt_caps: ValueCaps,
    pub rx_caps: ValueCaps,
    pub ry_caps: ValueCaps,
    pub rt_caps: ValueCaps,
}

impl DeviceCaps {
    /// Port of `check_value_caps`: stores the caps for a generic desktop axis usage. Other
    /// usages leave the caps unchanged.
    pub fn check_value_caps(&mut self, usage: u16, caps: ValueCaps) {
        match usage {
            HID_USAGE_GENERIC_X => self.lx_caps = caps,
            HID_USAGE_GENERIC_Y => self.ly_caps = caps,
            HID_USAGE_GENERIC_Z => self.lt_caps = caps,
            HID_USAGE_GENERIC_RX => self.rx_caps = caps,
            HID_USAGE_GENERIC_RY => self.ry_caps = caps,
            HID_USAGE_GENERIC_RZ => self.rt_caps = caps,
            _ => {}
        }
    }
}

/// A decoded input report. `buttons` holds the button-page usages (at most 10). `hat` is the
/// hat-switch value and `x`..`rz` are the raw generic desktop values. A failed
/// `HidP_GetUsageValue` call contributes 0, as its C initialiser does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Report<'a> {
    pub buttons: &'a [u16],
    pub hat: u32,
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub rx: u32,
    pub ry: u32,
    pub rz: u32,
}

/// The fields of `struct xinput_state` that the translation writes. The padding word is omitted.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct XinputState {
    pub lx_axis: u16,
    pub ly_axis: u16,
    pub rx_axis: u16,
    pub ry_axis: u16,
    pub trigger: u16,
    pub buttons: u16,
}

/// Port of `translate_report_to_xinput_state` after the `HidP_*` decoding. The hat value selects
/// a direction bit group, each button usage 1..=10 sets one bit, and the axes are scaled to
/// 16 bits (the Y axes inverted). The trigger is `0x8000 + (lt - rt) * 128` with unsigned wrap.
pub fn translate_report_to_xinput_state(report: &Report, caps: &DeviceCaps) -> XinputState {
    let mut state = XinputState::default();
    if !(1..=8).contains(&report.hat) {
        state.buttons = 0;
    } else {
        state.buttons = (report.hat << 10) as u16;
    }
    for &usage in report.buttons {
        if !(1..=10).contains(&usage) {
            continue;
        }
        state.buttons |= 1u16 << (usage - 1);
    }
    state.lx_axis = scale_value(report.x, caps.lx_caps, 0, 65535) as u16;
    state.ly_axis = 65535i32.wrapping_sub(scale_value(report.y, caps.ly_caps, 0, 65535)) as u16;
    state.rx_axis = scale_value(report.rx, caps.rx_caps, 0, 65535) as u16;
    state.ry_axis = 65535i32.wrapping_sub(scale_value(report.ry, caps.ry_caps, 0, 65535)) as u16;
    let rt = scale_value(report.rz, caps.rt_caps, 0, 255) as u32;
    let lt = scale_value(report.z, caps.lt_caps, 0, 255) as u32;
    state.trigger = 0x8000u32.wrapping_add(lt.wrapping_sub(rt).wrapping_mul(128)) as u16;
    state
}
