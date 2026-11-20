//! Headless subset of Wine's `dlls/msctf/msctf.c`: the cookie table behind the `DWORD` handles
//! that msctf returns from its advise calls.
//!
//! Ported (the vectors in `oracle_checks.rs` come from Wine's C built with ASan and UBSan, and
//! `edge_checks.rs` covers the rest):
//! - `generate_Cookie` -> [`CookieTable::generate`]: the lowest free slot is reused, else a slot is
//!   appended. The id is the slot index plus one, so id 0 is never issued and reads as missing.
//!   Wine's out-of-memory return of 0 is not taken: a Rust allocation failure aborts.
//! - `get_Cookie_magic` and `get_Cookie_data` -> [`CookieTable::get_magic`] and
//!   [`CookieTable::get_data`]: an unknown or removed id gives `None`, where Wine gives 0 or NULL.
//! - `remove_Cookie` -> [`CookieTable::remove`]: frees the slot and hands back its payload.
//! - `enumerate_Cookie` -> [`CookieTable::enumerate`]: the first live id with the given magic at or
//!   after the cursor. A hit moves the cursor past it, and a miss leaves it alone.
//!
//! The payload is generic where Wine stores an `LPVOID`. Wine's 0 and NULL mean "no cookie", so a
//! stored zero magic or NULL payload cannot be told apart from a missing one in Wine; here a stored
//! value always reads back as `Some`.
//!
//! Replaced by ownership (no logic to port): the global `cookies` array and its `calloc` and
//! `_recalloc` growth. The table grows on demand and frees with its owner.
//!
//! Skipped:
//! - `advise_sink`, `unadvise_sink` and `free_sinks`: they store COM interface pointers and
//!   release them, so there is no value logic to test in isolation.
//! - `ClassFactory`, the activated text service list, `set_textservice_sink` and
//!   `get_textservice_sink`, the global compartment manager and the TLS and process ids: COM or
//!   per-process state.
//! - `TF_SELECTION_to_TS_SELECTION_ACP`: an unsafe downcast from an `ITfRange` followed by a copy
//!   of four fields, with no logic of its own.
//! - `next_LanguageProfile`, `add_userkey` and the other registry readers: the registry is outside
//!   the portable set.

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// The cookie table msctf hands out as `DWORD` ids. Slot `i` is id `i + 1`. A removed slot is
/// `None` and is reused by the next `generate`, as in Wine.
pub struct CookieTable<T> {
    slots: Vec<Option<Slot<T>>>,
}

struct Slot<T> {
    magic: u32,
    data: T,
}

impl<T> Default for CookieTable<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> CookieTable<T> {
    pub fn new() -> Self {
        Self { slots: Vec::new() }
    }

    /// Stores `data` under `magic` and returns its id, which is never 0.
    pub fn generate(&mut self, magic: u32, data: T) -> u32 {
        let index = match self.slots.iter().position(Option::is_none) {
            Some(index) => index,
            None => {
                self.slots.push(None);
                self.slots.len() - 1
            }
        };
        self.slots[index] = Some(Slot { magic, data });
        index as u32 + 1
    }

    fn slot(&self, id: u32) -> Option<&Slot<T>> {
        self.slots.get((id as usize).wrapping_sub(1))?.as_ref()
    }

    pub fn get_magic(&self, id: u32) -> Option<u32> {
        self.slot(id).map(|slot| slot.magic)
    }

    pub fn get_data(&self, id: u32) -> Option<&T> {
        self.slot(id).map(|slot| &slot.data)
    }

    /// Frees the slot for `id` and returns its payload, or `None` if the id is not live.
    pub fn remove(&mut self, id: u32) -> Option<T> {
        self.slots
            .get_mut((id as usize).wrapping_sub(1))?
            .take()
            .map(|slot| slot.data)
    }

    /// Returns the first live id with `magic` in the slots at or after the 0-based slot `*index`,
    /// or 0 when there is none. A hit sets `*index` to the slot after the returned one; a miss
    /// leaves it alone.
    pub fn enumerate(&self, magic: u32, index: &mut u32) -> u32 {
        let hit = self
            .slots
            .iter()
            .enumerate()
            .skip(*index as usize)
            .find(|(_, slot)| slot.as_ref().is_some_and(|slot| slot.magic == magic));
        match hit {
            Some((position, _)) => {
                *index = position as u32 + 1;
                *index
            }
            None => 0,
        }
    }
}
