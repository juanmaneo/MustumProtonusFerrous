//! Port of the portable logic in Wine's `dlls/traffic/traffic_main.c` and `dlls/traffic/traffic.spec`.
//!
//! Ported:
//! - `TcRegisterClient` and `TcDeregisterClient`: the `ERROR_CALL_NOT_IMPLEMENTED` return and the
//!   `INVALID_HANDLE_VALUE` write: [`tc_register_client`], [`tc_deregister_client`], with [`Handle`],
//!   [`INVALID_HANDLE_VALUE`] and [`ERROR_CALL_NOT_IMPLEMENTED`]
//! - `traffic.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The 19 `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   one helper per export, such as [`tc_add_class_map`], plus [`wine_spec_unimplemented_stub`]
//!   and [`StubException`]
//!
//! Skipped:
//! - The `FIXME` trace of the arguments, and the `version`, `context` and `list` parameters of
//!   `TcRegisterClient` and the `ClientHandle` parameter of `TcDeregisterClient`, which the C
//!   bodies never read.
//! - The `WINAPI` calling convention and the Windows `HANDLE` type: [`Handle`] is pointer-sized.

mod client;
mod exports;
mod stub;

pub use client::{
    tc_deregister_client, tc_register_client, Handle, ERROR_CALL_NOT_IMPLEMENTED,
    INVALID_HANDLE_VALUE,
};
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    tc_add_class_map, tc_add_filter, tc_add_flow, tc_close_interface, tc_delete_filter,
    tc_delete_flow, tc_enumerate_flows, tc_enumerate_interfaces, tc_get_flow_name_a,
    tc_get_flow_name_w, tc_modify_flow, tc_open_interface_a, tc_open_interface_w, tc_query_flow_a,
    tc_query_flow_w, tc_query_interface, tc_set_flow_a, tc_set_flow_w, tc_set_interface,
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
