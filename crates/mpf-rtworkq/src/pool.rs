//! Pool sizing, callback priority, and timer due time from `pool_queue_init`,
//! `pool_queue_submit`, and `queue_submit_timer`.

use crate::{RTWQ_STANDARD_WORKQUEUE, RTWQ_WINDOW_WORKQUEUE};

/// `pool_queue_init`: standard and window queues get one pool thread, the rest four.
pub fn pool_max_threads(queue_type: u32) -> u32 {
    if queue_type == RTWQ_STANDARD_WORKQUEUE || queue_type == RTWQ_WINDOW_WORKQUEUE {
        1
    } else {
        4
    }
}

/// `TP_CALLBACK_PRIORITY`, for the `envs` entries of a pool queue.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CallbackPriority {
    High,
    Normal,
    Low,
}

/// `pool_queue_submit`: priority zero is normal, negative is low, and positive is high.
pub fn callback_priority(priority: i32) -> CallbackPriority {
    if priority == 0 {
        CallbackPriority::Normal
    } else if priority < 0 {
        CallbackPriority::Low
    } else {
        CallbackPriority::High
    }
}

/// A `FILETIME` as its two 32-bit halves.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileTime {
    pub low: u32,
    pub high: u32,
}

/// `queue_submit_timer`: converts `timeout` (scaled as milliseconds, so `* 1000 * 10`
/// gives 100 ns units) into a `FILETIME`. The C multiplication is signed and can
/// overflow; the port wraps, matching the value the x86-64 build produces.
pub fn due_time(timeout: i64) -> FileTime {
    let t = timeout.wrapping_mul(1000).wrapping_mul(10);
    FileTime {
        low: t as u32,
        high: (t >> 32) as u32,
    }
}
