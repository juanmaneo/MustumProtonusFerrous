//! Safe Rust subset of Wine's `dlls/kernelbase/path.c` path APIs.
//!
//! Implemented functions use NUL-terminated UTF-16 buffers and Wine HRESULT
//! values. No raw pointers or platform APIs are needed for this subset.

/// Success status returned when an extended path prefix was removed.
pub const S_OK: i32 = 0;
/// Success status returned when the path did not have a recognized prefix.
pub const S_FALSE: i32 = 1;
/// Invalid argument HRESULT.
pub const E_INVALIDARG: i32 = 0x8007_0057u32 as i32;

const MAX_PATH_CCH: usize = 0x8000;

fn ascii_lower(unit: u16) -> u16 {
    if (b'A' as u16..=b'Z' as u16).contains(&unit) {
        unit + 32
    } else {
        unit
    }
}

fn wlen(path: &[u16]) -> usize {
    path.iter()
        .position(|&unit| unit == 0)
        .unwrap_or(path.len())
}

fn char_at(path: &[u16], index: usize) -> u16 {
    path.get(index).copied().unwrap_or(0)
}

fn starts_with(path: &[u16], prefix: &[u8]) -> bool {
    prefix
        .iter()
        .enumerate()
        .all(|(index, &unit)| char_at(path, index) == u16::from(unit))
}

fn is_prefixed_unc(path: &[u16]) -> bool {
    b"\\\\?\\UNC\\"
        .iter()
        .enumerate()
        .all(|(index, &unit)| ascii_lower(char_at(path, index)) == ascii_lower(u16::from(unit)))
}

fn is_prefixed_disk(path: &[u16]) -> bool {
    starts_with(path, b"\\\\?\\")
        && matches!(char_at(path, 4), 65..=90 | 97..=122)
        && char_at(path, 5) == b':' as u16
}

/// Removes a recognized `\\?\` prefix in place, matching Wine's
/// `PathCchStripPrefix` behavior. The size is measured in UTF-16 code units
/// and includes space for the terminator.
pub fn path_cch_strip_prefix(path: &mut Vec<u16>, size: usize) -> i32 {
    let len = wlen(path);
    if size == 0 || size > MAX_PATH_CCH {
        return E_INVALIDARG;
    }
    if is_prefixed_unc(path) {
        let result_len = len.saturating_sub(6);
        if size < result_len + 1 {
            return E_INVALIDARG;
        }
        let mut stripped = Vec::with_capacity(result_len + 1);
        stripped.extend_from_slice(&path[..2]);
        stripped.extend_from_slice(&path[8..len]);
        stripped.push(0);
        *path = stripped;
        S_OK
    } else if is_prefixed_disk(path) {
        let result_len = len.saturating_sub(4);
        if size < result_len + 1 {
            return E_INVALIDARG;
        }
        let mut stripped = path[4..len].to_vec();
        stripped.push(0);
        *path = stripped;
        S_OK
    } else {
        S_FALSE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn w(text: &str) -> Vec<u16> {
        text.encode_utf16().chain(std::iter::once(0)).collect()
    }

    #[test]
    fn strips_prefixed_drive_paths() {
        let mut path = w("\\\\?\\C:\\folder");
        assert_eq!(path_cch_strip_prefix(&mut path, 64), S_OK);
        assert_eq!(path, w("C:\\folder"));
    }

    #[test]
    fn strips_prefixed_unc_paths() {
        let mut path = w("\\\\?\\UNC\\server\\share");
        assert_eq!(path_cch_strip_prefix(&mut path, 64), S_OK);
        assert_eq!(path, w("\\\\server\\share"));

        let mut too_small = w("\\\\?\\UNC\\server\\share");
        let original = too_small.clone();
        assert_eq!(path_cch_strip_prefix(&mut too_small, 14), E_INVALIDARG);
        assert_eq!(too_small, original);
    }

    #[test]
    fn leaves_unprefixed_and_non_drive_prefixes_unchanged() {
        for input in ["C:\\folder", "\\\\?\\Qfolder", "\\\\?\\1:"] {
            let mut path = w(input);
            let original = path.clone();
            assert_eq!(path_cch_strip_prefix(&mut path, 64), S_FALSE);
            assert_eq!(path, original);
        }
    }

    #[test]
    fn rejects_invalid_sizes_without_mutating_the_path() {
        for size in [0, MAX_PATH_CCH + 1] {
            let mut path = w("\\\\?\\C:\\folder");
            let original = path.clone();
            assert_eq!(path_cch_strip_prefix(&mut path, size), E_INVALIDARG);
            assert_eq!(path, original);
        }
        let mut too_small = w("\\\\?\\C:\\folder");
        let original = too_small.clone();
        assert_eq!(path_cch_strip_prefix(&mut too_small, 9), E_INVALIDARG);
        assert_eq!(too_small, original);
    }
}
