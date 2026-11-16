//! Safe PIDL byte-layout helpers from Wine's `dlls/shell32/pidl.c`.
//!
//! A PIDL is a sequence of little-endian `SHITEMID.cb` entries followed by
//! a two-byte zero terminator. Wine's pointer-based exports assume valid
//! memory; these slice-based equivalents return `None` for truncated or
//! malformed input rather than reading outside the supplied slice.

/// Clones a complete terminated PIDL; malformed or truncated input returns `None`.
pub fn il_clone(pidl: &[u8]) -> Option<Vec<u8>> {
    let size = il_get_size(pidl)?;
    Some(pidl[..size].to_vec())
}

/// Clones the first item and replaces its successor with a terminator.
pub fn il_clone_first(pidl: &[u8]) -> Option<Vec<u8>> {
    let item_size = read_item_size(pidl, 0)?;
    if item_size == 0 {
        return pidl.get(..2).map(<[u8]>::to_vec);
    }
    if item_size < 2 {
        return None;
    }
    let end = item_size + 2;
    let mut clone = pidl.get(..end)?.to_vec();
    clone[item_size..end].fill(0);
    Some(clone)
}

/// Returns the byte size of a terminated PIDL, including its zero terminator.
pub fn il_get_size(pidl: &[u8]) -> Option<usize> {
    let mut offset = 0usize;
    loop {
        let item_size = read_item_size(pidl, offset)?;
        if item_size == 0 {
            return offset.checked_add(2);
        }
        if item_size < 2 {
            return None;
        }
        offset += item_size;
    }
}

/// Returns the next item's offset; `None` means malformed input or a terminator.
pub fn il_get_next(pidl: &[u8], offset: usize) -> Option<usize> {
    let item_size = read_item_size(pidl, offset)?;
    if item_size == 0 || item_size < 2 {
        return None;
    }
    let next = offset + item_size;
    read_item_size(pidl, next)?;
    Some(next)
}

/// Returns the byte offset of the final non-terminator item.
///
/// For an empty (desktop) PIDL, the terminator at byte zero is the last ID,
/// matching Wine's `ILFindLastID` behavior.
pub fn il_find_last_id(pidl: &[u8]) -> Option<usize> {
    let mut offset = 0usize;
    let mut last = 0usize;
    loop {
        let item_size = read_item_size(pidl, offset)?;
        if item_size == 0 {
            return Some(last);
        }
        if item_size < 2 {
            return None;
        }
        last = offset;
        offset += item_size;
    }
}

/// Removes the final item by writing a zero length at its header.
///
/// Returns `false` for an empty PIDL, matching `ILRemoveLastID` on the desktop.
pub fn il_remove_last_id(pidl: &mut [u8]) -> bool {
    match il_find_last_id(pidl) {
        Some(offset) if read_item_size(pidl, offset) != Some(0) => {
            pidl[offset..offset + 2].fill(0);
            true
        }
        Some(_) | None => false,
    }
}

fn read_item_size(pidl: &[u8], offset: usize) -> Option<usize> {
    let bytes = pidl.get(offset..)?.get(..2)?;
    Some(usize::from(u16::from_le_bytes([bytes[0], bytes[1]])))
}

#[cfg(test)]
mod tests {
    use super::*;

    const PIDL: &[u8] = &[3, 0, 0xaa, 4, 0, 0xbb, 0xcc, 0, 0];

    #[test]
    fn clone_copies_entire_list_and_first_item() {
        assert_eq!(il_clone(PIDL), Some(PIDL.to_vec()));
        assert_eq!(il_clone_first(PIDL), Some(vec![3, 0, 0xaa, 0, 0]));
        assert_eq!(il_clone_first(&[0, 0]), Some(vec![0, 0]));
        assert_eq!(il_clone_first(&[0]), None);
        assert_eq!(il_clone(&[0, 0]), Some(vec![0, 0]));
    }

    #[test]
    fn clone_rejects_truncated_and_malformed_lists() {
        assert_eq!(il_clone(&[3, 0, 0xaa]), None);
        assert_eq!(il_clone_first(&[3, 0, 0xaa]), None);
        assert_eq!(il_clone_first(&[4, 0, 0xaa]), None);
        assert_eq!(il_clone_first(&[]), None);
        assert_eq!(il_clone_first(&[1]), None);
        assert_eq!(il_clone_first(&[1, 0, 0, 0]), None);
    }

    #[test]
    fn size_includes_terminator() {
        assert_eq!(il_get_size(PIDL), Some(9));
        assert_eq!(il_get_size(&[0, 0]), Some(2));
    }

    #[test]
    fn next_advances_by_little_endian_item_size() {
        assert_eq!(il_get_next(PIDL, 0), Some(3));
        assert_eq!(il_get_next(PIDL, 3), Some(7));
        assert_eq!(il_get_next(PIDL, 7), None);
    }

    #[test]
    fn last_id_returns_final_item_or_empty_terminator() {
        assert_eq!(il_find_last_id(PIDL), Some(3));
        assert_eq!(il_find_last_id(&[0, 0]), Some(0));
    }

    #[test]
    fn remove_last_id_shortens_and_preserves_terminator() {
        let mut pidl = PIDL.to_vec();
        assert!(il_remove_last_id(&mut pidl));
        assert_eq!(pidl, [3, 0, 0xaa, 0, 0, 0xbb, 0xcc, 0, 0]);
        assert_eq!(il_get_size(&pidl), Some(5));
        assert!(il_remove_last_id(&mut pidl));
        assert_eq!(pidl, [0, 0, 0xaa, 0, 0, 0xbb, 0xcc, 0, 0]);
        assert!(!il_remove_last_id(&mut pidl));
    }

    #[test]
    fn remove_last_id_rejects_malformed_input() {
        assert!(!il_remove_last_id(&mut []));
        assert!(!il_remove_last_id(&mut [1, 0, 0, 0]));
    }

    #[test]
    fn truncated_headers_and_items_are_rejected() {
        assert_eq!(il_get_size(&[]), None);
        assert_eq!(il_get_size(&[1]), None);
        assert_eq!(il_get_size(&[3, 0, 0xaa]), None);
        assert_eq!(il_get_next(&[0], 0), None);
        assert_eq!(il_get_next(&[1, 0], 0), None);
        assert_eq!(il_get_next(&[3, 0, 0xaa], 0), None);
        assert_eq!(il_find_last_id(&[3, 0, 0xaa]), None);
    }

    #[test]
    fn malformed_lengths_and_offsets_are_rejected() {
        assert_eq!(il_get_size(&[1, 0, 0, 0]), None);
        assert_eq!(il_find_last_id(&[1, 0, 0, 0]), None);
        assert_eq!(il_get_next(PIDL, usize::MAX), None);
        assert_eq!(il_get_next(PIDL, 9), None);
    }
}
