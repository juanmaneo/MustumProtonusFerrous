//! Port of the portable logic in Wine's `dlls/wkscli/wkscli.c` and `dlls/wkscli/wkscli.spec`.
//!
//! Ported:
//! - `NetGetJoinInformation` workgroup report: [`net_get_join_information`] and
//!   [`NET_SETUP_WORKGROUP_NAME`]
//! - The explicit functions that return a fixed status: [`net_use_add`], [`net_use_del`],
//!   [`net_use_enum`], [`net_use_get_info`] and [`net_wksta_transport_enum`], with the
//!   `NERR_Success`, `ERROR_INVALID_PARAMETER` and `ERROR_NOT_SUPPORTED` values
//! - `wkscli.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The 17 `@ stub` exports: [`wine_spec_unimplemented_stub`], [`StubException`], and one
//!   helper per stub export named after its C name in snake case
//!
//! Skipped:
//! - The `FIXME` traces of the arguments. The C bodies never read the `Server`, `servername`,
//!   `server`, `usename`, `level`, `bufptr`, `parm_err`, `prefmaxsize`, `entriesread`,
//!   `totalentries`, `resumehandle`, `forcecond`, `name`, `pbuf`, `prefmaxlen`, `read_entries`,
//!   `total_entries` and `hresume` parameters, so the ported functions do not take them.
//! - The `NetApiBufferAllocate` status. The port always has the buffer, and the C body ignores
//!   the status.
//! - The `WINAPI`, `NET_API_FUNCTION` and `__cdecl` calling conventions.

mod exports;
mod join;
mod status;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use join::{net_get_join_information, NET_SETUP_WORKGROUP_NAME};
pub use status::{
    net_use_add, net_use_del, net_use_enum, net_use_get_info, net_wksta_transport_enum,
    ERROR_INVALID_PARAMETER, ERROR_NOT_SUPPORTED, NERR_SUCCESS,
};
pub use stub::{
    net_add_alternate_computer_name, net_enumerate_computer_names, net_get_joinable_ous,
    net_join_domain, net_remove_alternate_computer_name, net_rename_machine_in_domain,
    net_set_primary_computer_name, net_unjoin_domain, net_validate_name, net_wksta_get_info,
    net_wksta_set_info, net_wksta_statistics_get, net_wksta_transport_add, net_wksta_transport_del,
    net_wksta_user_enum, net_wksta_user_get_info, net_wksta_user_set_info,
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
