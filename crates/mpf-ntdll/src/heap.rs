//! Process heap: `RtlAllocateHeap` / `RtlFreeHeap` equivalents.
//!
//! Mirrors `dlls/ntdll/heap.c` in reference Wine at the API level. The
//! backing store is Rust's global allocator; block layouts are tracked so
//! `RtlFreeHeap` can validate and deallocate safely.

use crate::ntstatus::{NtStatus, STATUS_INVALID_HANDLE, STATUS_SUCCESS};
use std::alloc::{alloc, dealloc, Layout};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

const HEAP_ALIGN: usize = 16;

pub struct Heap {
    blocks: Mutex<HashMap<usize, Layout>>,
}

impl Heap {
    pub fn new() -> Self {
        Heap {
            blocks: Mutex::new(HashMap::new()),
        }
    }

    /// Allocates `size` bytes; returns null on failure or size 0, like
    /// RtlAllocateHeap with a NULL result on failure.
    pub fn allocate(&self, size: usize, zero_memory: bool) -> *mut u8 {
        if size == 0 {
            return core::ptr::null_mut();
        }
        let layout = Layout::from_size_align(size, HEAP_ALIGN).unwrap();
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return core::ptr::null_mut();
        }
        if zero_memory {
            unsafe { ptr.write_bytes(0, size) };
        }
        self.blocks.lock().unwrap().insert(ptr as usize, layout);
        ptr
    }

    /// Frees a block previously returned by `allocate`. Returns false (and
    /// does not deallocate) for unknown pointers.
    pub fn free(&self, ptr: *mut u8) -> bool {
        if ptr.is_null() {
            return false;
        }
        let block = self.blocks.lock().unwrap().remove_entry(&(ptr as usize));
        match block {
            // Free exactly the block registered by `allocate`, never an
            // arbitrary caller-supplied pointer.
            Some((addr, layout)) => {
                unsafe { dealloc(addr as *mut u8, layout) };
                true
            }
            None => false,
        }
    }

    pub fn size_of(&self, ptr: *const u8) -> Option<usize> {
        self.blocks
            .lock()
            .unwrap()
            .get(&(ptr as usize))
            .map(|l| l.size())
    }
}

impl Default for Heap {
    fn default() -> Self {
        Self::new()
    }
}

/// The process heap, like `RtlProcessHeap()` in ntdll.
pub fn rtl_process_heap() -> &'static Heap {
    static HEAP: OnceLock<Heap> = OnceLock::new();
    HEAP.get_or_init(Heap::new)
}

pub fn rtl_allocate_heap(heap: &Heap, size: usize, zero_memory: bool) -> *mut u8 {
    heap.allocate(size, zero_memory)
}

/// Returns STATUS_SUCCESS on success, STATUS_INVALID_HANDLE for unknown
/// pointers or NULL (freeing NULL is a no-op success in ntdll's heap).
pub fn rtl_free_heap(heap: &Heap, ptr: *mut u8) -> NtStatus {
    if ptr.is_null() {
        return STATUS_SUCCESS;
    }
    if heap.free(ptr) {
        STATUS_SUCCESS
    } else {
        STATUS_INVALID_HANDLE
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ntstatus::is_success;

    #[test]
    fn allocate_zeroes_on_request() {
        let heap = Heap::new();
        let p = heap.allocate(64, true);
        assert!(!p.is_null());
        let bytes = unsafe { std::slice::from_raw_parts(p, 64) };
        assert!(bytes.iter().all(|&b| b == 0));
        assert_eq!(heap.size_of(p), Some(64));
        assert!(heap.free(p));
    }

    #[test]
    fn allocate_without_zero_keeps_contents_writable() {
        let heap = Heap::new();
        let p = heap.allocate(8, false);
        unsafe { p.write_bytes(0xAA, 8) };
        assert_eq!(unsafe { *p }, 0xAA);
        assert!(heap.free(p));
    }

    #[test]
    fn zero_size_and_unknown_pointers() {
        let heap = Heap::new();
        assert!(heap.allocate(0, false).is_null());
        assert!(!heap.free(0x1234 as *mut u8));
        assert!(!heap.free(core::ptr::null_mut()));
    }

    #[test]
    fn rtl_wrappers_and_process_heap() {
        let heap = rtl_process_heap();
        let p = rtl_allocate_heap(heap, 32, true);
        assert!(!p.is_null());
        assert!(is_success(rtl_free_heap(heap, p)));
        assert_eq!(
            rtl_free_heap(heap, 0x1234 as *mut u8),
            STATUS_INVALID_HANDLE
        );
        assert!(is_success(rtl_free_heap(heap, core::ptr::null_mut())));
        assert!(std::ptr::eq(rtl_process_heap(), heap));
    }
}
