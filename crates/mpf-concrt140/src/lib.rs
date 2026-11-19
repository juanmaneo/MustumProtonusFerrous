//! concrt140 subset for MPF: the headless index and sizing helpers of `dlls/concrt140/details.c`.
//!
//! Reference Wine's `dlls/concrt140` loads `msvcp140.dll` and registers the concurrency RTTI.
//! Most of `details.c` touches threads, the CRT heap, the thread pool, COM or SEH. This crate ports
//! the pure arithmetic that `details.c` applies to its values. It works on integers and on a slot
//! table, with no heap, locks or threads. Sizes follow the 64-bit `size_t` of the x64 build.
//!
//! Ported (pure logic, verified against the verbatim C by oracle rows and edge tests):
//! - `segment_index_of` (`_vector_base_v4__Segment_index_of`): the segment that holds an index.
//! - `vector_capacity` (`_Concurrent_vector_base_v4__Internal_capacity`): the capacity implied by
//!   the segment table.
//! - `queue_alloc_count` (`_Concurrent_queue_base_v4_ctor`): items per page for an item size.
//! - `queue_is_empty` and `queue_size` (`_Concurrent_queue_base_v4__Internal_empty` and `_size`).
//! - `byte_reverse` (`_Byte_reverse_table`): the bit reversal of one byte.
//!
//! Deviations from the C, all in edge cases the oracle excludes:
//! - The C shifts an `int` by `i` in `_Internal_capacity`. UBSan flags this for `i >= 31`
//!   (`left shift of 1 by 31 places cannot be represented in type 'int'`). This crate shifts a
//!   `u64` instead, so the result is the intended `2^i`.
//! - `2^64` does not fit in `size_t`. When all 64 heap slots are allocated, the C shifts by 64
//!   (undefined behaviour). This crate saturates at `u64::MAX`.
//! - A slot past the end of the slice counts as `Empty`.
//!
//! Skipped (reason per item):
//! - `_Concurrent_queue_base_v4__Internal_push`, `_move_push`, `_pop_if_present`, `_swap`,
//!   `_finish_clear`, `threadsafe_queue_push`, `threadsafe_queue_pop`, `queue_push_finally`,
//!   `spin_wait`: page allocation through the vtable, interlocked counters, and threads.
//! - `_Concurrent_queue_base_v4_ctor`, `_dtor`, `_vector_dtor`: `operator new` and `operator delete`.
//!   Only the `alloc_count` choice is ported.
//! - `_Concurrent_vector_base_v4__Internal_reserve`, `_clear`, `_compact`, `_copy`, `_assign`,
//!   `_grow_by`, `_grow_to_at_least_with_result`, `_push_back`, `_resize`, `_swap`, and
//!   `concurrent_vector_alloc_segment`: segment allocation with `malloc` and SEH, interlocked
//!   exchanges, and element callbacks. `_vector_base_v4__Internal_throw_exception` raises CRT
//!   exceptions.
//! - `_Runtime_object_ctor`, `_Runtime_object__GetId`: ids come from an interlocked counter, and the
//!   getter only reads a field.
//! - `_Schedule_chore`, `_Release_chore`, `threadpool_callback`: the Windows thread pool.
//! - `_ContextCallback_*`, `_TaskEventLogger_*`, `_Task_impl_base__IsNonBlockingThread`,
//!   `_ExceptionHolder__ReportUnhandledError`, `_GetCombinableSize`, and
//!   `is_current_task_group_canceling`: context and task-runtime calls, or `FIXME` stubs.
//! - `DllMain`, `init_concurrency_details`, `operator_new`, `operator_delete`, and the RTTI and
//!   vtable tables: module loading and process state.
//! - The `TRACE` and `FIXME` logging.

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// `STORAGE_SIZE`: the inline segment slots of a `_Concurrent_vector_base_v4`.
const STORAGE_SIZE: usize = 3;
/// `SEGMENT_SIZE`: the slots of a heap segment table, `sizeof(void*) * 8`.
const SEGMENT_SIZE: usize = 64;

/// The state of one slot in a vector's segment table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Segment {
    /// A NULL pointer: the segment is not allocated.
    Empty,
    /// `SEGMENT_ALLOC_MARKER`: another thread is allocating the segment.
    AllocMarker,
    /// A non-NULL pointer: the segment is allocated.
    Allocated,
}

/// `log2i`: the index of the highest set bit of `x | 1`, so 0 for `x == 0`.
fn log2i(x: u32) -> u32 {
    31 - (x | 1).leading_zeros()
}

/// `_vector_base_v4__Segment_index_of`: the segment that holds element `x`.
pub fn segment_index_of(x: u64) -> u32 {
    let high = (x >> 32) as u32;
    if high != 0 {
        log2i(high) + 32
    } else {
        log2i(x as u32)
    }
}

/// `_Concurrent_vector_base_v4__Internal_capacity`: the capacity implied by the segment table.
///
/// `in_storage` selects the inline table (`STORAGE_SIZE` slots) instead of the heap table
/// (`SEGMENT_SIZE` slots). The capacity is `2^i`, where `i` is the first slot that is not
/// allocated, or 0 when slot 0 is not allocated.
pub fn vector_capacity(segments: &[Segment], in_storage: bool) -> u64 {
    let last_block = if in_storage {
        STORAGE_SIZE
    } else {
        SEGMENT_SIZE
    };
    let index = (0..last_block)
        .find(|&i| segments.get(i).copied().unwrap_or(Segment::Empty) != Segment::Allocated)
        .unwrap_or(last_block);
    if index == 0 {
        0
    } else {
        1u64.checked_shl(index as u32).unwrap_or(u64::MAX)
    }
}

