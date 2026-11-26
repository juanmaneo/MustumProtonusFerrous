//! Export stubs from Wine's `dlls/fltmgr.sys/main.c`. Each function returns the
//! value its C body returns; the FIXME-only void exports have no effect.

use crate::{NtStatus, STATUS_NOT_SUPPORTED, STATUS_SUCCESS};

/// `UlongToHandle(0xdeadbeaf)`. The value is `deadbeaf`, not `deadbeef`.
pub const FLT_FILTER_HANDLE: u64 = 0xdead_beaf;

pub fn driver_entry() -> NtStatus {
    STATUS_SUCCESS
}

/// Returns the status and the handle written through `filter` when that pointer is non-null.
pub fn flt_register_filter(filter_requested: bool) -> (NtStatus, Option<u64>) {
    (
        STATUS_SUCCESS,
        filter_requested.then_some(FLT_FILTER_HANDLE),
    )
}

pub fn flt_start_filtering() -> NtStatus {
    STATUS_SUCCESS
}

pub fn flt_allocate_context() -> NtStatus {
    STATUS_NOT_SUPPORTED
}

pub fn flt_get_stream_context() -> NtStatus {
    STATUS_NOT_SUPPORTED
}

pub fn flt_set_stream_context() -> NtStatus {
    STATUS_NOT_SUPPORTED
}

pub fn flt_query_information_file() -> NtStatus {
    STATUS_NOT_SUPPORTED
}

pub fn flt_get_file_name_information_unsafe() -> NtStatus {
    STATUS_NOT_SUPPORTED
}

pub fn flt_read_file() -> NtStatus {
    STATUS_NOT_SUPPORTED
}

pub fn flt_parse_file_name_information() -> NtStatus {
    STATUS_NOT_SUPPORTED
}

pub fn flt_get_file_name_information() -> NtStatus {
    STATUS_NOT_SUPPORTED
}

pub fn flt_create_communication_port() -> NtStatus {
    STATUS_NOT_SUPPORTED
}

/// Writes 0 through `returned` when it is non-null, as Wine does.
pub fn flt_enumerate_volumes(returned: Option<&mut u32>) -> NtStatus {
    if let Some(returned) = returned {
        *returned = 0;
    }
    STATUS_NOT_SUPPORTED
}

pub fn flt_initialize_push_lock() {}

pub fn flt_delete_push_lock() {}

pub fn flt_acquire_push_lock_exclusive() {}

pub fn flt_release_push_lock() {}

pub fn flt_unregister_filter() {}

pub fn flt_release_context() {}

pub fn flt_release_file_name_information() {}

pub fn flt_close_communication_port() {}
