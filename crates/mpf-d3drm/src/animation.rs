//! Animation key search of Wine's `dlls/d3drm/frame.c`. The keys are sorted by time.

/// A `struct d3drm_animation_key`. Wine's value is a union of rotation, scale and position, so the
/// port keeps the value generic.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Key<T> {
    pub time: f32,
    pub value: T,
}

/// `d3drm_animation_lookup_key`: the index of the key at `time`. Without an exact match it returns
/// the last index the binary search examined, which is not the insertion point. Zero keys give 0.
pub fn lookup_key<T>(keys: &[Key<T>], time: f32) -> usize {
    let (mut start, mut end, mut cur) = (0, keys.len(), 0);
    while start < end {
        cur = start + (end - start) / 2;
        if time == keys[cur].time {
            return cur;
        }
        if time < keys[cur].time {
            end = cur;
        } else {
            start = cur + 1;
        }
    }
    cur
}

/// `d3drm_animation_get_index_min`: the first key with time `time`, found by walking back from
/// [`lookup_key`].
pub fn index_min<T>(keys: &[Key<T>], time: f32) -> usize {
    let mut i = lookup_key(keys, time);
    while i > 0 && keys[i - 1].time == time {
        i -= 1;
    }
    i
}

/// `d3drm_animation_get_index_max`: the last key with time `time`, found by walking forward from
/// [`lookup_key`]. Wine's loop bound `count - 1` wraps for zero keys and reads past the array, so
/// the port returns 0 there.
pub fn index_max<T>(keys: &[Key<T>], time: f32) -> usize {
    let mut i = lookup_key(keys, time);
    while i + 1 < keys.len() && keys[i + 1].time == time {
        i += 1;
    }
    i
}

/// `d3drm_animation_get_insert_position`: 0 before the first key, the key count at or after the
/// last key, and otherwise [`index_max`].
pub fn insert_position<T>(keys: &[Key<T>], time: f32) -> usize {
    if keys.is_empty() || time < keys[0].time {
        0
    } else if time >= keys[keys.len() - 1].time {
        keys.len()
    } else {
        index_max(keys, time)
    }
}

/// `d3drm_animation_get_range`: the first index and the count of the keys in `min..=max`, or
/// `None` when no key can match.
///
/// Wine computes the count as `index_max(&keys[first..], max) - first + 1`. The tail search already
/// returns an index relative to `first`, so the count is short by `first`, and it wraps when that
/// index is below `first`. The port keeps Wine's result. For example, the keys `[0, 1, 2, 3]` with
/// `min = max = 3` give `(3, usize::MAX - 1)`.
pub fn range<T>(keys: &[Key<T>], min: f32, max: f32) -> Option<(usize, usize)> {
    if keys.is_empty() || max < keys[0].time || min > keys[keys.len() - 1].time {
        return None;
    }
    let first = index_min(keys, min);
    let count = index_max(&keys[first..], max)
        .wrapping_sub(first)
        .wrapping_add(1);
    Some((first, count))
}