/// `_Concurrent_queue_base_v4_ctor`: the items per page for an item size, a power of two.
pub fn queue_alloc_count(item_size: u64) -> u64 {
    match item_size {
        0..=8 => 32,
        9..=16 => 16,
        17..=32 => 8,
        33..=64 => 4,
        65..=128 => 2,
        _ => 1,
    }
}

/// `_Concurrent_queue_base_v4__Internal_empty`.
pub fn queue_is_empty(head_pos: u64, tail_pos: u64) -> bool {
    head_pos == tail_pos
}

/// `_Concurrent_queue_base_v4__Internal_size`: `tail_pos - head_pos` in `size_t` arithmetic, so it
/// wraps when `tail_pos < head_pos`, as the C does.
pub fn queue_size(head_pos: u64, tail_pos: u64) -> u64 {
    tail_pos.wrapping_sub(head_pos)
}

/// `_Byte_reverse_table` (`?_Byte_reverse_table@details@Concurrency@@3QBEB`): the bit reversal of
/// each byte.
const BYTE_REVERSE_TABLE: [u8; 256] = [
    0x00, 0x80, 0x40, 0xc0, 0x20, 0xa0, 0x60, 0xe0, 0x10, 0x90, 0x50, 0xd0, 0x30, 0xb0, 0x70, 0xf0,
    0x08, 0x88, 0x48, 0xc8, 0x28, 0xa8, 0x68, 0xe8, 0x18, 0x98, 0x58, 0xd8, 0x38, 0xb8, 0x78, 0xf8,
    0x04, 0x84, 0x44, 0xc4, 0x24, 0xa4, 0x64, 0xe4, 0x14, 0x94, 0x54, 0xd4, 0x34, 0xb4, 0x74, 0xf4,
    0x0c, 0x8c, 0x4c, 0xcc, 0x2c, 0xac, 0x6c, 0xec, 0x1c, 0x9c, 0x5c, 0xdc, 0x3c, 0xbc, 0x7c, 0xfc,
    0x02, 0x82, 0x42, 0xc2, 0x22, 0xa2, 0x62, 0xe2, 0x12, 0x92, 0x52, 0xd2, 0x32, 0xb2, 0x72, 0xf2,
    0x0a, 0x8a, 0x4a, 0xca, 0x2a, 0xaa, 0x6a, 0xea, 0x1a, 0x9a, 0x5a, 0xda, 0x3a, 0xba, 0x7a, 0xfa,
    0x06, 0x86, 0x46, 0xc6, 0x26, 0xa6, 0x66, 0xe6, 0x16, 0x96, 0x56, 0xd6, 0x36, 0xb6, 0x76, 0xf6,
    0x0e, 0x8e, 0x4e, 0xce, 0x2e, 0xae, 0x6e, 0xee, 0x1e, 0x9e, 0x5e, 0xde, 0x3e, 0xbe, 0x7e, 0xfe,
    0x01, 0x81, 0x41, 0xc1, 0x21, 0xa1, 0x61, 0xe1, 0x11, 0x91, 0x51, 0xd1, 0x31, 0xb1, 0x71, 0xf1,
    0x09, 0x89, 0x49, 0xc9, 0x29, 0xa9, 0x69, 0xe9, 0x19, 0x99, 0x59, 0xd9, 0x39, 0xb9, 0x79, 0xf9,
    0x05, 0x85, 0x45, 0xc5, 0x25, 0xa5, 0x65, 0xe5, 0x15, 0x95, 0x55, 0xd5, 0x35, 0xb5, 0x75, 0xf5,
    0x0d, 0x8d, 0x4d, 0xcd, 0x2d, 0xad, 0x6d, 0xed, 0x1d, 0x9d, 0x5d, 0xdd, 0x3d, 0xbd, 0x7d, 0xfd,
    0x03, 0x83, 0x43, 0xc3, 0x23, 0xa3, 0x63, 0xe3, 0x13, 0x93, 0x53, 0xd3, 0x33, 0xb3, 0x73, 0xf3,
    0x0b, 0x8b, 0x4b, 0xcb, 0x2b, 0xab, 0x6b, 0xeb, 0x1b, 0x9b, 0x5b, 0xdb, 0x3b, 0xbb, 0x7b, 0xfb,
    0x07, 0x87, 0x47, 0xc7, 0x27, 0xa7, 0x67, 0xe7, 0x17, 0x97, 0x57, 0xd7, 0x37, 0xb7, 0x77, 0xf7,
    0x0f, 0x8f, 0x4f, 0xcf, 0x2f, 0xaf, 0x6f, 0xef, 0x1f, 0x9f, 0x5f, 0xdf, 0x3f, 0xbf, 0x7f, 0xff,
];

/// The bit reversal of `byte`, as `_Byte_reverse_table` maps it.
pub fn byte_reverse(byte: u8) -> u8 {
    BYTE_REVERSE_TABLE[usize::from(byte)]
}
