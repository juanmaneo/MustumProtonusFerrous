//! Port of the pointer-array functions in Wine's `dlls/comctl32/dpa.c`.
//!
//! The C code allocates from the process heap. Here each `HeapAlloc` and `HeapReAlloc`
//! is one call to the `alloc` callback with the byte size; the callback returns `false`
//! to fail that call. Frees are not modelled.

/// `sizeof(DPA)` on the 64-bit target.
const DPA_STRUCT_SIZE: usize = 32;

/// `sizeof(LPVOID)` on the 64-bit target.
const PTR_SIZE: usize = 8;

pub const DPAS_SORTED: u32 = 0x0001;
pub const DPAS_INSERTBEFORE: u32 = 0x0002;
pub const DPAS_INSERTAFTER: u32 = 0x0004;

/// A `DPA`. `ptrs` holds `max_count` slots. An empty `ptrs` is a NULL block, which
/// `DPA_CreateEx` and `DPA_DeleteAllPtrs` leave behind when their block allocation fails.
pub struct Dpa {
    pub item_count: i32,
    pub ptrs: Vec<usize>,
    pub grow: i32,
    pub max_count: i32,
}

/// A zeroed block of `slots` pointers, or an empty block when the allocation fails.
fn new_block(slots: i32, alloc: &mut dyn FnMut(usize) -> bool) -> Vec<usize> {
    let mut ptrs = Vec::new();
    resize_block(&mut ptrs, slots, alloc);
    ptrs
}

/// `HeapReAlloc` with `HEAP_ZERO_MEMORY`: the block keeps its items, and new slots are zero.
/// A failed call leaves the block unchanged.
fn resize_block(ptrs: &mut Vec<usize>, slots: i32, alloc: &mut dyn FnMut(usize) -> bool) -> bool {
    if !alloc(slots as usize * PTR_SIZE) {
        return false;
    }
    ptrs.resize(slots as usize, 0);
    true
}

/// `DPA_CreateEx`. A failed header allocation returns `None`. A failed block allocation
/// returns a handle with a NULL block, as the C code does.
pub fn dpa_create_ex(grow: i32, alloc: &mut dyn FnMut(usize) -> bool) -> Option<Box<Dpa>> {
    if !alloc(DPA_STRUCT_SIZE) {
        return None;
    }
    let grow = grow.max(8);
    let max_count = grow * 2;
    Some(Box::new(Dpa {
        item_count: 0,
        ptrs: new_block(max_count, alloc),
        grow,
        max_count,
    }))
}

/// `DPA_Create`, which is `DPA_CreateEx` on the process heap.
pub fn dpa_create(grow: i32, alloc: &mut dyn FnMut(usize) -> bool) -> Option<Box<Dpa>> {
    dpa_create_ex(grow, alloc)
}

/// `DPA_Grow`. A failed block allocation leaves the grow size unchanged.
pub fn dpa_grow(dpa: Option<&mut Dpa>, grow: i32, alloc: &mut dyn FnMut(usize) -> bool) -> bool {
    let Some(dpa) = dpa else {
        return false;
    };
    let grow = grow.max(8);
    let items = grow * ((dpa.max_count - 1) / grow + 1);
    if items > dpa.max_count {
        if !resize_block(&mut dpa.ptrs, items, alloc) {
            return false;
        }
        dpa.max_count = items;
    }
    dpa.grow = grow;
    true
}

/// `DPA_Clone` with a NULL destination. The header is allocated before the block. A failed
/// header allocation panics, because the C code dereferences the NULL result.
pub fn dpa_clone(src: Option<&Dpa>, alloc: &mut dyn FnMut(usize) -> bool) -> Option<Box<Dpa>> {
    let src = src?;
    if !alloc(DPA_STRUCT_SIZE) {
        panic!("DPA_Clone: header allocation failed");
    }
    let mut dst = Box::new(Dpa {
        item_count: 0,
        ptrs: Vec::new(),
        grow: src.grow,
        max_count: 0,
    });
    clone_into_handle(src, &mut dst, alloc);
    Some(dst)
}

/// `DPA_Clone` into an existing handle. Returns `false` when the source is NULL.
pub fn dpa_clone_into(
    src: Option<&Dpa>,
    dst: &mut Dpa,
    alloc: &mut dyn FnMut(usize) -> bool,
) -> bool {
    match src {
        Some(src) => {
            clone_into_handle(src, dst, alloc);
            true
        }
        None => false,
    }
}

