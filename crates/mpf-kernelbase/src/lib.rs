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

fn is_prefixed_volume(path: &[u16]) -> bool {
    let prefix = b"\\\\?\\Volume";
    if !prefix
        .iter()
        .enumerate()
        .all(|(index, &unit)| ascii_lower(char_at(path, index)) == ascii_lower(u16::from(unit)))
    {
        return false;
    }
    (0..=37).all(|index| {
        let unit = char_at(path, 10 + index);
        match index {
            0 => unit == b'{' as u16,
            9 | 14 | 19 | 24 => unit == b'-' as u16,
            37 => unit == b'}' as u16,
            _ => matches!(unit, 48..=57 | 65..=70 | 97..=102),
        }
    })
}

fn next_segment(path: &[u16], start: usize) -> (usize, bool) {
    let len = wlen(path);
    let end = (start..len)
        .find(|&index| path[index] == b'\\' as u16)
        .unwrap_or(len);
    if end < len {
        (end + 1, true)
    } else {
        (end, false)
    }
}

fn root_end(path: &[u16]) -> Option<usize> {
    if wlen(path) == 0 {
        return None;
    }
    if is_prefixed_volume(path) {
        Some(if char_at(path, 48) == b'\\' as u16 {
            48
        } else {
            47
        })
    } else if is_prefixed_unc(path) {
        Some(7)
    } else if is_prefixed_disk(path) {
        Some(if char_at(path, 6) == b'\\' as u16 {
            6
        } else {
            5
        })
    } else if path[0] == b'\\' as u16 && char_at(path, 1) == b'\\' as u16 {
        Some(1)
    } else if path[0] == b'\\' as u16 {
        Some(0)
    } else if matches!(char_at(path, 0), 65..=90 | 97..=122) && char_at(path, 1) == b':' as u16 {
        Some(if char_at(path, 2) == b'\\' as u16 {
            2
        } else {
            1
        })
    } else {
        None
    }
}

/// Returns the UTF-16 offset just past the drive or UNC share root,
/// equivalent to Wine's `PathCchSkipRoot`. Root-only paths can return an offset
/// equal to the string length; non-roots return `E_INVALIDARG` and offset zero.
pub fn path_cch_skip_root(path: &[u16]) -> (i32, usize) {
    let len = wlen(path);
    if len == 0
        || (starts_with(path, b"\\\\?")
            && !is_prefixed_volume(path)
            && !is_prefixed_unc(path)
            && !is_prefixed_disk(path))
    {
        return (E_INVALIDARG, 0);
    }
    let Some(mut end) = root_end(path).map(|end| end + 1) else {
        return (E_INVALIDARG, 0);
    };
    if is_prefixed_unc(path) {
        end = next_segment(path, end).0;
        end = next_segment(path, end).0;
    } else if path[0] == b'\\' as u16
        && char_at(path, 1) == b'\\' as u16
        && char_at(path, 2) != b'?' as u16
    {
        end = next_segment(path, end).0;
        if char_at(path, end) != b'\\' as u16 {
            end = next_segment(path, end).0;
        }
    }
    (S_OK, end)
}

/// Reports whether a path consists only of a drive, volume, or UNC root.
/// Equivalent to Wine's `PathCchIsRoot` behavior.
pub fn path_cch_is_root(path: &[u16]) -> bool {
    let len = wlen(path);
    if len == 0 {
        return false;
    }
    let Some(end) = root_end(path) else {
        return false;
    };
    let is_unc = is_prefixed_unc(path)
        || (path[0] == b'\\' as u16
            && char_at(path, 1) == b'\\' as u16
            && char_at(path, 2) != b'?' as u16);
    if is_unc {
        let next = end + 1;
        if next == len {
            return true;
        }
        let (first_end, first_has_separator) = next_segment(path, next);
        if first_has_separator && first_end == len {
            return false;
        }
        if !first_has_separator {
            return true;
        }
        let (second_end, second_has_separator) = next_segment(path, first_end);
        !second_has_separator && second_end == len
    } else {
        char_at(path, end) == b'\\' as u16 && end + 1 == len
    }
}

