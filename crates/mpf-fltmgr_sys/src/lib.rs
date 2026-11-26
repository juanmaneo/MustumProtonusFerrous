//! Port of the portable logic in Wine's `dlls/fltmgr.sys/main.c`.
//!
//! Ported:
//! - `FltBuildDefaultSecurityDescriptor`: SID, ACL and ACE layout, the sizes
//!   requested from the pool, and the three allocation-failure exits:
//!   [`build_default_security_descriptor`], [`build_default_security_descriptor_with`]
//! - Export return values and fixed values: [`driver_entry`], [`flt_register_filter`]
//!   (writes [`FLT_FILTER_HANDLE`]), [`flt_enumerate_volumes`] (zeroes `returned`),
//!   and the `STATUS_NOT_SUPPORTED` stubs, one function per export.
//! - The FIXME-only void exports, modelled as functions with no effect.
//!
//! Skipped:
//! - `FltGetRoutineAddress`: `GetModuleHandleW` and `GetProcAddress`.
//! - `FltFreeSecurityDescriptor`, `ExAllocatePool` and `ExFreePool`: kernel pool.
//! - `FIXME` and `TRACE` output. `FltAllocateContext` reads `*out` only inside its
//!   FIXME argument list, which is not modelled.
//! - Argument checks in `RtlCreateAcl`, `RtlCreateSecurityDescriptor` and
//!   `RtlAddAccessAllowedAce`: the builder passes fixed valid values, so they are unreachable.
//! - Pointer layout: the DACL is an owned value, not an offset inside the descriptor allocation.

mod filter;
mod security;

pub use filter::{
    driver_entry, flt_acquire_push_lock_exclusive, flt_allocate_context,
    flt_close_communication_port, flt_create_communication_port, flt_delete_push_lock,
    flt_enumerate_volumes, flt_get_file_name_information, flt_get_file_name_information_unsafe,
    flt_get_stream_context, flt_initialize_push_lock, flt_parse_file_name_information,
    flt_query_information_file, flt_read_file, flt_register_filter, flt_release_context,
    flt_release_file_name_information, flt_release_push_lock, flt_set_stream_context,
    flt_start_filtering, flt_unregister_filter, FLT_FILTER_HANDLE,
};
pub use security::{
    build_default_security_descriptor, build_default_security_descriptor_with, length_required_sid,
    AccessAllowedAce, Acl, SecurityDescriptor, Sid,
};

/// `NTSTATUS`.
pub type NtStatus = u32;

pub const STATUS_SUCCESS: NtStatus = 0x0000_0000;
pub const STATUS_NO_MEMORY: NtStatus = 0xC000_0017;
pub const STATUS_INSUFFICIENT_RESOURCES: NtStatus = 0xC000_009A;
pub const STATUS_NOT_SUPPORTED: NtStatus = 0xC000_00BB;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
