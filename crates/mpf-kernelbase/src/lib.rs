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
/// `PathGetCharTypeW` flag for a character valid in long file names.
pub const GCT_LFNCHAR: u32 = 0x1;
/// `PathGetCharTypeW` flag for a character valid in DOS 8.3 names.
pub const GCT_SHORTCHAR: u32 = 0x2;
/// `PathGetCharTypeW` flag for a wildcard character.
pub const GCT_WILD: u32 = 0x4;
/// `PathGetCharTypeW` flag for a path separator.
pub const GCT_SEPARATOR: u32 = 0x8;
/// `PathIsValidCharW` class for wildcard letters.
pub const PATH_CHAR_CLASS_LETTER: u32 = 0x0000_0001;
/// `PathIsValidCharW` class for the asterisk.
pub const PATH_CHAR_CLASS_ASTERISK: u32 = 0x0000_0002;
/// `PathIsValidCharW` class for a dot.
pub const PATH_CHAR_CLASS_DOT: u32 = 0x0000_0004;
/// `PathIsValidCharW` class for a backslash.
pub const PATH_CHAR_CLASS_BACKSLASH: u32 = 0x0000_0008;
/// `PathIsValidCharW` class for a colon.
pub const PATH_CHAR_CLASS_COLON: u32 = 0x0000_0010;
/// `PathIsValidCharW` class for a semicolon.
pub const PATH_CHAR_CLASS_SEMICOLON: u32 = 0x0000_0020;
/// `PathIsValidCharW` class for a comma.
pub const PATH_CHAR_CLASS_COMMA: u32 = 0x0000_0040;
/// `PathIsValidCharW` class for a space.
pub const PATH_CHAR_CLASS_SPACE: u32 = 0x0000_0080;
/// `PathIsValidCharW` class for other valid characters.
pub const PATH_CHAR_CLASS_OTHER_VALID: u32 = 0x0000_0100;
/// `PathIsValidCharW` class for a double quote.
pub const PATH_CHAR_CLASS_DOUBLEQUOTE: u32 = 0x0000_0200;
/// `PathIsValidCharW` mask accepting any ASCII character class.
pub const PATH_CHAR_CLASS_ANY: u32 = u32::MAX;

const MAX_PATH_CCH: usize = 0x8000;
const MAX_PATH: usize = 260;
const FILE_ATTRIBUTE_DIRECTORY: u32 = 0x10;

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

