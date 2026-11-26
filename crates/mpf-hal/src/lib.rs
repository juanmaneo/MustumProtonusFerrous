//! Port of the portable logic in Wine's `dlls/hal/hal.c` and `dlls/hal/hal.spec`.
//!
//! Ported:
//! - `ExTryToAcquireFastMutex` count transition: [`ex_try_to_acquire_fast_mutex`], [`FastMutex`]
//! - `KeAcquireSpinLock` and `KfAcquireSpinLock` IRQL output: [`ke_acquire_spin_lock`],
//!   [`kf_acquire_spin_lock`]
//! - IRQL functions that return a fixed value or do nothing: [`kf_lower_irql`], [`kf_raise_irql`],
//!   [`ke_get_current_irql`]
//! - Port I/O functions that return a fixed value or do nothing: [`read_port_uchar`],
//!   [`read_port_ulong`], [`write_port_uchar`], [`write_port_ulong`]
//! - Bus functions that claim there is no bus: [`hal_get_bus_data`],
//!   [`hal_get_bus_data_by_offset`], [`hal_translate_bus_address`]
//! - `hal.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - `@ stub` exports: [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per
//!   stub, named after the export in snake_case, such as [`hal_clear_software_interrupt`]
//!
//! Skipped:
//! - `ExAcquireFastMutex`, `ExReleaseFastMutex`, `KeAcquireInStackQueuedSpinLock`,
//!   `KeReleaseInStackQueuedSpinLock`, `KfReleaseSpinLock` and `KeReleaseSpinLock`. Each one only
//!   calls an ntoskrnl function (for example `ExAcquireFastMutexUnsafe`) that is not in `dlls/hal`.
//! - `KeQueryPerformanceCounter`. Its result is the counter from `NtQueryPerformanceCounter`,
//!   which is OS state.
//! - `KeLowerIrql`, which `hal.spec` forwards to `ntoskrnl.exe.KeLowerIrql`. `hal.c` has no body.
//! - The lock argument of `KeAcquireSpinLock` and `KfAcquireSpinLock`, which only goes to ntoskrnl.
//! - The `#ifdef` gates on `__i386__`, `__arm__` and `__aarch64__`. The port builds on every target.
//! - The `-fastcall`, `-arch=` and `-ret64` decorations and the calling conventions.
//! - The `FIXME` and `TRACE` output.

mod bus;
mod exports;
mod fast_mutex;
mod irql;
mod port;
mod spin_lock;
mod stub;

pub use bus::{hal_get_bus_data, hal_get_bus_data_by_offset, hal_translate_bus_address};
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use fast_mutex::{ex_try_to_acquire_fast_mutex, FastMutex};
pub use irql::{ke_get_current_irql, kf_lower_irql, kf_raise_irql};
pub use port::{read_port_uchar, read_port_ulong, write_port_uchar, write_port_ulong};
pub use spin_lock::{ke_acquire_spin_lock, kf_acquire_spin_lock};
pub use stub::{
    hal_acquire_display_ownership, hal_adjust_resource_list, hal_all_processors_started,
    hal_allocate_adapter_channel, hal_allocate_common_buffer, hal_allocate_crash_dump_registers,
    hal_assign_slot_resources, hal_begin_system_interrupt, hal_calibrate_performance_counter,
    hal_clear_software_interrupt, hal_disable_system_interrupt, hal_display_string,
    hal_enable_system_interrupt, hal_end_system_interrupt, hal_flush_common_buffer,
    hal_free_common_buffer, hal_get_adapter, hal_get_environment_variable,
    hal_get_interrupt_vector, hal_handle_nmi, hal_init_system, hal_initialize_processor,
    hal_make_beep, hal_processor_idle, hal_query_display_parameters, hal_query_real_time_clock,
    hal_read_dma_counter, hal_report_resource_usage, hal_request_ipi,
    hal_request_software_interrupt, hal_return_to_firmware, hal_set_bus_data,
    hal_set_bus_data_by_offset, hal_set_display_parameters, hal_set_environment_variable,
    hal_set_profile_interval, hal_set_real_time_clock, hal_set_time_increment,
    hal_start_next_processor, hal_start_profile_interrupt, hal_stop_profile_interrupt,
    hal_system_vector_dispatch_entry, io_assign_drive_letters, io_flush_adapter_buffers,
    io_free_adapter_channel, io_free_map_registers, io_map_transfer, io_read_partition_table,
    io_set_partition_information, io_write_partition_table, kd_com_port_in_use,
    ke_acquire_in_stack_queued_spin_lock_raise_to_synch, ke_acquire_queued_spin_lock,
    ke_acquire_queued_spin_lock_raise_to_synch, ke_acquire_spin_lock_raise_to_synch,
    ke_flush_write_buffer, ke_raise_irql, ke_raise_irql_to_dpc_level, ke_raise_irql_to_synch_level,
    ke_release_queued_spin_lock, ke_stall_execution_processor, ke_try_to_acquire_queued_spin_lock,
    ke_try_to_acquire_queued_spin_lock_raise_to_synch, read_port_buffer_uchar,
    read_port_buffer_ulong, read_port_buffer_ushort, read_port_ushort,
    wine_spec_unimplemented_stub, write_port_buffer_uchar, write_port_buffer_ulong,
    write_port_buffer_ushort, write_port_ushort, StubException, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
