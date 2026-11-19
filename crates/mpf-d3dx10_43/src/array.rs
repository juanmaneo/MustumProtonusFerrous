//! Capacity growth of `d3dx_array_reserve` from `sprite.c`.

/// The capacity that `d3dx_array_reserve` leaves an array with, for an array of `capacity`
/// elements of `size` bytes that must hold `count` elements. Returns `None` where the C returns
/// `false`. The C also reallocates the array; this port leaves the allocation to the caller.
///
/// Growth starts at four elements and doubles until `count` fits. Two cases differ from the C,
/// see the crate docs: the first step is clamped to the largest element count whose byte size
/// fits in `usize`, and a zero `size` returns `None` instead of dividing by zero.
pub fn array_reserve_capacity(capacity: usize, count: usize, size: usize) -> Option<usize> {
    if count <= capacity {
        return Some(capacity);
    }

    let max_capacity = usize::MAX.checked_div(size)?;
    if count > max_capacity {
        return None;
    }

    let mut new_capacity = capacity.max(4).min(max_capacity);
    while new_capacity < count && new_capacity <= max_capacity / 2 {
        new_capacity *= 2;
    }
    if new_capacity < count {
        new_capacity = max_capacity;
    }
    Some(new_capacity)
}
