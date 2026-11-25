//! Port of the portable logic in Wine's `dlls/rtworkq/queue.c`.
//!
//! Ported:
//! - Callback queue id routing (`get_system_queue`) and the queue type that `grab_queue`
//!   picks for a system queue: [`is_user_queue`], [`system_queue`], [`system_queue_type`]
//! - Pool thread maximum and callback priority (`pool_queue_init`, `pool_queue_submit`):
//!   [`pool_max_threads`], [`callback_priority`]
//! - Timer due time (`queue_submit_timer`): [`due_time`]
//! - Work item keys (`get_item_key`, `generate_item_key`, and the key tests in `queue_cancel_item`):
//!   [`item_key`], [`ItemKeyGenerator`], [`item_kind`]
//! - User queue handles (`get_queue_obj`, `alloc_user_queue`): [`encode_user_queue_id`],
//!   [`decode_user_queue_id`], [`find_user_queue`], [`next_queue_generation`]
//! - Serial queue sequencing against a pool target (`serial_queue_submit`,
//!   `serial_queue_finalization_callback`): [`SerialQueue`]
//!
//! Skipped:
//! - Thread pool, timers, waits, critical sections, and COM MTA: OS objects.
//! - Async results and callbacks: COM vtables, refcounting, the async result cache, and
//!   `invoke_async_callback`.
//! - Chained serial queues: they move items between Wine `struct list` nodes.
//!   [`SerialQueue`] models a pool target only.
//! - The actions in `queue_cancel_item` (waiting on and closing thread pool objects).
//! - `RtwqSetLongRunning`, MMCSS, deadlines, joins, shared work queues, periodic callbacks,
//!   platform events, and the `FIXME`/`E_NOTIMPL` stubs.
//! - `queue_serial_callback_GetParameters` reports `queue->id`, which Wine never assigns,
//!   so it always returns 0.

mod item_key;
mod pool;
mod serial;
mod user_queue;

pub use item_key::{item_key, item_kind, ItemKeyGenerator, ItemKind};
pub use pool::{callback_priority, due_time, pool_max_threads, CallbackPriority, FileTime};
pub use serial::{SerialQueue, Submission};
pub use user_queue::{
    decode_user_queue_id, encode_user_queue_id, find_user_queue, next_queue_generation,
    UserQueueSlot,
};

/// `RTWQ_CALLBACK_QUEUE_*` ids from `rtwq.idl`.
pub const RTWQ_CALLBACK_QUEUE_STANDARD: u32 = 0x0000_0001;
pub const RTWQ_CALLBACK_QUEUE_RT: u32 = 0x0000_0002;
pub const RTWQ_CALLBACK_QUEUE_IO: u32 = 0x0000_0003;
pub const RTWQ_CALLBACK_QUEUE_TIMER: u32 = 0x0000_0004;
pub const RTWQ_CALLBACK_QUEUE_MULTITHREADED: u32 = 0x0000_0005;
pub const RTWQ_CALLBACK_QUEUE_LONG_FUNCTION: u32 = 0x0000_0007;
/// Any id with these bits set is a user queue handle, not a system queue.
pub const RTWQ_CALLBACK_QUEUE_PRIVATE_MASK: u32 = 0xffff_0000;

/// `RTWQ_WORKQUEUE_TYPE` values.
pub const RTWQ_STANDARD_WORKQUEUE: u32 = 0;
pub const RTWQ_WINDOW_WORKQUEUE: u32 = 1;
pub const RTWQ_MULTITHREADED_WORKQUEUE: u32 = 2;

/// A system queue, as an index into Wine's `system_queues` table. Index 5
/// (`SYS_QUEUE_DO_NOT_USE`) is never produced.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SystemQueue {
    Standard = 0,
    Rt = 1,
    Io = 2,
    Timer = 3,
    Multithreaded = 4,
    LongFunction = 6,
}

/// `get_system_queue`'s private-mask test. Handles from `alloc_user_queue` always
/// set bits in the high word, so they never resolve to a system queue.
pub fn is_user_queue(queue_id: u32) -> bool {
    queue_id & RTWQ_CALLBACK_QUEUE_PRIVATE_MASK != 0
}

/// `get_system_queue`: `None` for user queue handles. Undefined ids use the standard queue.
pub fn system_queue(queue_id: u32) -> Option<SystemQueue> {
    if is_user_queue(queue_id) {
        return None;
    }
    Some(match queue_id {
        RTWQ_CALLBACK_QUEUE_STANDARD => SystemQueue::Standard,
        RTWQ_CALLBACK_QUEUE_RT => SystemQueue::Rt,
        RTWQ_CALLBACK_QUEUE_IO => SystemQueue::Io,
        RTWQ_CALLBACK_QUEUE_TIMER => SystemQueue::Timer,
        RTWQ_CALLBACK_QUEUE_MULTITHREADED => SystemQueue::Multithreaded,
        RTWQ_CALLBACK_QUEUE_LONG_FUNCTION => SystemQueue::LongFunction,
        _ => SystemQueue::Standard,
    })
}

/// The `queue_type` that `grab_queue` gives a system queue: I/O, multithreaded, and
/// long-function queues are multithreaded; the rest are standard.
pub fn system_queue_type(queue: SystemQueue) -> u32 {
    match queue {
        SystemQueue::Io | SystemQueue::Multithreaded | SystemQueue::LongFunction => {
            RTWQ_MULTITHREADED_WORKQUEUE
        }
        SystemQueue::Standard | SystemQueue::Rt | SystemQueue::Timer => RTWQ_STANDARD_WORKQUEUE,
    }
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
