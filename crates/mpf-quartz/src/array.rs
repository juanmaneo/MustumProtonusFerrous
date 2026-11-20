//! The capacity policy of `array_reserve` in `dlls/quartz/main.c`. The caller does the realloc.

/// The outcome of [reserve_plan].
#[derive(Debug, PartialEq, Eq)]
pub enum Reserve {
    /// The current capacity already holds `count` elements. C returns true without reallocating.
    Kept,
    /// Reallocate to `capacity` elements, which is `bytes` bytes.
    Grow { capacity: u32, bytes: usize },
    /// `count` elements cannot be allocated. C returns false.
    Fail,
}

/// `array_reserve`. Capacities are `unsigned int` in C, so the element limit truncates to 32 bits.
pub fn reserve_plan(capacity: usize, count: usize, size: usize) -> Reserve {
    if count <= capacity {
        return Reserve::Kept;
    }
    let Some(quotient) = usize::MAX.checked_div(size) else {
        return Reserve::Fail;
    };
    let max_capacity = quotient as u32;
    if count > max_capacity as usize {
        return Reserve::Fail;
    }

    let mut new_capacity = capacity.max(4) as u32;
    while (new_capacity as usize) < count && new_capacity <= max_capacity / 2 {
        new_capacity *= 2;
    }
    if (new_capacity as usize) < count {
        new_capacity = max_capacity;
    }

    let Some(bytes) = (new_capacity as usize).checked_mul(size) else {
        return Reserve::Fail;
    };
    Reserve::Grow {
        capacity: new_capacity,
        bytes,
    }
}
