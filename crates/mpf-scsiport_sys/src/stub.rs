/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = scsiport.sys` in `dlls/scsiport.sys/Makefile.in`.
const SCSIPORT_SYS_NAME: &str = "scsiport.sys";

/// The record that `__wine_spec_unimplemented_stub` raises.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`. The C function raises
/// the record in a loop and never returns, so the port returns the record instead.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        // The module and the function name.
        number_parameters: 2,
        module,
        function,
    }
}

/// `DllInitialize`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn dll_initialize() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "DllInitialize")
}

/// `PortNotification`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn port_notification() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "PortNotification")
}

/// `ScsiDebugPrint`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_debug_print() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiDebugPrint")
}

/// `ScsiPortCompleteRequest`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_complete_request() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortCompleteRequest")
}

/// `ScsiPortConvertPhysicalAddressToUlong`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_convert_physical_address_to_ulong() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortConvertPhysicalAddressToUlong")
}

/// `ScsiPortConvertUlongToPhysicalAddress`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_convert_ulong_to_physical_address() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortConvertUlongToPhysicalAddress")
}

/// `ScsiPortFlushDma`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_flush_dma() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortFlushDma")
}

/// `ScsiPortFreeDeviceBase`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_free_device_base() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortFreeDeviceBase")
}

/// `ScsiPortGetBusData`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_get_bus_data() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortGetBusData")
}

/// `ScsiPortGetDeviceBase`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_get_device_base() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortGetDeviceBase")
}

/// `ScsiPortGetLogicalUnit`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_get_logical_unit() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortGetLogicalUnit")
}

/// `ScsiPortGetPhysicalAddress`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_get_physical_address() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortGetPhysicalAddress")
}

/// `ScsiPortGetSrb`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_get_srb() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortGetSrb")
}

/// `ScsiPortGetUncachedExtension`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_get_uncached_extension() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortGetUncachedExtension")
}

/// `ScsiPortGetVirtualAddress`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_get_virtual_address() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortGetVirtualAddress")
}

/// `ScsiPortInitialize`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_initialize() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortInitialize")
}

/// `ScsiPortIoMapTransfer`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_io_map_transfer() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortIoMapTransfer")
}

/// `ScsiPortLogError`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_log_error() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortLogError")
}

/// `ScsiPortMoveMemory`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_move_memory() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortMoveMemory")
}

/// `ScsiPortNotification`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_notification() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortNotification")
}

/// `ScsiPortQuerySystemTime`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_query_system_time() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortQuerySystemTime")
}

/// `ScsiPortReadPortBufferUchar`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_read_port_buffer_uchar() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortReadPortBufferUchar")
}

/// `ScsiPortReadPortBufferUlong`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_read_port_buffer_ulong() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortReadPortBufferUlong")
}

/// `ScsiPortReadPortBufferUshort`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_read_port_buffer_ushort() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortReadPortBufferUshort")
}

/// `ScsiPortReadPortUchar`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_read_port_uchar() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortReadPortUchar")
}

/// `ScsiPortReadPortUlong`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_read_port_ulong() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortReadPortUlong")
}

/// `ScsiPortReadPortUshort`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_read_port_ushort() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortReadPortUshort")
}

/// `ScsiPortReadRegisterBufferUchar`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_read_register_buffer_uchar() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortReadRegisterBufferUchar")
}

/// `ScsiPortReadRegisterBufferUlong`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_read_register_buffer_ulong() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortReadRegisterBufferUlong")
}

/// `ScsiPortReadRegisterBufferUshort`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_read_register_buffer_ushort() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortReadRegisterBufferUshort")
}

/// `ScsiPortReadRegisterUchar`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_read_register_uchar() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortReadRegisterUchar")
}

/// `ScsiPortReadRegisterUlong`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_read_register_ulong() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortReadRegisterUlong")
}

/// `ScsiPortReadRegisterUshort`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_read_register_ushort() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortReadRegisterUshort")
}

/// `ScsiPortSetBusDataByOffset`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_set_bus_data_by_offset() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortSetBusDataByOffset")
}

/// `ScsiPortStallExecution`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_stall_execution() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortStallExecution")
}

/// `ScsiPortValidateRange`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_validate_range() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortValidateRange")
}

/// `ScsiPortWritePortBufferUchar`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_write_port_buffer_uchar() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortWritePortBufferUchar")
}

/// `ScsiPortWritePortBufferUlong`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_write_port_buffer_ulong() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortWritePortBufferUlong")
}

/// `ScsiPortWritePortBufferUshort`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_write_port_buffer_ushort() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortWritePortBufferUshort")
}

/// `ScsiPortWritePortUchar`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_write_port_uchar() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortWritePortUchar")
}

/// `ScsiPortWritePortUlong`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_write_port_ulong() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortWritePortUlong")
}

/// `ScsiPortWritePortUshort`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_write_port_ushort() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortWritePortUshort")
}

/// `ScsiPortWriteRegisterBufferUchar`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_write_register_buffer_uchar() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortWriteRegisterBufferUchar")
}

/// `ScsiPortWriteRegisterBufferUlong`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_write_register_buffer_ulong() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortWriteRegisterBufferUlong")
}

/// `ScsiPortWriteRegisterBufferUshort`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_write_register_buffer_ushort() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortWriteRegisterBufferUshort")
}

/// `ScsiPortWriteRegisterUchar`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_write_register_uchar() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortWriteRegisterUchar")
}

/// `ScsiPortWriteRegisterUlong`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_write_register_ulong() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortWriteRegisterUlong")
}

/// `ScsiPortWriteRegisterUshort`, exported as `@ stub` in `dlls/scsiport.sys/scsiport.sys.spec`.
pub fn scsi_port_write_register_ushort() -> StubException {
    wine_spec_unimplemented_stub(SCSIPORT_SYS_NAME, "ScsiPortWriteRegisterUshort")
}
