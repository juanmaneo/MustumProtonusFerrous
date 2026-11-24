//! Capacity growth from `array_reserve` in Wine's `dlls/devenum/mediacatenum.c`.

/// The smallest capacity `array_reserve` grows an array to.
const MIN_CAPACITY: u32 = 8;

/// Port of the capacity arithmetic in `array_reserve`, without the allocation. Returns the
/// capacity an array needs to hold `count` elements of `size` bytes, or `None` where Wine returns
/// `FALSE`. The result equals `capacity` when no reallocation is needed. A zero `size` with growth
/// is undefined in C (division by zero) and is rejected here.
pub fn reserve_capacity(capacity: u32, count: u32, size: u32) -> Option<u32> {
    if count <= capacity {
        return Some(capacity);
    }
    let max_capacity = u32::MAX.checked_div(size)?;
    if count > max_capacity {
        return None;
    }
    let mut new_capacity = MIN_CAPACITY.max(capacity);
    while new_capacity < count && new_capacity <= max_capacity / 2 {
        new_capacity *= 2;
    }
    Some(new_capacity.max(count))
}
