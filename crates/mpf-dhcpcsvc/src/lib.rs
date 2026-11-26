//! Port of the portable logic in Wine's `dlls/dhcpcsvc/dhcpcsvc.c`, and the stub exports of
//! `dlls/dhcpcsvc/dhcpcsvc.spec`.
//!
//! Ported:
//! - `DhcpCApiInitialize` and `DhcpCApiCleanup`: [`dhcp_c_api_initialize`], [`dhcp_c_api_cleanup`]
//! - `get_adapter_luid` and `get_adapter_name`: [`get_adapter_luid`], [`get_adapter_name`]
//! - `DhcpRequestParams`: argument checks, query size, reply handling and reporting of
//!   `*buflen`: [`request_params`], [`request_size`], [`fill_request`], [`collect_reply`],
//!   [`more_data_buflen`]
//! - The `struct mountmgr_dhcp_request_params` layout and the
//!   `IOCTL_MOUNTMGR_QUERY_DHCP_REQUEST_PARAMS` code: [`query_dhcp_request_params_ioctl`],
//!   [`params_offset`]
//!
//! Modelled:
//! - The host calls (`RtlGUIDFromString`, `ConvertInterface*`, `WideCharToMultiByte`,
//!   `CreateFileW`, `DeviceIoControl`, `calloc`, `CloseHandle`, `GetLastError`) through the
//!   [`Os`] and [`MountMgr`] traits.
//! - The 42 `@ stub` exports. `__wine_spec_unimplemented_stub` raises `EXCEPTION_WINE_STUB`
//!   with the module and export names: [`call_stub_export`], [`StubFault`].
//!
//! Deviations from the C code:
//! - A query whose size does not fit in 32 bits fails with `ERROR_INVALID_PARAMETER`. C truncates
//!   the size, then writes past its allocation.
//! - A malformed driver reply (count above the requested count, data outside the reply, or a
//!   total above 32 bits) fails with `ERROR_INVALID_DATA`. C overruns its arrays.
//! - Data larger than the caller's buffer fails with `ERROR_INSUFFICIENT_BUFFER`. C writes past
//!   the buffer.
//! - The alias is read through a 16-unit buffer, so only its first 15 units are converted.
//! - Name conversion goes through [`Os::wide_to_unix`]. The host supplies `CP_UNIXCP`.
//!
//! Skipped:
//! - `FIXME` and `TRACE` output. The `flags` value is not checked; C only logs other values.
//! - Send options (`send_params`): C logs and ignores them, so the port takes no send list.
//! - `class_id`, `reserved` and `request_id`: unused by the C code.
//! - Stub exception fields beyond the code, flags, parameter count and the two names
//!   (`ExceptionRecord`, `ExceptionAddress`).
//! - `IsVendor` stays `false`, as in C.
//! - `RtlInitUnicodeString`: Rust slices carry their own length.

mod adapter;
mod api;
mod layout;
mod request;
mod stubs;

pub use adapter::{get_adapter_luid, get_adapter_name, Guid, Luid, Os, IF_NAMESIZE};
pub use api::{
    dhcp_c_api_cleanup, dhcp_c_api_initialize, DHCP_API_VERSION, ERROR_INSUFFICIENT_BUFFER,
    ERROR_INVALID_DATA, ERROR_INVALID_PARAMETER, ERROR_MORE_DATA, ERROR_OUTOFMEMORY, ERROR_SUCCESS,
};
pub use layout::{
    collect_reply, fill_request, more_data_buflen, params_offset, query_dhcp_request_params_ioctl,
    query_size, request_size, ReceivedParam, Reply,
};
pub use request::{request_params, MountMgr};
pub use stubs::{
    call_stub_export, unimplemented_stub, StubFault, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
    MODULE_NAME, STUB_EXPORTS,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
