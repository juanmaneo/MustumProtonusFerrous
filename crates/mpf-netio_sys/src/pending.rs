//! The pending-I/O slots of a socket: the choice of a slot for a new request, and
//! the lookup of a slot by its thread pool wait.

/// A pending-I/O slot, as far as the slot choice looks at it.
pub struct Slot<C> {
    /// The slot holds an IRP (`io->irp` is not NULL).
    pub in_use: bool,
    /// The callback the slot was last used with (`io->callback`).
    pub callback: Option<C>,
}

/// Picks the slot for a request that completes through `callback`, as
/// `allocate_pending_io` does. A free slot last used with `callback` is preferred,
/// so its wait object can be reused. Otherwise the first free slot is taken.
/// `None` means that every slot is in use.
pub fn select_slot<C: PartialEq>(slots: &[Slot<C>], callback: &C) -> Option<usize> {
    let mut first_free = None;
    for (i, slot) in slots.iter().enumerate() {
        if slot.in_use {
            continue;
        }
        if slot.callback.as_ref() == Some(callback) {
            return Some(i);
        }
        first_free.get_or_insert(i);
    }
    first_free
}

/// Finds the slot whose thread pool wait is `wait`, as `find_pending_io` does.
/// A slot without a wait never matches.
pub fn find_wait<W: PartialEq>(waits: &[Option<W>], wait: &W) -> Option<usize> {
    waits.iter().position(|w| w.as_ref() == Some(wait))
}
