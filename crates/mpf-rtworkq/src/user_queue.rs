//! User queue handles from `get_queue_obj` and `alloc_user_queue`. A handle is
//! `(slot + FIRST_USER_QUEUE_HANDLE) << 16 | generation`.

const FIRST_USER_QUEUE_HANDLE: u16 = 5;
const MAX_USER_QUEUE_HANDLES: usize = 124;

/// One `struct queue_handle` entry. A zero `refcount` marks a free slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UserQueueSlot {
    pub refcount: i32,
    pub generation: u16,
}

/// `alloc_user_queue`: the handle for `slot` at `generation`. `slot` must be below 124.
pub fn encode_user_queue_id(slot: usize, generation: u16) -> u32 {
    (((slot as u32) + u32::from(FIRST_USER_QUEUE_HANDLE)) << 16) | u32::from(generation)
}

/// `get_queue_obj`: splits a handle into `(slot, generation)`. The high word is
/// reduced with a wrapping subtraction, so values below 5 land outside the table.
pub fn decode_user_queue_id(queue_id: u32) -> Option<(usize, u16)> {
    let slot = usize::from(((queue_id >> 16) as u16).wrapping_sub(FIRST_USER_QUEUE_HANDLE));
    (slot < MAX_USER_QUEUE_HANDLES).then_some((slot, queue_id as u16))
}

/// `get_queue_obj`: the slot that `queue_id` refers to. A slot is live when its
/// `refcount` is nonzero and its generation matches. Slots past the end of `slots`
/// are not live.
pub fn find_user_queue(queue_id: u32, slots: &[UserQueueSlot]) -> Option<usize> {
    let (slot, generation) = decode_user_queue_id(queue_id)?;
    let entry = slots.get(slot)?;
    (entry.refcount != 0 && entry.generation == generation).then_some(slot)
}

/// `alloc_user_queue`: the next generation. C increments a `WORD` and compares the
/// stored value, so 0xfffe goes to 1 and 0xffff goes to 0.
pub fn next_queue_generation(generation: u16) -> u16 {
    let next = generation.wrapping_add(1);
    if next == 0xffff {
        1
    } else {
        next
    }
}
