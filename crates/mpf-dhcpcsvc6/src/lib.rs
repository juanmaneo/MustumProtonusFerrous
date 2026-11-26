//! Port of the portable logic in Wine's `dlls/dhcpcsvc6/dhcpcsvc6.spec`.
//!
//! `dlls/dhcpcsvc6` has no C source, so its only code is the `@ stub` exports. The port covers
//! the export table and the stub records those exports raise.
//!
//! Ported:
//! - `dhcpcsvc6.spec` export table: [`EXPORTS`], [`export_stub`]
//! - The 22 `@ stub` exports, one helper each, named by snake_casing the C name, for example
//!   [`dhcpv6_c_api_initialize`] for `Dhcpv6CApiInitialize`
//! - The `__wine_spec_unimplemented_stub` record they raise: [`wine_spec_unimplemented_stub`],
//!   [`StubException`], with `EXCEPTION_WINE_STUB` and `EXCEPTION_NONCONTINUABLE` from
//!   `include/winternl.h` and `include/winnt.h`
//!
//! Skipped:
//! - The loop in `__wine_spec_unimplemented_stub` that raises the record and never returns, and
//!   the `ExceptionAddress` field. The port returns the record instead.
//! - Ordinal function names. Every export in the spec has a name, so the port takes names only.
//! - `EXTRADLLFLAGS = -Wb,--prefer-native` in `dlls/dhcpcsvc6/Makefile.in`, which is a build flag.

mod exports;
mod stub;

pub use exports::{export_stub, EXPORTS};
pub use stub::{
    dhcpv6_acquire_parameters, dhcpv6_c_api_cleanup, dhcpv6_c_api_initialize,
    dhcpv6_cancel_operation, dhcpv6_enable_dhcp, dhcpv6_enable_tracing, dhcpv6_free_lease_info,
    dhcpv6_free_lease_info_array, dhcpv6_get_trace_array, dhcpv6_get_user_classes,
    dhcpv6_is_enabled, dhcpv6_query_lease_info, dhcpv6_query_lease_info_array,
    dhcpv6_release_parameters, dhcpv6_release_prefix, dhcpv6_release_prefix_ex,
    dhcpv6_renew_prefix, dhcpv6_renew_prefix_ex, dhcpv6_request_params, dhcpv6_request_prefix,
    dhcpv6_request_prefix_ex, dhcpv6_set_user_class, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
