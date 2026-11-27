//! Port of the item-array functions in Wine's `dlls/comctl32/dsa.c`.
//!
//! The C code allocates from the process heap. Here each `Alloc` and `ReAlloc` is one call
//! to the `alloc` callback with the byte size; the callback returns `false` to fail that call.
//! Frees are not modelled.

/// `sizeof(DSA)` on the 64-bit target.
const DSA_STRUCT_SIZE: usize = 32;

/// A `DSA`. `data` holds `max_count` items of `item_size` bytes each. It is empty for a NULL
/// block, which `DSA_Create` and `DSA_DeleteAllItems` leave behind.
pub struct Dsa {
    pub item_count: i32,
    pub data: Vec<u8>,
    pub max_count: i32,
    pub item_size: i32,
    pub grow: i32,
}

/// The byte size of one item. The C code passes it to `memmove` as a `size_t`, so a negative
/// size crashes on the first copy. The port panics at the same point.
fn item_bytes(item_size: i32) -> usize {
    usize::try_from(item_size).expect("negative DSA item size")
}

/// The `DWORD` size that `ReAlloc` receives for an `INT` byte count.
fn dword_size(size: i32) -> usize {
    size as u32 as usize
}

fn item(dsa: &Dsa, index: i32) -> &[u8] {
    let size = item_bytes(dsa.item_size);
    let start = index as usize * size;
    &dsa.data[start..start + size]
}

/// `DSA_Create`. A failed header allocation returns `None`. The grow size is at least 1.
pub fn dsa_create(
    item_size: i32,
    grow: i32,
    alloc: &mut dyn FnMut(usize) -> bool,
) -> Option<Box<Dsa>> {
    if !alloc(DSA_STRUCT_SIZE) {
        return None;
    }
    Some(Box::new(Dsa {
        item_count: 0,
        data: Vec::new(),
        max_count: 0,
        item_size,
        grow: grow.max(1),
    }))
}

/// `DSA_GetItem`: copies item `index` into `dest`. Returns `false` for a NULL handle or an
/// index out of range. `dest` must hold at least one item.
pub fn dsa_get_item(dsa: Option<&Dsa>, index: i32, dest: &mut [u8]) -> bool {
    match dsa {
        Some(dsa) if index >= 0 && index < dsa.item_count => {
            let src = item(dsa, index);
            dest[..src.len()].copy_from_slice(src);
            true
        }
        _ => false,
    }
}

/// `DSA_GetItemPtr`: the item at `index`, or `None` for a NULL handle or an index out of range.
pub fn dsa_get_item_ptr(dsa: Option<&Dsa>, index: i32) -> Option<&[u8]> {
    match dsa {
        Some(dsa) if index >= 0 && index < dsa.item_count => Some(item(dsa, index)),
        _ => None,
    }
}

/// `DSA_SetItem`: stores `src` as item `index`. An index past the end extends the item count.
/// Returns `false` for a NULL handle, a negative index, or a failed block allocation.
pub fn dsa_set_item(
    dsa: Option<&mut Dsa>,
    index: i32,
    src: &[u8],
    alloc: &mut dyn FnMut(usize) -> bool,
) -> bool {
    match dsa {
        Some(dsa) if index >= 0 => set_item(dsa, index, src, alloc),
        _ => false,
    }
}

fn set_item(dsa: &mut Dsa, index: i32, src: &[u8], alloc: &mut dyn FnMut(usize) -> bool) -> bool {
    if dsa.item_count <= index {
        if dsa.max_count > index {
            dsa.item_count = index + 1;
        } else {
            let new_items = dsa.grow * (index / dsa.grow + 1);
            if !alloc(dword_size(dsa.item_size.wrapping_mul(new_items))) {
                return false;
            }
            // The size check comes after the allocation, as in the C code, so that a negative
            // item size panics before the block is resized.
            dsa.data
                .resize(item_bytes(dsa.item_size) * new_items as usize, 0);
            dsa.max_count = new_items;
            dsa.item_count = index + 1;
        }
    }
    let size = item_bytes(dsa.item_size);
    let start = index as usize * size;
    dsa.data[start..start + size].copy_from_slice(&src[..size]);
    true
}

