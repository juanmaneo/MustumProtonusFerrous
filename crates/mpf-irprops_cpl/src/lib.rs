//! Port of the portable logic in Wine's `dlls/irprops.cpl/irprops.cpl.spec` and
//! `dlls/irprops.cpl/Makefile.in`. The reference tree has no C source for this DLL, so the port
//! covers the export table and the `@ stub` records.
//!
//! Ported:
//! - `irprops.cpl.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The 33 `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per export, from
//!   [`bluetooth_authenticate_device`] to [`bluetooth_update_device_record`]
//!
//! Skipped:
//! - The 13 `@ stdcall -import` exports, which jump through `__imp_` to `bluetoothapis`, and the
//!   4 `@ stdcall` forwarders to `bthprops.cpl`. Their bodies live in other DLLs, so they are
//!   listed in [`EXPORTS`] and not modelled.
//! - `BluetoothGetDeviceInfo`, the one `@ stdcall` export with no target. `dlls/irprops.cpl` has
//!   no body for it, so it is listed as [`ExportKind::StdcallNoBody`] and not modelled.
//! - `#@ stub CPlApplet`: commented out in the spec, so the DLL does not export it.
//! - The stdcall argument lists (`ptr`, `long`), and the ordinal form of the stub record, since
//!   every export in this spec has a name.
//! - The `__cdecl` calling convention.

mod exports;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    bluetooth_authenticate_device, bluetooth_authenticate_multiple_devices,
    bluetooth_authentication_agent, bluetooth_disconnect_device,
    bluetooth_display_device_properties, bluetooth_enumerate_installed_services,
    bluetooth_find_browse_group_close, bluetooth_find_class_id_close,
    bluetooth_find_first_browse_group, bluetooth_find_first_class_id,
    bluetooth_find_first_profile_descriptor, bluetooth_find_first_protocol_descriptor_stack,
    bluetooth_find_first_protocol_entry, bluetooth_find_first_service,
    bluetooth_find_next_browse_group, bluetooth_find_next_class_id,
    bluetooth_find_next_profile_descriptor, bluetooth_find_next_protocol_descriptor_stack,
    bluetooth_find_next_protocol_entry, bluetooth_find_next_service,
    bluetooth_find_profile_descriptor_close, bluetooth_find_protocol_descriptor_stack_close,
    bluetooth_find_protocol_entry_close, bluetooth_find_service_close,
    bluetooth_map_class_of_device_to_image_index, bluetooth_map_class_of_device_to_string,
    bluetooth_register_for_authentication, bluetooth_sdp_get_string, bluetooth_select_devices,
    bluetooth_select_devices_free, bluetooth_send_authentication_response,
    bluetooth_set_service_state, bluetooth_update_device_record, wine_spec_unimplemented_stub,
    StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
