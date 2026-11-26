/// The capacity that `mf_array_reserve` in `dlls/mfmediaengine/main.c` keeps after reserving room
/// for `count` elements of `size` bytes. `None` is its FALSE return, for a count too large to
/// allocate. The `realloc` is the OS allocator and is not modelled.
pub fn array_reserve_capacity(capacity: usize, count: usize, size: usize) -> Option<usize> {
    if count <= capacity {
        return Some(capacity);
    }
    let max_capacity = usize::MAX / size;
    if count > max_capacity {
        return None;
    }
    let mut new_capacity = capacity.max(4);
    while new_capacity < count && new_capacity <= max_capacity / 2 {
        new_capacity *= 2;
    }
    if new_capacity < count {
        new_capacity = max_capacity;
    }
    Some(new_capacity)
}
