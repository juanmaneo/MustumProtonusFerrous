//! Work item keys from `get_item_key`, `generate_item_key`, and the key tests in
//! `queue_cancel_item`. The mask lives in the high word and the counter in the low word.

const WAIT_ITEM_KEY_MASK: u32 = 0x8200_0000;
const SCHEDULED_ITEM_KEY_MASK: u32 = 0x8000_0000;

/// `get_item_key`.
pub fn item_key(mask: u32, key: u32) -> u64 {
    (u64::from(mask) << 32) | u64::from(key)
}

/// The `next_item_key` counter. Starts at zero and wraps like `InterlockedIncrement`.
#[derive(Default)]
pub struct ItemKeyGenerator {
    pub(crate) next: i32,
}

impl ItemKeyGenerator {
    /// `generate_item_key`: increments the counter, then builds the key from it.
    pub fn generate(&mut self, mask: u32) -> u64 {
        self.next = self.next.wrapping_add(1);
        item_key(mask, self.next as u32)
    }
}

/// The kind of pending item a key refers to, as `queue_cancel_item` tests it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemKind {
    Wait,
    Scheduled,
    Unknown,
}

/// `queue_cancel_item`'s key tests on the high word. The wait mask is checked first,
/// so a key with both mask bits set is a wait item.
pub fn item_kind(key: u64) -> ItemKind {
    let high = (key >> 32) as u32;
    if (high & WAIT_ITEM_KEY_MASK) == WAIT_ITEM_KEY_MASK {
        ItemKind::Wait
    } else if (high & SCHEDULED_ITEM_KEY_MASK) == SCHEDULED_ITEM_KEY_MASK {
        ItemKind::Scheduled
    } else {
        ItemKind::Unknown
    }
}