/// The block-replacing part of `DPA_Clone`. The C code frees the old block first. Frees are
/// not modelled, and the block is replaced below either way. The copy panics when a failed
/// block allocation leaves an empty block that must hold items; the C code writes through
/// NULL there.
fn clone_into_handle(src: &Dpa, dst: &mut Dpa, alloc: &mut dyn FnMut(usize) -> bool) {
    let new_items = dst.grow * ((src.item_count - 1) / dst.grow + 1);
    dst.ptrs = new_block(new_items, alloc);
    dst.max_count = new_items;
    dst.item_count = src.item_count;
    let count = src.item_count as usize;
    dst.ptrs[..count].copy_from_slice(&src.ptrs[..count]);
}

/// `DPA_GetPtr`: the item at `index`, or 0 for a NULL handle, a NULL block or an index out of range.
pub fn dpa_get_ptr(dpa: Option<&Dpa>, index: isize) -> usize {
    match dpa {
        Some(dpa) if !dpa.ptrs.is_empty() && index >= 0 && index < dpa.item_count as isize => {
            dpa.ptrs[index as usize]
        }
        _ => 0,
    }
}

/// `DPA_GetPtrIndex`: the first index holding `p`, or -1.
pub fn dpa_get_ptr_index(dpa: Option<&Dpa>, p: usize) -> i32 {
    match dpa {
        Some(dpa) if !dpa.ptrs.is_empty() => dpa.ptrs[..dpa.item_count as usize]
            .iter()
            .position(|&ptr| ptr == p)
            .map_or(-1, |index| index as i32),
        _ => -1,
    }
}

/// `DPA_InsertPtr`. An index past the end appends. Returns the index used, or -1.
pub fn dpa_insert_ptr(
    dpa: Option<&mut Dpa>,
    index: i32,
    p: usize,
    alloc: &mut dyn FnMut(usize) -> bool,
) -> i32 {
    match dpa {
        Some(dpa) if index >= 0 => insert_ptr(dpa, index, p, alloc),
        _ => -1,
    }
}

fn insert_ptr(dpa: &mut Dpa, index: i32, p: usize, alloc: &mut dyn FnMut(usize) -> bool) -> i32 {
    let index = index.min(dpa.item_count);
    let count = dpa.item_count;
    if !set_ptr(dpa, count, 0, alloc) {
        return -1;
    }
    if index < count {
        dpa.ptrs
            .copy_within(index as usize..count as usize, index as usize + 1);
    }
    dpa.ptrs[index as usize] = p;
    index
}

/// `DPA_SetPtr`. Writing past the end extends the item count. Returns `false` for a NULL
/// handle, a negative index, or a failed block allocation.
pub fn dpa_set_ptr(
    dpa: Option<&mut Dpa>,
    index: i32,
    p: usize,
    alloc: &mut dyn FnMut(usize) -> bool,
) -> bool {
    match dpa {
        Some(dpa) if index >= 0 => set_ptr(dpa, index, p, alloc),
        _ => false,
    }
}

fn set_ptr(dpa: &mut Dpa, index: i32, p: usize, alloc: &mut dyn FnMut(usize) -> bool) -> bool {
    if dpa.item_count <= index {
        if dpa.max_count <= index {
            let new_items = dpa.grow * (index / dpa.grow + 1);
            if !resize_block(&mut dpa.ptrs, new_items, alloc) {
                return false;
            }
            dpa.max_count = new_items;
        }
        dpa.item_count = index + 1;
    }
    dpa.ptrs[index as usize] = p;
    true
}

/// `DPA_DeletePtr`. Returns the removed item, or 0 for an invalid index. When the shrinking
/// reallocation fails, the item is still removed but 0 is returned.
pub fn dpa_delete_ptr(
    dpa: Option<&mut Dpa>,
    index: i32,
    alloc: &mut dyn FnMut(usize) -> bool,
) -> usize {
    match dpa {
        Some(dpa) if index >= 0 && index < dpa.item_count => delete_ptr(dpa, index, alloc),
        _ => 0,
    }
}

fn delete_ptr(dpa: &mut Dpa, index: i32, alloc: &mut dyn FnMut(usize) -> bool) -> usize {
    let removed = dpa.ptrs[index as usize];
    if index < dpa.item_count - 1 {
        dpa.ptrs
            .copy_within(index as usize + 1..dpa.item_count as usize, index as usize);
    }
    dpa.item_count -= 1;
    if dpa.max_count - dpa.item_count >= dpa.grow {
        let new_items = (dpa.grow * 2).max(dpa.item_count);
        if !resize_block(&mut dpa.ptrs, new_items, alloc) {
            return 0;
        }
        dpa.max_count = new_items;
    }
    removed
}

