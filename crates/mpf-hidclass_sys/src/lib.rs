//! Headless subset of Wine's `dlls/hidclass.sys` (`device.c`).
//!
//! Ported:
//! - `hid_queue_create`, `hid_queue_push_report`, `hid_queue_pop_report` and
//!   `hid_queue_resize`: `ReportQueue`, the per-handle ring of input reports.
//! - `hid_report_create`: `HidReport::new`.
//! - `find_report_with_type_and_id`: `find_report_with_type_and_id`.
//! - `find_device_string` and its `device_strings` table: `find_device_string`.
//!
//! Skipped:
//! - The IRP queue (`hid_queue_pop_irp`, `hid_queue_push_irp`,
//!   `hid_queue_remove_pending_irps`), cancel routines and `hid_queue_destroy`:
//!   they hold IRPs and locks, and the ring's ownership is Rust's.
//! - Reference counting (`hid_report_incref`, `hid_report_decref`): `Option<T>`
//!   slots and `T`'s own ownership replace it.
//! - The device thread, the IOCTL and read/write dispatch, the PnP paths and
//!   `DriverEntry`, which need IRPs, events, threads or the kernel.
//!
//! Divergences from the C:
//! - Case folding in `find_device_string` is ASCII only. Wine's `wcsnicmp`
//!   folds other letters too.
//! - `find_device_string` returns `None` when the device id has no backslash.
//!   C reads `NULL + 1` there.
//! - `find_report_with_type_and_id` takes a `ReportType`. C's other type
//!   values match nothing, and the enum cannot express them.
//! - `HidReport::new` returns `None` when the payload is longer than the
//!   report. C writes past the allocation there.

mod queue;
mod report;
mod strings;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use queue::{ReportQueue, DEFAULT_LENGTH, MAX_LENGTH, MIN_LENGTH};
pub use report::{HidReport, ReportIds, ReportType};
pub use strings::HID_STRING_ID_IPRODUCT;

/// `STATUS_INVALID_PARAMETER`, returned by `hid_queue_resize` for a bad length.
pub const STATUS_INVALID_PARAMETER: i32 = 0xC000_000D_u32 as i32;

/// `find_report_with_type_and_id`: the index of the first matching entry.
pub fn find_report_with_type_and_id(
    reports: &[ReportIds],
    collection: u8,
    report_type: ReportType,
    id: u8,
    any_id: bool,
) -> Option<usize> {
    report::find_report_with_type_and_id(reports, collection, report_type, id, any_id)
}

/// `find_device_string`, the product string of a known controller.
pub fn find_device_string(device_id: &[u16], index: u32) -> Option<&'static str> {
    strings::find_device_string(device_id, index)
}
