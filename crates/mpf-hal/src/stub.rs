/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = hal.dll` in `dlls/hal/Makefile.in`.
const HAL_DLL_NAME: &str = "hal.dll";

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

/// `HalClearSoftwareInterrupt`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_clear_software_interrupt() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalClearSoftwareInterrupt")
}

/// `HalRequestSoftwareInterrupt`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_request_software_interrupt() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalRequestSoftwareInterrupt")
}

/// `HalSystemVectorDispatchEntry`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_system_vector_dispatch_entry() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalSystemVectorDispatchEntry")
}

/// `KeAcquireInStackQueuedSpinLockRaiseToSynch`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn ke_acquire_in_stack_queued_spin_lock_raise_to_synch() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "KeAcquireInStackQueuedSpinLockRaiseToSynch")
}

/// `KeAcquireQueuedSpinLock`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn ke_acquire_queued_spin_lock() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "KeAcquireQueuedSpinLock")
}

/// `KeAcquireQueuedSpinLockRaiseToSynch`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn ke_acquire_queued_spin_lock_raise_to_synch() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "KeAcquireQueuedSpinLockRaiseToSynch")
}

/// `KeAcquireSpinLockRaiseToSynch`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn ke_acquire_spin_lock_raise_to_synch() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "KeAcquireSpinLockRaiseToSynch")
}

/// `KeReleaseQueuedSpinLock`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn ke_release_queued_spin_lock() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "KeReleaseQueuedSpinLock")
}

/// `KeTryToAcquireQueuedSpinLock`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn ke_try_to_acquire_queued_spin_lock() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "KeTryToAcquireQueuedSpinLock")
}

/// `KeTryToAcquireQueuedSpinLockRaiseToSynch`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn ke_try_to_acquire_queued_spin_lock_raise_to_synch() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "KeTryToAcquireQueuedSpinLockRaiseToSynch")
}

/// `HalAcquireDisplayOwnership`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_acquire_display_ownership() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalAcquireDisplayOwnership")
}

/// `HalAdjustResourceList`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_adjust_resource_list() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalAdjustResourceList")
}

/// `HalAllProcessorsStarted`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_all_processors_started() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalAllProcessorsStarted")
}

/// `HalAllocateAdapterChannel`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_allocate_adapter_channel() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalAllocateAdapterChannel")
}

/// `HalAllocateCommonBuffer`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_allocate_common_buffer() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalAllocateCommonBuffer")
}

/// `HalAllocateCrashDumpRegisters`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_allocate_crash_dump_registers() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalAllocateCrashDumpRegisters")
}

/// `HalAssignSlotResources`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_assign_slot_resources() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalAssignSlotResources")
}

/// `HalBeginSystemInterrupt`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_begin_system_interrupt() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalBeginSystemInterrupt")
}

/// `HalCalibratePerformanceCounter`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_calibrate_performance_counter() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalCalibratePerformanceCounter")
}

/// `HalDisableSystemInterrupt`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_disable_system_interrupt() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalDisableSystemInterrupt")
}

/// `HalDisplayString`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_display_string() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalDisplayString")
}

/// `HalEnableSystemInterrupt`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_enable_system_interrupt() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalEnableSystemInterrupt")
}

/// `HalEndSystemInterrupt`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_end_system_interrupt() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalEndSystemInterrupt")
}

/// `HalFlushCommonBuffer`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_flush_common_buffer() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalFlushCommonBuffer")
}

/// `HalFreeCommonBuffer`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_free_common_buffer() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalFreeCommonBuffer")
}

/// `HalGetAdapter`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_get_adapter() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalGetAdapter")
}

/// `HalGetEnvironmentVariable`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_get_environment_variable() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalGetEnvironmentVariable")
}

/// `HalGetInterruptVector`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_get_interrupt_vector() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalGetInterruptVector")
}

/// `HalHandleNMI`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_handle_nmi() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalHandleNMI")
}

/// `HalInitSystem`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_init_system() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalInitSystem")
}

/// `HalInitializeProcessor`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_initialize_processor() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalInitializeProcessor")
}

/// `HalMakeBeep`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_make_beep() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalMakeBeep")
}

/// `HalProcessorIdle`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_processor_idle() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalProcessorIdle")
}

/// `HalQueryDisplayParameters`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_query_display_parameters() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalQueryDisplayParameters")
}

/// `HalQueryRealTimeClock`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_query_real_time_clock() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalQueryRealTimeClock")
}

/// `HalReadDmaCounter`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_read_dma_counter() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalReadDmaCounter")
}

/// `HalReportResourceUsage`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_report_resource_usage() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalReportResourceUsage")
}

/// `HalRequestIpi`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_request_ipi() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalRequestIpi")
}

/// `HalReturnToFirmware`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_return_to_firmware() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalReturnToFirmware")
}

/// `HalSetBusData`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_set_bus_data() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalSetBusData")
}

/// `HalSetBusDataByOffset`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_set_bus_data_by_offset() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalSetBusDataByOffset")
}

/// `HalSetDisplayParameters`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_set_display_parameters() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalSetDisplayParameters")
}

/// `HalSetEnvironmentVariable`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_set_environment_variable() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalSetEnvironmentVariable")
}

/// `HalSetProfileInterval`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_set_profile_interval() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalSetProfileInterval")
}

/// `HalSetRealTimeClock`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_set_real_time_clock() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalSetRealTimeClock")
}

/// `HalSetTimeIncrement`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_set_time_increment() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalSetTimeIncrement")
}

/// `HalStartNextProcessor`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_start_next_processor() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalStartNextProcessor")
}

/// `HalStartProfileInterrupt`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_start_profile_interrupt() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalStartProfileInterrupt")
}

/// `HalStopProfileInterrupt`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn hal_stop_profile_interrupt() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "HalStopProfileInterrupt")
}

/// `IoAssignDriveLetters`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn io_assign_drive_letters() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "IoAssignDriveLetters")
}

/// `IoFlushAdapterBuffers`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn io_flush_adapter_buffers() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "IoFlushAdapterBuffers")
}

/// `IoFreeAdapterChannel`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn io_free_adapter_channel() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "IoFreeAdapterChannel")
}

/// `IoFreeMapRegisters`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn io_free_map_registers() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "IoFreeMapRegisters")
}

/// `IoMapTransfer`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn io_map_transfer() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "IoMapTransfer")
}

/// `IoReadPartitionTable`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn io_read_partition_table() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "IoReadPartitionTable")
}

/// `IoSetPartitionInformation`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn io_set_partition_information() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "IoSetPartitionInformation")
}

/// `IoWritePartitionTable`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn io_write_partition_table() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "IoWritePartitionTable")
}

/// `KdComPortInUse`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn kd_com_port_in_use() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "KdComPortInUse")
}

/// `KeFlushWriteBuffer`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn ke_flush_write_buffer() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "KeFlushWriteBuffer")
}

/// `KeRaiseIrql`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn ke_raise_irql() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "KeRaiseIrql")
}

/// `KeRaiseIrqlToDpcLevel`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn ke_raise_irql_to_dpc_level() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "KeRaiseIrqlToDpcLevel")
}

/// `KeRaiseIrqlToSynchLevel`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn ke_raise_irql_to_synch_level() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "KeRaiseIrqlToSynchLevel")
}

/// `KeStallExecutionProcessor`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn ke_stall_execution_processor() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "KeStallExecutionProcessor")
}

/// `READ_PORT_BUFFER_UCHAR`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn read_port_buffer_uchar() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "READ_PORT_BUFFER_UCHAR")
}

/// `READ_PORT_BUFFER_ULONG`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn read_port_buffer_ulong() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "READ_PORT_BUFFER_ULONG")
}

/// `READ_PORT_BUFFER_USHORT`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn read_port_buffer_ushort() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "READ_PORT_BUFFER_USHORT")
}

/// `READ_PORT_USHORT`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn read_port_ushort() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "READ_PORT_USHORT")
}

/// `WRITE_PORT_BUFFER_UCHAR`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn write_port_buffer_uchar() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "WRITE_PORT_BUFFER_UCHAR")
}

/// `WRITE_PORT_BUFFER_ULONG`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn write_port_buffer_ulong() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "WRITE_PORT_BUFFER_ULONG")
}

/// `WRITE_PORT_BUFFER_USHORT`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn write_port_buffer_ushort() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "WRITE_PORT_BUFFER_USHORT")
}

/// `WRITE_PORT_USHORT`, exported as `@ stub` in `dlls/hal/hal.spec`.
pub fn write_port_ushort() -> StubException {
    wine_spec_unimplemented_stub(HAL_DLL_NAME, "WRITE_PORT_USHORT")
}
