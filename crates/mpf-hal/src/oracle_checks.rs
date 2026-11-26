//! Oracle-derived test vectors for the ported hal logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of the ported functions in
//! `dlls/hal/hal.c`, and `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c` (with its
//! loop reduced to one call), under `target/canon/hal/` with `gcc -fsanitize=address,undefined -g`.
//! The harness printed 89 rows: 18 for the explicit functions (rows 001-018) and 71 for the
//! `@ stub` exports (rows 019-089). ASan and UBSan flagged no row, so none is excluded.

use super::*;

#[test]
fn oracle_ex_try_to_acquire_fast_mutex_rows() {
    // row 001 ExTryToAcquireFastMutex count_in=1 -> ret=1 count_out=0
    let mut mutex = FastMutex { count: 1 };
    assert!(ex_try_to_acquire_fast_mutex(&mut mutex));
    assert_eq!(mutex.count, 0);

    // row 002 ExTryToAcquireFastMutex count_in=0 -> ret=0 count_out=0
    let mut mutex = FastMutex { count: 0 };
    assert!(!ex_try_to_acquire_fast_mutex(&mut mutex));
    assert_eq!(mutex.count, 0);

    // row 003 ExTryToAcquireFastMutex count_in=2 -> ret=0 count_out=2
    let mut mutex = FastMutex { count: 2 };
    assert!(!ex_try_to_acquire_fast_mutex(&mut mutex));
    assert_eq!(mutex.count, 2);

    // row 004 ExTryToAcquireFastMutex count_in=-1 -> ret=0 count_out=-1
    let mut mutex = FastMutex { count: -1 };
    assert!(!ex_try_to_acquire_fast_mutex(&mut mutex));
    assert_eq!(mutex.count, -1);

    // row 005 ExTryToAcquireFastMutex count_in=2147483647 -> ret=0 count_out=2147483647
    let mut mutex = FastMutex { count: i32::MAX };
    assert!(!ex_try_to_acquire_fast_mutex(&mut mutex));
    assert_eq!(mutex.count, i32::MAX);
}

#[test]
fn oracle_spin_lock_rows() {
    // row 006 KeAcquireSpinLock irql_in=255 -> irql_out=0
    let mut irql = 255;
    ke_acquire_spin_lock(&mut irql);
    assert_eq!(irql, 0);

    // row 007 KeAcquireSpinLock irql_in=31 -> irql_out=0
    let mut irql = 31;
    ke_acquire_spin_lock(&mut irql);
    assert_eq!(irql, 0);

    // row 008 KfAcquireSpinLock -> ret=0
    assert_eq!(kf_acquire_spin_lock(), 0);
}

#[test]
fn oracle_irql_rows() {
    // row 009 KfLowerIrql new_irql=2 -> void
    kf_lower_irql(2);

    // row 010 KfRaiseIrql new_irql=2 -> ret=0
    assert_eq!(kf_raise_irql(2), 0);

    // row 011 KeGetCurrentIrql -> ret=0
    assert_eq!(ke_get_current_irql(), 0);
}

#[test]
fn oracle_port_rows() {
    // row 012 READ_PORT_UCHAR cell_in=90 -> ret=0 cell_out=90
    let mut cell: u8 = 90;
    assert_eq!(read_port_uchar(&mut cell), 0);
    assert_eq!(cell, 90);

    // row 013 READ_PORT_ULONG cell_in=0xdeadbeef -> ret=0 cell_out=0xdeadbeef
    let mut cell: u32 = 0xdead_beef;
    assert_eq!(read_port_ulong(&mut cell), 0);
    assert_eq!(cell, 0xdead_beef);

    // row 014 WRITE_PORT_UCHAR cell_in=90 value=51 -> void cell_out=90
    let mut cell: u8 = 90;
    write_port_uchar(&mut cell, 51);
    assert_eq!(cell, 90);

    // row 015 WRITE_PORT_ULONG cell_in=0xdeadbeef value=0x12345678 -> void cell_out=0xdeadbeef
    let mut cell: u32 = 0xdead_beef;
    write_port_ulong(&mut cell, 0x1234_5678);
    assert_eq!(cell, 0xdead_beef);
}

