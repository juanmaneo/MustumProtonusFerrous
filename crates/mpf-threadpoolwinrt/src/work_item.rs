//! Port of the argument checks and the routing at the top of `run_async` in Wine's
//! `dlls/threadpoolwinrt/main.c`, and of the pool index used by `submit_threadpool_work`.

use crate::hresult::{E_INVALIDARG, E_OUTOFMEMORY, HRESULT};
use crate::thread::{standalone_thread_plan, ThreadPlan};

/// `WorkItemOptions_TimeSliced` from `include/windows.system.threading.idl`.
const TIME_SLICED: i32 = 0x1;

/// The maximum thread count that `pool_init_once` sets on each thread pool.
pub const POOL_MAX_THREADS: u32 = 10;

/// `WorkItemPriority` from `include/windows.system.threading.idl`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Priority {
    Low,
    Normal,
    High,
}

impl Priority {
    /// Maps a raw `WorkItemPriority` value. Any other value is rejected by `run_async`.
    pub fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            -1 => Some(Self::Low),
            0 => Some(Self::Normal),
            1 => Some(Self::High),
            _ => None,
        }
    }

    /// The index `priority + 1` into `pools[3]` in `submit_threadpool_work`.
    pub fn pool_index(self) -> usize {
        match self {
            Self::Low => 0,
            Self::Normal => 1,
            Self::High => 2,
        }
    }
}

/// Where `run_async` submits the work item.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Route {
    /// `submit_threadpool_work`, on `pools[pool_index]`.
    ThreadPool { pool_index: usize },
    /// `submit_standalone_thread_work`.
    Standalone(ThreadPlan),
}

/// The part of `run_async` before it submits the work item. The checks run in the C order: the
/// NULL handler, then the priority range, then the allocation of the work item. `options` is
/// compared with `WorkItemOptions_TimeSliced`, and every other value takes the thread pool route.
pub fn plan_run_async(
    handler_present: bool,
    priority: i32,
    options: i32,
    alloc_succeeds: bool,
) -> Result<Route, HRESULT> {
    if !handler_present {
        return Err(E_INVALIDARG);
    }
    let priority = Priority::from_raw(priority).ok_or(E_INVALIDARG)?;
    if !alloc_succeeds {
        return Err(E_OUTOFMEMORY);
    }
    if options == TIME_SLICED {
        Ok(Route::Standalone(standalone_thread_plan(priority)))
    } else {
        Ok(Route::ThreadPool {
            pool_index: priority.pool_index(),
        })
    }
}
