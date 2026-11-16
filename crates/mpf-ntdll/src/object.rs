//! Object manager basics: the handle table, mirroring the handle-allocation
//! part of `dlls/ntdll/om.c` in reference Wine. Handles are 4-aligned
//! values, as on Windows.

use crate::ntstatus::{NtStatus, STATUS_INVALID_HANDLE, STATUS_SUCCESS};
use std::collections::HashMap;

pub struct ObjectManager {
    next: u64,
    objects: HashMap<u64, u32>,
}

impl ObjectManager {
    pub fn new() -> Self {
        ObjectManager {
            next: 4, // handle 0 is never a valid object handle
            objects: HashMap::new(),
        }
    }

    /// Allocates a handle for an object of `object_type` (an OBJECT_TYPE_* tag).
    pub fn allocate_handle(&mut self, object_type: u32) -> u64 {
        let handle = self.next;
        self.next += 4;
        self.objects.insert(handle, object_type);
        handle
    }

    pub fn close_handle(&mut self, handle: u64) -> NtStatus {
        match self.objects.remove(&handle) {
            Some(_) => STATUS_SUCCESS,
            None => STATUS_INVALID_HANDLE,
        }
    }

    pub fn query_object(&self, handle: u64) -> Option<u32> {
        self.objects.get(&handle).copied()
    }

    pub fn handle_count(&self) -> usize {
        self.objects.len()
    }
}

impl Default for ObjectManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ntstatus::is_success;

    #[test]
    fn handles_are_4_aligned_and_unique() {
        let mut om = ObjectManager::new();
        let h1 = om.allocate_handle(1);
        let h2 = om.allocate_handle(2);
        assert_eq!(h1 % 4, 0);
        assert_ne!(h1, h2);
        assert_eq!(om.query_object(h1), Some(1));
        assert_eq!(om.query_object(h2), Some(2));
        assert_eq!(om.handle_count(), 2);
    }

    #[test]
    fn close_handle_semantics() {
        let mut om = ObjectManager::new();
        let h = om.allocate_handle(7);
        assert!(is_success(om.close_handle(h)));
        assert_eq!(om.query_object(h), None);
        assert_eq!(om.close_handle(h), STATUS_INVALID_HANDLE);
        assert_eq!(om.close_handle(0), STATUS_INVALID_HANDLE);
    }
}
