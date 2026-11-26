use crate::stub::*;

/// A helper that builds the record one `@ stub` export raises.
pub type StubHelper = fn() -> StubException;

/// The `scsiport.sys.spec` export table in spec order. Every export is `@ stub`, paired with
/// the helper that builds the record its stub raises.
pub const EXPORTS: [(&str, StubHelper); 48] = [
    ("DllInitialize", dll_initialize),
    ("PortNotification", port_notification),
    ("ScsiDebugPrint", scsi_debug_print),
    ("ScsiPortCompleteRequest", scsi_port_complete_request),
    (
        "ScsiPortConvertPhysicalAddressToUlong",
        scsi_port_convert_physical_address_to_ulong,
    ),
    (
        "ScsiPortConvertUlongToPhysicalAddress",
        scsi_port_convert_ulong_to_physical_address,
    ),
    ("ScsiPortFlushDma", scsi_port_flush_dma),
    ("ScsiPortFreeDeviceBase", scsi_port_free_device_base),
    ("ScsiPortGetBusData", scsi_port_get_bus_data),
    ("ScsiPortGetDeviceBase", scsi_port_get_device_base),
    ("ScsiPortGetLogicalUnit", scsi_port_get_logical_unit),
    ("ScsiPortGetPhysicalAddress", scsi_port_get_physical_address),
    ("ScsiPortGetSrb", scsi_port_get_srb),
    (
        "ScsiPortGetUncachedExtension",
        scsi_port_get_uncached_extension,
    ),
    ("ScsiPortGetVirtualAddress", scsi_port_get_virtual_address),
    ("ScsiPortInitialize", scsi_port_initialize),
    ("ScsiPortIoMapTransfer", scsi_port_io_map_transfer),
    ("ScsiPortLogError", scsi_port_log_error),
    ("ScsiPortMoveMemory", scsi_port_move_memory),
    ("ScsiPortNotification", scsi_port_notification),
    ("ScsiPortQuerySystemTime", scsi_port_query_system_time),
    (
        "ScsiPortReadPortBufferUchar",
        scsi_port_read_port_buffer_uchar,
    ),
    (
        "ScsiPortReadPortBufferUlong",
        scsi_port_read_port_buffer_ulong,
    ),
    (
        "ScsiPortReadPortBufferUshort",
        scsi_port_read_port_buffer_ushort,
    ),
    ("ScsiPortReadPortUchar", scsi_port_read_port_uchar),
    ("ScsiPortReadPortUlong", scsi_port_read_port_ulong),
    ("ScsiPortReadPortUshort", scsi_port_read_port_ushort),
    (
        "ScsiPortReadRegisterBufferUchar",
        scsi_port_read_register_buffer_uchar,
    ),
    (
        "ScsiPortReadRegisterBufferUlong",
        scsi_port_read_register_buffer_ulong,
    ),
    (
        "ScsiPortReadRegisterBufferUshort",
        scsi_port_read_register_buffer_ushort,
    ),
    ("ScsiPortReadRegisterUchar", scsi_port_read_register_uchar),
    ("ScsiPortReadRegisterUlong", scsi_port_read_register_ulong),
    ("ScsiPortReadRegisterUshort", scsi_port_read_register_ushort),
    (
        "ScsiPortSetBusDataByOffset",
        scsi_port_set_bus_data_by_offset,
    ),
    ("ScsiPortStallExecution", scsi_port_stall_execution),
    ("ScsiPortValidateRange", scsi_port_validate_range),
    (
        "ScsiPortWritePortBufferUchar",
        scsi_port_write_port_buffer_uchar,
    ),
    (
        "ScsiPortWritePortBufferUlong",
        scsi_port_write_port_buffer_ulong,
    ),
    (
        "ScsiPortWritePortBufferUshort",
        scsi_port_write_port_buffer_ushort,
    ),
    ("ScsiPortWritePortUchar", scsi_port_write_port_uchar),
    ("ScsiPortWritePortUlong", scsi_port_write_port_ulong),
    ("ScsiPortWritePortUshort", scsi_port_write_port_ushort),
    (
        "ScsiPortWriteRegisterBufferUchar",
        scsi_port_write_register_buffer_uchar,
    ),
    (
        "ScsiPortWriteRegisterBufferUlong",
        scsi_port_write_register_buffer_ulong,
    ),
    (
        "ScsiPortWriteRegisterBufferUshort",
        scsi_port_write_register_buffer_ushort,
    ),
    ("ScsiPortWriteRegisterUchar", scsi_port_write_register_uchar),
    ("ScsiPortWriteRegisterUlong", scsi_port_write_register_ulong),
    (
        "ScsiPortWriteRegisterUshort",
        scsi_port_write_register_ushort,
    ),
];

/// The record the stub for `name` raises, or `None` when the spec does not export `name`.
/// Names are case-sensitive.
pub fn export_stub(name: &str) -> Option<StubException> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, stub)| stub())
}
