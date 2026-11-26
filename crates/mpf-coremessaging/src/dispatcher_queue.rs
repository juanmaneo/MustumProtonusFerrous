//! `DispatcherQueueController` creation and shutdown from Wine's `dlls/coremessaging/main.c`.

use crate::{AsyncInfo, CallbackResult, Hresult, E_INVALIDARG, E_POINTER, STATUS_PENDING, S_OK};

/// `sizeof(DispatcherQueueOptions)`: three 4-byte fields.
pub const DISPATCHER_QUEUE_OPTIONS_SIZE: u32 = 12;
/// `DQTYPE_THREAD_DEDICATED` from `dispatcherqueue.idl`.
pub const DQTYPE_THREAD_DEDICATED: i32 = 1;
/// `DQTYPE_THREAD_CURRENT` from `dispatcherqueue.idl`.
pub const DQTYPE_THREAD_CURRENT: i32 = 2;

/// `DispatcherQueueOptions`. `CreateDispatcherQueueController` never checks `apartment_type`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DispatcherQueueOptions {
    pub dw_size: u32,
    pub thread_type: i32,
    pub apartment_type: i32,
}

/// `CreateDispatcherQueueController` up to the `calloc`. `has_out` says whether
/// `queue_controller` is non-NULL. The allocation and the vtable are not modelled.
pub fn create_dispatcher_queue_controller(
    options: DispatcherQueueOptions,
    has_out: bool,
) -> Result<(), Hresult> {
    if !has_out {
        return Err(E_POINTER);
    }
    if options.dw_size != DISPATCHER_QUEUE_OPTIONS_SIZE {
        return Err(E_INVALIDARG);
    }
    if options.thread_type != DQTYPE_THREAD_DEDICATED
        && options.thread_type != DQTYPE_THREAD_CURRENT
    {
        return Err(E_INVALIDARG);
    }
    Ok(())
}

/// `shutdown_queue_async`: `STATUS_PENDING` on the synchronous call, `S_OK` when run on
/// the threadpool. The result is never written.
pub fn shutdown_queue_callback(called_async: bool) -> CallbackResult {
    let hr = if called_async { S_OK } else { STATUS_PENDING };
    CallbackResult { hr, value: None }
}

/// `IDispatcherQueueController::ShutdownQueueAsync`. `has_out` says whether `operation`
/// is non-NULL. On success, the returned operation is the one `async_action_create`
/// started, so it is `Started` until the threadpool completes it.
pub fn shutdown_queue_async(has_out: bool) -> Result<AsyncInfo, Hresult> {
    if !has_out {
        return Err(E_POINTER);
    }
    let mut operation = AsyncInfo::new(shutdown_queue_callback);
    operation.start();
    Ok(operation)
}
