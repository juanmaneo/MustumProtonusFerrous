//! Port of the thread creation plan in `submit_standalone_thread_work` (Wine's `dlls/threadpoolwinrt/main.c`).
//! A thread is created suspended unless the priority is `Normal`. A non-`Normal` thread gets
//! `SetThreadPriority` and is then resumed.

use crate::work_item::Priority;

/// `CREATE_SUSPENDED` from `include/winbase.h`.
pub const CREATE_SUSPENDED: u32 = 0x0000_0004;

/// `THREAD_PRIORITY_HIGHEST` and `THREAD_PRIORITY_LOWEST`, which are `THREAD_BASE_PRIORITY_MAX` and
/// `THREAD_BASE_PRIORITY_MIN` in `include/winnt.h`.
pub const THREAD_PRIORITY_HIGHEST: i32 = 2;
pub const THREAD_PRIORITY_LOWEST: i32 = -2;

/// The `CreateThread` flags and the priority that `SetThreadPriority` is given, if any.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ThreadPlan {
    pub creation_flags: u32,
    pub thread_priority: Option<i32>,
}

/// The thread plan for a standalone work item.
pub fn standalone_thread_plan(priority: Priority) -> ThreadPlan {
    match priority {
        Priority::Normal => ThreadPlan {
            creation_flags: 0,
            thread_priority: None,
        },
        Priority::High => ThreadPlan {
            creation_flags: CREATE_SUSPENDED,
            thread_priority: Some(THREAD_PRIORITY_HIGHEST),
        },
        Priority::Low => ThreadPlan {
            creation_flags: CREATE_SUSPENDED,
            thread_priority: Some(THREAD_PRIORITY_LOWEST),
        },
    }
}
