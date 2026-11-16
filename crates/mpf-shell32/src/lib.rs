//! Safe PIDL byte-layout helpers from Wine's `dlls/shell32/pidl.c`.
//!
//! A PIDL is a sequence of little-endian `SHITEMID.cb` entries followed by
//! a two-byte zero terminator. Wine's pointer-based exports assume valid
//! memory; these slice-based equivalents return `None` for truncated or
//! malformed input rather than reading outside the supplied slice.

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

fn read_item_size(pidl: &[u8], offset: usize) -> Option<usize> {
    let bytes = pidl.get(offset..)?.get(..2)?;
    Some(usize::from(u16::from_le_bytes([bytes[0], bytes[1]])))
}

#[cfg(test)]
mod tests {
    use super::*;

    const PIDL: &[u8] = &[3, 0, 0xaa, 4, 0, 0xbb, 0xcc, 0, 0];

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
