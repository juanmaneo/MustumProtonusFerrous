//! Port of the portable logic in Wine's `dlls/dsrole/dsrole.c`.
//!
//! Ported:
//! - `DsRoleGetPrimaryDomainInformation` argument checks and dispatch
//!   (`ERROR_INVALID_PARAMETER`, the `ERROR_CALL_NOT_IMPLEMENTED` default branch):
//!   [`route`], [`InfoRoute`]
//! - Return-value and buffer flow of the basic-level path, including the
//!   `ERROR_OUTOFMEMORY` mapping for both LSA and allocation failure:
//!   [`get_primary_domain_information`], [`Query`], [`Reply`]
//! - Basic-info buffer size (`sizeof(DSROLE_PRIMARY_DOMAIN_INFO_BASIC)` plus the
//!   flat name) and the `lstrlenW`/`lstrcpyW` domain copy:
//!   [`basic_struct_size`], [`basic_alloc_size`], [`wide_strlen`], [`copy_domain_name`]
//!
//! Skipped:
//! - `LsaOpenPolicy`, `LsaQueryInformationPolicy`, `LsaClose`, `LsaFreeMemory`,
//!   `LsaNtStatusToWinError`: LSA IPC, modelled only as the `policy_open` flag.
//! - `HeapAlloc`, `HeapFree`, `GetProcessHeap`, `DsRoleFreeMemory`: process heap.
//! - `TRACE` and `FIXME` output.
//! - The Wine code does not check `LsaQueryInformationPolicy`'s status and would
//!   dereference an unset `DomainInfo` if it failed. Not modelled.

mod primary_domain;

pub use primary_domain::{
    basic_alloc_size, basic_struct_size, copy_domain_name, get_primary_domain_information, route,
    wide_strlen, InfoRoute, PrimaryDomainBasic, Query, Reply, ERROR_CALL_NOT_IMPLEMENTED,
    ERROR_INVALID_PARAMETER, ERROR_OUTOFMEMORY, ERROR_SUCCESS, PRIMARY_DOMAIN_INFO_BASIC,
    PRIMARY_DOMAIN_INFO_OPERATION_STATE, ROLE_STANDALONE_WORKSTATION,
};

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