/// `DSA_InsertItem`. An index past the end appends. Returns the index used, or -1 for a NULL
/// handle, a negative index, a size overflow, or a failed block allocation.
pub fn dsa_insert_item(
    dsa: Option<&mut Dsa>,
    index: i32,
    src: &[u8],
    alloc: &mut dyn FnMut(usize) -> bool,
) -> i32 {
    match dsa {
        Some(dsa) if index >= 0 => insert_item(dsa, index, src, alloc),
        _ => -1,
    }
}

fn insert_item(dsa: &mut Dsa, index: i32, src: &[u8], alloc: &mut dyn FnMut(usize) -> bool) -> i32 {
    let index = index.min(dsa.item_count);
    if dsa.item_count >= dsa.max_count {
        let new_items = dsa.max_count + dsa.grow;
        let size = dsa.item_size.wrapping_mul(new_items);
        // A zero item size divides by zero here, as the C code does.
        if size / dsa.item_size != new_items {
            return -1;
        }
        if !alloc(dword_size(size)) {
            return -1;
        }
        dsa.data
            .resize(item_bytes(dsa.item_size) * new_items as usize, 0);
        dsa.max_count = new_items;
    }
    let size = item_bytes(dsa.item_size);
    if index < dsa.item_count {
        let start = index as usize * size;
        let end = dsa.item_count as usize * size;
        dsa.data.copy_within(start..end, start + size);
    }
    dsa.item_count += 1;
    let start = index as usize * size;
    dsa.data[start..start + size].copy_from_slice(&src[..size]);
    index
}

/// `DSA_DeleteItem`. Returns the index removed, or -1 for a NULL handle, an index out of
/// range, or a failed shrinking reallocation. A failed shrink leaves the item removed but the
/// block unchanged.
pub fn dsa_delete_item(
    dsa: Option<&mut Dsa>,
    index: i32,
    alloc: &mut dyn FnMut(usize) -> bool,
) -> i32 {
    match dsa {
        Some(dsa) if index >= 0 && index < dsa.item_count => delete_item(dsa, index, alloc),
        _ => -1,
    }
}

fn delete_item(dsa: &mut Dsa, index: i32, alloc: &mut dyn FnMut(usize) -> bool) -> i32 {
    let size = item_bytes(dsa.item_size);
    if index < dsa.item_count - 1 {
        let start = index as usize * size;
        let end = dsa.item_count as usize * size;
        dsa.data.copy_within(start + size..end, start);
    }
    dsa.item_count -= 1;
    if dsa.max_count - dsa.item_count >= dsa.grow {
        if !alloc(size * dsa.item_count as usize) {
            return -1;
        }
        dsa.max_count = dsa.item_count;
        dsa.data.truncate(size * dsa.item_count as usize);
    }
    index
}

/// `DSA_DeleteAllItems`: drops the items and the block. Frees are not modelled.
pub fn dsa_delete_all_items(dsa: Option<&mut Dsa>) -> bool {
    match dsa {
        Some(dsa) => {
            dsa.item_count = 0;
            dsa.data.clear();
            dsa.max_count = 0;
            true
        }
        None => false,
    }
}

/// `DSA_EnumCallback`: calls `keep_going` with each item in order, and stops when it returns `false`.
pub fn dsa_enum_callback(dsa: Option<&Dsa>, keep_going: &mut dyn FnMut(&[u8]) -> bool) {
    let Some(dsa) = dsa else {
        return;
    };
    for index in 0..dsa.item_count {
        if !keep_going(item(dsa, index)) {
            return;
        }
    }
}

/// `DSA_Destroy`: returns `false` for a NULL handle. Frees are not modelled.
pub fn dsa_destroy(dsa: Option<Box<Dsa>>) -> bool {
    dsa.is_some()
}

/// `DSA_DestroyCallback`: enumerates the items, then destroys the handle.
pub fn dsa_destroy_callback(dsa: Option<Box<Dsa>>, keep_going: &mut dyn FnMut(&[u8]) -> bool) {
    dsa_enum_callback(dsa.as_deref(), keep_going);
    dsa_destroy(dsa);
}
