//! Deterministic helpers from Wine's `dlls/uiautomationcore`.
//!
//! Ported:
//! - `uia_compare_safearrays` (`uia_utils.c`): [`compare_int_safearrays`]. The
//!   `SAFEARRAY` is modelled as a one-dimensional view; its lower bound only
//!   indexes elements and never changes the result, so it is not stored.
//! - `uia_array_reserve` (`uia_private.h`): [`reserve_capacity`], the growth
//!   policy of the growable arrays, without the reallocation.
//! - `uia_property_guid_compare` and `uia_prop_info_from_guid` (`uia_ids.c`):
//!   [`find_guid`], a binary search over a table sorted by `GUID` bytes.
//! - `uia_prop_info_from_id` (`uia_ids.c`): [`index_from_id`].
//!
//! Skipped: COM and GIT marshalling, HWND and red-black-tree maps, providers,
//! events, the client and node logic, and the static GUID tables, which are
//! data rather than logic.
//!
//! Divergences from C, all deliberate:
//! - `reserve_capacity` returns `None` for `size == 0` when the array must
//!   grow. C divides by zero there.
//!
//! Oracle: `oracle_checks.rs` rows come from the verbatim Wine functions
//! (`uia_utils.c` 301-386, `uia_ids.c` 26-31, 318-338, `uia_private.h`
//! 186-211) compiled with `gcc -std=gnu11 -fsanitize=address,undefined`.
//! Win32 types, SAFEARRAY accessors and the GUID tables are shimmed. No
//! sanitizer findings. No vectors were excluded.

use std::cmp::Ordering;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// A `GUID` in memory order: `Data1` to `Data3` little-endian, then `Data4`.
/// Byte-wise ordering is the `memcmp` order the Wine tables are sorted by.
pub type Guid = [u8; 16];

/// Builds a [`Guid`] from its fields, as they are laid out in memory.
pub const fn guid(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Guid {
    let a = data1.to_le_bytes();
    let b = data2.to_le_bytes();
    let c = data3.to_le_bytes();
    [
        a[0], a[1], a[2], a[3], b[0], b[1], c[0], c[1], data4[0], data4[1], data4[2], data4[3],
        data4[4], data4[5], data4[6], data4[7],
    ]
}

/// `UIAutomationType_IntArray`, the only array type that `uia_compare_safearrays` compares.
pub const UIA_TYPE_INT_ARRAY: i32 = 0x0001_0001;

/// A one-dimensional `SAFEARRAY` of `int` elements.
#[derive(Clone, Copy, Debug)]
pub struct SafeArray<'a> {
    /// `SafeArrayGetDim`. Only one-dimensional arrays are usable.
    pub dims: u32,
    pub values: &'a [i32],
}

impl SafeArray<'_> {
    /// The elements, as `get_safearray_bounds` accepts them: one dimension only.
    fn elements(&self) -> Option<&[i32]> {
        (self.dims == 1).then_some(self.values)
    }
}

/// `uia_compare_safearrays`, returning -1, 0 or 1.
///
/// A non-one-dimensional array gives -1. Element counts are compared first, so
/// a count mismatch decides the result for any `prop_type`. With equal counts,
/// a `prop_type` other than [`UIA_TYPE_INT_ARRAY`] gives -1, even for equal
/// arrays. Otherwise the elements are compared lexicographically.
pub fn compare_int_safearrays(sa1: &SafeArray, sa2: &SafeArray, prop_type: i32) -> i32 {
    let (Some(a), Some(b)) = (sa1.elements(), sa2.elements()) else {
        return -1;
    };
    let by_count = a.len().cmp(&b.len());
    if by_count != Ordering::Equal {
        return by_count as i32;
    }
    if prop_type != UIA_TYPE_INT_ARRAY {
        return -1;
    }
    a.cmp(b) as i32
}

/// `uia_array_reserve` without the reallocation. Returns the capacity after the call, or
/// `None` where C returns FALSE.
///
/// A `count` that already fits leaves `capacity` alone. Otherwise the capacity
/// doubles from `max(1, capacity)` until it reaches `count`, and is clamped to
/// `count`. A `count` above `usize::MAX / size` fails.
pub fn reserve_capacity(capacity: usize, count: usize, size: usize) -> Option<usize> {
    if count <= capacity {
        return Some(capacity);
    }
    let max_capacity = usize::MAX.checked_div(size)?;
    if count > max_capacity {
        return None;
    }
    let mut new_capacity = capacity.max(1);
    while new_capacity < count && new_capacity <= max_capacity / 2 {
        new_capacity *= 2;
    }
    Some(new_capacity.max(count))
}

/// `uia_prop_info_from_guid`: the id for `key` in `table`, which must be sorted by
/// [`Guid`] with unique keys.
pub fn find_guid(table: &[(Guid, i32)], key: &Guid) -> Option<i32> {
    table
        .binary_search_by(|(g, _)| g.cmp(key))
        .ok()
        .map(|i| table[i].1)
}

/// `uia_prop_info_from_id`: the table index for `id`. `id_idx` maps the
/// offset from `id_min` to a table index, and its length gives the id range.
pub fn index_from_id(id: i32, id_min: i32, id_idx: &[usize]) -> Option<usize> {
    let offset = i64::from(id) - i64::from(id_min);
    if offset < 0 || offset >= id_idx.len() as i64 {
        return None;
    }
    Some(id_idx[offset as usize])
}
