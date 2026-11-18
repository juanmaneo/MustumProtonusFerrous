//! The growth rule of `array_reserve`, which sizes the USB event queue.

/// `array_reserve`: the capacity once room is reserved for `count` elements, or
/// `None` when `count` is over the limit. The C code keeps capacities in
/// `unsigned int`, so the limit and the doubling steps are 32-bit.
///
/// `size` must be non-zero, as the C code divides by it when the capacity grows.
pub fn reserve_capacity(capacity: usize, count: usize, size: usize) -> Option<usize> {
    if count <= capacity {
        return Some(capacity);
    }
    let max_capacity = (usize::MAX / size) as u32;
    if count > max_capacity as usize {
        return None;
    }
    let mut new_capacity = 4usize.max(capacity) as u32;
    while (new_capacity as usize) < count && new_capacity <= max_capacity / 2 {
        new_capacity *= 2;
    }
    if (new_capacity as usize) < count {
        new_capacity = max_capacity;
    }
    Some(new_capacity as usize)
}
