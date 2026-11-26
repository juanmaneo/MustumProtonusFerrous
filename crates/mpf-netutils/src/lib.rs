//! Port of the portable logic in Wine's `dlls/netutils/main.c` and `dlls/netutils/netutils.spec`.
//!
//! Ported:
//! - `NetApiBufferAllocate`, `NetApiBufferFree`, `NetApiBufferReallocate` and `NetApiBufferSize`:
//!   the NULL checks, the zero-size branch of the reallocate, the `HeapSize` truncation and the
//!   return values, over the process-heap calls supplied through [`ProcessHeap`]:
//!   [`net_api_buffer_allocate`], [`net_api_buffer_free`], [`net_api_buffer_reallocate`],
//!   [`net_api_buffer_size`]
//! - `NERR_Success` and `ERROR_INVALID_PARAMETER`: [`NERR_SUCCESS`], [`ERROR_INVALID_PARAMETER`]
//! - `netutils.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]. The
//!   `NetapipBufferAllocate` export is an alias of `NetApiBufferAllocate`.
//! - The 17 `@ stub` exports: [`wine_spec_unimplemented_stub`], [`StubException`] and one helper
//!   per stub, such as [`net_remote_computer_supports`]
//!
//! Skipped:
//! - The process heap: `GetProcessHeap`, `HeapAlloc`, `HeapReAlloc`, `HeapFree`, `HeapSize` and
//!   `GetLastError` are OS calls. The caller implements [`ProcessHeap`].
//! - The `TRACE` calls and the `WINAPI` calling convention.
//! - The NULL `NewBuffer` and `ByteCount` out-pointers of `NetApiBufferReallocate` and
//!   `NetApiBufferSize` when the success path stores through them. C dereferences them without a
//!   check, which crashes. The port takes references, so that case cannot be expressed.

mod buffer;
mod exports;
mod stub;

pub use buffer::{
    net_api_buffer_allocate, net_api_buffer_free, net_api_buffer_reallocate, net_api_buffer_size,
    ProcessHeap, ERROR_INVALID_PARAMETER, NERR_SUCCESS,
};
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    net_remote_computer_supports, netp_is_computer_name_valid, netp_is_domain_name_valid,
    netp_is_group_name_valid, netp_is_remote, netp_is_remote_name_valid, netp_is_share_name_valid,
    netp_is_unc_computer_name_valid, netp_is_user_name_valid, netpw_list_canonicalize,
    netpw_list_traverse, netpw_name_canonicalize, netpw_name_compare, netpw_name_validate,
    netpw_path_canonicalize, netpw_path_compare, netpw_path_type, wine_spec_unimplemented_stub,
    StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
