//! Port of the portable logic in Wine's `dlls/mouhid.sys/main.c`.
//!
//! Ported:
//! - `sign_extend` and `scale_value`, which turn a raw HID value into a
//!   logical-range value through `HIDP_VALUE_CAPS`: [`sign_extend`], [`scale_value`], [`ValueCaps`]
//! - `MulDiv` as used by `scale_value` (body from `dlls/kernelbase/main.c`): [`mul_div`]
//! - `process_hid_report`, `add_contact` and `release_contacts`: [`ContactTracker`], with the
//!   `HidP_GetUsageValue` and `HidP_GetUsages` reads behind [`ReportReader`] and the
//!   `WM_POINTER*` messages returned as [`PointerEvent`]
//! - Releasing every tracked contact when the device is removed (the `release_contacts`
//!   call in `driver_pnp`): [`ContactTracker::release_all`]
//!
//! Skipped:
//! - `initialize_device`: `HidP_*` preparsed-data queries and the `contact_max` setup
//! - `start_device_read`, `read_completion`, `driver_ioctl`, `call_hid_device`,
//!   `set_event_completion`: IRP and IO plumbing
//! - `driver_pnp` apart from the contact release, `driver_create`, `add_device`,
//!   `driver_unload`, `DriverEntry`, `impl_from_DEVICE_OBJECT`: driver lifecycle and device extension access
//! - The `contact_max` clamp in `process_hid_report`: `contact_count` is never read after it,
//!   and the only other effect is a `WARN`
//! - The `calloc` failure branch in `add_contact`: Rust allocation aborts instead of returning
//!
//! Where the C source has undefined behaviour (a shift by -1 for `BitSize` 0, signed overflow
//! in the subtractions of `scale_value` and `MulDiv`, negating `INT_MIN`), the port follows what
//! x86 does: shift counts are taken modulo 32 and arithmetic wraps.

mod contacts;

pub use contacts::{Contact, ContactCaps, ContactTracker, PointerEvent, PointerKind, ReportReader};

/// The fields of `HIDP_VALUE_CAPS` that the value conversion reads.
pub struct ValueCaps {
    pub bit_size: u16,
    pub logical_min: i32,
    pub logical_max: i32,
}

/// `sign_extend` from `main.c`. `BitSize` 0 makes the C shift `1 << -1` undefined;
/// the count is taken modulo 32 as x86 does.
pub fn sign_extend(value: u32, caps: &ValueCaps) -> i32 {
    let sign = 1u32.wrapping_shl(u32::from(caps.bit_size).wrapping_sub(1));
    if sign <= 1 || caps.logical_min >= 0 {
        return value as i32;
    }
    value.wrapping_sub((value & sign) << 1) as i32
}

/// `scale_value` from `main.c`: maps a raw value from the logical range of `caps` onto `min..=max`.
/// Values outside the logical range, and inverted ranges, scale to 0.
pub fn scale_value(value: u32, caps: &ValueCaps, min: i32, max: i32) -> i32 {
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

/// `MulDiv` from `dlls/kernelbase/main.c`: `a * b / c` rounded half away from zero.
/// Returns -1 for a zero divisor or a result outside `-2147483647..=2147483647`.
pub fn mul_div(a: i32, b: i32, c: i32) -> i32 {
    if c == 0 {
        return -1;
    }
    let (a, c) = if c < 0 {
        (a.wrapping_neg(), c.wrapping_neg())
    } else {
        (a, c)
    };
    let half = i64::from(c / 2);
    let product = i64::from(a) * i64::from(b);
    let ret = if (a < 0 && b < 0) || (a >= 0 && b >= 0) {
        (product + half) / i64::from(c)
    } else {
        (product - half) / i64::from(c)
    };
    if ret > i64::from(i32::MAX) || ret < -i64::from(i32::MAX) {
        return -1;
    }
    ret as i32
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
