//! Filter enumeration and filter-add exports from Wine's `dlls/fwpuclnt/main.c`.
//!
//! Like [`crate::engine`], each export is a stub that reports
//! `RPC_S_CANNOT_SUPPORT`. Out-parameters are returned as values, and an
//! out-parameter the C body never writes keeps the caller's value.

use crate::engine::{Handle, NULL, RPC_S_CANNOT_SUPPORT};

/// The out-parameters of `FwpmFilterEnum0`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FilterEnum {
    pub status: u32,
    /// `*entries`, written as `NULL`.
    pub entries: Handle,
    /// `*nEntriesReturned`, written as `0`.
    pub n_entries_returned: u32,
}

/// `FwpmFilterCreateEnumHandle0`. Returns `(status, *enumHandle)`.
pub fn fwpm_filter_create_enum_handle0() -> (u32, Handle) {
    (RPC_S_CANNOT_SUPPORT, NULL)
}

/// `FwpmFilterDestroyEnumHandle0`.
pub fn fwpm_filter_destroy_enum_handle0() -> u32 {
    RPC_S_CANNOT_SUPPORT
}

/// `FwpmFilterEnum0`.
pub fn fwpm_filter_enum0() -> FilterEnum {
    FilterEnum {
        status: RPC_S_CANNOT_SUPPORT,
        entries: NULL,
        n_entries_returned: 0,
    }
}

/// `FwpmFilterAdd0`. Returns `(status, *pFilterId)`. The stub never writes
/// `*pFilterId`, so the caller's value comes back unchanged.
pub fn fwpm_filter_add0(filter_id: u64) -> (u32, u64) {
    (RPC_S_CANNOT_SUPPORT, filter_id)
}