/// Truncates a path to its root, matching Wine's `PathCchStripToRoot`.
/// The size is measured in UTF-16 code units and includes the terminator.
pub fn path_cch_strip_to_root(path: &mut Vec<u16>, size: usize) -> i32 {
    let len = wlen(path);
    if len == 0 || size == 0 || size > MAX_PATH_CCH {
        return E_INVALIDARG;
    }
    let (status, mut end) = path_cch_skip_root(path);
    if status != S_OK {
        path.clear();
        path.push(0);
        return E_INVALIDARG;
    }
    if end > 0
        && char_at(path, end - 1) == b'\\' as u16
        && ((is_prefixed_unc(path) && char_at(path, 8) != 0)
            || (path[0] == b'\\' as u16
                && char_at(path, 1) == b'\\' as u16
                && char_at(path, 2) != 0
                && char_at(path, 2) != b'?' as u16))
    {
        end -= 1;
    }
    if end >= size {
        return E_INVALIDARG;
    }
    if char_at(path, end) == 0 {
        return S_FALSE;
    }
    path.truncate(end);
    path.push(0);
    S_OK
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
    fn skip_root_matches_wine_drive_and_unc_offsets() {
        for (path, expected) in [
            ("C:\\", 3),
            ("\\", 1),
            ("\\\\.\\", 4),
            ("\\\\?\\UNC\\", 8),
            ("\\\\?\\C:\\", 7),
            ("C:\\\\", 3),
            ("\\\\", 2),
            ("\\\\.\\\\", 4),
            ("\\\\?\\UNC\\\\", 9),
            ("\\\\?\\C:\\\\", 7),
            ("C:\\a", 3),
            ("\\a", 1),
            ("\\\\.\\a", 5),
            ("\\\\?\\UNC\\a", 9),
            ("C:\\\\a", 3),
            ("\\\\a", 3),
            ("\\\\.\\\\a", 4),
            ("\\\\?\\UNC\\\\a", 10),
            ("\\\\?\\C:\\\\a", 7),
            ("C:\\a\\", 3),
            ("\\a\\", 1),
            ("\\\\.\\a\\", 6),
            ("\\\\?\\UNC\\a\\", 10),
            ("\\\\?\\C:\\a\\", 7),
            ("\\\\\\\\", 3),
            ("\\\\a\\", 4),
            ("\\\\a\\b", 5),
            ("\\\\a\\b\\", 6),
            ("\\\\a\\b\\\\", 6),
            ("\\\\a\\b\\\\c", 6),
            ("\\\\a\\b\\c", 6),
            ("\\\\a\\b\\c\\", 6),
            ("\\\\a\\b\\c\\d", 6),
            ("\\\\a\\\\b\\c\\", 4),
            ("\\\\aa\\bb\\cc\\", 8),
            ("\\\\?\\UNC\\\\", 9),
            ("\\\\?\\UNC\\a\\b", 11),
            ("\\\\?\\UNC\\a\\b\\", 12),
            ("\\\\?\\UNC\\a\\b\\c", 12),
            ("\\\\?\\UNC\\a\\b\\c\\", 12),
            ("\\\\?\\UNC\\a\\b\\c\\d", 12),
            ("\\\\?\\C:", 6),
            ("\\\\?\\Volume{e51a1864-6f2d-4019-b73d-f4e60e600c26}", 48),
            ("\\\\?\\Volume{e51a1864-6f2d-4019-b73d-f4e60e600c26}\\", 49),
            ("\\\\?\\unc\\a\\b", 11),
            ("\\\\?\\volume{e51a1864-6f2d-4019-b73d-f4e60e600c26}\\", 49),
            ("\\\\?\\volume{e51a1864-6f2d-4019-b73d-f4e60e600c26}\\a", 49),
            ("C:", 2),
            ("\\?", 1),
            ("\\?\\UNC", 1),
            ("\\.", 1),
            ("\\\\..", 4),
            ("\\\\..a", 5),
            ("\\\\server\\share\\file", 15),
            ("\\\\?\\UNC\\server\\share\\file", 21),
        ] {
            assert_eq!(path_cch_skip_root(&w(path)), (S_OK, expected), "{path}");
        }
    }

    #[test]
    fn skip_root_rejects_empty_relative_and_invalid_extended_prefixes() {
        assert_eq!(root_end(&[]), None);
        for path in [
            "",
            "a",
            "\\\\?\\",
            "\\\\?\\UNC",
            "\\\\?\\::\\",
            "\\\\?\\Volume{e51a1864-6f2d-4019-b73d-f4e60e600c2g}",
            ":",
            ":\\",
            "C\\",
        ] {
            assert_eq!(path_cch_skip_root(&w(path)), (E_INVALIDARG, 0), "{path}");
        }
    }

    #[test]
    fn is_root_matches_wine_vectors() {
        for (path, expected) in [
            ("", false),
            ("a", false),
            ("C:", false),
            ("C:\\", true),
            ("C:\\a", false),
            ("\\\\?\\C:\\", true),
            ("\\\\?\\C:", false),
            ("\\\\?\\C:\\a", false),
            ("\\", true),
            ("\\a\\", false),
            ("\\a\\b", false),
            ("\\\\", true),
            ("\\\\a", true),
            ("\\\\a\\", false),
            ("\\\\a\\b", true),
            ("\\\\a\\b\\", false),
            ("\\\\a\\b\\c", false),
            ("\\\\?\\UNC\\", true),
            ("\\\\?\\UNC\\a", true),
            ("\\\\?\\UNC\\a\\", false),
            ("\\\\?\\UNC\\a\\b", true),
            ("\\\\?\\UNC\\a\\b\\", false),
            ("\\\\?\\UNC\\a\\b\\c", false),
            ("\\\\?\\Volume{e51a1864-6f2d-4019-b73d-f4e60e600c26}", false),
            (
                "\\\\?\\Volume{e51a1864-6f2d-4019-b73d-f4e60e600c26}\\",
                true,
            ),
            (
                "\\\\?\\Volume{e51a1864-6f2d-4019-b73d-f4e60e600c26}\\a",
                false,
            ),
            ("..\\a", false),
            ("\\\\\\\\", false),
            ("\\\\a\\\\b", false),
            ("\\a", false),
            ("X:", false),
            ("\\server", false),
        ] {
            assert_eq!(path_cch_is_root(&w(path)), expected, "{path}");
        }
    }

    #[test]
    fn strip_to_root_matches_wine_vectors() {
        let volume = "\\\\?\\Volume{e51a1864-6f2d-4019-b73d-f4e60e600c26}";
        for (input, expected, status) in [
            ("C:\\", "C:\\", S_FALSE),
            ("C:\\a", "C:\\", S_OK),
            ("C:\\a\\b", "C:\\", S_OK),
            ("C:", "C:", S_FALSE),
            ("C:a", "C:", S_OK),
            ("C:a\\b\\c", "C:", S_OK),
            ("\\", "\\", S_FALSE),
            ("\\a", "\\", S_OK),
            ("\\a\\b\\c", "\\", S_OK),
            ("\\\\", "\\\\", S_FALSE),
            ("\\\\a", "\\\\a", S_FALSE),
            ("\\\\a\\b", "\\\\a\\b", S_FALSE),
            ("\\\\a\\b\\c", "\\\\a\\b", S_OK),
            ("\\\\\\", "\\\\", S_OK),
            ("\\\\?\\UNC\\", "\\\\?\\UNC\\", S_FALSE),
            ("\\\\?\\UNC\\a", "\\\\?\\UNC\\a", S_FALSE),
            ("\\\\?\\UNC\\a\\b", "\\\\?\\UNC\\a\\b", S_FALSE),
            ("\\\\?\\UNC\\a\\b\\", "\\\\?\\UNC\\a\\b", S_OK),
            ("\\\\?\\UNC\\a\\b\\c", "\\\\?\\UNC\\a\\b", S_OK),
            ("\\\\?\\C:", "\\\\?\\C:", S_FALSE),
            ("\\\\?\\C:a", "\\\\?\\C:", S_OK),
            ("\\\\?\\C:a\\b\\c", "\\\\?\\C:", S_OK),
            ("\\\\?\\C:\\", "\\\\?\\C:\\", S_FALSE),
            ("\\\\?\\C:\\a\\b", "\\\\?\\C:\\", S_OK),
            (volume, volume, S_FALSE),
            (&format!("{volume}a"), volume, S_OK),
            (&format!("{volume}a\\b\\c"), volume, S_OK),
            (&format!("{volume}\\"), &format!("{volume}\\"), S_FALSE),
            (&format!("{volume}\\a\\b"), &format!("{volume}\\"), S_OK),
        ] {
            let mut path = w(input);
            assert_eq!(
                path_cch_strip_to_root(&mut path, MAX_PATH_CCH),
                status,
                "{input}"
            );
            assert_eq!(path, w(expected), "{input}");
        }
    }

    #[test]
    fn strip_to_root_validates_size_and_clears_invalid_paths() {
        let mut path = w("C:\\");
        assert_eq!(
            path_cch_strip_to_root(&mut path, MAX_PATH_CCH + 1),
            E_INVALIDARG
        );
        assert_eq!(path, w("C:\\"));
        for size in [0, 1, 2] {
            let mut path = w("C:\\a");
            let original = path.clone();
            assert_eq!(path_cch_strip_to_root(&mut path, size), E_INVALIDARG);
            assert_eq!(path, original);
        }
        for input in ["", "C", "\\\\?\\UNC"] {
            let mut path = w(input);
            assert_eq!(
                path_cch_strip_to_root(&mut path, MAX_PATH_CCH),
                E_INVALIDARG
            );
            assert_eq!(path.first(), Some(&0));
        }
        let mut path = w("C:\\a");
        assert_eq!(path_cch_strip_to_root(&mut path, 4), S_OK);
        assert_eq!(path, w("C:\\"));
        let mut unc = w("\\\\a\\b\\c");
        assert_eq!(path_cch_strip_to_root(&mut unc, 5), E_INVALIDARG);
        assert_eq!(unc, w("\\\\a\\b\\c"));
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
