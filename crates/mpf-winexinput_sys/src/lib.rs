//! Port of the portable logic in Wine's `dlls/winexinput.sys/main.c`.
//!
//! Ported:
//! - `sign_extend` and `scale_value`: [`sign_extend`], [`scale_value`], [`ValueCaps`]
//! - `MulDiv` (`dlls/kernelbase/main.c`), which `scale_value` calls: [`mul_div`]
//! - `check_value_caps`: [`DeviceCaps::check_value_caps`]
//! - The arithmetic of `translate_report_to_xinput_state` (hat and button bits, axis and
//!   trigger scaling) on decoded report values: [`translate_report_to_xinput_state`]
//!
//! Skipped:
//! - `HidP_GetUsages` and `HidP_GetUsageValue`: report decoding. The caller supplies the values.
//! - IRP and PnP handling: `read_completion`, `try_complete_pending_read`,
//!   `gamepad_internal_ioctl`, `internal_ioctl`, `pdo_pnp`, `fdo_pnp`, `driver_pnp`,
//!   `create_child_pdos`, `initialize_device`, `add_device`, `sync_ioctl`, `DriverEntry`.
//! - Device ID queries: `query_instance_id`, `query_device_id`, `query_hardware_ids`,
//!   `query_compatible_ids`, `get_device_id`.
//! - `remove_pending_irps`, `set_event_completion`, `driver_unload`: IRP queues and events.
//! - `xinput_report_desc`: a constant HID report descriptor built by the push/pop macros.

mod report;
mod scale;

pub use report::{translate_report_to_xinput_state, DeviceCaps, Report, XinputState};
pub use scale::{mul_div, scale_value, sign_extend, ValueCaps};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
