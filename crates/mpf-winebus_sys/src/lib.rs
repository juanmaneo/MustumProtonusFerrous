//! Headless subset of Wine's `dlls/winebus.sys` (`hid.c`, `bus_udev.c`).
//!
//! Ported:
//! - The input report layout state machine of `hid.c`: `hid_device_add_button_count`,
//!   `hid_device_add_hatswitch_count`, `hid_device_add_axis_count` and
//!   `hid_device_determine_axis_size`, as `HidDeviceLayout` and `determine_axis_size`.
//! - The report field writers: `hid_device_set_abs_axis`, `hid_device_set_rel_axis`,
//!   `hid_device_set_button`, `hid_device_set_hatswitch_x` and `_y`,
//!   `hid_device_move_hatswitch`, and the static `hid_device_set_axis`, as
//!   `HidDeviceLayout` methods over a caller-owned report slice.
//! - `hatswitch_decompose` and `hatswitch_compose`: `hatswitch_decompose` and
//!   `hatswitch_compose`.
//! - `hid_device_sync_report` and `hid_device_drop_report`: `sync_report` and
//!   `drop_report`, over caller-owned report and last-report slices.
//! - `set_effect_type_from_usage` from `bus_udev.c`: `ff_effect_type_from_usage`.
//!
//! Skipped:
//! - The report descriptor emitters (`hid_device_add_buttons`, `hid_device_add_axes`,
//!   `hid_device_add_physical`, and the rest of the descriptor builders). They are
//!   deterministic but write into heap buffers, so they are left as a follow-up.
//! - The bus drivers (SDL, udev, IOHID), the `bus_event` queue, the IRP and PnP
//!   paths, and `hid_device_set_effect_state`. They need OS handles, threads or
//!   the kernel.
//! - The gamepad axis scaling inside `set_abs_axis_value`. It is inline C with no
//!   separate verbatim function, so it is not ported.
//! - The `report_buf` and `last_report_buf` allocations and `struct
//!   hid_device_state`'s pointer fields. The caller owns the buffers.
//! - `ERR` and `TRACE` output, `DriverEntry`, and the `id` field.
//!
//! Divergences from the C:
//! - `set_hatswitch_x`, `set_hatswitch_y` and `move_hatswitch` accept
//!   `index == hatswitch_count`, as C does. For an even count that index lies past
//!   the last hat byte. C writes there, and ASan flags it. Rust panics when that
//!   byte is outside the slice.
//! - Writes to a report index outside the slice panic in Rust. C is undefined there.
//! - Misaligned 16-bit and 32-bit axis stores are byte-wise little-endian in Rust.
//!   C's `*(USHORT *)` and `*(ULONG *)` stores are flagged by UBSan at odd or
//!   unaligned offsets, and match the byte-wise result on x86.
//! - `ERR` logs are dropped, and the failure path returns `false` or `None`.

mod effect;
mod report;

use std::mem;

pub use effect::{ff_effect_type_from_usage, FfEffectType};
pub use report::{
    determine_axis_size, hatswitch_compose, hatswitch_decompose, HidDeviceLayout, UsageAndPage,
    ABS_AXIS_USAGE_LIMIT, AXIS_LIMIT, REPORT_BIT_LIMIT,
};

/// `hid_device_sync_report`. Returns `false` when a dropped report was restored.
///
/// `report` and `last` must both have the report length.
pub fn sync_report(report: &mut [u8], last: &mut [u8], dropped: &mut bool) -> bool {
    let was_dropped = mem::replace(dropped, false);
    if was_dropped {
        report.copy_from_slice(last);
    } else {
        last.copy_from_slice(report);
    }
    !was_dropped
}

/// `hid_device_drop_report`: the next sync restores the last report.
pub fn drop_report(dropped: &mut bool) {
    *dropped = true;
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
