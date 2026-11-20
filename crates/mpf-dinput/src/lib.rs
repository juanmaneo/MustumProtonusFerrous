//! dinput subset for MPF: the pure value and mapping logic of `dlls/dinput`.
//!
//! Reference Wine's `dlls/dinput` is mostly device code: device handles, the HID
//! parser, the registry, COM objects and the event queue. The functions below compute
//! a result from their arguments alone. Host state that they need (the
//! `MapVirtualKeyW` result, the HID value caps, the axis properties) is passed in as
//! parameters.
//!
//! Ported (rows recorded from a verbatim Wine C oracle, plus edge checks):
//! - `map_dik_code` (`keyboard.c`): the scan code to `DIK_*` mapping, including the
//!   JP106 remap. The `MapVirtualKeyW` result is a parameter.
//! - `object_usage_to_guid`, `effect_guid_to_usage`, `effect_usage_to_guid`
//!   (`joystick_hid.c`): the HID and PID usage tables, with the `GUID_*` values.
//! - `scale_to_logical_value`, `clamp_to_physical_value`, `sign_extend`, `scale_value`,
//!   `scale_axis_value` (`joystick_hid.c`): axis value scaling.
//! - `device_type_for_version` (`joystick_hid.c`): the pre-DirectInput 8 device type.
//! - `convert_directions` (`joystick_hid.c`): PID effect directions between the
//!   Cartesian, polar and spherical frames.
//! - `mul_div` (kernelbase `MulDiv`): the rounding helper that the scaling uses.
//!
//! Divergences from the C code:
//! - `scale_to_logical_value` returns -1 when `physical_min == physical_max` and the
//!   physical bounds are not both zero. The C code divides by zero there and traps.
//! - Intermediate arithmetic is done in `i64` or `i128`, and the result is truncated
//!   to `i32` at the end. The C code uses `LONG` arithmetic, which overflows (undefined
//!   behaviour) for large inputs. `mul_div` keeps the C result check, and returns -1
//!   when the result leaves `±2147483647`.
//! - `bit_size` outside the range where the C shifts are defined is handled: a
//!   `bit_size` of 1 or less does no sign extension, the sign shift is capped at 63,
//!   and `scale_axis_value` clamps its `bit_max` shift to `0..=62`.
//! - `convert_directions` returns the converted axes as a `Vec`, with the length as
//!   `cAxes`. It returns `None` where C leaves the output untouched (a flags word with
//!   no frame bit set). The C code has a fixed `MAX_PID_AXES` buffer; this port has no
//!   limit.
//! - Spherical to polar reads the first angle when there is one, and reads 0 when there
//!   is none. C reads `rglDirection[0]` unconditionally, which is undefined for an
//!   empty input.
//! - The `FIXME`, `WARN` and `TRACE` logging is not modelled.
//!
//! Skipped (no logic is ported; each item needs OS, device or handle state):
//! - Device and event state: `keyboard_handle_event` (`keyboard.c`), `warp_check`
//!   (`mouse.c`, uses the cursor and `GetCurrentTime`), `find_next_effect_id`
//!   (`joystick_hid.c`, walks the device's PID effect pool through feature reports),
//!   and `queue_event` (`device.c`).
//! - OS queries and conversions: `get_keyboard_subtype` (`GetKeyboardType`), the
//!   `WideCharToMultiByte` path in `ansi.c`, and the registry helpers in `config.c` and
//!   `joystick_hid.c`.
//! - HID report writes: `set_parameter_value`, `set_parameter_value_clamp`,
//!   `set_parameter_value_angle` and `set_parameter_value_us` (`joystick_hid.c`) write
//!   through `set_report_value`, which needs the device's report layout. Their value
//!   mapping (`scale_to_logical_value`, `clamp_to_physical_value`) is ported. The unit
//!   exponent loops in the `_angle` and `_us` wrappers are not, because they belong with
//!   that write.

mod device_type;
mod direction;
mod guid;
mod keyboard;
mod scale;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use device_type::device_type_for_version;
pub use direction::{convert_directions, DIEFF_CARTESIAN, DIEFF_POLAR, DIEFF_SPHERICAL};
pub use guid::{
    effect_guid_to_usage, effect_usage_to_guid, object_usage_to_guid, Guid, GUID_BUTTON,
    GUID_CONSTANT_FORCE, GUID_CUSTOM_FORCE, GUID_DAMPER, GUID_FRICTION, GUID_INERTIA, GUID_POV,
    GUID_RAMP_FORCE, GUID_RX_AXIS, GUID_RY_AXIS, GUID_RZ_AXIS, GUID_SAWTOOTH_DOWN,
    GUID_SAWTOOTH_UP, GUID_SINE, GUID_SLIDER, GUID_SPRING, GUID_SQUARE, GUID_TRIANGLE,
    GUID_UNKNOWN, GUID_X_AXIS, GUID_Y_AXIS, GUID_Z_AXIS,
};
pub use keyboard::map_dik_code;
pub use scale::{
    clamp_to_physical_value, scale_axis_value, scale_to_logical_value, scale_value, sign_extend,
    ObjectProperties, ValueRange,
};

/// `MulDiv` from kernelbase: `a * b / c`, rounded half away from zero. Returns -1 when
/// `c` is zero or the result is outside `±2147483647`.
pub(crate) fn mul_div(a: i64, b: i64, c: i64) -> i32 {
    if c == 0 {
        return -1;
    }
    let (a, c) = if c < 0 { (-a, -c) } else { (a, c) };
    let product = i128::from(a) * i128::from(b);
    let half = i128::from(c / 2);
    let rounded = if (a < 0) == (b < 0) {
        product + half
    } else {
        product - half
    };
    let ret = rounded / i128::from(c);
    if !(-i128::from(i32::MAX)..=i128::from(i32::MAX)).contains(&ret) {
        return -1;
    }
    ret as i32
}