#[test]
fn oracle_bus_rows() {
    let expected = [1u8, 2, 3, 4];

    // row 016 HalGetBusData bus_type=1 bus=0 slot=0 length=4 -> ret=0 buf_changed=0
    let mut buf = expected;
    assert_eq!(hal_get_bus_data(1, 0, 0, buf.as_mut_ptr().cast(), 4), 0);
    assert_eq!(buf, expected);

    // row 017 HalGetBusDataByOffset bus_type=1 bus=0 slot=0 offset=0 length=4 -> ret=0 buf_changed=0
    let mut buf = expected;
    assert_eq!(
        hal_get_bus_data_by_offset(1, 0, 0, buf.as_mut_ptr().cast(), 0, 4),
        0
    );
    assert_eq!(buf, expected);

    // row 018 HalTranslateBusAddress iface=1 bus=0 address=0x1000 -> ret=0 space_out=7 translated_out=9
    let mut space: u32 = 7;
    let mut translated: i64 = 9;
    assert!(!hal_translate_bus_address(
        1,
        0,
        0x1000,
        &mut space,
        &mut translated
    ));
    assert_eq!(space, 7);
    assert_eq!(translated, 9);
}

/// The `@ stub` rows, one per export in `hal.spec` order. Each row is the helper, then the
/// `StubException` fields the harness printed for `__wine_spec_unimplemented_stub("hal.dll", name)`.
type StubRow = (
    fn() -> StubException,
    u32,
    u32,
    u32,
    &'static str,
    &'static str,
);