/// `DPA_DeleteAllPtrs`. Replaces the block with a zeroed one of `2 * grow` slots. A failed
/// allocation leaves a NULL block, and the call still returns `true`, as in the C code.
pub fn dpa_delete_all_ptrs(dpa: Option<&mut Dpa>, alloc: &mut dyn FnMut(usize) -> bool) -> bool {
    match dpa {
        Some(dpa) => {
            dpa.item_count = 0;
            dpa.max_count = dpa.grow * 2;
            dpa.ptrs = new_block(dpa.max_count, alloc);
            true
        }
        None => false,
    }
}

/// `DPA_Sort`. Sorts the items with `cmp`, which returns a negative, zero or positive value.
pub fn dpa_sort(dpa: Option<&mut Dpa>, cmp: &mut dyn FnMut(usize, usize) -> i32) -> bool {
    match dpa {
        Some(dpa) => {
            if dpa.item_count > 1 && !dpa.ptrs.is_empty() {
                let last = dpa.item_count as usize - 1;
                quick_sort(&mut dpa.ptrs, 0, last, cmp);
            }
            true
        }
        None => false,
    }
}

/// `DPA_QuickSort`: sorts `ptrs[l..=r]`. The C code's `r < l` branch is unreachable from
/// `DPA_Sort`, so it is not ported.
fn quick_sort(
    ptrs: &mut [usize],
    mut l: usize,
    r: usize,
    cmp: &mut dyn FnMut(usize, usize) -> i32,
) {
    if l == r {
        return;
    }
    let mut m = (l + r) / 2;
    quick_sort(ptrs, l, m, cmp);
    quick_sort(ptrs, m + 1, r, cmp);
    while l <= m && m < r {
        if cmp(ptrs[l], ptrs[m + 1]) > 0 {
            let t = ptrs[m + 1];
            ptrs.copy_within(l..=m, l + 1);
            ptrs[l] = t;
            m += 1;
        }
        l += 1;
    }
}

/// `DPA_Search`. `DPAS_SORTED` selects a binary search, which ignores `start`, and
/// `DPAS_INSERTBEFORE` or `DPAS_INSERTAFTER` then return the insertion index. Otherwise
/// the search is linear from `start`. Returns -1 when nothing matches, and for a NULL
/// handle or a NULL `find` value (0).
pub fn dpa_search(
    dpa: Option<&Dpa>,
    find: usize,
    start: i32,
    cmp: &mut dyn FnMut(usize, usize) -> i32,
    options: u32,
) -> i32 {
    let dpa = match dpa {
        Some(dpa) if find != 0 => dpa,
        _ => return -1,
    };
    if (options & DPAS_SORTED) != 0 {
        let mut l = 0;
        let mut r = dpa.item_count - 1;
        while r >= l {
            let x = l + (r - l) / 2;
            let n = cmp(find, dpa.ptrs[x as usize]);
            if n == 0 {
                return x;
            } else if n < 0 {
                r = x - 1;
            } else {
                l = x + 1;
            }
        }
        if (options & (DPAS_INSERTBEFORE | DPAS_INSERTAFTER)) != 0 {
            return l;
        }
    } else {
        // The C code maps nStart == -1 to 0 and reads other negative starts out of bounds.
        // Clamping to 0 for every negative start is not oracle-derived.
        for index in start.max(0)..dpa.item_count {
            if cmp(find, dpa.ptrs[index as usize]) == 0 {
                return index;
            }
        }
    }
    -1
}

/// `DPA_EnumCallback`: calls `keep_going` on each item in order, and stops when it returns `false`.
pub fn dpa_enum_callback(dpa: Option<&Dpa>, keep_going: &mut dyn FnMut(usize) -> bool) {
    let Some(dpa) = dpa else {
        return;
    };
    for &item in dpa.ptrs.iter().take(dpa.item_count.max(0) as usize) {
        if !keep_going(item) {
            return;
        }
    }
}

/// `DPA_Destroy`: returns `false` for a NULL handle. Frees are not modelled.
pub fn dpa_destroy(dpa: Option<Box<Dpa>>) -> bool {
    dpa.is_some()
}

/// `DPA_DestroyCallback`: enumerates the items, then destroys the handle.
pub fn dpa_destroy_callback(dpa: Option<Box<Dpa>>, keep_going: &mut dyn FnMut(usize) -> bool) {
    dpa_enum_callback(dpa.as_deref(), keep_going);
    dpa_destroy(dpa);
}
