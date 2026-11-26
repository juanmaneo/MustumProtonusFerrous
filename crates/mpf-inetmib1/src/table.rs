//! Table lookups from `main.c`: `findValueInTable`, `findOidInTable`, `findNextOidInTable`
//! and `getItemAndInstanceFromTable`.
//!
//! Indices are 1-based like the C code, and 0 means "not found". An empty slice stands for
//! both a NULL table and a table with `numEntries == 0`.

use crate::item::SNMP_PDU_GETNEXT;
use crate::oid::oid_n_cmp;

/// `findValueInTable`: binary search using glibc's `bsearch` probe order, so duplicate
/// entries resolve to the same one that Wine finds.
pub fn find_value_in_table<K, T>(key: &K, entries: &[T], compare: impl Fn(&K, &T) -> i32) -> usize {
    let (mut low, mut high) = (0, entries.len());
    while low < high {
        let idx = (low + high) / 2;
        let cmp = compare(key, &entries[idx]);
        if cmp < 0 {
            high = idx;
        } else if cmp > 0 {
            low = idx + 1;
        } else {
            return idx + 1;
        }
    }
    0
}

/// `findOidInTable`: converts `oid` to a key and searches for it.
pub fn find_oid_in_table<K, T>(
    oid: &[u32],
    entries: &[T],
    make_key: impl Fn(&[u32]) -> K,
    compare: impl Fn(&K, &T) -> i32,
) -> usize {
    find_value_in_table(&make_key(oid), entries, compare)
}

/// `findNextOidInTable`: the index of the entry after the match for `oid`, skipping any
/// further entries with the same key.
///
/// A key below the first entry gives 1. If the match is the last entry, the result is
/// `entries.len() + 1`. An empty table gives 0. Deviation: the C code compares the key with
/// `entries[0]` even when the table is empty, which reads past the end. This port returns 0
/// without reading.
pub fn find_next_oid_in_table<K, T>(
    oid: &[u32],
    entries: &[T],
    make_key: impl Fn(&[u32]) -> K,
    compare: impl Fn(&K, &T) -> i32,
) -> usize {
    let key = make_key(oid);
    let index = find_value_in_table(&key, entries, &compare);
    if index == 0 {
        match entries.first() {
            Some(first) if compare(&key, first) < 0 => 1,
            _ => 0,
        }
    } else {
        let mut next = index + 1;
        while next <= entries.len() && compare(&key, &entries[next - 1]) == 0 {
            next += 1;
        }
        next
    }
}

/// `getItemAndInstanceFromTable`: resolves `oid`, which must start with `base`, to an
/// `(item, instance)` pair.
///
/// The instance is the table index of the row whose key is the `instance_len` arcs after the
/// item. For GETNEXT, a missing item or instance resolves to the first one.
pub fn get_item_and_instance_from_table<K, T>(
    oid: &[u32],
    base: &[u32],
    instance_len: usize,
    pdu: u8,
    entries: &[T],
    make_key: impl Fn(&[u32]) -> K,
    compare: impl Fn(&K, &T) -> i32,
) -> Option<(u32, u32)> {
    if entries.is_empty() {
        return None;
    }
    let base_len = base.len();
    let row_len = base_len + instance_len + 1;
    let cmp = oid_n_cmp(oid, base, base_len);
    if pdu == SNMP_PDU_GETNEXT {
        if cmp < 0 {
            Some((1, 1))
        } else if cmp == 0 && oid.len() < row_len {
            let item = match oid.get(base_len) {
                Some(&item) if item != 0 => item,
                _ => 1,
            };
            Some((item, 1))
        } else if cmp == 0 && oid.len() == row_len {
            match oid[base_len] {
                0 => Some((1, 1)),
                item => {
                    let instance = find_next_oid_in_table(
                        &oid[base_len + 1..row_len],
                        entries,
                        &make_key,
                        &compare,
                    );
                    if instance == 0 || instance > entries.len() {
                        None
                    } else {
                        Some((item, instance as u32))
                    }
                }
            }
        } else {
            None
        }
    } else if cmp == 0 && oid.len() == row_len {
        match oid[base_len] {
            0 => None,
            item => {
                match find_oid_in_table(&oid[base_len + 1..row_len], entries, &make_key, &compare) {
                    0 => None,
                    instance => Some((item, instance as u32)),
                }
            }
        }
    } else {
        None
    }
}