const STUB_ROWS: [StubRow; 71] = [
    (
        hal_clear_software_interrupt,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalClearSoftwareInterrupt",
    ), // row 019
    (
        hal_request_software_interrupt,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalRequestSoftwareInterrupt",
    ), // row 020
    (
        hal_system_vector_dispatch_entry,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalSystemVectorDispatchEntry",
    ), // row 021
    (
        ke_acquire_in_stack_queued_spin_lock_raise_to_synch,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "KeAcquireInStackQueuedSpinLockRaiseToSynch",
    ), // row 022
    (
        ke_acquire_queued_spin_lock,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "KeAcquireQueuedSpinLock",
    ), // row 023
    (
        ke_acquire_queued_spin_lock_raise_to_synch,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "KeAcquireQueuedSpinLockRaiseToSynch",
    ), // row 024
    (
        ke_acquire_spin_lock_raise_to_synch,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "KeAcquireSpinLockRaiseToSynch",
    ), // row 025
    (
        ke_release_queued_spin_lock,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "KeReleaseQueuedSpinLock",
    ), // row 026
    (
        ke_try_to_acquire_queued_spin_lock,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "KeTryToAcquireQueuedSpinLock",
    ), // row 027
    (
        ke_try_to_acquire_queued_spin_lock_raise_to_synch,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "KeTryToAcquireQueuedSpinLockRaiseToSynch",
    ), // row 028
    (
        hal_acquire_display_ownership,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalAcquireDisplayOwnership",
    ), // row 029
    (
        hal_adjust_resource_list,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalAdjustResourceList",
    ), // row 030
    (
        hal_all_processors_started,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalAllProcessorsStarted",
    ), // row 031
    (
        hal_allocate_adapter_channel,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalAllocateAdapterChannel",
    ), // row 032
    (
        hal_allocate_common_buffer,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalAllocateCommonBuffer",
    ), // row 033
    (
        hal_allocate_crash_dump_registers,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalAllocateCrashDumpRegisters",
    ), // row 034
    (
        hal_assign_slot_resources,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalAssignSlotResources",
    ), // row 035
    (
        hal_begin_system_interrupt,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalBeginSystemInterrupt",
    ), // row 036
    (
        hal_calibrate_performance_counter,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalCalibratePerformanceCounter",
    ), // row 037
    (
        hal_disable_system_interrupt,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalDisableSystemInterrupt",
    ), // row 038
    (
        hal_display_string,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalDisplayString",
    ), // row 039
    (
        hal_enable_system_interrupt,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalEnableSystemInterrupt",
    ), // row 040
    (
        hal_end_system_interrupt,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalEndSystemInterrupt",
    ), // row 041
    (
        hal_flush_common_buffer,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalFlushCommonBuffer",
    ), // row 042
    (
        hal_free_common_buffer,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalFreeCommonBuffer",
    ), // row 043
    (
        hal_get_adapter,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalGetAdapter",
    ), // row 044
    (
        hal_get_environment_variable,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalGetEnvironmentVariable",
    ), // row 045
    (
        hal_get_interrupt_vector,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalGetInterruptVector",
    ), // row 046
    (
        hal_handle_nmi,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalHandleNMI",
    ), // row 047
    (
        hal_init_system,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalInitSystem",
    ), // row 048
    (
        hal_initialize_processor,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalInitializeProcessor",
    ), // row 049
    (
        hal_make_beep,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalMakeBeep",
    ), // row 050
    (
        hal_processor_idle,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalProcessorIdle",
    ), // row 051
    (
        hal_query_display_parameters,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalQueryDisplayParameters",
    ), // row 052
    (
        hal_query_real_time_clock,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalQueryRealTimeClock",
    ), // row 053
    (
        hal_read_dma_counter,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalReadDmaCounter",
    ), // row 054
    (
        hal_report_resource_usage,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalReportResourceUsage",
    ), // row 055
    (
        hal_request_ipi,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalRequestIpi",
    ), // row 056
    (
        hal_return_to_firmware,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalReturnToFirmware",
    ), // row 057
    (
        hal_set_bus_data,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalSetBusData",
    ), // row 058
    (
        hal_set_bus_data_by_offset,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalSetBusDataByOffset",
    ), // row 059
    (
        hal_set_display_parameters,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalSetDisplayParameters",
    ), // row 060
    (
        hal_set_environment_variable,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalSetEnvironmentVariable",
    ), // row 061
    (
        hal_set_profile_interval,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalSetProfileInterval",
    ), // row 062
    (
        hal_set_real_time_clock,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalSetRealTimeClock",
    ), // row 063
    (
        hal_set_time_increment,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalSetTimeIncrement",
    ), // row 064
    (
        hal_start_next_processor,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalStartNextProcessor",
    ), // row 065
    (
        hal_start_profile_interrupt,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalStartProfileInterrupt",
    ), // row 066
    (
        hal_stop_profile_interrupt,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "HalStopProfileInterrupt",
    ), // row 067
    (
        io_assign_drive_letters,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "IoAssignDriveLetters",
    ), // row 068
    (
        io_flush_adapter_buffers,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "IoFlushAdapterBuffers",
    ), // row 069
    (
        io_free_adapter_channel,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "IoFreeAdapterChannel",
    ), // row 070
    (
        io_free_map_registers,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "IoFreeMapRegisters",
    ), // row 071
    (
        io_map_transfer,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "IoMapTransfer",
    ), // row 072
    (
        io_read_partition_table,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "IoReadPartitionTable",
    ), // row 073
    (
        io_set_partition_information,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "IoSetPartitionInformation",
    ), // row 074
    (
        io_write_partition_table,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "IoWritePartitionTable",
    ), // row 075
    (
        kd_com_port_in_use,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "KdComPortInUse",
    ), // row 076
    (
        ke_flush_write_buffer,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "KeFlushWriteBuffer",
    ), // row 077
    (
        ke_raise_irql,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "KeRaiseIrql",
    ), // row 078
    (
        ke_raise_irql_to_dpc_level,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "KeRaiseIrqlToDpcLevel",
    ), // row 079
    (
        ke_raise_irql_to_synch_level,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "KeRaiseIrqlToSynchLevel",
    ), // row 080
    (
        ke_stall_execution_processor,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "KeStallExecutionProcessor",
    ), // row 081
    (
        read_port_buffer_uchar,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "READ_PORT_BUFFER_UCHAR",
    ), // row 082
    (
        read_port_buffer_ulong,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "READ_PORT_BUFFER_ULONG",
    ), // row 083
    (
        read_port_buffer_ushort,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "READ_PORT_BUFFER_USHORT",
    ), // row 084
    (
        read_port_ushort,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "READ_PORT_USHORT",
    ), // row 085
    (
        write_port_buffer_uchar,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "WRITE_PORT_BUFFER_UCHAR",
    ), // row 086
    (
        write_port_buffer_ulong,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "WRITE_PORT_BUFFER_ULONG",
    ), // row 087
    (
        write_port_buffer_ushort,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "WRITE_PORT_BUFFER_USHORT",
    ), // row 088
    (
        write_port_ushort,
        0x8000_0100,
        0x01,
        2,
        "hal.dll",
        "WRITE_PORT_USHORT",
    ), // row 089
];

#[test]
fn oracle_stub_rows() {
    for (helper, code, flags, number_parameters, module, function) in STUB_ROWS {
        assert_eq!(
            helper(),
            StubException {
                code,
                flags,
                number_parameters,
                module,
                function,
            }
        );
        assert_eq!(export_kind(function), Some(ExportKind::Stub));
    }
}
