//! Port of the portable logic in Wine's `dlls/msvcp140_atomic_wait/main.c` and
//! `dlls/msvcp140_atomic_wait/msvcp140_atomic_wait.spec`.
//!
//! Ported:
//! - The shared mutex table behind `__std_acquire_shared_mutex_for_instance` and
//!   `__std_release_shared_mutex_for_instance`: [`SharedMutexList`],
//!   [`std_acquire_shared_mutex_for_instance`], [`std_release_shared_mutex_for_instance`]
//! - `__std_bulk_submit_threadpool_work`, one submit per count: [`std_bulk_submit_threadpool_work`]
//! - The time zone name copy in `__std_tzdb_get_time_zones` and `__std_tzdb_get_current_zone`:
//!   [`std_tzdb_get_time_zones`], [`std_tzdb_get_current_zone`], [`TzdbTimeZones`],
//!   [`TzdbCurrentZone`], [`TzdbError`]
//! - `__std_tzdb_get_leap_seconds`, which returns NULL and writes 0 to `*new_size`:
//!   [`std_tzdb_get_leap_seconds`], [`TzdbLeapSecond`]
//! - `msvcp140_atomic_wait.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The 11 `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per stub export
//!
//! Skipped:
//! - The 15 `@ stdcall` exports that only forward to an OS call or the C runtime and have no logic
//!   of their own: `WaitOnAddress` and `WakeByAddress*` (`__std_atomic_wait_direct`,
//!   `__std_atomic_notify_*_direct`, `__std_execution_wait_on_uchar`,
//!   `__std_execution_wake_by_address_all`), the threadpool calls (`__std_create_threadpool_work`,
//!   `__std_close_threadpool_work`, `__std_submit_threadpool_work`,
//!   `__std_wait_for_threadpool_work_callbacks`), `_Thrd_hardware_concurrency`
//!   (`__std_parallel_algorithms_hw_threads`), `calloc` and `free` (`__std_calloc_crt`,
//!   `__std_free_crt`), and the `delete` functions for time zones, the current zone and leap
//!   seconds, which only free memory.
//! - The process-global `shared_mutex_list` and its critical section. [`SharedMutexList`] takes
//!   `&mut self`, so the caller supplies the lock.
//! - The SRWLOCK inside each shared mutex element, which is OS state.
//! - Allocation failure: the C code dereferences a NULL `malloc` result, and the port has no such
//!   case.
//! - `TRACE` and `FIXME` output.

mod exports;
mod shared_mutex;
mod stub;
mod threadpool;
mod tzdb;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use shared_mutex::{
    std_acquire_shared_mutex_for_instance, std_release_shared_mutex_for_instance, SharedMutexList,
};
pub use stub::{
    std_atomic_compare_exchange_128, std_atomic_get_mutex, std_atomic_has_cmpxchg16b,
    std_atomic_notify_all_indirect, std_atomic_notify_one_indirect, std_atomic_set_api_level,
    std_atomic_wait_get_deadline, std_atomic_wait_get_remaining_timeout, std_atomic_wait_indirect,
    std_tzdb_delete_sys_info, std_tzdb_get_sys_info, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};
pub use threadpool::std_bulk_submit_threadpool_work;
pub use tzdb::{
    std_tzdb_get_current_zone, std_tzdb_get_leap_seconds, std_tzdb_get_time_zones, TzdbCurrentZone,
    TzdbError, TzdbLeapSecond, TzdbTimeZones,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
