//! Port of the portable logic in Wine's `dlls/scsiport.sys/scsiport.c` and
//! `dlls/scsiport.sys/scsiport.sys.spec`.
//!
//! Ported:
//! - `DriverEntry`: the return value, [`driver_entry`], which returns [`STATUS_SUCCESS`]
//! - `scsiport.sys.spec` export table: [`EXPORTS`], [`export_stub`]
//! - The 48 `@ stub` exports, each modelled as the record `__wine_spec_unimplemented_stub`
//!   raises: [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per export
//!   (the snake_case of its name, for example [`scsi_port_flush_dma`])
//!
//! Skipped:
//! - The `TRACE` line and the `path` argument. The C body reads `path->Buffer` only inside the
//!   `TRACE` arguments, which are not evaluated unless the `scsiport` trace channel is on.
//! - The `DRIVER_OBJECT` and `UNICODE_STRING` arguments: [`driver_entry`] takes none.
//! - `ExceptionRecord` and `ExceptionAddress` of the `EXCEPTION_RECORD`, which the stub record
//!   does not model.
//! - Ordinal-only exports: the spec has none, and [`wine_spec_unimplemented_stub`] takes a name.
//! - The `__cdecl` and `WINAPI` calling conventions.

mod driver;
mod exports;
mod stub;

pub use driver::{driver_entry, NtStatus, STATUS_SUCCESS};
pub use exports::{export_stub, EXPORTS};
pub use stub::{
    dll_initialize, port_notification, scsi_debug_print, scsi_port_complete_request,
    scsi_port_convert_physical_address_to_ulong, scsi_port_convert_ulong_to_physical_address,
    scsi_port_flush_dma, scsi_port_free_device_base, scsi_port_get_bus_data,
    scsi_port_get_device_base, scsi_port_get_logical_unit, scsi_port_get_physical_address,
    scsi_port_get_srb, scsi_port_get_uncached_extension, scsi_port_get_virtual_address,
    scsi_port_initialize, scsi_port_io_map_transfer, scsi_port_log_error, scsi_port_move_memory,
    scsi_port_notification, scsi_port_query_system_time, scsi_port_read_port_buffer_uchar,
    scsi_port_read_port_buffer_ulong, scsi_port_read_port_buffer_ushort, scsi_port_read_port_uchar,
    scsi_port_read_port_ulong, scsi_port_read_port_ushort, scsi_port_read_register_buffer_uchar,
    scsi_port_read_register_buffer_ulong, scsi_port_read_register_buffer_ushort,
    scsi_port_read_register_uchar, scsi_port_read_register_ulong, scsi_port_read_register_ushort,
    scsi_port_set_bus_data_by_offset, scsi_port_stall_execution, scsi_port_validate_range,
    scsi_port_write_port_buffer_uchar, scsi_port_write_port_buffer_ulong,
    scsi_port_write_port_buffer_ushort, scsi_port_write_port_uchar, scsi_port_write_port_ulong,
    scsi_port_write_port_ushort, scsi_port_write_register_buffer_uchar,
    scsi_port_write_register_buffer_ulong, scsi_port_write_register_buffer_ushort,
    scsi_port_write_register_uchar, scsi_port_write_register_ulong,
    scsi_port_write_register_ushort, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
