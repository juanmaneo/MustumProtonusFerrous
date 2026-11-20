//! `mf_array_reserve` from `dlls/mf/mf_private.h`: the capacity growth policy of the element arrays
//! that the mf objects keep. The C reallocates the array to the capacity it picks. This module picks
//! the capacity, and the caller does the allocation.

/// The outcome of a reserve request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reserve {
    /// The current capacity already holds the count.
    Fits,
    /// The array grows to this many elements.
    Grow(usize),
    /// The count cannot be addressed in bytes for this element size.
    TooLarge,
}

/// Plans the capacity for `count` elements of `element_size` bytes, given the current `capacity` in
/// elements. The capacity starts at four and doubles until it holds `count`. When doubling would
/// pass the largest count whose byte size fits in `usize`, the capacity is that largest count.
pub fn reserve_capacity(capacity: usize, count: usize, element_size: usize) -> Reserve {
    if count <= capacity {
        return Reserve::Fits;
    }
    let Some(max_capacity) = usize::MAX.checked_div(element_size) else {
        return Reserve::TooLarge;
    };
    if count > max_capacity {
        return Reserve::TooLarge;
    }
    let mut new_capacity = capacity.max(4);
    while new_capacity < count && new_capacity <= max_capacity / 2 {
        new_capacity *= 2;
    }
    if new_capacity < count {
        new_capacity = max_capacity;
    }
    Reserve::Grow(new_capacity)
}
