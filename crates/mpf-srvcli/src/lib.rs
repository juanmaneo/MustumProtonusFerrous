//! Port of the portable logic in Wine's `dlls/srvcli/srvcli.spec`. The reference has no C sources,
//! so the ported code is the export table, its forwards and its stub records.
//!
//! Ported:
//! - `srvcli.spec` export table: [`EXPORTS`], [`Export`], [`ExportKind`] and [`export_by_name`],
//!   with 61 named exports numbered 1 to 61 in spec order
//! - the 9 `@ stdcall` forwards to `netapi32`: [`forward_target`]
//! - the 52 `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per export, such as
//!   [`i_net_dfs_get_version`]
//!
//! Skipped:
//! - The forwarded functions themselves, which live in `netapi32`. The crate records only where
//!   each forward points.
//! - The `for (;;)` loop in `__wine_spec_unimplemented_stub`. The port returns the record, and no
//!   caller sees a return value, as in the C code.
//! - The `.def` and PE layout: i386 `@N` decorations, the parameter lists in the spec lines, and
//!   the `PRIVATE` flag that winebuild writes for stubs.
//! - Other targets and calling conventions.

mod exports;
mod stub;

pub use exports::{export_by_name, forward_target, Export, ExportKind, EXPORTS};
pub use stub::{
    i_net_dfs_get_version, i_net_server_set_service_bits, i_net_server_set_service_bits_ex,
    local_alias_get, local_file_close, local_file_enum, local_file_enum_ex, local_file_get_info,
    local_file_get_info_ex, local_server_certificate_mapping_add,
    local_server_certificate_mapping_enum, local_server_certificate_mapping_get,
    local_server_certificate_mapping_remove, local_session_del, local_session_enum,
    local_session_enum_ex, local_session_get_info, local_session_get_info_ex, local_share_add,
    local_share_del_ex, local_share_enum, local_share_enum_ex, local_share_get_info,
    local_share_get_info_ex, local_share_set_info, net_connection_enum, net_file_close,
    net_file_get_info, net_server_alias_add, net_server_alias_del, net_server_alias_enum,
    net_server_computer_name_add, net_server_computer_name_del, net_server_set_info,
    net_server_statistics_get, net_server_transport_add, net_server_transport_add_ex,
    net_server_transport_del, net_server_transport_enum, net_session_del, net_session_get_info,
    net_share_check, net_share_del_ex, net_share_del_sticky, net_share_enum_sticky,
    net_share_set_info, netps_name_canonicalize, netps_name_compare, netps_name_validate,
    netps_path_canonicalize, netps_path_compare, netps_path_type, wine_spec_unimplemented_stub,
    StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
