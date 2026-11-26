/// `THUNK_POOL_SIZE`: how many thunks fit in one 4096-byte page.
pub fn pool_capacity(code_size: usize) -> usize {
    4096 / code_size
}

/// Byte offset of thunk `index` from the start of the code array
/// (`AtlThunk_DataToCode`).
pub fn thunk_code_offset(index: usize, code_size: usize) -> usize {
    index * code_size
}

/// `first_free` and `free_count` of `struct thunk_pool`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PoolCounters {
    pub first_free: u32,
    pub free_count: u32,
}

impl PoolCounters {
    /// Claims the next slot (`AtlThunk_AllocateData`). Returns the slot index and
    /// whether the pool is now exhausted, in which case the caller clears
    /// `current_pool`.
    pub fn allocate(&mut self, capacity: u32) -> (u32, bool) {
        let index = self.first_free;
        self.first_free += 1;
        (index, self.first_free == capacity)
    }

    /// Records one released slot (`AtlThunk_FreeData`). Returns true when every
    /// slot of the pool has been released and the page should be freed.
    pub fn free(&mut self, capacity: u32) -> bool {
        self.free_count += 1;
        self.free_count == capacity
    }

    /// `DllMain` test: the current pool may be released at process detach when
    /// no slot is outstanding.
    pub fn is_releasable(&self) -> bool {
        self.free_count == self.first_free
    }
}
