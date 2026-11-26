/// How `dlls/msvcp140_atomic_wait/msvcp140_atomic_wait.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: a real C function. Some are ported (see the crate docs), the rest are OS forwarders.
    Stdcall,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `msvcp140_atomic_wait.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 32] = [
    (
        "__std_acquire_shared_mutex_for_instance",
        ExportKind::Stdcall,
    ),
    ("__std_atomic_compare_exchange_128", ExportKind::Stub),
    ("__std_atomic_get_mutex", ExportKind::Stub),
    ("__std_atomic_has_cmpxchg16b", ExportKind::Stub),
    ("__std_atomic_notify_all_direct", ExportKind::Stdcall),
    ("__std_atomic_notify_all_indirect", ExportKind::Stub),
    ("__std_atomic_notify_one_direct", ExportKind::Stdcall),
    ("__std_atomic_notify_one_indirect", ExportKind::Stub),
    ("__std_atomic_set_api_level", ExportKind::Stub),
    ("__std_atomic_wait_direct", ExportKind::Stdcall),
    ("__std_atomic_wait_get_deadline", ExportKind::Stub),
    ("__std_atomic_wait_get_remaining_timeout", ExportKind::Stub),
    ("__std_atomic_wait_indirect", ExportKind::Stub),
    ("__std_bulk_submit_threadpool_work", ExportKind::Stdcall),
    ("__std_calloc_crt", ExportKind::Stdcall),
    ("__std_close_threadpool_work", ExportKind::Stdcall),
    ("__std_create_threadpool_work", ExportKind::Stdcall),
    ("__std_execution_wait_on_uchar", ExportKind::Stdcall),
    ("__std_execution_wake_by_address_all", ExportKind::Stdcall),
    ("__std_free_crt", ExportKind::Stdcall),
    ("__std_parallel_algorithms_hw_threads", ExportKind::Stdcall),
    (
        "__std_release_shared_mutex_for_instance",
        ExportKind::Stdcall,
    ),
    ("__std_submit_threadpool_work", ExportKind::Stdcall),
    ("__std_tzdb_delete_current_zone", ExportKind::Stdcall),
    ("__std_tzdb_delete_leap_seconds", ExportKind::Stdcall),
    ("__std_tzdb_delete_sys_info", ExportKind::Stub),
    ("__std_tzdb_delete_time_zones", ExportKind::Stdcall),
    ("__std_tzdb_get_current_zone", ExportKind::Stdcall),
    ("__std_tzdb_get_leap_seconds", ExportKind::Stdcall),
    ("__std_tzdb_get_sys_info", ExportKind::Stub),
    ("__std_tzdb_get_time_zones", ExportKind::Stdcall),
    (
        "__std_wait_for_threadpool_work_callbacks",
        ExportKind::Stdcall,
    ),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
