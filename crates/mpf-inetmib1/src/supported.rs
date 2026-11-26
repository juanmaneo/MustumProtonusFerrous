//! `findSupportedQuery` and the longest-match search in `SnmpExtensionQuery`, over the
//! `supportedIDs` names.

use crate::oid::oid_n_cmp;

/// The names of `supportedIDs` in `main.c`, in the same order. The binary search relies on
/// this order.
pub const SUPPORTED_IDS: &[&[u32]] = &[
    &[1, 3, 6, 1, 2, 1, 2, 1],
    &[1, 3, 6, 1, 2, 1, 2, 2, 1],
    &[1, 3, 6, 1, 2, 1, 4],
    &[1, 3, 6, 1, 2, 1, 4, 20, 1],
    &[1, 3, 6, 1, 2, 1, 4, 21, 1],
    &[1, 3, 6, 1, 2, 1, 4, 22, 1],
    &[1, 3, 6, 1, 2, 1, 5],
    &[1, 3, 6, 1, 2, 1, 6],
    &[1, 3, 6, 1, 2, 1, 7],
    &[1, 3, 6, 1, 2, 1, 7, 5, 1],
];

/// `findSupportedQuery`: inclusive binary search over `table` that compares only the first
/// `ids.len()` arcs. A name matches when it starts with the query, and the first match
/// probed is returned.
pub fn find_supported_query(ids: &[u32], table: &[&[u32]]) -> Option<usize> {
    if ids.is_empty() {
        return None;
    }
    let (mut low, mut high) = (0_isize, table.len() as isize - 1);
    while low <= high {
        let mid = (low + high) / 2;
        let cmp = oid_n_cmp(ids, table[mid as usize], ids.len());
        if cmp == 0 {
            return Some(mid as usize);
        }
        if cmp > 0 {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    None
}

/// The loop in `SnmpExtensionQuery` that tries the longest prefix of `ids` first and stops at
/// the shortest name in `table`.
pub fn find_longest_supported_query(ids: &[u32], table: &[&[u32]]) -> Option<usize> {
    let min_len = table
        .iter()
        .map(|name| name.len())
        .min()
        .unwrap_or(usize::MAX);
    (min_len..=ids.len())
        .rev()
        .find_map(|len| find_supported_query(&ids[..len], table))
}
