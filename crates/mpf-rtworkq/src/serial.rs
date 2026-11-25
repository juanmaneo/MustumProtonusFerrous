//! Serial queue sequencing from `serial_queue_submit`, `serial_queue_is_ack_token`, and
//! `serial_queue_finalization_callback`, for a pool target.
//!
//! C keeps items on an intrusive `struct list` and compares pointers. This model keeps
//! the same order and returns the item that C would hand to the target queue. Tokens
//! and items are opaque values that compare by equality.

use std::collections::VecDeque;

/// One `serial_queue_submit` call.
pub struct Submission<T> {
    /// The work item being submitted.
    pub item: T,
    /// `item->result`. An acknowledgement carries the reply token of the item it acknowledges.
    pub result: T,
    /// `item->reply_result`, present when `RTWQ_REPLY_CALLBACK` is set.
    pub reply: Option<T>,
}

struct Pending<T> {
    item: T,
    reply: Option<T>,
}

/// The pending items of a serial queue whose target is a pool queue.
pub struct SerialQueue<T> {
    pending: VecDeque<Pending<T>>,
}

impl<T> Default for SerialQueue<T> {
    fn default() -> Self {
        Self {
            pending: VecDeque::new(),
        }
    }
}

impl<T: Copy + PartialEq> SerialQueue<T> {
    /// `serial_queue_submit`. Returns the item to hand to the target queue now, if any.
    ///
    /// When `result` equals the reply token of the head, the submission acknowledges the
    /// head: the head is removed and the next item is returned. Anything else is queued,
    /// and it is dispatched only if the queue was empty.
    ///
    /// C never passes a NULL `result` (`alloc_work_item` dereferences it), so a head with
    /// no reply token (`reply: None`) is never acknowledged. C also requires the result's
    /// `pCallback` to be this queue's callback; the caller must pass only this queue's tokens.
    pub fn submit(&mut self, sub: Submission<T>) -> Option<T> {
        let ack = self
            .pending
            .front()
            .is_some_and(|head| head.reply == Some(sub.result));
        if ack {
            self.pending.pop_front();
            return self.pending.front().map(|next| next.item);
        }
        let dispatch = self.pending.is_empty().then_some(sub.item);
        self.pending.push_back(Pending {
            item: sub.item,
            reply: sub.reply,
        });
        dispatch
    }

    /// `serial_queue_finalization_callback`: removes one entry equal to `item` and returns
    /// the item to hand to the target queue now, if any. Wine only finishes the head;
    /// removing another pending item is a model extension.
    pub fn finish(&mut self, item: T) -> Option<T> {
        if let Some(index) = self.pending.iter().position(|p| p.item == item) {
            self.pending.remove(index);
        }
        self.pending.front().map(|next| next.item)
    }
}
