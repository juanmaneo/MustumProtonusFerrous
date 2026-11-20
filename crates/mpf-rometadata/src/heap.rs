//! Metadata heap readers from `assembly.c`: compressed unsigned integers (`decode_int`), blobs
//! (`assembly_get_blob`), strings (`assembly_get_string`), GUIDs (`assembly_get_guid`), the
//! string heap trim (`trim_string_heap`) and the bounds test in `assembly_addr_offset`.
//!
//! Divergence: `blob_at` returns the declared size even when that size runs past the end of the
//! heap, as the C code does. The caller must check the size against the heap if it needs to.

/// The size of one GUID in the GUID heap (C `sizeof(GUID)`).
const GUID_SIZE: u32 = 16;

/// Decodes a compressed unsigned integer from the start of `encoded` (C `decode_int`). Returns
/// the value and the number of bytes it occupies. A leading byte of `0xE0` or more is invalid.
pub fn decode_compressed_uint(encoded: &[u8]) -> Option<(u32, usize)> {
    match *encoded {
        [b0, ..] if (b0 & 0x80) == 0 => Some((u32::from(b0), 1)),
        [b0, b1, ..] if (b0 & 0x40) == 0 => Some(((u32::from(b0 & 0x3f) << 8) + u32::from(b1), 2)),
        [b0, b1, b2, b3, ..] if (b0 & 0x20) == 0 => Some((
            (u32::from(b0 & 0x1f) << 24)
                + (u32::from(b1) << 16)
                + (u32::from(b2) << 8)
                + u32::from(b3),
            4,
        )),
        _ => None,
    }
}

/// Reads the blob at `idx` (C `assembly_get_blob`). `heap` is the whole blob heap. Returns the
/// declared size and the bytes after the length prefix, through the end of the heap. The prefix
/// must end inside the heap.
pub fn blob_at(heap: &[u8], idx: u32) -> Option<(u32, &[u8])> {
    let rest = heap.get(idx as usize..)?;
    let (size, prefix) = decode_compressed_uint(rest)?;
    if prefix >= rest.len() {
        return None;
    }
    Some((size, &rest[prefix..]))
}

/// The string at `idx`, from `idx` to the end of the string heap (C `assembly_get_string`). `None`
/// when `idx` is at or past the end.
pub fn string_at(heap: &[u8], idx: u32) -> Option<&[u8]> {
    let idx = idx as usize;
    if idx < heap.len() {
        Some(&heap[idx..])
    } else {
        None
    }
}

/// The byte offset of the 1-based GUID `idx` in a GUID heap of `heap_len` bytes (C
/// `assembly_get_guid`). The offset is computed in 32 bits, so it wraps as in C. `None` when the
/// offset is past the end of the heap.
pub fn guid_offset(idx: u32, heap_len: u32) -> Option<u32> {
    let offset = idx.wrapping_sub(1).wrapping_mul(GUID_SIZE);
    (offset < heap_len).then_some(offset)
}

/// Trims the padding NULs from the end of the string heap (C `trim_string_heap`). The first
/// entry must be empty, and one trailing NUL is kept. Returns `None` when the first byte is not
/// NUL, which C reports as a corrupt file.
pub fn trim_string_heap(heap: &[u8]) -> Option<&[u8]> {
    match heap.first() {
        None => Some(heap),
        Some(first) if *first != 0 => None,
        Some(_) => {
            let mut end = heap.len();
            while end > 1 && heap[end - 1] == 0 && heap[end - 2] == 0 {
                end -= 1;
            }
            Some(&heap[..end])
        }
    }
}

/// Whether `size` bytes starting at `offset` lie inside a buffer of `data_len` bytes (the bounds
/// test in C `assembly_addr_offset`).
pub fn span_in_bounds(data_len: u64, offset: u32, size: u64) -> bool {
    u64::from(offset) < data_len && size <= data_len - u64::from(offset)
}