/// Returns the offset from Wine's legacy `PathSkipRootW`, or `None` when the
/// path is empty or has no recognized drive/UNC root.
pub fn path_skip_root(path: &[u16]) -> Option<usize> {
    let len = wlen(path);
    if len == 0 {
        return None;
    }
    if char_at(path, 0) == b'\\' as u16 && char_at(path, 1) == b'\\' as u16 {
        let server_separator = path[2..len]
            .iter()
            .position(|&unit| unit == b'\\' as u16)
            .map(|index| index + 2)?;
        let share_separator = path[server_separator + 1..len]
            .iter()
            .position(|&unit| unit == b'\\' as u16)
            .map(|index| index + server_separator + 1)?;
        return Some(share_separator + 1);
    }
    if char_at(path, 1) == b':' as u16 && char_at(path, 2) == b'\\' as u16 {
        return Some(3);
    }
    None
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

/// Wraps a path containing spaces in quotes when Wine's MAX_PATH bound permits.
pub fn path_quote_spaces(path: &mut Vec<u16>) {
    let len = wlen(path);
    if path[..len].contains(&(b' ' as u16)) && len + 3 < MAX_PATH {
        let mut quoted = Vec::with_capacity(len + 3);
        quoted.push(b'"' as u16);
        quoted.extend_from_slice(&path[..len]);
        quoted.push(b'"' as u16);
        quoted.push(0);
        *path = quoted;
    }
}

/// Reports whether a name exceeds DOS 8.3 restrictions, matching
/// `PathIsLFNFileSpecW`. `None` represents Wine's false result for NULL.
pub fn path_is_lfn_file_spec(path: Option<&[u16]>) -> bool {
    let Some(path) = path else {
        return false;
    };
    let len = wlen(path);
    let mut name_len = 0;
    let mut extension_len = 0;
    for &unit in &path[..len] {
        if unit == b' ' as u16 {
            return true;
        }
        if unit == b'.' as u16 {
            if extension_len != 0 {
                return true;
            }
            extension_len = 1;
        } else if extension_len != 0 {
            extension_len += 1;
            if extension_len > 4 {
                return true;
            }
        } else {
            name_len += 1;
            if name_len > 8 {
                return true;
            }
        }
    }
    false
}

/// Parses and removes the optional icon index, then unquotes and trims the
/// path, matching Wine's `PathParseIconLocationW`.
pub fn path_parse_icon_location(path: &mut Vec<u16>) -> i32 {
    let len = wlen(path);
    let mut icon_index = 0;
    if let Some(comma) = path[..len].iter().position(|&unit| unit == b',' as u16) {
        let index = &path[comma + 1..len];
        if index
            .first()
            .is_some_and(|&unit| unit == b'-' as u16 || (b'0' as u16..=b'9' as u16).contains(&unit))
        {
            let negative = index.first() == Some(&(b'-' as u16));
            let digits = if negative { &index[1..] } else { index };
            let mut value = 0_i64;
            for &unit in digits
                .iter()
                .take_while(|&&unit| (b'0' as u16..=b'9' as u16).contains(&unit))
            {
                value = value
                    .wrapping_mul(10)
                    .wrapping_add(i64::from(unit - b'0' as u16));
            }
            icon_index = (if negative {
                value.wrapping_neg()
            } else {
                value
            }) as i32;
        }
        path.truncate(comma);
        path.push(0);
    }
    path_unquote_spaces(path);
    path_remove_blanks(path);
    icon_index
}

/// Removes a trailing backslash unless the path is a root, matching
/// `PathRemoveBackslashW`. Returns the offset corresponding to Wine's pointer.
pub fn path_remove_backslash(path: &mut Vec<u16>) -> usize {
    let len = wlen(path);
    let last = len.saturating_sub(1);
    if len > 0 && !path_is_root(path) && path[last] == b'\\' as u16 {
        path[last] = 0;
        path.truncate(last + 1);
    }
    last
}

/// Adds a trailing backslash when missing.
pub fn path_cch_add_backslash(path: &mut Vec<u16>, size: usize) -> i32 {
    path_cch_add_backslash_ex(path, size).0
}

/// Returns the argument offset after the first unquoted space, matching
/// `PathGetArgsW`. An empty string returns offset zero.
pub fn path_get_args(path: &[u16]) -> usize {
    let len = wlen(path);
    let mut seen_quote = false;
    for (index, &unit) in path[..len].iter().enumerate() {
        if unit == b' ' as u16 && !seen_quote {
            return index + 1;
        }
        if unit == b'"' as u16 {
            seen_quote = !seen_quote;
        }
    }
    len
}

/// Tests one UTF-16 code unit against Wine's `PathIsValidCharW` class mask.
pub fn path_is_valid_char(ch: u16, class: u32) -> bool {
    let character_class = if ch > 0x7e {
        PATH_CHAR_CLASS_OTHER_VALID
    } else {
        match ch {
            0x00..=0x1f | 0x2f | 0x3c | 0x3e | 0x7c => 0,
            0x20 => PATH_CHAR_CLASS_SPACE,
            0x22 => PATH_CHAR_CLASS_DOUBLEQUOTE,
            0x2a => PATH_CHAR_CLASS_ASTERISK,
            0x2c => PATH_CHAR_CLASS_COMMA,
            0x2e => PATH_CHAR_CLASS_DOT,
            0x3a => PATH_CHAR_CLASS_COLON,
            0x3b => PATH_CHAR_CLASS_SEMICOLON,
            0x3f => PATH_CHAR_CLASS_LETTER,
            0x5c => PATH_CHAR_CLASS_BACKSLASH,
            0x41..=0x5a | 0x61..=0x7a => PATH_CHAR_CLASS_ANY,
            _ => PATH_CHAR_CLASS_OTHER_VALID,
        }
    };
    class & character_class != 0
}

/// Classifies one UTF-16 code unit using Wine's `PathGetCharTypeW` rules.
pub fn path_get_char_type(ch: u16) -> u32 {
    if ch == 0 || ch < b' ' as u16 || matches!(ch, 0x22 | 0x2f | 0x3c | 0x3e | 0x7c) {
        return 0;
    }
    if matches!(ch, 0x2a | 0x3f) {
        return GCT_WILD;
    }
    if matches!(ch, 0x3a | 0x5c) {
        return GCT_SEPARATOR;
    }

    let mut flags = 0;
    if ch < 126 {
        if (ch & 1 != 0 && ch != b';' as u16)
            || matches!(ch, 0x24 | 0x26 | 0x28 | 0x2e | 0x40 | 0x5e | 0x27 | 0x60)
            || (b'0' as u16..=b'9' as u16).contains(&ch)
            || (b'A' as u16..=b'Z' as u16).contains(&ch)
            || (b'a' as u16..=b'z' as u16).contains(&ch)
        {
            flags |= GCT_SHORTCHAR;
        }
    } else {
        flags |= GCT_SHORTCHAR;
    }
    flags | GCT_LFNCHAR
}

/// Finds the file-name component's UTF-16 offset, matching Wine's
/// `PathFindFileNameW` separator and drive-colon rules.
pub fn path_find_file_name(path: &[u16]) -> usize {
    let len = wlen(path);
    let mut last = 0;
    for index in 0..len {
        let unit = path[index];
        let next = char_at(path, index + 1);
        if (unit == b'\\' as u16 || unit == b'/' as u16 || unit == b':' as u16)
            && next != 0
            && next != b'\\' as u16
            && next != b'/' as u16
        {
            last = index + 1;
        }
    }
    last
}

/// Finds the next path component after the first backslash, matching
/// `PathFindNextComponentW`. Empty strings return `None` like Wine's null result.
pub fn path_find_next_component(path: &[u16]) -> Option<usize> {
    let len = wlen(path);
    if len == 0 {
        return None;
    }
    match path[..len].iter().position(|&unit| unit == b'\\' as u16) {
        Some(index) => Some(index + 1 + usize::from(char_at(path, index + 1) == b'\\' as u16)),
        None => Some(len),
    }
}

/// Replaces a path with its file-name component, matching `PathStripPathW`.
pub fn path_strip_path(path: &mut Vec<u16>) {
    let filename = path_find_file_name(path);
    if filename != 0 {
        let len = wlen(path);
        let mut stripped = path[filename..len].to_vec();
        stripped.push(0);
        *path = stripped;
    }
}

/// Finds the extension's UTF-16 offset using Wine's legacy `PathFindExtensionW`
/// rules. An absent extension points to the terminating NUL.
pub fn path_find_extension(path: &[u16]) -> usize {
    let len = wlen(path);
    let mut last_dot = None;
    for (index, &unit) in path[..len].iter().enumerate() {
        if unit == b'\\' as u16 || unit == b' ' as u16 {
            last_dot = None;
        } else if unit == b'.' as u16 {
            last_dot = Some(index);
        }
    }
    last_dot.unwrap_or(len)
}

/// Removes leading and trailing ASCII spaces in place, matching Wine's
/// `PathRemoveBlanksW` behavior.
pub fn path_remove_blanks(path: &mut Vec<u16>) {
    let len = wlen(path);
    if len == 0 {
        return;
    }
    let first = path[..len]
        .iter()
        .position(|&unit| unit != b' ' as u16)
        .unwrap_or(len);
    let mut end = len;
    while end > first && path[end - 1] == b' ' as u16 {
        end -= 1;
    }
    let mut trimmed = path[first..end].to_vec();
    trimmed.push(0);
    *path = trimmed;
}

/// Removes one matching pair of surrounding quotes, matching Wine's
/// `PathUnquoteSpacesW` behavior.
pub fn path_unquote_spaces(path: &mut Vec<u16>) {
    let len = wlen(path);
    if len == 0 || char_at(path, 0) != b'"' as u16 || char_at(path, len - 1) != b'"' as u16 {
        return;
    }
    if len == 1 {
        path.clear();
        path.push(0);
        return;
    }
    let mut unquoted = path[1..len - 1].to_vec();
    unquoted.push(0);
    *path = unquoted;
}

/// Removes the legacy `PathFindExtensionW` result in place, if present.
pub fn path_remove_extension(path: &mut Vec<u16>) {
    let extension = path_find_extension(path);
    if char_at(path, extension) != 0 {
        path.truncate(extension);
        path.push(0);
    }
}

/// Replaces the legacy extension-search result verbatim when the result fits
/// within MAX_PATH UTF-16 code units including the terminator.
pub fn path_rename_extension(path: &mut Vec<u16>, extension: &[u16]) -> bool {
    let path_len = wlen(path);
    let extension_len = wlen(extension);
    if path_len >= MAX_PATH || extension_len == extension.len() {
        return false;
    }
    let start = path_find_extension(path);
    if start + extension_len >= MAX_PATH {
        return false;
    }
    path.truncate(start);
    path.extend_from_slice(&extension[..extension_len]);
    path.push(0);
    true
}

/// Appends an extension when the legacy extension search finds none and the
/// result fits within MAX_PATH UTF-16 code units including the terminator.
pub fn path_add_extension(path: &mut Vec<u16>, extension: &[u16]) -> bool {
    let path_len = wlen(path);
    let extension_len = wlen(extension);
    if path_len >= MAX_PATH || extension_len == extension.len() {
        return false;
    }
    if path_find_extension(path) < path_len || path_len + extension_len >= MAX_PATH {
        return false;
    }
    path.truncate(path_len);
    path.extend_from_slice(&extension[..extension_len]);
    path.push(0);
    true
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

/// Returns Wine's case-insensitive, separator-aware common-prefix length.
pub fn path_common_prefix(file1: &[u16], file2: &[u16]) -> usize {
    let (mut first, mut second) = (0, 0);
    if path_is_unc(file1) {
        if !path_is_unc(file2) {
            return 0;
        }
        first = 2;
        second = 2;
    } else if path_is_unc(file2) {
        return 0;
    }

    let mut common = 0;
    loop {
        let left = char_at(file1, first);
        let right = char_at(file2, second);
        if (left == 0 || left == b'\\' as u16) && (right == 0 || right == b'\\' as u16) {
            common = first;
        }
        if left == 0 || ascii_lower(left) != ascii_lower(right) {
            break;
        }
        first += 1;
        second += 1;
    }
    if common == 2 {
        common += 1;
    }
    common
}

/// Checks whether `prefix` matches a Wine-recognized prefix of `path`.
pub fn path_is_prefix(prefix: &[u16], path: &[u16]) -> bool {
    path_common_prefix(path, prefix) == wlen(prefix)
}

/// Compares roots using Wine's `PathSkipRootW` and common-prefix semantics.
pub fn path_is_same_root(path1: &[u16], path2: &[u16]) -> bool {
    let Some(root_end) = path_skip_root(path1) else {
        return false;
    };
    path_common_prefix(path1, path2) + 1 >= root_end
}

/// Reports whether a nonempty path has a syntactically valid URL scheme,
/// matching Wine's `PathIsURLW` behavior. Unknown schemes are accepted.
pub fn path_is_url(path: Option<&[u16]>) -> bool {
    let Some(path) = path else {
        return false;
    };
    let len = wlen(path);
    if len == 0 {
        return false;
    }
    let Some(colon) = path[..len].iter().position(|&unit| unit == b':' as u16) else {
        return false;
    };
    if colon <= 1 {
        return false;
    }
    path[..colon].iter().all(|&unit| {
        char::from_u32(u32::from(unit))
            .is_some_and(|ch| ch.is_alphanumeric() || matches!(unit, 43 | 45 | 46))
    })
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

fn copy_legacy_path(path: &[u16]) -> Vec<u16> {
    let len = wlen(path).min(MAX_PATH - 1);
    let mut copy = path[..len].to_vec();
    copy.push(0);
    copy
}

fn join_legacy_path(dir: &[u16], file: &[u16], strip_file_root: bool) -> Option<Vec<u16>> {
    let mut tmp = copy_legacy_path(dir);
    let mut file = file;
    if strip_file_root {
        path_strip_to_root(&mut tmp);
        file = &file[1..];
    }
    /* copy_legacy_path caps the copy at MAX_PATH - 1, so the add cannot fail. */
    path_add_backslash(&mut tmp);
    let file = &file[..wlen(file)];
    if wlen(&tmp) + file.len() >= MAX_PATH {
        return None;
    }
    let len = wlen(&tmp);
    tmp.truncate(len);
    tmp.extend_from_slice(file);
    tmp.push(0);
    Some(tmp)
}

/// Combines directory and file paths following Wine's `PathCombineW` rules.
/// `None` indicates a null source pair or an overflowing result, matching the
/// API's null return.
pub fn path_combine(dir: Option<&[u16]>, file: Option<&[u16]>) -> Option<Vec<u16>> {
    let joined = match (dir, file) {
        (None, None) => return None,
        (Some(dir), None) => copy_legacy_path(dir),
        (Some(dir), Some(file)) if wlen(file) == 0 => copy_legacy_path(dir),
        (Some(dir), Some(file)) if wlen(dir) > 0 && path_is_relative(file) => {
            join_legacy_path(dir, file, false)?
        }
        (Some(dir), Some(file))
            if wlen(dir) > 0 && char_at(file, 0) == b'\\' as u16 && !path_is_unc(file) =>
        {
            join_legacy_path(dir, file, true)?
        }
        (_, Some(file)) => copy_legacy_path(file),
    };
    path_canonicalize(Some(&joined))
}

/// Appends a path in place following Wine's `PathAppendW` rules.
pub fn path_append(path: Option<&mut Vec<u16>>, append: Option<&[u16]>) -> bool {
    let (Some(path), Some(append)) = (path, append) else {
        return false;
    };
    let append_start = if path_is_unc(append) {
        0
    } else {
        append
            .iter()
            .take_while(|&&unit| unit == b'\\' as u16)
            .count()
    };
    match path_combine(Some(path.as_slice()), Some(&append[append_start..])) {
        Some(combined) => {
            *path = combined;
            true
        }
        None => {
            path.clear();
            path.push(0);
            false
        }
    }
}

fn canonicalize_at(buf: &[u16], index: usize) -> u16 {
    if index < buf.len() {
        buf[index]
    } else {
        0
    }
}

/// Canonicalizes a path into a fresh buffer, matching Wine's
/// `PathCanonicalizeW`. `None` represents Wine's failure for a NULL source;
/// an empty source canonicalizes to a single backslash.
pub fn path_canonicalize(path: Option<&[u16]>) -> Option<Vec<u16>> {
    let path = path?;
    let len = wlen(path);
    let mut buf: Vec<u16> = Vec::new();
    let mut pos = 0usize;
    if len == 0 {
        buf.push(b'\\' as u16);
        buf.push(0);
        return Some(buf);
    }
    let mut src = 0usize;
    if path[0] == b'\\' as u16 {
        buf.push(path[0]);
        pos = 1;
        src = 1;
    } else if char_at(path, 1) == b':' as u16 {
        buf.push(path[0]);
        buf.push(path[1]);
        pos = 2;
        src = 2;
        if char_at(path, 2) == b'\\' as u16 {
            buf.push(b'\\' as u16);
            pos = 3;
            src = 3;
        }
    }
    while src < len {
        let unit = path[src];
        if unit == b'.' as u16 {
            let next = char_at(path, src + 1);
            let preceded = src == 0
                || char_at(path, src - 1) == b'\\' as u16
                || char_at(path, src - 1) == b':' as u16;
            if next == b'\\' as u16 && preceded {
                src += 2; /* Skip .\ */
                continue;
            }
            if next == b'.' as u16 && !buf.is_empty() && buf[buf.len() - 1] == b'\\' as u16 {
                /* \.. backs up a directory, over the root if it has no \
                 * following X:. .. is ignored if it would remove a UNC server
                 * name or initial \\ */
                if buf.len() > 1 && (buf[buf.len() - 2] != b'\\' as u16 || buf.len() > 2) {
                    if buf[buf.len() - 2] == b':' as u16
                        && (buf.len() > 3 || buf[buf.len() - 3] == b':' as u16)
                    {
                        pos -= 2;
                        while pos > 0 && canonicalize_at(&buf, pos) != b'\\' as u16 {
                            pos -= 1;
                        }
                        if canonicalize_at(&buf, pos) == b'\\' as u16 {
                            pos += 1;
                        } else {
                            pos = 0;
                        }
                    } else if buf[buf.len() - 2] != b':' as u16 && !path_is_unc_server_share(&buf) {
                        pos -= 2;
                    }
                }
                while pos > 0 && canonicalize_at(&buf, pos) != b'\\' as u16 {
                    pos -= 1;
                }
                buf.truncate(pos);
                if pos == 0 {
                    buf.push(b'\\' as u16);
                    src += 1;
                }
                src += 2; /* Skip .. in src path */
                continue;
            }
            buf.push(unit);
            pos += 1;
            src += 1;
        } else {
            buf.push(unit);
            pos += 1;
            src += 1;
        }
    }
    /* Append \ to naked drive specs */
    if buf.len() == 2 && buf[1] == b':' as u16 {
        buf.push(b'\\' as u16);
    }
    buf.push(0);
    Some(buf)
}

fn ascii_upper(unit: u16) -> u16 {
    if (b'a' as u16..=b'z' as u16).contains(&unit) {
        unit - 32
    } else {
        unit
    }
}

fn wide_eq(text: &[u16], ascii: &str) -> bool {
    text[..wlen(text)].iter().copied().eq(ascii.encode_utf16())
}

/// Matches one mask against a name from the given offsets, matching the
/// recursive `path_match_maskW` helper in Wine's `PathMatchSpecExW`. Case
/// folding is ASCII-only; Wine uses `towupper`, which also folds non-ASCII.
fn path_match_mask(name: &[u16], mut name_at: usize, mask: &[u16], mut mask_at: usize) -> bool {
    while char_at(name, name_at) != 0
        && char_at(mask, mask_at) != 0
        && char_at(mask, mask_at) != b';' as u16
    {
        if char_at(mask, mask_at) == b'*' as u16 {
            loop {
                if path_match_mask(name, name_at, mask, mask_at + 1) {
                    return true;
                }
                let current = char_at(name, name_at);
                name_at += 1;
                if current == 0 {
                    break;
                }
            }
            return false;
        }
        if ascii_upper(char_at(mask, mask_at)) != ascii_upper(char_at(name, name_at))
            && char_at(mask, mask_at) != b'?' as u16
        {
            return false;
        }
        name_at += 1;
        mask_at += 1;
    }
    if char_at(name, name_at) == 0 {
        while char_at(mask, mask_at) == b'*' as u16 {
            mask_at += 1;
        }
        if char_at(mask, mask_at) == 0 || char_at(mask, mask_at) == b';' as u16 {
            return true;
        }
    }
    false
}

/// Tests a path against a `;`-separated mask list, matching Wine's
/// `PathMatchSpecExW`. Flags are ignored, as in Wine. Returns `S_OK` on a match.
pub fn path_match_spec_ex(path: &[u16], mask: &[u16]) -> i32 {
    if wide_eq(mask, "*.*") {
        return S_OK;
    }
    let mut mask_at = 0;
    while char_at(mask, mask_at) != 0 {
        while char_at(mask, mask_at) == b' ' as u16 {
            mask_at += 1;
        }
        if path_match_mask(path, 0, mask, mask_at) {
            return S_OK;
        }
        while char_at(mask, mask_at) != 0 && char_at(mask, mask_at) != b';' as u16 {
            mask_at += 1;
        }
        if char_at(mask, mask_at) == b';' as u16 {
            mask_at += 1;
        }
    }
    S_FALSE
}

/// Builds the path from `from` to `to`, matching Wine's `PathRelativePathToW`.
/// Returns `None` where Wine returns FALSE. Wine does not bound the `..\`
/// accumulation, so neither does this port.
pub fn path_relative_path_to(
    from: Option<&[u16]>,
    from_attributes: u32,
    to: Option<&[u16]>,
    to_attributes: u32,
) -> Option<Vec<u16>> {
    let (from, to) = (from?, to?);
    let mut from_w = copy_legacy_path(from);
    let mut to_w = copy_legacy_path(to);
    if from_attributes & FILE_ATTRIBUTE_DIRECTORY == 0 {
        path_remove_file_spec(&mut from_w);
    }
    if to_attributes & FILE_ATTRIBUTE_DIRECTORY == 0 {
        path_remove_file_spec(&mut to_w);
    }

    let len = path_common_prefix(&from_w, &to_w);
    if len == 0 {
        return None;
    }

    let mut path = Vec::new();
    let mut from_at = len;
    if char_at(&from_w, from_at) == 0 {
        path.push(b'.' as u16);
    }
    if char_at(&from_w, from_at) == b'\\' as u16 {
        from_at += 1;
    }
    while char_at(&from_w, from_at) != 0 {
        from_at += path_find_next_component(&from_w[from_at..]).unwrap_or(0);
        path.extend(
            if char_at(&from_w, from_at) != 0 {
                "..\\"
            } else {
                ".."
            }
            .encode_utf16(),
        );
    }

    let mut tail_at = len;
    if char_at(to, tail_at) != 0 && char_at(to, tail_at - 1) != 0 {
        if char_at(to, tail_at) != b'\\' as u16 {
            tail_at -= 1;
        }
        let tail = &to[tail_at..];
        let tail = &tail[..wlen(tail)];
        if path.len() + tail.len() >= MAX_PATH {
            return None;
        }
        path.extend_from_slice(tail);
    }
    path.push(0);
    Some(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn w(text: &str) -> Vec<u16> {
        text.encode_utf16().chain(std::iter::once(0)).collect()
    }

    #[test]
    fn legacy_skip_root_matches_wine_drive_and_unc_rules() {
        for (path, expected) in [
            ("", None),
            ("relative", None),
            ("C:", None),
            ("C:\\", Some(3)),
            ("C:\\folder", Some(3)),
            ("\\\\server", None),
            ("\\\\server\\share", None),
            ("\\\\server\\share\\file", Some(15)),
            ("\\\\server\\", None),
            ("\\\\server\\share\\", Some(15)),
            ("/server/share/file", None),
            ("\\\\server/share/file", None),
        ] {
            assert_eq!(path_skip_root(&w(path)), expected, "{path:?}");
        }
        assert_eq!(path_skip_root(&[]), None);
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
    fn legacy_rename_extension_matches_wine_rules() {
        for (input, extension, expected, result) in [
            ("file.old", ".new", "file.new", true),
            ("file", ".new", "file.new", true),
            ("dir.with.dot\\file", ".new", "dir.with.dot\\file.new", true),
            ("file.old", "", "file", true),
        ] {
            let mut path = w(input);
            assert_eq!(
                path_rename_extension(&mut path, &w(extension)),
                result,
                "{input}"
            );
            assert_eq!(path, w(expected), "{input}");
        }
        let mut invalid_extension = w("file.old");
        let before = invalid_extension.clone();
        assert!(!path_rename_extension(&mut invalid_extension, &[]));
        assert_eq!(invalid_extension, before);

        let mut too_long = vec![b'a' as u16; MAX_PATH - 2];
        too_long.push(b'.' as u16);
        too_long.push(0);
        let before = too_long.clone();
        assert!(!path_rename_extension(&mut too_long, &w(".x")));
        assert_eq!(too_long, before);
    }

    #[test]
    fn get_args_skips_only_spaces_outside_quotes() {
        for (input, expected) in [
            ("", 0),
            ("program.exe arg", 12),
            ("\"program name.exe\" arg", 19),
            ("program\targ", 11),
            ("program.exe", 11),
            ("\"unterminated name", 18),
            ("program  arg", 8),
        ] {
            assert_eq!(path_get_args(&w(input)), expected, "{input:?}");
        }
        assert_eq!(path_get_args(&[]), 0);
    }

    #[test]
    fn is_valid_char_matches_wine_class_masks() {
        for (character, class, expected) in [
            (b'A' as u16, PATH_CHAR_CLASS_LETTER, true),
            (b'a' as u16, PATH_CHAR_CLASS_ANY, true),
            (b'?' as u16, PATH_CHAR_CLASS_LETTER, true),
            (b'?' as u16, PATH_CHAR_CLASS_OTHER_VALID, false),
            (b'*' as u16, PATH_CHAR_CLASS_ASTERISK, true),
            (b'.' as u16, PATH_CHAR_CLASS_DOT, true),
            (b'\\' as u16, PATH_CHAR_CLASS_BACKSLASH, true),
            (b':' as u16, PATH_CHAR_CLASS_COLON, true),
            (b';' as u16, PATH_CHAR_CLASS_SEMICOLON, true),
            (b',' as u16, PATH_CHAR_CLASS_COMMA, true),
            (b' ' as u16, PATH_CHAR_CLASS_SPACE, true),
            (b'"' as u16, PATH_CHAR_CLASS_DOUBLEQUOTE, true),
            (b'/' as u16, PATH_CHAR_CLASS_ANY, false),
            (0x7f, PATH_CHAR_CLASS_ANY, true),
            (0xffff, PATH_CHAR_CLASS_OTHER_VALID, true),
            (b'A' as u16, PATH_CHAR_CLASS_DOT, true),
            (b'!' as u16, PATH_CHAR_CLASS_OTHER_VALID, true),
        ] {
            assert_eq!(
                path_is_valid_char(character, class),
                expected,
                "{character:#x} / {class:#x}"
            );
        }
    }

    #[test]
    fn get_char_type_matches_wine_classifications() {
        for (character, expected) in [
            (0, 0),
            (b'\n' as u16, 0),
            (b'<' as u16, 0),
            (b'/' as u16, 0),
            (b'*' as u16, GCT_WILD),
            (b'?' as u16, GCT_WILD),
            (b'\\' as u16, GCT_SEPARATOR),
            (b':' as u16, GCT_SEPARATOR),
            (b'A' as u16, GCT_SHORTCHAR | GCT_LFNCHAR),
            (b'1' as u16, GCT_SHORTCHAR | GCT_LFNCHAR),
            (b'!' as u16, GCT_SHORTCHAR | GCT_LFNCHAR),
            (b' ' as u16, GCT_LFNCHAR),
            (b';' as u16, GCT_LFNCHAR),
            (126, GCT_SHORTCHAR | GCT_LFNCHAR),
            (0x0100, GCT_SHORTCHAR | GCT_LFNCHAR),
        ] {
            assert_eq!(path_get_char_type(character), expected, "{character:#x}");
        }
    }

    #[test]
    fn find_next_component_matches_wine_backslash_rules() {
        for (input, expected) in [
            ("", None),
            ("plain", Some(5)),
            ("\\component", Some(1)),
            ("first\\second", Some(6)),
            ("first\\\\second", Some(7)),
            ("first/second", Some(12)),
            ("first\\", Some(6)),
            ("first\\\\", Some(7)),
        ] {
            assert_eq!(path_find_next_component(&w(input)), expected, "{input:?}");
        }
        assert_eq!(path_find_next_component(&[]), None);
    }

    #[test]
    fn strip_path_matches_wine_filename_offset() {
        for (input, expected) in [
            ("", ""),
            ("filename.txt", "filename.txt"),
            ("C:\\folder\\filename.txt", "filename.txt"),
            ("C:/folder\\filename.txt", "filename.txt"),
            ("\\\\server\\share", "share"),
            ("\\\\server\\\\share", "share"),
        ] {
            let mut path = w(input);
            path_strip_path(&mut path);
            assert_eq!(path, w(expected), "{input:?}");
        }
        let mut unterminated = "C:\\folder\\file".encode_utf16().collect::<Vec<_>>();
        path_strip_path(&mut unterminated);
        assert_eq!(unterminated, w("file"));
    }

    #[test]
    fn is_lfn_file_spec_matches_wine_dos_name_limits() {
        for (path, expected) in [
            ("", false),
            ("12345678.123", false),
            ("no_extension", true),
            ("123456789", true),
            ("name.extx", true),
            ("name.a.b", true),
            ("name with space", true),
            ("a.", false),
        ] {
            assert_eq!(path_is_lfn_file_spec(Some(&w(path))), expected, "{path:?}");
        }
        assert!(!path_is_lfn_file_spec(None));
        assert!(!path_is_lfn_file_spec(Some(&[])));
    }

    #[test]
    fn parse_icon_location_matches_wine_mutation_and_index_rules() {
        for (input, expected_path, expected_index) in [
            ("path", "path", 0),
            (" path ", "path", 0),
            ("\" path with spaces \",7", "path with spaces", 7),
            ("file.exe,-12suffix", "file.exe", -12),
            ("file.exe,12suffix", "file.exe", 12),
            ("file.exe,+12", "file.exe", 0),
            ("file.exe, 12", "file.exe", 0),
            ("file.exe,-", "file.exe", 0),
            ("file.exe,not-a-number", "file.exe", 0),
            ("file.exe,3,4", "file.exe", 3),
        ] {
            let mut path = w(input);
            assert_eq!(
                path_parse_icon_location(&mut path),
                expected_index,
                "{input:?}"
            );
            assert_eq!(path, w(expected_path), "{input:?}");
        }
        let mut unterminated = " path ,5".encode_utf16().collect::<Vec<_>>();
        assert_eq!(path_parse_icon_location(&mut unterminated), 5);
        assert_eq!(unterminated, w("path"));
    }

    #[test]
    fn quote_spaces_matches_wine_bounds_and_mutation_rules() {
        for (input, expected) in [
            ("", ""),
            ("plain", "plain"),
            ("has space", "\"has space\""),
            ("  leading and trailing  ", "\"  leading and trailing  \""),
        ] {
            let mut path = w(input);
            path_quote_spaces(&mut path);
            assert_eq!(path, w(expected), "{input:?}");
        }
        let mut too_long = vec![b'a' as u16; MAX_PATH - 3];
        too_long[1] = b' ' as u16;
        too_long.push(0);
        let original = too_long.clone();
        path_quote_spaces(&mut too_long);
        assert_eq!(too_long, original);
        let mut unterminated = "has space".encode_utf16().collect::<Vec<_>>();
        path_quote_spaces(&mut unterminated);
        assert_eq!(unterminated, w("\"has space\""));
    }

    #[test]
    fn legacy_remove_backslash_matches_wine_root_and_pointer_rules() {
        for (input, expected, offset) in [
            ("", "", 0),
            ("folder", "folder", 5),
            ("folder\\", "folder", 6),
            ("C:\\", "C:\\", 2),
            ("C:\\folder\\", "C:\\folder", 9),
            ("\\\\server\\share", "\\\\server\\share", 13),
            ("\\\\server\\share\\", "\\\\server\\share", 14),
            ("/folder/", "/folder/", 7),
        ] {
            let mut path = w(input);
            assert_eq!(path_remove_backslash(&mut path), offset, "{input:?}");
            assert_eq!(path, w(expected), "{input:?}");
        }
        let mut unterminated = "folder\\".encode_utf16().collect::<Vec<_>>();
        assert_eq!(path_remove_backslash(&mut unterminated), 6);
        assert_eq!(unterminated, w("folder"));
    }

    #[test]
    fn remove_blanks_matches_wine_vectors() {
        for (input, expected) in [
            ("", ""),
            (" ", ""),
            ("test", "test"),
            (" test", "test"),
            ("  test", "test"),
            ("test ", "test"),
            ("test  ", "test"),
            (" test  ", "test"),
            ("  test ", "test"),
            ("\t test \t", "\t test \t"),
        ] {
            let mut path = w(input);
            path_remove_blanks(&mut path);
            assert_eq!(path, w(expected), "{input:?}");
        }
        let mut unterminated = " test ".encode_utf16().collect::<Vec<_>>();
        path_remove_blanks(&mut unterminated);
        assert_eq!(unterminated, w("test"));
    }

    #[test]
    fn unquote_spaces_matches_wine_vectors() {
        for (input, expected) in [
            ("abcdef", "abcdef"),
            ("\"abcdef\"", "abcdef"),
            ("\"abcdef", "\"abcdef"),
            ("abcdef\"", "abcdef\""),
            ("\"\"abcdef\"\"", "\"abcdef\""),
            ("abc\"def", "abc\"def"),
            ("\"abc\"def", "\"abc\"def"),
            ("\"abc\"def\"", "abc\"def"),
            ("'abcdef'", "'abcdef'"),
            ("\"\"", ""),
            ("\"", ""),
            ("", ""),
        ] {
            let mut path = w(input);
            path_unquote_spaces(&mut path);
            assert_eq!(path, w(expected), "{input}");
        }
        let mut unterminated = "\"quoted\"".encode_utf16().collect::<Vec<_>>();
        path_unquote_spaces(&mut unterminated);
        assert_eq!(unterminated, w("quoted"));
    }

    #[test]
    fn legacy_remove_extension_matches_wine_rules() {
        for (input, expected) in [
            ("file.txt", "file"),
            ("file.tar.gz", "file.tar"),
            ("dir.with.dot\\file.txt", "dir.with.dot\\file"),
            ("file.txt more", "file.txt more"),
            ("file", "file"),
            ("", ""),
        ] {
            let mut path = w(input);
            path_remove_extension(&mut path);
            assert_eq!(path, w(expected), "{input}");
        }
        let mut unterminated = "file.txt".encode_utf16().collect::<Vec<_>>();
        path_remove_extension(&mut unterminated);
        assert_eq!(unterminated, w("file"));
    }

    #[test]
    fn legacy_add_extension_matches_wine_rules() {
        for (input, extension, expected, result) in [
            ("file", ".txt", "file.txt", true),
            ("file", "txt", "filetxt", true),
            ("file.txt", ".new", "file.txt", false),
            (".hidden", ".txt", ".hidden", false),
            ("file.", ".txt", "file.", false),
            ("file name", ".txt", "file name.txt", true),
            ("", ".txt", ".txt", true),
        ] {
            let mut path = w(input);
            assert_eq!(
                path_add_extension(&mut path, &w(extension)),
                result,
                "{input}"
            );
            assert_eq!(path, w(expected), "{input}");
        }
        let mut empty_extension = w("file");
        assert!(path_add_extension(&mut empty_extension, &w("")));
        assert_eq!(empty_extension, w("file"));
        assert!(!path_add_extension(&mut empty_extension, &[]));

        let mut at_limit = vec![b'a' as u16; MAX_PATH - 1];
        at_limit.push(0);
        let before = at_limit.clone();
        assert!(!path_add_extension(&mut at_limit, &w("x")));
        assert_eq!(at_limit, before);
    }

    #[test]
    fn find_file_name_matches_wine_rules() {
        for (path, expected) in [
            ("", 0),
            ("file.txt", 0),
            ("dir\\file.txt", 4),
            ("dir/file.txt", 4),
            ("C:file.txt", 2),
            ("C:\\file.txt", 3),
            ("\\\\server\\share", 9),
            ("dir\\\\file", 5),
            ("dir\\", 0),
            ("dir//file", 5),
            ("dir\\/file", 5),
        ] {
            assert_eq!(path_find_file_name(&w(path)), expected, "{path}");
        }
        assert_eq!(path_find_file_name(&[]), 0);
    }

    #[test]
    fn find_extension_matches_wine_legacy_rules() {
        for (path, expected) in [
            ("", 0),
            ("file.txt", 4),
            ("file", 4),
            ("file.", 4),
            ("dir\\file.txt", 8),
            ("dir with space\\file.txt", 19),
            ("dir.with.dot\\file", 17),
            ("file.tar.gz", 8),
            ("file.txt more", 13),
        ] {
            assert_eq!(path_find_extension(&w(path)), expected, "{path}");
        }
        assert_eq!(path_find_extension(&[]), 0);
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
    fn common_prefix_matches_wine_case_separator_and_unc_rules() {
        for (first, second, expected) in [
            ("C:\\one\\a", "c:\\one\\b", 6),
            ("one\\.two", "one\\.three", 3),
            ("same", "same", 4),
            ("same", "same\\child", 4),
            ("same\\child", "same", 4),
            ("\\\\server", "C:\\server", 0),
            ("C:\\server", "\\\\server", 0),
            ("\\\\one", "\\\\two", 0),
            ("\\\\server\\share", "\\\\SERVER\\other", 8),
            ("", "anything", 0),
        ] {
            assert_eq!(
                path_common_prefix(&w(first), &w(second)),
                expected,
                "{first} / {second}"
            );
        }
        assert_eq!(path_common_prefix(&[], &[]), 0);
    }

    #[test]
    fn prefix_matches_wine_wrapper_including_empty_and_unc_cases() {
        assert!(path_is_prefix(&w("C:\\"), &w("C:\\folder")));
        assert!(!path_is_prefix(&w("C:\\folder"), &w("C:\\foo")));
        assert!(path_is_prefix(&w(""), &w("anything")));
        assert!(path_is_prefix(&w("\\\\server"), &w("\\\\SERVER\\share")));
        assert!(!path_is_prefix(&w("\\\\server"), &w("C:\\server")));
        assert!(path_is_prefix(&[], &[]));
    }

    #[test]
    fn same_root_matches_wine_skip_root_and_prefix_rules() {
        for (first, second, expected) in [
            ("C:\\one", "C:\\two", true),
            ("C:\\one", "c:\\two", true),
            ("C:\\one", "D:\\two", false),
            ("\\\\server\\share\\one", "\\\\server\\share\\two", true),
            ("\\\\server\\share\\one", "\\\\server\\other\\two", false),
            ("relative\\one", "relative\\two", false),
            ("", "C:\\two", false),
            ("C:\\one", "", false),
        ] {
            assert_eq!(
                path_is_same_root(&w(first), &w(second)),
                expected,
                "{first} / {second}"
            );
        }
        assert!(!path_is_same_root(&[], &w("C:\\two")));
    }

    #[test]
    fn is_url_matches_wine_scheme_syntax() {
        for (path, expected) in [
            (Some("http://www.winehq.org/"), true),
            (Some("https://www.winehq.org/"), true),
            (Some("ftp://www.winehq.org/"), true),
            (Some("xxx://www.winehq.org/"), true),
            (Some("1xx://www.winehq.org/"), true),
            (Some("-xx://www.winehq.org/"), true),
            (Some("xx0://www.winehq.org/"), true),
            (Some("ab-://www.winehq.org/"), true),
            (Some("HTTP://www.winehq.org/"), true),
            (Some("a+-.://www.winehq.org/"), true),
            (Some("test.txt?test=c:/dir"), false),
            (Some("test.txt"), false),
            (Some("x://www.winehq.org/"), false),
            (Some("xx$://www.winehq.org/"), false),
            (Some("htt?p://www.winehq.org/"), false),
            (Some(" http://www.winehq.org/"), false),
            (Some(""), false),
            (None, false),
        ] {
            let path = path.map(w);
            assert_eq!(path_is_url(path.as_deref()), expected);
        }
        assert!(!path_is_url(Some(&[])));
    }

    #[test]
    fn canonicalize_matches_wine_vectors_and_quirks() {
        assert_eq!(path_canonicalize(None), None);
        for (input, expected) in [
            ("", "\\"),
            ("C:\\", "C:\\"),
            (" ", " "),
            ("one\\two", "one\\two"),
            ("C:\\one\\.\\..\\two\\three\\..", "C:\\two"),
            (
                "C:\\one/two/three\\four/five\\six",
                "C:\\one/two/three\\four/five\\six",
            ),
            ("C:\\one/.\\two", "C:\\one/.\\two"),
            ("C:\\one/.\\two\\..", "C:\\one/."),
            ("../../one/two/", "../../one/two/"),
            ("../../one/two/\\*", "../../one/two/\\*"),
            ("C:\\one/.\\..\\two/three\\..\\four/.five", "C:\\four/.five"),
            ("a\\..\\b", "\\b"),
            ("C:\\..\\b", "C:\\b"),
            (".\\a", "a"),
            ("C:.\\", "C:\\"),
            ("\\\\server\\..\\share", "\\\\server\\share"),
            ("\\\\server\\share\\..", "\\\\server"),
            ("\\\\..\\a", "\\\\a"),
            ("\\\\\\..\\a", "\\\\\\a"),
            ("::\\..", "\\"),
            ("C:\\a\\..b", "C:b"),
            ("C:\\a\\b\\..\\c", "C:\\a\\c"),
            ("C:\\foo:\\..", "C:\\"),
            ("C:", "C:\\"),
            ("C:a", "C:a"),
        ] {
            assert_eq!(
                path_canonicalize(Some(&w(input))),
                Some(w(expected)),
                "{input}"
            );
        }
    }

    #[test]
    fn combine_matches_wine_vectors_and_failure_rules() {
        for (dir, file, expected) in [
            (Some("C:\\"), Some("a"), Some("C:\\a")),
            (Some("C:\\b"), Some("..\\a"), Some("C:\\a")),
            (Some("C:"), Some("a"), Some("C:\\a")),
            (Some("C:\\"), Some("."), Some("C:\\.")),
            (Some("C:\\"), Some(".."), Some("C:\\")),
            (Some("C:\\a"), Some(""), Some("C:\\a")),
            (Some("\\"), Some("a"), Some("\\a")),
            (Some("\\a"), Some("b"), Some("\\a\\b")),
            (
                Some("\\\\192.168.1.1\\test"),
                Some("a"),
                Some("\\\\192.168.1.1\\test\\a"),
            ),
            (
                Some("\\\\192.168.1.1\\test"),
                Some(".."),
                Some("\\\\192.168.1.1"),
            ),
            (Some("\\\\"), Some("a"), Some("\\\\a")),
            (Some("C:a\\b"), Some("\\1"), Some("C:\\1")),
            (Some("C:\\a\\b"), Some("\\1"), Some("C:\\1")),
            (Some("\\a\\b"), Some("\\1"), Some("\\1")),
            (Some("\\\\a\\b"), Some("\\1"), Some("\\\\a\\b\\1")),
            (Some("C:\\a\\b"), Some("\\"), Some("C:\\")),
            (Some("X:\\"), Some("C:"), Some("C:\\")),
            (Some("X:\\"), Some("C:\\a"), Some("C:\\a")),
            (Some("X:\\"), Some("\\\\"), Some("\\\\")),
            (Some("X:\\"), Some("\\\\a"), Some("\\\\a")),
            (Some("C:\\a"), Some(".\\b"), Some("C:\\a\\b")),
            (Some("C:\\a"), Some("..\\b"), Some("C:\\b")),
            (Some(""), Some(""), Some("\\")),
            (Some("a"), Some("b"), Some("a\\b")),
            (None, Some("\\b"), Some("\\b")),
            (Some("C:\\a"), None, Some("C:\\a")),
            (Some(""), Some("C:\\a"), Some("C:\\a")),
            (Some("C:\\a\\b"), Some("D:\\root"), Some("D:\\root")),
            // Legacy PathCombineW strips the dir to its root and concatenates
            // the file after skipping one separator, so the single backslash of
            // the file merges with the root backslash. The Wine combine_tests
            // table used by PathCchCombine and PathAllocCombine records
            // C:\nonstd for this pair instead.
            (Some("C:\\a\\b"), Some("\\nonstd"), Some("C:\\nonstd")),
        ] {
            let dir = dir.map(w);
            let file = file.map(w);
            assert_eq!(
                path_combine(dir.as_deref(), file.as_deref()),
                expected.map(w),
                "dir={dir:?}, file={file:?}"
            );
        }
        assert_eq!(path_combine(None, None), None);

        assert_eq!(
            path_combine(Some(&w(&"a".repeat(MAX_PATH - 1))), Some(&w("b"))),
            None
        );

        // The joined length check triggers after a successful backslash add.
        assert_eq!(
            path_combine(Some(&w(&"a".repeat(MAX_PATH - 6))), Some(&w("bbbbb"))),
            None
        );
        // Same overflow through the rooted-file branch, which strips the file's leading backslash.
        let rooted_overflow = format!("\\{}", "b".repeat(MAX_PATH - 3));
        assert_eq!(
            path_combine(Some(&w("C:\\")), Some(&w(&rooted_overflow))),
            None
        );
    }

    #[test]
    fn append_matches_wine_vectors_in_place_and_failure_rules() {
        for (input, append, expected) in [
            ("C:\\", "a", "C:\\a"),
            ("C:\\b", "..\\a", "C:\\a"),
            ("C:", "a", "C:\\a"),
            ("C:\\", ".", "C:\\."),
            ("C:\\", "..", "C:\\"),
            ("C:\\a", "", "C:\\a"),
            ("\\\\192.168.1.1\\test", "a", "\\\\192.168.1.1\\test\\a"),
            ("\\a", "b", "\\a\\b"),
            ("\\", "a", "\\a"),
            ("\\\\", "a", "\\\\a"),
            ("C:a\\b", "\\1", "C:a\\b\\1"),
            ("C:\\a\\b", "\\1", "C:\\a\\b\\1"),
            ("\\\\a\\b", "\\1", "\\\\a\\b\\1"),
            ("C:\\a\\b", "\\", "C:\\a\\b"),
            ("X:\\", "C:", "C:\\"),
            ("X:\\", "C:\\", "C:\\"),
            ("X:\\", "\\\\", "\\\\"),
            ("C:\\a", ".\\b", "C:\\a\\b"),
            ("C:\\a", "..\\b", "C:\\b"),
            ("", "", "\\"),
            ("a", "b", "a\\b"),
        ] {
            let mut path = w(input);
            assert!(
                path_append(Some(&mut path), Some(&w(append))),
                "{input:?} + {append:?}"
            );
            assert_eq!(path, w(expected), "{input:?} + {append:?}");
        }
        assert!(!path_append(None, Some(&w("a"))));
        assert!(!path_append(Some(&mut w("a")), None));
        let mut path = w(&"a".repeat(MAX_PATH - 1));
        assert!(!path_append(Some(&mut path), Some(&w("b"))));
        assert_eq!(path, w(""));
        let mut path = w(&"a".repeat(MAX_PATH - 1));

        // A UNC append takes the file-only branch, so the call succeeds even
        // though the original path exceeded MAX_PATH.
        assert!(path_append(Some(&mut path), Some(&w("\\\\server"))));

        assert_eq!(path, w("\\\\server"));
    }

    #[test]
    fn match_spec_matches_wine_mask_vectors() {
        let file = "c:\\foo\\bar\\filename.ext";
        for (mask, expected) in [
            (".ext", S_FALSE),
            ("*.ext", S_OK),
            ("*.ext ", S_FALSE),
            ("  *.ext", S_OK),
            // Wine's todo_wine vectors: Windows expects S_OK, Wine's C returns S_FALSE.
            ("* .ext", S_FALSE),
            ("*. ext", S_FALSE),
            ("* . ext", S_FALSE),
            ("*.e?t", S_OK),
            ("filename.ext", S_FALSE),
            ("*bar\\filename.ext", S_OK),
            (" foo; *.ext", S_OK),
            ("*.ext;*.bar", S_OK),
            ("*bar*", S_OK),
        ] {
            assert_eq!(path_match_spec_ex(&w(file), &w(mask)), expected, "{mask:?}");
        }
    }

    #[test]
    fn match_spec_covers_wildcards_case_and_separators() {
        for (path, mask, expected) in [
            ("c:\\anything", "*.*", S_OK),
            ("a", "*.*", S_OK),
            ("", "*", S_OK),
            ("", "", S_FALSE),
            ("abc", "", S_FALSE),
            ("", ";", S_OK),
            ("", "x", S_FALSE),
            ("a", "x", S_FALSE),
            ("abc", "a?c", S_OK),
            ("ABC", "abc", S_OK),
            ("a.b", "a-b", S_FALSE),
            ("abc", "abd", S_FALSE),
            ("ab", "abc", S_FALSE),
            ("abc", "ab", S_FALSE),
            ("abc", "abc*", S_OK),
            ("abc", "*c", S_OK),
            ("abc", "*d", S_FALSE),
            ("abc", "a*c*", S_OK),
            ("abc", "a*x", S_FALSE),
            ("ab", "*b*", S_OK),
            ("a", "x;a", S_OK),
            ("a", "x;", S_FALSE),
            ("ab", "a;x", S_FALSE),
            ("ab", "ab;x", S_OK),
            ("a", "  a", S_OK),
            ("a", "a ;  ", S_FALSE),
        ] {
            assert_eq!(
                path_match_spec_ex(&w(path), &w(mask)),
                expected,
                "{path:?} vs {mask:?}"
            );
        }
    }

    // Wine has no upstream PathRelativePathTo tests. These vectors are traced
    // by hand from the C in dlls/kernelbase/path.c, not copied from upstream.
    #[test]
    fn relative_path_to_traced_from_wine_c() {
        const DIR: u32 = FILE_ATTRIBUTE_DIRECTORY;
        for (from, from_attr, to, to_attr, expected) in [
            ("C:\\a\\b", 0, "C:\\a\\c", 0, Some(".\\c")),
            ("C:\\a\\b", DIR, "C:\\a\\c", DIR, Some("..\\c")),
            ("C:\\a\\b\\c", DIR, "C:\\a", DIR, Some("..\\..")),
            ("C:\\a\\b", 0, "C:\\a", DIR, Some(".")),
            ("C:\\a", DIR, "C:\\ab", DIR, Some("..\\ab")),
            ("C:\\A\\b", 0, "c:\\a\\x", 0, Some(".\\x")),
            ("C:\\a\\b", DIR, "C:\\a\\bc", DIR, Some("..\\bc")),
            (
                "\\\\server\\share\\a",
                DIR,
                "\\\\server\\share\\b",
                DIR,
                Some("..\\b"),
            ),
            ("C:\\a", DIR, "D:\\a", DIR, None),
            ("\\\\server\\a", DIR, "C:\\a", DIR, None),
            ("C:\\a", DIR, "\\\\server\\a", DIR, None),
            ("C:\\a", DIR, "", DIR, None),
            ("", DIR, "C:\\", DIR, None),
        ] {
            assert_eq!(
                path_relative_path_to(Some(&w(from)), from_attr, Some(&w(to)), to_attr),
                expected.map(w),
                "{from:?} -> {to:?}"
            );
        }
    }

    #[test]
    fn relative_path_to_enforces_max_path_and_missing_arguments() {
        const DIR: u32 = FILE_ATTRIBUTE_DIRECTORY;
        let from = w("C:\\a");
        let fits = format!("C:\\a\\{}", "x".repeat(257));
        let overflows = format!("C:\\a\\{}", "x".repeat(258));
        assert_eq!(
            path_relative_path_to(Some(&from), DIR, Some(&w(&fits)), DIR),
            Some(w(&format!(".\\{}", "x".repeat(257))))
        );
        assert_eq!(
            path_relative_path_to(Some(&from), DIR, Some(&w(&overflows)), DIR),
            None
        );
        assert_eq!(path_relative_path_to(None, DIR, Some(&from), DIR), None);
        assert_eq!(path_relative_path_to(Some(&from), DIR, None, DIR), None);
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
