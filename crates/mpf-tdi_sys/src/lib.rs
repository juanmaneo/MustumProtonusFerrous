//! Port of the portable logic in Wine's `dlls/tdi.sys/main.c` and `dlls/tdi.sys/tdi.sys.spec`.
//!
//! Ported:
//! - `DriverEntry`, which returns `STATUS_SUCCESS` without reading its arguments: [`driver_entry`]
//! - `tdi.sys.spec` export table: [`EXPORTS`], [`export_stub`]
//! - The 56 `@ stub` exports, one helper per export named in snake_case (for example
//!   [`cte_allocate_string`]), and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`wine_spec_unimplemented_stub`], [`StubException`]
//!
//! Skipped:
//! - The `TRACE` line. It is the only reader of the `driver` and `path` arguments, and
//!   `path->Buffer` is read only when the trace channel is on.
//! - The `__cdecl` and `WINAPI` calling conventions and the `DRIVER_OBJECT` and `UNICODE_STRING`
//!   kernel types: the port takes no arguments.
//! - The infinite loop in `__wine_spec_unimplemented_stub`: the port returns the record it would raise.

mod driver;
mod exports;
mod stub;

pub use driver::driver_entry;
pub use exports::{export_stub, EXPORTS};
pub use stub::{
    cte_allocate_string, cte_block, cte_block_with_tracker, cte_init_event, cte_init_string,
    cte_init_timer, cte_initialize, cte_insert_block_tracker, cte_log_event,
    cte_remove_block_tracker, cte_schedule_critical_event, cte_schedule_delayed_event,
    cte_schedule_event, cte_signal, cte_start_timer, cte_system_up_time, dll_initialize,
    dll_unload, tdi_build_netbios_address, tdi_build_netbios_address_ea, tdi_copy_buffer_to_mdl,
    tdi_copy_buffer_to_mdl_with_reserved_mapping_at_dpc_level, tdi_copy_mdl_chain_to_mdl_chain,
    tdi_copy_mdl_to_buffer, tdi_default_chained_rcv_datagram_handler,
    tdi_default_chained_rcv_expedited_handler, tdi_default_chained_receive_handler,
    tdi_default_connect_handler, tdi_default_disconnect_handler, tdi_default_error_handler,
    tdi_default_rcv_datagram_handler, tdi_default_rcv_expedited_handler,
    tdi_default_receive_handler, tdi_default_send_possible_handler,
    tdi_deregister_address_change_handler, tdi_deregister_device_object,
    tdi_deregister_net_address, tdi_deregister_notification_handler, tdi_deregister_pnp_handlers,
    tdi_deregister_provider, tdi_enumerate_addresses, tdi_get9f_triage_block, tdi_initialize,
    tdi_map_user_request, tdi_match_pdo_with_chained_receive_context, tdi_open_netbios_address,
    tdi_pnp_power_complete, tdi_pnp_power_request, tdi_provider_ready,
    tdi_register_address_change_handler, tdi_register_device_object, tdi_register_net_address,
    tdi_register_notification_handler, tdi_register_pnp_handlers, tdi_register_provider,
    tdi_return_chained_receives, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

/// `NTSTATUS`.
pub type NtStatus = u32;

pub const STATUS_SUCCESS: NtStatus = 0x0000_0000;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
