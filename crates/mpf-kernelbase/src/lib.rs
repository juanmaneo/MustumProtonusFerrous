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
/// Insufficient buffer HRESULT used by Wine's path extension helpers.
pub const STRSAFE_E_INSUFFICIENT_BUFFER: i32 = 0x8007_007au32 as i32;

const MAX_PATH_CCH: usize = 0x8000;
const MAX_PATH: usize = 260;

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

/// Reports whether a path is a root, matching Wine's `PathIsRootW` wrapper.
pub fn path_is_root(path: &[u16]) -> bool {
    path_cch_is_root(path)
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

/// Truncates a path to its root, matching Wine's `PathStripToRootW` wrapper.
pub fn path_strip_to_root(path: &mut Vec<u16>) -> bool {
    path_cch_strip_to_root(path, MAX_PATH_CCH) >= 0
}

/// Removes one trailing backslash and returns the path-end offset and free size.
/// Offsets and capacity are measured in UTF-16 code units.
pub fn path_cch_remove_backslash_ex(
    path: &mut Vec<u16>,
    size: usize,
) -> (i32, Option<usize>, usize) {
    if size == 0 {
        return (E_INVALIDARG, None, 0);
    }
    let len = wlen(path);
    let path_len = len.min(size);
    if path_len == size && char_at(path, path_len) == 0 {
        return (E_INVALIDARG, None, 0);
    }
    let root = root_end(path);
    if path_len > 0 && char_at(path, path_len - 1) == b'\\' as u16 {
        let end = path_len - 1;
        let free = size - path_len + 1;
        if root.is_none_or(|root_end| end > root_end) {
            path.truncate(end);
            path.push(0);
            (S_OK, Some(end), free)
        } else {
            (S_FALSE, Some(end), free)
        }
    } else {
        (S_FALSE, Some(path_len), size - path_len)
    }
}

/// Removes one trailing backslash unless it is the path's root separator.
/// `size` is the available buffer capacity in UTF-16 code units.
pub fn path_cch_remove_backslash(path: &mut Vec<u16>, size: usize) -> i32 {
    path_cch_remove_backslash_ex(path, size).0
}

/// Adds a trailing backslash to a legacy MAX_PATH buffer, matching
/// Wine's `PathAddBackslashW`. Returns the end offset or `None` for oversized paths.
pub fn path_add_backslash(path: &mut Vec<u16>) -> Option<usize> {
    let length = wlen(path);
    if length >= MAX_PATH {
        return None;
    }
    if length != 0 && char_at(path, length - 1) != b'\\' as u16 {
        path.truncate(length);
        path.push(b'\\' as u16);
        path.push(0);
        Some(length + 1)
    } else {
        Some(length)
    }
}

/// Adds a trailing backslash when missing, returning `(HRESULT, end_offset,
/// remaining_capacity)` in UTF-16 code units. `None` offsets indicate a failed call.
pub fn path_cch_add_backslash_ex(path: &mut Vec<u16>, size: usize) -> (i32, Option<usize>, usize) {
    let length = wlen(path);
    let needs_termination = size != 0 && length != 0 && char_at(path, length - 1) != b'\\' as u16;
    if length
        >= if needs_termination {
            size.saturating_sub(1)
        } else {
            size
        }
    {
        return (STRSAFE_E_INSUFFICIENT_BUFFER, None, 0);
    }
    if !needs_termination {
        return (S_FALSE, Some(length), size - length);
    }
    path.truncate(length);
    path.push(b'\\' as u16);
    path.push(0);
    (S_OK, Some(length + 1), size - length - 1)
}

/// Adds a trailing backslash when missing.
pub fn path_cch_add_backslash(path: &mut Vec<u16>, size: usize) -> i32 {
    path_cch_add_backslash_ex(path, size).0
}

/// Finds the extension's UTF-16 offset using Wine's `PathCchFindExtension`
/// rules. An absent extension points to the terminating NUL.
pub fn path_cch_find_extension(path: &[u16], size: usize) -> (i32, Option<usize>) {
    if size == 0 || size > MAX_PATH_CCH || wlen(path) == path.len() {
        return (E_INVALIDARG, None);
    }
    let mut last_dot = None;
    let mut index = 0;
    while char_at(path, index) != 0 {
        match char_at(path, index) {
            unit if unit == b'\\' as u16 || unit == b' ' as u16 => last_dot = None,
            unit if unit == b'.' as u16 => last_dot = Some(index),
            _ => {}
        }
        index += 1;
        if index == size || index == MAX_PATH_CCH {
            return (E_INVALIDARG, None);
        }
    }
    (S_OK, Some(last_dot.unwrap_or(index)))
}

/// Removes the extension from a path with a valid NUL terminator within `size`.
pub fn path_cch_remove_extension(path: &mut Vec<u16>, size: usize) -> i32 {
    if size == 0 || size > MAX_PATH_CCH {
        return E_INVALIDARG;
    }
    let (_, Some(extension)) = path_cch_find_extension(path, size) else {
        return E_INVALIDARG;
    };
    let len = wlen(path);
    path.truncate(extension);
    path.push(0);
    if extension == len {
        S_FALSE
    } else {
        S_OK
    }
}

/// Adds an extension when no extension already exists, matching Wine's
/// `PathCchAddExtension` checks and buffer sizing.
pub fn path_cch_add_extension(path: &mut Vec<u16>, size: usize, extension: &[u16]) -> i32 {
    if size == 0 || size > MAX_PATH_CCH || wlen(extension) == extension.len() {
        return E_INVALIDARG;
    }
    let ext_len = wlen(extension);
    for (index, &unit) in extension[..ext_len].iter().enumerate() {
        if (unit == b'.' as u16 && index > 0) || unit == b' ' as u16 || unit == b'\\' as u16 {
            return E_INVALIDARG;
        }
    }
    let has_dot = char_at(extension, 0) == b'.' as u16;
    let (find_status, existing) = path_cch_find_extension(path, size);
    if find_status != S_OK {
        return find_status;
    }
    if char_at(path, existing.unwrap_or(0)) != 0 {
        return S_FALSE;
    }
    let path_len = wlen(path);
    let dot_len = usize::from(!has_dot);
    if path_len + dot_len + ext_len + 1 > size {
        return STRSAFE_E_INSUFFICIENT_BUFFER;
    }
    if ext_len == 0 || (has_dot && ext_len == 1) {
        return S_OK;
    }
    path.truncate(path_len);
    if !has_dot {
        path.push(b'.' as u16);
    }
    path.extend_from_slice(&extension[..ext_len]);
    path.push(0);
    S_OK
}

/// Renames a path extension using Wine's remove-then-add behavior.
pub fn path_cch_rename_extension(path: &mut Vec<u16>, size: usize, extension: &[u16]) -> i32 {
    let remove_status = path_cch_remove_extension(path, size);
    if remove_status < 0 {
        return remove_status;
    }
    let add_status = path_cch_add_extension(path, size, extension);
    if add_status < 0 {
        add_status
    } else {
        S_OK
    }
}

/// Reports whether a path begins with two backslashes, matching
/// Wine's `PathIsUNCW` behavior.
pub fn path_is_unc(path: &[u16]) -> bool {
    char_at(path, 0) == b'\\' as u16 && char_at(path, 1) == b'\\' as u16
}

/// Reports whether a UNC path contains exactly one separator after its server
/// name, matching Wine's `PathIsUNCServerShareW` behavior.
pub fn path_is_unc_server_share(path: &[u16]) -> bool {
    if !path_is_unc(path) {
        return false;
    }
    let mut seen_separator = false;
    for &unit in &path[2..wlen(path)] {
        if unit == b'\\' as u16 {
            if seen_separator {
                return false;
            }
            seen_separator = true;
        }
    }
    seen_separator
}

/// Removes the final file component with Wine's `PathRemoveFileSpecW` rules.
pub fn path_remove_file_spec(path: &mut Vec<u16>) -> bool {
    let len = wlen(path);
    if len == 0 {
        return false;
    }
    let mut root = if is_prefixed_volume(path) {
        Some(48)
    } else if is_prefixed_disk(path) {
        Some(6)
    } else if matches!(char_at(path, 0), 65..=90 | 97..=122) && char_at(path, 1) == b':' as u16 {
        Some(2)
    } else {
        None
    };
    if root.is_none() {
        let mut index = 0;
        if char_at(path, index) == b'\\' as u16 {
            index += 1;
        }
        if char_at(path, index + 1) != b'?' as u16 {
            if char_at(path, index) == b'\\' as u16 {
                index += 1;
            }
            if index > 1
                && matches!(char_at(path, index), 65..=90 | 97..=122)
                && char_at(path, index + 1) == b':' as u16
            {
                index += 2;
            }
            if char_at(path, index) == b'\\' as u16
                && char_at(path, index + 1) != 0
                && char_at(path, index + 1) != b'\\' as u16
            {
                index += 1;
            }
        }
        root = Some(index);
    } else if char_at(path, root.unwrap()) == b'\\' as u16 {
        root = root.map(|index| index + 1);
    }
    let root = root.unwrap();
    if let Some(mut separator) = (root..len).rev().find(|&index| path[index] == b'\\' as u16) {
        if separator != root {
            if separator > 0 && char_at(path, separator - 1) == b'\\' as u16 {
                separator -= 1;
            }
            path.truncate(separator);
            path.push(0);
            return true;
        }
    }
    if char_at(path, root) == 0 {
        return false;
    }
    path.truncate(root);
    path.push(0);
    true
}

/// Reports whether a path is a file spec, matching Wine's `PathIsFileSpecW`.
/// Backslashes and colons make a path a non-file-spec; forward slashes do not.
pub fn path_is_file_spec(path: &[u16]) -> bool {
    !path[..wlen(path)]
        .iter()
        .any(|&unit| unit == b'\\' as u16 || unit == b':' as u16)
}

/// Returns the drive index for a path, matching Wine's `PathGetDriveNumberW`.
/// ASCII drive letters map to 0–25; paths without a drive spec return -1.
pub fn path_get_drive_number(path: &[u16]) -> i32 {
    let start = if starts_with(path, b"\\\\?\\") { 4 } else { 0 };
    let drive = char_at(path, start);
    if drive == 0 || char_at(path, start + 1) != b':' as u16 {
        return -1;
    }
    match drive {
        65..=90 => i32::from(drive - 65),
        97..=122 => i32::from(drive - 97),
        _ => -1,
    }
}

/// Reports whether a path is a UNC server name, matching Wine's `PathIsUNCServerW`.
pub fn path_is_unc_server(path: &[u16]) -> bool {
    wlen(path) >= 2
        && char_at(path, 0) == b'\\' as u16
        && char_at(path, 1) == b'\\' as u16
        && !path[2..wlen(path)].contains(&(b'\\' as u16))
}

/// Reports whether a path is relative, matching Wine's `PathIsRelativeW`.
pub fn path_is_relative(path: &[u16]) -> bool {
    let len = wlen(path);
    len == 0 || (path[0] != b'\\' as u16 && char_at(path, 1) != b':' as u16)
}

/// Identifies UNC paths and returns the UTF-16 offset of the server name.
/// The returned offset is `Some(8)` for extended UNC paths and `Some(2)` for
/// conventional UNC paths; other paths return `None`.
pub fn path_is_unc_ex(path: &[u16]) -> (bool, Option<usize>) {
    if is_prefixed_unc(path) {
        (true, Some(8))
    } else if char_at(path, 0) == b'\\' as u16
        && char_at(path, 1) == b'\\' as u16
        && char_at(path, 2) != b'?' as u16
    {
        (true, Some(2))
    } else {
        (false, None)
    }
}

/// Removes the final file-spec component while preserving a drive or UNC root.
/// The size is measured in UTF-16 code units and includes the terminator.
pub fn path_cch_remove_file_spec(path: &mut Vec<u16>, size: usize) -> i32 {
    let len = wlen(path);
    if size == 0 || size > MAX_PATH_CCH {
        return E_INVALIDARG;
    }
    let (status, mut start) = path_cch_skip_root(path);
    if status != S_OK {
        start = 0;
    }
    if start > 0
        && char_at(path, start - 1) == b'\\' as u16
        && ((is_prefixed_unc(path) && char_at(path, 8) != 0)
            || (path[0] == b'\\' as u16
                && char_at(path, 1) == b'\\' as u16
                && char_at(path, 2) != 0
                && char_at(path, 2) != b'?' as u16))
    {
        start -= 1;
    }
    let mut last = (start..len)
        .rev()
        .find(|&index| path[index] == b'\\' as u16)
        .unwrap_or(start);
    if last > start
        && char_at(path, last - 1) == b'\\' as u16
        && char_at(path, last + 1) != b'?' as u16
    {
        last -= 1;
    }
    if last >= size {
        return E_INVALIDARG;
    }
    if char_at(path, last) == 0 {
        return S_FALSE;
    }
    path.truncate(last);
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
    fn is_root_matches_wine_pathisrootw_wrapper() {
        for (path, expected) in [
            ("C:\\", true),
            ("C:", false),
            ("\\\\server\\share", true),
            ("\\\\server\\share\\file", false),
            ("\\\\?\\C:\\", true),
            ("\\\\?\\UNC\\server\\share", true),
            ("relative\\path", false),
            ("", false),
        ] {
            assert_eq!(path_is_root(&w(path)), expected, "{path}");
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
    fn strip_to_root_matches_wine_pathstriptorootw_wrapper() {
        for (input, expected, result) in [
            ("C:\\a\\b", "C:\\", true),
            ("C:\\", "C:\\", true),
            ("\\\\server\\share\\file", "\\\\server\\share", true),
            ("\\\\server\\share", "\\\\server\\share", true),
            ("relative", "", false),
            ("", "", false),
        ] {
            let mut path = w(input);
            assert_eq!(path_strip_to_root(&mut path), result, "{input}");
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
    fn remove_file_spec_matches_wine_vectors() {
        for (input, expected, status) in [
            ("", "", S_FALSE),
            ("a", "", S_OK),
            ("a\\", "a", S_OK),
            ("a\\b", "a", S_OK),
            ("\\", "\\", S_FALSE),
            ("\\a", "\\", S_OK),
            ("\\a\\", "\\a", S_OK),
            ("\\a\\b", "\\a", S_OK),
            ("\\\\", "\\\\", S_FALSE),
            ("\\\\a", "\\\\a", S_FALSE),
            ("\\\\a\\", "\\\\a", S_OK),
            ("\\\\a\\b", "\\\\a\\b", S_FALSE),
            ("\\\\a\\b\\", "\\\\a\\b", S_OK),
            ("\\\\a\\b\\c", "\\\\a\\b", S_OK),
            ("\\\\\\\\\\\\", "\\\\\\\\", S_OK),
            ("C:", "C:", S_FALSE),
            ("C:a", "C:", S_OK),
            ("C:a\\", "C:a", S_OK),
            ("C:a\\b", "C:a", S_OK),
            ("C:\\a\\\\b", "C:\\a", S_OK),
            ("C:\\", "C:\\", S_FALSE),
            ("C:\\a", "C:\\", S_OK),
            ("C:\\a\\", "C:\\a", S_OK),
            ("C:\\a\\b", "C:\\a", S_OK),
            ("\\\\?\\", "\\\\?", S_OK),
            ("\\\\?\\a", "\\\\?", S_OK),
            ("\\\\?\\a\\", "\\\\?\\a", S_OK),
            ("\\\\?\\a\\b", "\\\\?\\a", S_OK),
            ("\\\\?\\C:", "\\\\?\\C:", S_FALSE),
            ("\\\\?\\C:a", "\\\\?\\C:", S_OK),
            ("\\\\?\\C:a\\", "\\\\?\\C:a", S_OK),
            ("\\\\?\\C:a\\b", "\\\\?\\C:a", S_OK),
            ("\\\\?\\C:\\", "\\\\?\\C:\\", S_FALSE),
            ("\\\\?\\C:\\a", "\\\\?\\C:\\", S_OK),
            ("\\\\?\\C:\\a\\", "\\\\?\\C:\\a", S_OK),
            ("\\\\?\\C:\\a\\b", "\\\\?\\C:\\a", S_OK),
            ("\\\\?\\UNC\\", "\\\\?\\UNC\\", S_FALSE),
            ("\\\\?\\UNC\\a", "\\\\?\\UNC\\a", S_FALSE),
            ("\\\\?\\UNC\\a\\", "\\\\?\\UNC\\a", S_OK),
            ("\\\\?\\UNC\\a\\b", "\\\\?\\UNC\\a\\b", S_FALSE),
            ("\\\\?\\UNC\\a\\b\\", "\\\\?\\UNC\\a\\b", S_OK),
            ("\\\\?\\UNC\\a\\b\\c", "\\\\?\\UNC\\a\\b", S_OK),
        ] {
            let mut path = w(input);
            assert_eq!(
                path_cch_remove_file_spec(&mut path, MAX_PATH_CCH),
                status,
                "{input}"
            );
            assert_eq!(path, w(expected), "{input}");
        }
        let volume = "\\\\?\\Volume{e51a1864-6f2d-4019-b73d-f4e60e600c26}";
        for (input, expected, status) in [
            (volume.to_string(), volume.to_string(), S_FALSE),
            (format!("{volume}a"), volume.to_string(), S_OK),
            (format!("{volume}a\\"), format!("{volume}a"), S_OK),
            (format!("{volume}a\\b"), format!("{volume}a"), S_OK),
            (format!("{volume}\\"), format!("{volume}\\"), S_FALSE),
            (format!("{volume}\\a"), format!("{volume}\\"), S_OK),
            (format!("{volume}\\a\\"), format!("{volume}\\a"), S_OK),
            (format!("{volume}\\a\\b"), format!("{volume}\\a"), S_OK),
        ] {
            let mut path = w(&input);
            assert_eq!(
                path_cch_remove_file_spec(&mut path, MAX_PATH_CCH),
                status,
                "{input}"
            );
            assert_eq!(path, w(&expected), "{input}");
        }
    }

    #[test]
    fn remove_file_spec_validates_size() {
        for size in [0, MAX_PATH_CCH + 1] {
            let mut path = w("C:\\a");
            let original = path.clone();
            assert_eq!(path_cch_remove_file_spec(&mut path, size), E_INVALIDARG);
            assert_eq!(path, original);
        }
        let mut path = w("C:\\a");
        assert_eq!(path_cch_remove_file_spec(&mut path, 3), E_INVALIDARG);
        assert_eq!(path, w("C:\\a"));
        assert_eq!(path_cch_remove_file_spec(&mut path, 4), S_OK);
        assert_eq!(path, w("C:\\"));
    }

    #[test]
    fn remove_backslash_matches_wine_vectors() {
        for (input, expected, status) in [
            ("", "", S_FALSE),
            ("C", "C", S_FALSE),
            ("C\\", "C", S_OK),
            ("C:", "C:", S_FALSE),
            ("C:\\", "C:\\", S_FALSE),
            ("C:\\\\", "C:\\", S_OK),
            ("C:\\a\\", "C:\\a", S_OK),
            ("C:\\a\\\\", "C:\\a\\", S_OK),
            ("\\", "\\", S_FALSE),
            ("\\\\", "\\\\", S_FALSE),
            ("\\?\\", "\\?", S_OK),
            ("\\?\\\\", "\\?\\", S_OK),
            ("\\a\\", "\\a", S_OK),
            ("\\a\\\\", "\\a\\", S_OK),
            ("\\\\a\\", "\\\\a", S_OK),
            ("\\\\a\\b\\", "\\\\a\\b", S_OK),
            ("\\\\a\\\\", "\\\\a\\", S_OK),
            ("\\\\?\\", "\\\\?", S_OK),
            ("\\\\?\\\\", "\\\\?\\", S_OK),
            ("\\\\?\\C:", "\\\\?\\C:", S_FALSE),
            ("\\\\?\\C:\\", "\\\\?\\C:\\", S_FALSE),
            ("\\?\\UNC\\", "\\?\\UNC", S_OK),
            ("\\\\?\\UNC", "\\\\?\\UNC", S_FALSE),
            ("\\\\?\\UNC\\", "\\\\?\\UNC\\", S_FALSE),
            ("\\\\?\\UNC\\a", "\\\\?\\UNC\\a", S_FALSE),
            ("\\\\?\\UNC\\a\\", "\\\\?\\UNC\\a", S_OK),
            ("\\\\?\\UNC\\a\\b\\", "\\\\?\\UNC\\a\\b", S_OK),
        ] {
            let mut path = w(input);
            assert_eq!(
                path_cch_remove_backslash(&mut path, MAX_PATH_CCH),
                status,
                "{input}"
            );
            assert_eq!(path, w(expected), "{input}");
        }
        let volume = "\\\\?\\Volume{e51a1864-6f2d-4019-b73d-f4e60e600c26}";
        for (input, expected, status) in [
            (volume.to_string(), volume.to_string(), S_FALSE),
            (format!("{volume}\\"), format!("{volume}\\"), S_FALSE),
            (format!("{volume}\\a"), format!("{volume}\\a"), S_FALSE),
            (format!("{volume}\\a\\"), format!("{volume}\\a"), S_OK),
        ] {
            let mut path = w(&input);
            assert_eq!(
                path_cch_remove_backslash(&mut path, MAX_PATH_CCH),
                status,
                "{input}"
            );
            assert_eq!(path, w(&expected), "{input}");
        }
    }

    #[test]
    fn remove_backslash_checks_capacity() {
        let mut path = w("C:\\a\\");
        assert_eq!(path_cch_remove_backslash(&mut path, 0), E_INVALIDARG);
        assert_eq!(path_cch_remove_backslash(&mut path, 5), E_INVALIDARG);
        assert_eq!(path, w("C:\\a\\"));
        assert_eq!(path_cch_remove_backslash(&mut path, 6), S_OK);
        assert_eq!(path, w("C:\\a"));
        let mut path = w("C:\\a");
        assert_eq!(path_cch_remove_backslash(&mut path, 4), E_INVALIDARG);
        assert_eq!(path, w("C:\\a"));
    }

    #[test]
    fn remove_backslash_ex_matches_wine_offsets_and_free_sizes() {
        for (input, expected, offset, free, status) in [
            ("", "", 0, 1, S_FALSE),
            ("C", "C", 1, 1, S_FALSE),
            ("C\\", "C", 1, 2, S_OK),
            ("C:", "C:", 2, 1, S_FALSE),
            ("C:\\", "C:\\", 2, 2, S_FALSE),
            ("C:\\\\", "C:\\", 3, 2, S_OK),
            ("C:\\a\\", "C:\\a", 4, 2, S_OK),
            ("C:\\a\\\\", "C:\\a\\", 5, 2, S_OK),
            ("\\", "\\", 0, 2, S_FALSE),
            ("\\\\", "\\\\", 1, 2, S_FALSE),
            ("\\?\\", "\\?", 2, 2, S_OK),
            ("\\?\\\\", "\\?\\", 3, 2, S_OK),
            ("\\a\\", "\\a", 2, 2, S_OK),
            ("\\a\\\\", "\\a\\", 3, 2, S_OK),
            ("\\\\a\\", "\\\\a", 3, 2, S_OK),
            ("\\\\a\\b\\", "\\\\a\\b", 5, 2, S_OK),
            ("\\\\a\\\\", "\\\\a\\", 4, 2, S_OK),
            ("\\\\?\\", "\\\\?", 3, 2, S_OK),
            ("\\\\?\\\\", "\\\\?\\", 4, 2, S_OK),
            ("\\\\?\\C:", "\\\\?\\C:", 6, 1, S_FALSE),
            ("\\\\?\\C:\\", "\\\\?\\C:\\", 6, 2, S_FALSE),
            ("\\?\\UNC\\", "\\?\\UNC", 6, 2, S_OK),
            ("\\\\?\\UNC", "\\\\?\\UNC", 7, 1, S_FALSE),
            ("\\\\?\\UNC\\", "\\\\?\\UNC\\", 7, 2, S_FALSE),
            ("\\\\?\\UNC\\a", "\\\\?\\UNC\\a", 9, 1, S_FALSE),
            ("\\\\?\\UNC\\a\\", "\\\\?\\UNC\\a", 9, 2, S_OK),
            ("\\\\?\\UNC\\a\\b\\", "\\\\?\\UNC\\a\\b", 11, 2, S_OK),
        ] {
            let mut path = w(input);
            let size = path.len();
            let result = path_cch_remove_backslash_ex(&mut path, size);
            assert_eq!(result, (status, Some(offset), free), "{input}");
            assert_eq!(path, w(expected), "{input}");
        }
        let mut invalid = w("C:\\a");
        assert_eq!(
            path_cch_remove_backslash_ex(&mut invalid, 0),
            (E_INVALIDARG, None, 0)
        );
        let mut truncated = w("C:\\a");
        assert_eq!(
            path_cch_remove_backslash_ex(&mut truncated, 4),
            (E_INVALIDARG, None, 0)
        );
    }

    #[test]
    fn legacy_add_backslash_matches_wine_rules() {
        for (input, expected, offset) in [
            ("", "", 0),
            ("one\\two", "one\\two\\", 8),
            ("one\\..\\two", "one\\..\\two\\", 11),
            (" ", " \\", 2),
            ("C:\\one\\", "C:\\one\\", 7),
        ] {
            let mut path = w(input);
            assert_eq!(path_add_backslash(&mut path), Some(offset), "{input}");
            assert_eq!(path, w(expected), "{input}");
        }
        let mut too_long = vec![b'a' as u16; MAX_PATH];
        too_long.push(0);
        let before = too_long.clone();
        assert_eq!(path_add_backslash(&mut too_long), None);
        assert_eq!(too_long, before);
    }

    #[test]
    fn add_backslash_matches_wine_vectors() {
        for (input, expected, size, expected_status, expected_end, remaining) in [
            ("C:", "C:\\", 260, S_OK, Some(3), 257),
            ("a.txt", "a.txt\\", 260, S_OK, Some(6), 254),
            ("a/b", "a/b\\", 260, S_OK, Some(4), 256),
            ("C:\\", "C:\\", 260, S_FALSE, Some(3), 257),
            ("C:\\", "C:\\", 4, S_FALSE, Some(3), 1),
            ("C:", "C:", 2, STRSAFE_E_INSUFFICIENT_BUFFER, None, 0),
            ("C:", "C:", 3, STRSAFE_E_INSUFFICIENT_BUFFER, None, 0),
            ("C:\\", "C:\\", 2, STRSAFE_E_INSUFFICIENT_BUFFER, None, 0),
            ("C:\\", "C:\\", 3, STRSAFE_E_INSUFFICIENT_BUFFER, None, 0),
            ("C:\\", "C:\\", 0, STRSAFE_E_INSUFFICIENT_BUFFER, None, 0),
            ("C:", "C:", 0, STRSAFE_E_INSUFFICIENT_BUFFER, None, 0),
            ("", "", 0, STRSAFE_E_INSUFFICIENT_BUFFER, None, 0),
            ("", "", 1, S_FALSE, Some(0), 1),
            ("", "", 2, S_FALSE, Some(0), 2),
        ] {
            let mut path = w(input);
            let result = path_cch_add_backslash_ex(&mut path, size);
            assert_eq!(
                result,
                (expected_status, expected_end, remaining),
                "{input} {size}"
            );
            assert_eq!(path, w(expected), "{input} {size}");
            assert_eq!(path_cch_add_backslash(&mut w(input), size), expected_status);
        }
    }

    #[test]
    fn find_extension_matches_wine_offsets() {
        for (path, expected) in [
            ("1.exe", 1),
            ("C:1.exe", 3),
            ("C:\\1.exe", 4),
            ("\\1.exe", 2),
            ("\\\\1.exe", 3),
            ("\\\\?\\C:1.exe", 7),
            ("\\\\?\\C:\\1.exe", 8),
            ("\\\\?\\UNC\\1.exe", 9),
            ("\\\\?\\UNC\\192.168.1.1\\1.exe", 21),
            (
                "\\\\?\\Volume{e51a1864-6f2d-4019-b73d-f4e60e600c26}\\1.exe",
                50,
            ),
            ("C:\\a/1.exe", 6),
            ("/1.exe", 2),
            ("//1.exe", 3),
            ("C:\\a/b/1.exe", 8),
            ("/a/1.exe", 4),
            ("", 0),
            (" ", 1),
            (".", 0),
            ("..", 1),
            ("a", 1),
            ("a.", 1),
            (".a.b.", 4),
            ("a. ", 3),
            ("a.\\", 3),
            ("\\\\?\\UNC\\192.168.1.1", 17),
            ("\\\\?\\UNC\\192.168.1.1\\", 20),
            ("\\\\?\\UNC\\192.168.1.1\\a", 21),
        ] {
            assert_eq!(
                path_cch_find_extension(&w(path), MAX_PATH_CCH),
                (S_OK, Some(expected)),
                "{path}"
            );
        }
        assert_eq!(path_cch_find_extension(&w("file"), 0), (E_INVALIDARG, None));
        assert_eq!(
            path_cch_find_extension(&w("file"), MAX_PATH_CCH + 1),
            (E_INVALIDARG, None)
        );
        assert_eq!(path_cch_find_extension(&w("file"), 4), (E_INVALIDARG, None));
        assert_eq!(
            path_cch_find_extension(&[b'x' as u16; 8], 8),
            (E_INVALIDARG, None)
        );
        assert_eq!(path_cch_find_extension(&w("file"), 5), (S_OK, Some(4)));
        assert_eq!(
            path_cch_find_extension(&vec![b'a' as u16; MAX_PATH_CCH], MAX_PATH_CCH),
            (E_INVALIDARG, None)
        );
    }

    #[test]
    fn remove_extension_matches_wine_vectors() {
        for (input, expected, status) in [
            ("1.exe", "1", S_OK),
            ("C:1.exe", "C:1", S_OK),
            ("C:\\1.exe", "C:\\1", S_OK),
            ("\\1.exe", "\\1", S_OK),
            ("\\\\1.exe", "\\\\1", S_OK),
            ("\\\\?\\C:1.exe", "\\\\?\\C:1", S_OK),
            ("\\\\?\\C:\\1.exe", "\\\\?\\C:\\1", S_OK),
            ("\\\\?\\UNC\\1.exe", "\\\\?\\UNC\\1", S_OK),
            (
                "\\\\?\\UNC\\192.168.1.1\\1.exe",
                "\\\\?\\UNC\\192.168.1.1\\1",
                S_OK,
            ),
            (
                "\\\\?\\Volume{e51a1864-6f2d-4019-b73d-f4e60e600c26}\\1.exe",
                "\\\\?\\Volume{e51a1864-6f2d-4019-b73d-f4e60e600c26}\\1",
                S_OK,
            ),
            ("", "", S_FALSE),
            (" ", " ", S_FALSE),
            (".", "", S_OK),
            ("..", ".", S_OK),
            ("a", "a", S_FALSE),
            ("a.", "a", S_OK),
            (".a.b.", ".a.b", S_OK),
            ("a. ", "a. ", S_FALSE),
            ("a.\\", "a.\\", S_FALSE),
            ("\\\\?\\UNC\\192.168.1.1", "\\\\?\\UNC\\192.168.1", S_OK),
            (
                "\\\\?\\UNC\\192.168.1.1\\",
                "\\\\?\\UNC\\192.168.1.1\\",
                S_FALSE,
            ),
            (
                "\\\\?\\UNC\\192.168.1.1\\a",
                "\\\\?\\UNC\\192.168.1.1\\a",
                S_FALSE,
            ),
        ] {
            let mut path = w(input);
            assert_eq!(
                path_cch_remove_extension(&mut path, MAX_PATH_CCH),
                status,
                "{input}"
            );
            assert_eq!(path, w(expected), "{input}");
        }
        let mut path = w("C:\\1.exe");
        let original = path.clone();
        assert_eq!(path_cch_remove_extension(&mut path, 0), E_INVALIDARG);
        assert_eq!(path, original);
        assert_eq!(
            path_cch_remove_extension(&mut path, MAX_PATH_CCH + 1),
            E_INVALIDARG
        );
        assert_eq!(path, original);
        assert_eq!(path_cch_remove_extension(&mut path, 8), E_INVALIDARG);
        assert_eq!(path, original);
        assert_eq!(path_cch_remove_extension(&mut path, 9), S_OK);
        assert_eq!(path, w("C:\\1"));
    }

    #[test]
    fn add_extension_matches_wine_vectors() {
        for (input, extension, expected, status) in [
            ("", ".exe", ".exe", S_OK),
            ("C:\\", "", "C:\\", S_OK),
            ("C:", ".exe", "C:.exe", S_OK),
            ("C:\\", ".exe", "C:\\.exe", S_OK),
            ("\\", ".exe", "\\.exe", S_OK),
            ("\\\\", ".exe", "\\\\.exe", S_OK),
            ("\\\\?\\C:", ".exe", "\\\\?\\C:.exe", S_OK),
            ("\\\\?\\C:\\", ".exe", "\\\\?\\C:\\.exe", S_OK),
            ("\\\\?\\UNC\\", ".exe", "\\\\?\\UNC\\.exe", S_OK),
            (
                "\\\\?\\UNC\\192.168.1.1\\",
                ".exe",
                "\\\\?\\UNC\\192.168.1.1\\.exe",
                S_OK,
            ),
            (
                "\\\\?\\Volume{e51a1864-6f2d-4019-b73d-f4e60e600c26}\\",
                ".exe",
                "\\\\?\\Volume{e51a1864-6f2d-4019-b73d-f4e60e600c26}\\.exe",
                S_OK,
            ),
            ("C:\\", "exe", "C:\\.exe", S_OK),
            ("C:\\", ".", "C:\\", S_OK),
            ("C:\\1.exe", ".txt", "C:\\1.exe", S_FALSE),
            ("C:\\", "./", "C:\\./", S_OK),
            ("C:\\", ".?", "C:\\.?", S_OK),
            ("C:\\", ".%", "C:\\.%", S_OK),
            ("C:\\", ".*", "C:\\.*", S_OK),
            ("C:\\", ".:", "C:\\.:", S_OK),
            ("C:\\", ".|", "C:\\.|", S_OK),
            ("C:\\", ".\"", "C:\\.\"", S_OK),
            ("C:\\", ".<", "C:\\.<", S_OK),
            ("C:\\", ".>", "C:\\.>", S_OK),
        ] {
            let mut path = w(input);
            assert_eq!(
                path_cch_add_extension(&mut path, MAX_PATH_CCH, &w(extension)),
                status,
                "{input} {extension}"
            );
            assert_eq!(path, w(expected), "{input} {extension}");
        }
        for extension in [
            " exe", ". exe", " ", "\\", "..", ". ", ".\\", ".a.", ".a ", ".a\\",
        ] {
            let mut path = w("C:\\");
            assert_eq!(
                path_cch_add_extension(&mut path, MAX_PATH_CCH, &w(extension)),
                E_INVALIDARG,
                "{extension}"
            );
        }
        let mut path = w("C:\\");
        assert_eq!(
            path_cch_add_extension(&mut path, 0, &w(".exe")),
            E_INVALIDARG
        );
        assert_eq!(
            path_cch_add_extension(&mut path, MAX_PATH_CCH + 1, &w(".exe")),
            E_INVALIDARG
        );
        assert_eq!(
            path_cch_add_extension(&mut path, 7, &w(".exe")),
            STRSAFE_E_INSUFFICIENT_BUFFER
        );
        assert_eq!(path, w("C:\\"));
        assert_eq!(path_cch_add_extension(&mut path, 8, &w(".exe")), S_OK);
        assert_eq!(path, w("C:\\.exe"));
        assert_eq!(
            path_cch_add_extension(&mut path, 4, &w(".exe")),
            E_INVALIDARG
        );
        let mut unterminated = vec![b'C' as u16, b':' as u16];
        assert_eq!(
            path_cch_add_extension(&mut unterminated, 8, &w(".exe")),
            E_INVALIDARG
        );
    }

    #[test]
    fn rename_extension_matches_wine_vectors() {
        for (input, extension, expected) in [
            ("1.exe", ".txt", "1.txt"),
            ("C:1.exe", ".txt", "C:1.txt"),
            ("C:\\1.exe", ".txt", "C:\\1.txt"),
            ("\\1.exe", ".txt", "\\1.txt"),
            ("\\\\1.exe", ".txt", "\\\\1.txt"),
            ("\\\\?\\C:1.exe", ".txt", "\\\\?\\C:1.txt"),
            ("\\\\?\\C:\\1.exe", ".txt", "\\\\?\\C:\\1.txt"),
            ("\\\\?\\UNC\\1.exe", ".txt", "\\\\?\\UNC\\1.txt"),
            (
                "\\\\?\\UNC\\192.168.1.1\\1.exe",
                ".txt",
                "\\\\?\\UNC\\192.168.1.1\\1.txt",
            ),
            (
                "\\\\?\\Volume{e51a1864-6f2d-4019-b73d-f4e60e600c26}\\1.exe",
                ".txt",
                "\\\\?\\Volume{e51a1864-6f2d-4019-b73d-f4e60e600c26}\\1.txt",
            ),
            ("C:\\1.exe", "", "C:\\1"),
            ("C:\\1.exe", "txt", "C:\\1.txt"),
        ] {
            let mut path = w(input);
            assert_eq!(
                path_cch_rename_extension(&mut path, MAX_PATH_CCH, &w(extension)),
                S_OK,
                "{input}"
            );
            assert_eq!(path, w(expected), "{input}");
        }
        let mut path = w("C:\\1.exe");
        assert_eq!(
            path_cch_rename_extension(&mut path, 0, &w(".txt")),
            E_INVALIDARG
        );
        assert_eq!(
            path_cch_rename_extension(&mut path, MAX_PATH_CCH + 1, &w(".txt")),
            E_INVALIDARG
        );
        assert_eq!(
            path_cch_rename_extension(&mut path, 64, &w(". bad")),
            E_INVALIDARG
        );
        let mut too_long = w("C:\\1.exe");
        assert_eq!(
            path_cch_rename_extension(&mut too_long, 9, &w(".longer")),
            STRSAFE_E_INSUFFICIENT_BUFFER
        );
        assert_eq!(too_long, w("C:\\1"));
    }

    #[test]
    fn is_unc_and_server_share_match_wine_implementation() {
        for (path, expected_unc, expected_share) in [
            ("", false, false),
            ("\\", false, false),
            ("\\\\", true, false),
            ("\\\\server", true, false),
            ("\\\\server\\", true, true),
            ("\\\\server\\share", true, true),
            ("\\\\server\\share\\file", true, false),
            ("\\\\server\\\\share", true, false),
            ("C:\\path", false, false),
        ] {
            let path = w(path);
            assert_eq!(path_is_unc(&path), expected_unc);
            assert_eq!(path_is_unc_server_share(&path), expected_share);
        }
    }

    #[test]
    fn path_remove_file_spec_matches_wine_vectors() {
        for (input, expected, result) in [
            ("Q:\\", "Q:\\", false),
            ("Q:\\\\", "Q:\\", true),
            ("Q:\\test1\\test2", "Q:\\test1", true),
            ("Q:relative\\file", "Q:relative", true),
            ("Q:\\test1\\test2\\", "Q:\\test1\\test2", true),
            ("Q:\\test1\\test2\\\\", "Q:\\test1\\test2", true),
            ("\\\\Q:\\", "\\\\Q:", true),
            ("\\\\Q:\\aaa", "\\\\Q:\\", true),
            ("\\\\o\\Q:\\aaa", "\\\\o\\Q:", true),
            ("\\Q:\\", "\\Q:", true),
            ("test", "", true),
            ("\\\\?\\Q:\\", "\\\\?\\Q:\\", false),
            ("\\\\?\\Q:\\a", "\\\\?\\Q:\\", true),
            ("\\\\.\\Q:\\", "\\\\.\\Q:", true),
            ("\\\\?\\", "\\\\?", true),
            ("\\\\?\\UNC\\a\\b\\c", "\\\\?\\UNC\\a\\b", true),
            (
                "\\\\?\\Volume{aaaaaaaa-bbbb-cccc-dddd-ffffffffffff}\\a",
                "\\\\?\\Volume{aaaaaaaa-bbbb-cccc-dddd-ffffffffffff}\\",
                true,
            ),
            ("\\\\a\\b\\c", "\\\\a\\b", true),
            ("\\\\a\\b", "\\\\a", true),
            ("\\\\a", "\\\\", true),
            ("\\a\\b", "\\a", true),
            ("\\a", "\\", true),
            ("", "", false),
        ] {
            let mut path = w(input);
            assert_eq!(path_remove_file_spec(&mut path), result, "{input}");
            assert_eq!(path, w(expected), "{input}");
        }
    }

    #[test]
    fn is_file_spec_matches_wine_rules() {
        for (path, expected) in [
            ("", true),
            ("file.txt", true),
            ("name with spaces.txt", true),
            ("directory\\file.txt", false),
            ("C:file.txt", false),
            ("C:\\file.txt", false),
            ("directory/file.txt", true),
        ] {
            assert_eq!(path_is_file_spec(&w(path)), expected, "{path}");
        }
        assert!(path_is_file_spec(&[]));
        let mut trailing_storage = w("file.txt");
        trailing_storage.extend(w("\\ignored"));
        assert!(path_is_file_spec(&trailing_storage));
    }

    #[test]
    fn get_drive_number_matches_wine_rules() {
        for (path, expected) in [
            ("", -1),
            ("C:", 2),
            ("C:\\folder", 2),
            ("a:relative", 0),
            ("z:\\", 25),
            ("\\\\?\\D:\\folder", 3),
            ("\\\\?\\z:relative", 25),
            ("\\\\server\\share", -1),
            ("1:\\folder", -1),
            ("C", -1),
            ("\\\\?\\", -1),
            ("\\\\?\\1:", -1),
        ] {
            assert_eq!(path_get_drive_number(&w(path)), expected, "{path}");
        }
        assert_eq!(path_get_drive_number(&[]), -1);
    }

    #[test]
    fn is_unc_server_matches_wine_rules() {
        for (path, expected) in [
            ("", false),
            ("\\", false),
            ("\\\\", true),
            ("\\\\server", true),
            ("\\\\server\\", false),
            ("\\\\server\\share", false),
            ("server", false),
        ] {
            assert_eq!(path_is_unc_server(&w(path)), expected, "{path}");
        }
        assert!(!path_is_unc_server(&[]));
    }

    #[test]
    fn is_relative_matches_wine_vectors() {
        for (path, expected) in [
            ("", true),
            ("test.txt", true),
            ("\\\\folder\\test.txt", false),
            ("file://folder/test.txt", true),
            ("C:\\test.txt", false),
            ("file:///C:/test.txt", true),
        ] {
            assert_eq!(path_is_relative(&w(path)), expected, "{path}");
        }
    }

    #[test]
    fn is_unc_ex_matches_wine_vectors() {
        for (path, expected) in [
            ("\\\\", Some(2)),
            ("\\\\a\\", Some(2)),
            ("\\\\.\\", Some(2)),
            ("\\\\?\\UNC\\", Some(8)),
            ("\\\\?\\UNC\\a", Some(8)),
            ("\\\\?\\unc\\", Some(8)),
            ("\\\\?\\unc\\a", Some(8)),
            ("", None),
            ("\\", None),
            ("C:\\", None),
            ("\\??\\", None),
            ("\\\\?\\", None),
            ("\\\\?\\UNC", None),
            ("\\\\?\\C:", None),
            ("\\\\?\\C:\\", None),
            ("\\\\?\\C:\\a", None),
            (
                "\\\\?\\Volume{e51a1864-6f2d-4019-b73d-f4e60e600c26}\\",
                None,
            ),
        ] {
            let result = path_is_unc_ex(&w(path));
            assert_eq!(result, (expected.is_some(), expected), "{path}");
        }
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
