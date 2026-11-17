//! shlwapi path helpers: the string-only subset of Wine's path routines.
//!
//! The shlwapi entry points forward to `dlls/kernelbase/path.c` in reference
//! Wine, and the logic here mirrors those bodies. Strings are NUL-terminated
//! UTF-16 slices, as the Win32 API uses. Functions take `&[u16]` and return
//! indices into that slice instead of raw pointers, so callers never handle
//! pointer arithmetic. Only the subset that needs no filesystem is ported.

use mpf_rpcrt4::Uuid;

/// `MAX_PATH`, the limit `PathAddBackslash` enforces.
pub const MAX_PATH: usize = 260;

const BACKSLASH: u16 = b'\\' as u16;
const SLASH: u16 = b'/' as u16;
const COLON: u16 = b':' as u16;
const DOT: u16 = b'.' as u16;
const SPACE: u16 = b' ' as u16;
const QUOTE: u16 = b'"' as u16;
const QUESTION: u16 = b'?' as u16;
const AMPERSAND: u16 = b'&' as u16;
const PATHCCH_MAX_CCH: usize = 0x8000;

/// Length of the UTF-16 string before its first NUL, or the slice length.
fn wlen(s: &[u16]) -> usize {
    s.iter().position(|&c| c == 0).unwrap_or(s.len())
}

fn char_at(path: &[u16], index: usize) -> u16 {
    path.get(index).copied().unwrap_or(0)
}

fn ascii_lower(c: u16) -> u16 {
    if (b'A' as u16..=b'Z' as u16).contains(&c) {
        c + 32
    } else {
        c
    }
}

fn ascii_upper(c: u16) -> u16 {
    if (b'a' as u16..=b'z' as u16).contains(&c) {
        c - 32
    } else {
        c
    }
}

fn ascii_alpha(c: u16) -> bool {
    (b'A' as u16..=b'Z' as u16).contains(&c) || (b'a' as u16..=b'z' as u16).contains(&c)
}

fn starts_with(path: &[u16], offset: usize, prefix: &[u8], ignore_ascii_case: bool) -> bool {
    prefix.iter().enumerate().all(|(index, &byte)| {
        let actual = char_at(path, offset + index);
        let expected = u16::from(byte);
        if ignore_ascii_case {
            ascii_lower(actual) == ascii_lower(expected)
        } else {
            actual == expected
        }
    })
}

fn is_drive_spec_at(path: &[u16], offset: usize) -> bool {
    ascii_alpha(char_at(path, offset)) && char_at(path, offset + 1) == COLON
}

fn is_prefixed_unc(path: &[u16]) -> bool {
    starts_with(path, 0, b"\\\\?\\UNC\\", true)
}

fn is_prefixed_disk(path: &[u16]) -> bool {
    starts_with(path, 0, b"\\\\?\\", false) && is_drive_spec_at(path, 4)
}

fn is_prefixed_volume(path: &[u16]) -> bool {
    if !starts_with(path, 0, b"\\\\?\\Volume", true) {
        return false;
    }
    for index in 0..=37 {
        let unit = char_at(path, 10 + index);
        let valid = match index {
            0 => unit == b'{' as u16,
            9 | 14 | 19 | 24 => unit == b'-' as u16,
            37 => unit == b'}' as u16,
            _ => matches!(unit, 48..=57 | 65..=70 | 97..=102),
        };
        if !valid {
            return false;
        }
    }
    true
}

fn get_root_end(path: &[u16]) -> Option<usize> {
    if wlen(path) == 0 {
        return None;
    }
    if is_prefixed_volume(path) {
        return Some(if char_at(path, 48) == BACKSLASH {
            48
        } else {
            47
        });
    }
    if is_prefixed_unc(path) {
        return Some(7);
    }
    if is_prefixed_disk(path) {
        return Some(if char_at(path, 6) == BACKSLASH { 6 } else { 5 });
    }
    if path[0] == BACKSLASH && char_at(path, 1) == BACKSLASH {
        return Some(1);
    }
    if path[0] == BACKSLASH {
        return Some(0);
    }
    if is_drive_spec_at(path, 0) {
        return Some(if char_at(path, 2) == BACKSLASH { 2 } else { 1 });
    }
    None
}

fn get_next_segment(path: &[u16], mut next: usize) -> (usize, bool) {
    let len = wlen(path);
    while next < len && path[next] != BACKSLASH {
        next += 1;
    }
    if next < len {
        (next + 1, true)
    } else {
        (next, false)
    }
}

fn cch_skip_root(path: &[u16]) -> Option<usize> {
    let len = wlen(path);
    if len == 0
        || (starts_with(path, 0, b"\\\\?", true)
            && !is_prefixed_volume(path)
            && !is_prefixed_unc(path)
            && !is_prefixed_disk(path))
    {
        return None;
    }
    let mut root_end = get_root_end(path)? + 1;
    if is_prefixed_unc(path) {
        root_end = get_next_segment(path, root_end).0;
        root_end = get_next_segment(path, root_end).0;
    } else if path[0] == BACKSLASH && char_at(path, 1) == BACKSLASH && char_at(path, 2) != QUESTION
    {
        root_end = get_next_segment(path, root_end).0;
        if char_at(path, root_end) != BACKSLASH {
            root_end = get_next_segment(path, root_end).0;
        }
    }
    Some(root_end)
}

/// `PathFindFileNameW`: index of the file name portion of `path`.
///
/// A separator (`\`, `/` or `:`) that is followed by something other than
/// another separator starts the file name. With no separator the index is 0.
pub fn path_find_file_name(path: &[u16]) -> usize {
    let len = wlen(path);
    let mut last = 0;
    for i in 0..len {
        let c = path[i];
        let next = path.get(i + 1).copied().unwrap_or(0);
        if (c == BACKSLASH || c == SLASH || c == COLON)
            && next != 0
            && next != BACKSLASH
            && next != SLASH
        {
            last = i + 1;
        }
    }
    last
}

/// `PathFindExtensionW`: index of the extension, the last `.` after the last
/// separator. A space or backslash resets the search. Without an extension
/// the index is the string length.
pub fn path_find_extension(path: &[u16]) -> usize {
    let len = wlen(path);
    let mut last_point = None;
    for (i, &c) in path[..len].iter().enumerate() {
        if c == BACKSLASH || c == SPACE {
            last_point = None;
        } else if c == DOT {
            last_point = Some(i);
        }
    }
    last_point.unwrap_or(len)
}

/// `PathIsRelativeW`: true for empty or NULL-like paths, and for paths that
/// are neither UNC (`\\`) nor rooted (`X:`).
pub fn path_is_relative(path: &[u16]) -> bool {
    let len = wlen(path);
    if len == 0 {
        return true;
    }
    let first = path[0];
    let second_is_colon = path.get(1).copied() == Some(COLON);
    !(first == BACKSLASH || second_is_colon)
}

/// `PathAddBackslashW`: appends `\` unless the path already ends with one.
/// An empty path stays empty. Returns the new length, or `None` when the
/// path is already at or beyond `MAX_PATH`, so that `path` is unchanged.
/// `path` must have room for the terminating NUL.
pub fn path_add_backslash(path: &mut Vec<u16>) -> Option<usize> {
    let len = wlen(path);
    if len >= MAX_PATH {
        return None;
    }
    if len == 0 {
        return Some(0);
    }
    if path[len - 1] != BACKSLASH {
        path.truncate(len);
        path.push(BACKSLASH);
        path.push(0);
    }
    Some(wlen(path))
}

/// `PathStripPathW`: removes every component before the file name, moving
/// the file name to the front of the string.
pub fn path_strip_path(path: &mut Vec<u16>) {
    let start = path_find_file_name(path);
    if start != 0 {
        path.drain(..start);
    }
}

/// `PathIsUNCW`: checks whether a path starts with two backslashes.
pub fn path_is_unc(path: &[u16]) -> bool {
    wlen(path) >= 2 && path[0] == BACKSLASH && path[1] == BACKSLASH
}

/// `PathIsUNCServerW`: a UNC prefix with no later backslash.
pub fn path_is_unc_server(path: &[u16]) -> bool {
    let len = wlen(path);
    len >= 2 && path[0] == BACKSLASH && path[1] == BACKSLASH && !path[2..len].contains(&BACKSLASH)
}

/// `PathIsUNCServerShareW`: a UNC path with exactly one separator after its prefix.
pub fn path_is_unc_server_share(path: &[u16]) -> bool {
    let len = wlen(path);
    if len < 2 || path[0] != BACKSLASH || path[1] != BACKSLASH {
        return false;
    }
    let mut seen_separator = false;
    for &unit in &path[2..len] {
        if unit == BACKSLASH {
            if seen_separator {
                return false;
            }
            seen_separator = true;
        }
    }
    seen_separator
}

/// `PathIsRootW`, following Wine's `PathCchIsRoot` root classification.
pub fn path_is_root(path: &[u16]) -> bool {
    let len = wlen(path);
    if len == 0 {
        return false;
    }
    let Some(root_end) = get_root_end(path) else {
        return false;
    };
    let unc = is_prefixed_unc(path)
        || (path[0] == BACKSLASH && char_at(path, 1) == BACKSLASH && char_at(path, 2) != QUESTION);
    if unc {
        let next = root_end + 1;
        if next >= len {
            return true;
        }
        let (next, had_separator) = get_next_segment(path, next);
        if had_separator && next >= len {
            return false;
        }
        if next >= len {
            return true;
        }
        let (next, had_separator) = get_next_segment(path, next);
        return !had_separator && next >= len;
    }
    char_at(path, root_end) == BACKSLASH && root_end + 1 >= len
}

/// `PathSkipRootW`: returns the UTF-16 offset just past a drive or UNC share root.
pub fn path_skip_root(path: &[u16]) -> Option<usize> {
    let len = wlen(path);
    if len == 0 {
        return None;
    }
    if path[0] == BACKSLASH && char_at(path, 1) == BACKSLASH {
        let server_separator = (2..len).find(|&index| path[index] == BACKSLASH)?;
        let share_separator =
            (server_separator + 1..len).find(|&index| path[index] == BACKSLASH)?;
        return Some(share_separator + 1);
    }
    if len >= 3 && char_at(path, 1) == COLON && char_at(path, 2) == BACKSLASH {
        return Some(3);
    }
    None
}

/// `PathStripToRootW`: keeps the drive or UNC share root and removes later segments.
pub fn path_strip_to_root(path: &mut Vec<u16>) -> bool {
    let len = wlen(path);
    let Some(mut root_end) = cch_skip_root(path) else {
        path.clear();
        path.push(0);
        return false;
    };
    if root_end > 0
        && char_at(path, root_end - 1) == BACKSLASH
        && ((is_prefixed_unc(path) && char_at(path, 8) != 0)
            || (char_at(path, 0) == BACKSLASH
                && char_at(path, 1) == BACKSLASH
                && char_at(path, 2) != 0
                && char_at(path, 2) != QUESTION))
    {
        root_end -= 1;
    }
    if root_end >= PATHCCH_MAX_CCH || root_end >= len {
        return false;
    }
    path.truncate(root_end);
    path.push(0);
    true
}

/// `PathRemoveFileSpecW`: removes the final path component in place.
pub fn path_remove_file_spec(path: &mut Vec<u16>) -> bool {
    let len = wlen(path);
    if len == 0 {
        return false;
    }
    let mut root_end = if is_prefixed_volume(path) {
        48
    } else if is_prefixed_disk(path) {
        6
    } else if is_drive_spec_at(path, 0) {
        2
    } else {
        let mut end = usize::from(path[0] == BACKSLASH);
        if char_at(path, end + 1) != QUESTION {
            if char_at(path, end) == BACKSLASH {
                end += 1;
            }
            if end > 1 && is_drive_spec_at(path, end) {
                end += 2;
            }
            if char_at(path, end) == BACKSLASH
                && char_at(path, end + 1) != 0
                && char_at(path, end + 1) != BACKSLASH
            {
                end += 1;
            }
        }
        end
    };
    if root_end != 0
        && (is_prefixed_volume(path) || is_prefixed_disk(path) || is_drive_spec_at(path, 0))
        && char_at(path, root_end) == BACKSLASH
    {
        root_end += 1;
    }
    if let Some(mut separator) = (root_end..len).rfind(|&index| path[index] == BACKSLASH) {
        if separator != root_end {
            // A distinct separator from `root_end..len` has a preceding unit.
            if path[..separator].ends_with(&[BACKSLASH]) {
                separator -= 1;
            }
            path.truncate(separator);
            path.push(0);
            return true;
        }
    }
    if root_end >= len {
        return false;
    }
    path.truncate(root_end);
    path.push(0);
    true
}

/// `PathRemoveExtensionW`: truncates at the last extension dot, if present.
pub fn path_remove_extension(path: &mut Vec<u16>) {
    let len = wlen(path);
    let extension = path_find_extension(path);
    if extension < len {
        path.truncate(extension);
        path.push(0);
    }
}

/// `PathRemoveBackslashW`: removes a trailing backslash unless `path` is a root.
/// Returns the index corresponding to Wine's returned end pointer.
pub fn path_remove_backslash(path: &mut Vec<u16>) -> usize {
    let len = wlen(path);
    let end = len.saturating_sub(1);
    if len > 0 && !path_is_root(path) && path[end] == BACKSLASH {
        path[end] = 0;
        path.truncate(end + 1);
    }
    end
}

/// `PathGetDriveNumberW`: returns A=0 through Z=25, or -1 when absent.
pub fn path_get_drive_number(path: &[u16]) -> i32 {
    let mut start = 0;
    if starts_with(path, 0, b"\\\\?\\", false) {
        start = 4;
    }
    let drive = char_at(path, start);
    if drive == 0 || char_at(path, start + 1) != COLON {
        return -1;
    }
    if (b'A' as u16..=b'Z' as u16).contains(&drive) {
        return i32::from(drive - b'A' as u16);
    }
    if (b'a' as u16..=b'z' as u16).contains(&drive) {
        return i32::from(drive - b'a' as u16);
    }
    -1
}

/// `PathIsFileSpecW`: true when the string contains neither `\\` nor `:`.
pub fn path_is_file_spec(path: &[u16]) -> bool {
    let len = wlen(path);
    path[..len]
        .iter()
        .all(|&unit| unit != BACKSLASH && unit != COLON)
}

/// `PathIsLFNFileSpecW`: checks whether a name exceeds DOS 8.3 restrictions.
pub fn path_is_lfn_file_spec(path: &[u16]) -> bool {
    let len = wlen(path);
    let mut name_len = 0;
    let mut extension_len = 0;
    for &unit in &path[..len] {
        if unit == SPACE {
            return true;
        }
        if unit == DOT {
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

/// `PathRemoveBlanksW`: removes leading and trailing spaces in place.
pub fn path_remove_blanks(path: &mut Vec<u16>) {
    let len = wlen(path);
    if len == 0 {
        return;
    }
    let first = path[..len]
        .iter()
        .position(|&unit| unit != SPACE)
        .unwrap_or(len);
    let mut text = path[first..len].to_vec();
    while text.last() == Some(&SPACE) {
        text.pop();
    }
    *path = text;
    path.push(0);
}

/// `PathQuoteSpacesW`: wraps a path containing spaces in quotes if it fits `MAX_PATH`.
pub fn path_quote_spaces(path: &mut Vec<u16>) {
    let len = wlen(path);
    if path[..len].contains(&SPACE) && len + 3 < MAX_PATH {
        let mut quoted = Vec::with_capacity(len + 3);
        quoted.push(QUOTE);
        quoted.extend_from_slice(&path[..len]);
        quoted.push(QUOTE);
        quoted.push(0);
        *path = quoted;
    }
}

/// `PathUnquoteSpacesW`: removes a matching pair of surrounding quotes.
pub fn path_unquote_spaces(path: &mut Vec<u16>) {
    let len = wlen(path);
    if len == 0 || path[0] != QUOTE || path[len - 1] != QUOTE {
        return;
    }
    let mut unquoted = if len == 1 {
        Vec::new()
    } else {
        path[1..len - 1].to_vec()
    };
    unquoted.push(0);
    *path = unquoted;
}

/// `PathMakePrettyW`: lowercases a DOS-style path after its first character.
pub fn path_make_pretty(path: &mut [u16]) -> bool {
    let len = wlen(path);
    if path[..len]
        .iter()
        .any(|&unit| (b'a' as u16..=b'z' as u16).contains(&unit))
    {
        return false;
    }
    if len > 1 {
        for unit in &mut path[1..len] {
            *unit = ascii_lower(*unit);
        }
    }
    true
}

/// `PathCommonPrefixW`: returns Wine's case-insensitive, separator-aware prefix length.
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
        if (left == 0 || left == BACKSLASH) && (right == 0 || right == BACKSLASH) {
            common = first;
        }
        if left == 0 || ascii_upper(left) != ascii_upper(right) {
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

/// `PathIsPrefixW`: checks whether `prefix` is a Wine-recognized path prefix.
pub fn path_is_prefix(prefix: &[u16], path: &[u16]) -> bool {
    path_common_prefix(path, prefix) == wlen(prefix)
}

/// `PathIsSameRootW`: compares paths using `PathSkipRootW` and common-prefix semantics.
pub fn path_is_same_root(path1: &[u16], path2: &[u16]) -> bool {
    let Some(root_end) = path_skip_root(path1) else {
        return false;
    };
    path_common_prefix(path1, path2) + 1 >= root_end
}

/// `PathBuildRootW`: writes the root `X:\` for drive index `0..=25` (A-Z).
///
/// Writes nothing for an out-of-range drive, and requires at least 4 units.
pub fn path_build_root(path: &mut [u16], drive: i32) {
    if !(0..26).contains(&drive) || path.len() < 4 {
        return;
    }
    path[0] = u16::from(b'A') + drive as u16;
    path[1] = COLON;
    path[2] = BACKSLASH;
    path[3] = 0;
}

/// `PathGetArgsW`: index just past the first space outside double quotes, or
/// the string length if there is none.
pub fn path_get_args(path: &[u16]) -> usize {
    let len = wlen(path);
    let mut seen_quote = false;
    for (i, &c) in path[..len].iter().enumerate() {
        if c == SPACE && !seen_quote {
            return i + 1;
        }
        if c == QUOTE {
            seen_quote = !seen_quote;
        }
    }
    len
}

/// `PathRemoveArgsW`: cuts the arguments off `path`, along with the separating
/// space, mirroring Wine's handling of a trailing space.
pub fn path_remove_args(path: &mut Vec<u16>) {
    let args = path_get_args(path);
    let has_args = path.get(args).is_some_and(|&c| c != 0);
    if has_args || (args > 0 && path[args - 1] == SPACE) {
        path.truncate(args - 1);
        path.push(0);
    }
}

/// `PathIsNetworkPathW`: true for a UNC path, or for a drive that `is_net_drive`
/// reports as remote. `is_net_drive` stands in for shell32's `IsNetDrive`
/// (ordinal 66), which this crate does not depend on. A null `path` is not a
/// network path.
pub fn path_is_network_path(path: Option<&[u16]>, is_net_drive: impl Fn(i32) -> bool) -> bool {
    let Some(path) = path else {
        return false;
    };
    if char_at(path, 0) == BACKSLASH && char_at(path, 1) == BACKSLASH {
        return true;
    }
    let drive = path_get_drive_number(path);
    drive != -1 && is_net_drive(drive)
}

/// `PathUndecorateW`: removes a `[digits]` suffix that sits directly before the
/// extension, so `name[12].txt` becomes `name.txt`. Nothing changes unless the
/// text before the extension ends in `]`, and the bracket is removed only when
/// it is not preceded by a backslash. Only ASCII digits count.
pub fn path_undecorate(path: &mut Vec<u16>) {
    let ext = path_find_extension(path);
    if ext == 0 || path[ext - 1] != u16::from(b']') {
        return;
    }
    let mut skip = ext.saturating_sub(2);
    while skip > 0 && (u16::from(b'0')..=u16::from(b'9')).contains(&path[skip]) {
        skip -= 1;
    }
    if skip > 0 && path[skip] == u16::from(b'[') && path[skip - 1] != BACKSLASH {
        path.drain(skip..ext);
    }
}

/// `FixSlashesAndColonW`: turns each `/` before the terminator into `\`. Despite
/// the name, `:` is left alone.
pub fn fix_slashes_and_colon(s: &mut [u16]) {
    let len = wlen(s);
    for c in &mut s[..len] {
        if *c == SLASH {
            *c = BACKSLASH;
        }
    }
}

/// `StrChrW`: index of the first `ch` before the terminator. As with `wcschr`,
/// `ch == 0` matches the terminator itself.
pub fn str_chr(s: &[u16], ch: u16) -> Option<usize> {
    let len = wlen(s);
    if ch == 0 {
        return Some(len);
    }
    s[..len].iter().position(|&c| c == ch)
}

/// `StrChrNW`: like `str_chr`, but examines at most `max_len` characters and
/// never matches the terminator.
pub fn str_chr_n(s: &[u16], ch: u16, max_len: usize) -> Option<usize> {
    let len = wlen(s).min(max_len);
    s[..len].iter().position(|&c| c == ch)
}

/// `StrStrNW`: index of the first occurrence of `search` that starts within the
/// first `max_len` characters. An empty `search` or a zero `max_len` never
/// matches.
pub fn str_str_n(s: &[u16], search: &[u16], max_len: usize) -> Option<usize> {
    let needle = &search[..wlen(search)];
    let hay = &s[..wlen(s)];
    if needle.is_empty() || max_len == 0 {
        return None;
    }
    (0..max_len.min(hay.len())).find(|&pos| hay[pos..].starts_with(needle))
}

/// `StrStrW`: index of the first occurrence of `search` in `s`. An empty
/// `search` never matches.
pub fn str_str(s: &[u16], search: &[u16]) -> Option<usize> {
    str_str_n(s, search, usize::MAX)
}

/// `StrCpyNW`: copies at most `count - 1` characters of `src` into `dst` and
/// NUL-terminates the result. `count` is capped at `dst.len()`, because the
/// slice bound replaces the caller's promise about the buffer size.
pub fn str_cpy_n(dst: &mut [u16], src: &[u16], count: usize) {
    let count = count.min(dst.len());
    if count == 0 {
        return;
    }
    let copied = wlen(src).min(count - 1);
    dst[..copied].copy_from_slice(&src[..copied]);
    dst[copied] = 0;
}

/// `StrCpyNXW`: copies at most `count - 1` units of `src`, stopping at its NUL,
/// then writes a NUL after the copied text. Returns the index of that NUL.
///
/// A null `src`, or a `count` of zero or less, writes nothing and returns 0, as
/// the C code returns `dst` unchanged. `count` is capped at `dst.len()`.
pub fn str_cpy_nx(dst: &mut [u16], src: Option<&[u16]>, count: i32) -> usize {
    let count = usize::try_from(count).unwrap_or(0).min(dst.len());
    let Some(src) = src.filter(|_| count > 0) else {
        return 0;
    };
    let copied = wlen(src).min(count - 1);
    dst[..copied].copy_from_slice(&src[..copied]);
    dst[copied] = 0;
    copied
}

/// `StrCpyW`: copies `src` and its terminator into `dst`. Returns `false` and
/// leaves `dst` untouched when the copy does not fit, where `lstrcpyW` would
/// overflow.
pub fn str_cpy(dst: &mut [u16], src: &[u16]) -> bool {
    let len = wlen(src);
    if len >= dst.len() {
        return false;
    }
    dst[..len].copy_from_slice(&src[..len]);
    dst[len] = 0;
    true
}

/// `StrCatW`: appends `src` to the string in `dst`. Returns `false` and leaves
/// `dst` untouched when the result and its terminator do not fit.
pub fn str_cat(dst: &mut [u16], src: &[u16]) -> bool {
    let start = wlen(dst);
    let len = wlen(src);
    if start + len >= dst.len() {
        return false;
    }
    dst[start..start + len].copy_from_slice(&src[..len]);
    dst[start + len] = 0;
    true
}

/// `StrNCatW`: appends at most `cch_max - 1` characters of `src` to `dst`, as
/// `StrCpyNW` does at the end of the string. A non-positive `cch_max` leaves
/// `dst` unchanged.
pub fn str_ncat(dst: &mut [u16], src: &[u16], cch_max: i32) {
    let start = wlen(dst);
    let count = usize::try_from(cch_max).unwrap_or(0);
    str_cpy_n(&mut dst[start..], src, count);
}

/// `StrCatChainW`: writes `cat` into `s` at `at` and terminates it. `at` of
/// `None` (Wine's -1) appends after the existing text. At most `max_len - 1`
/// units are written, and `max_len` is capped at `s.len()`.
///
/// An `at` equal to `max_len` is moved back one unit. A larger `at`, or a null
/// `cat`, writes nothing. Returns the index of the terminator, or `at` when
/// nothing is written.
pub fn str_cat_chain(
    s: &mut [u16],
    max_len: usize,
    at: Option<usize>,
    cat: Option<&[u16]>,
) -> usize {
    let max_len = max_len.min(s.len());
    let mut at = at.unwrap_or_else(|| wlen(s));
    if max_len == 0 {
        return at;
    }
    if at == max_len {
        at -= 1;
    }
    if let Some(cat) = cat.filter(|_| at < max_len) {
        let cat = &cat[..wlen(cat)];
        let n = cat.len().min(max_len - 1 - at);
        s[at..at + n].copy_from_slice(&cat[..n]);
        at += n;
        s[at] = 0;
    }
    at
}

/// `StrCatBuffW`: appends `cat` to `s` within a buffer of `max_len` units that
/// also holds the text already in `s`. Nothing is appended once that text fills
/// the buffer. A null `s` does nothing, where Wine returns NULL.
pub fn str_cat_buff(s: Option<&mut [u16]>, cat: &[u16], max_len: i32) {
    let Some(s) = s else {
        return;
    };
    let len = wlen(s);
    let remaining = i64::from(max_len) - i64::try_from(len).unwrap_or(i64::MAX);
    if remaining > 0 {
        str_cpy_n(&mut s[len..], cat, usize::try_from(remaining).unwrap_or(0));
    }
}

/// `StrSpnW`: length of the leading run of `s` made only of characters in
/// `accept`. A null argument gives 0.
pub fn str_spn(s: Option<&[u16]>, accept: Option<&[u16]>) -> usize {
    let (Some(s), Some(accept)) = (s, accept) else {
        return 0;
    };
    let accept = &accept[..wlen(accept)];
    s[..wlen(s)]
        .iter()
        .take_while(|&&c| accept.contains(&c))
        .count()
}

/// `StrCSpnW`: length of the leading run of `s` with no character from
/// `reject`. A null argument gives 0.
pub fn str_cspn(s: Option<&[u16]>, reject: Option<&[u16]>) -> usize {
    let (Some(s), Some(reject)) = (s, reject) else {
        return 0;
    };
    let reject = &reject[..wlen(reject)];
    s[..wlen(s)]
        .iter()
        .take_while(|&&c| !reject.contains(&c))
        .count()
}

/// `StrPBrkW`: index of the first character of `s` that is in `accept`, if any.
pub fn str_pbrk(s: Option<&[u16]>, accept: Option<&[u16]>) -> Option<usize> {
    let s = s?;
    let accept = accept?;
    let accept = &accept[..wlen(accept)];
    s[..wlen(s)].iter().position(|c| accept.contains(c))
}

/// `StrDupW`: a NUL-terminated copy of `s` up to its terminator. A null `s`
/// gives the empty string. Wine allocates with `LocalAlloc`, which this port
/// replaces with a `Vec`.
pub fn str_dup(s: Option<&[u16]>) -> Vec<u16> {
    match s {
        None => vec![0],
        Some(s) => {
            let mut copy = s[..wlen(s)].to_vec();
            copy.push(0);
            copy
        }
    }
}

/// `SHStripMneumonicW`: removes the first `&` accelerator marker from `s` and
/// returns the character it marks. A `&&` removes one `&` and marks nothing, and
/// a marker with no character after it is left alone. In those cases the first
/// character is returned.
pub fn sh_strip_mneumonic(s: &mut Vec<u16>) -> u16 {
    let mut ch = char_at(s, 0);
    if let Some(amp) = str_chr(s, AMPERSAND) {
        let next = char_at(s, amp + 1);
        if next != 0 {
            if next != AMPERSAND {
                ch = next;
            }
            s.remove(amp);
        }
    }
    ch
}

/// `STIF_SUPPORT_HEX`: `StrToInt64ExW` also reads a `0x` or `0X` prefix.
pub const STIF_SUPPORT_HEX: u32 = 0x1;

/// The ASCII byte at `index`, or `None` for a unit past the terminator or
/// outside ASCII. Non-ASCII digits never parse, as in Wine.
fn ascii_at(s: &[u16], index: usize) -> Option<u8> {
    u8::try_from(char_at(s, index)).ok()
}

fn digit_at(s: &[u16], index: usize, radix: u32) -> Option<u32> {
    ascii_at(s, index).and_then(|c| (c as char).to_digit(radix))
}

/// `StrToInt64ExW`: skips leading spaces, tabs, and newlines, reads an optional
/// sign, then a decimal number, or a hexadecimal one after a `0x` prefix when
/// `flags` has `STIF_SUPPORT_HEX`. Text after the digits is ignored. Returns
/// `None` where Wine returns FALSE.
///
/// A sign before a hexadecimal number is skipped, not applied, so `-0x44fd`
/// reads as `0x44fd`. Overflow wraps, as the C arithmetic does in practice.
pub fn str_to_int64_ex(s: &[u16], flags: u32) -> Option<i64> {
    let s = &s[..wlen(s)];
    let mut pos = 0;
    while matches!(ascii_at(s, pos), Some(b' ' | b'\t' | b'\n')) {
        pos += 1;
    }
    let negative = ascii_at(s, pos) == Some(b'-');
    if negative || ascii_at(s, pos) == Some(b'+') {
        pos += 1;
    }
    let hex = (flags & STIF_SUPPORT_HEX) != 0
        && ascii_at(s, pos) == Some(b'0')
        && matches!(ascii_at(s, pos + 1), Some(b'x' | b'X'));
    let radix = if hex {
        pos += 2;
        16
    } else {
        10
    };
    digit_at(s, pos, radix)?;
    let mut value: i64 = 0;
    while let Some(digit) = digit_at(s, pos, radix) {
        value = value
            .wrapping_mul(i64::from(radix))
            .wrapping_add(i64::from(digit));
        pos += 1;
    }
    Some(if negative && !hex {
        value.wrapping_neg()
    } else {
        value
    })
}

/// `StrToIntExW`: `str_to_int64_ex` truncated to 32 bits.
pub fn str_to_int_ex(s: &[u16], flags: u32) -> Option<i32> {
    str_to_int64_ex(s, flags).map(|value| value as i32)
}

/// `StrToIntW`: the number at the start of `s`, or 0. Only text that starts with
/// `-` or a digit is parsed, so leading spaces and `0x` prefixes give 0.
pub fn str_to_int(s: &[u16]) -> i32 {
    match ascii_at(s, 0) {
        Some(b'-' | b'0'..=b'9') => str_to_int_ex(s, 0).unwrap_or(0),
        _ => 0,
    }
}

/// `StrRChrW`: index of the last `ch` before `end`, or before the terminator when
/// `end` is `None`. `end` is an exclusive bound and is clamped to the slice.
pub fn str_rchr(s: &[u16], end: Option<usize>, ch: u16) -> Option<usize> {
    let end = end.unwrap_or_else(|| wlen(s)).min(s.len());
    (0..end).rev().find(|&i| s[i] == ch)
}

/// `StrTrimW`: removes the leading and trailing characters of `s` that appear in
/// `trim`. Returns `true` when anything was removed.
pub fn str_trim(s: &mut Vec<u16>, trim: &[u16]) -> bool {
    let is_trim = |c: u16| str_chr(trim, c).is_some();
    let lead = s[..wlen(s)].iter().take_while(|&&c| is_trim(c)).count();
    let mut trimmed = false;
    if lead > 0 {
        s.drain(..lead);
        trimmed = true;
    }
    let len = wlen(s);
    let tail = s[..len].iter().rev().take_while(|&&c| is_trim(c)).count();
    if tail > 0 {
        s[len - tail] = 0;
        trimmed = true;
    }
    trimmed
}

/// `SHSearchMapInt`: the value paired with the first key equal to `key`, or -1.
///
/// The slices replace the C `iLen`, so a short list is never read past its end.
/// Pairs are matched up to the shorter of the two slices.
pub fn sh_search_map_int(keys: &[i32], values: &[i32], key: i32) -> i32 {
    keys.iter()
        .zip(values)
        .find(|(k, _)| **k == key)
        .map_or(-1, |(_, value)| *value)
}

/// `SHStringFromGUIDW`: writes `guid` as uppercase `{8-4-4-4-12}` text with a
/// terminating NUL. Returns the length in units including the NUL, or `None`
/// when `dst` cannot hold all 39 units. Nothing is written in that case.
pub fn sh_string_from_guid(guid: &Uuid, dst: &mut [u16]) -> Option<usize> {
    let d = guid.data4;
    let text = format!(
        "{{{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}",
        guid.data1, guid.data2, guid.data3, d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7]
    );
    let units: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
    dst.get_mut(..units.len())?.copy_from_slice(&units);
    Some(units.len())
}

/// `count` hexadecimal digits starting at `start`, most significant first.
fn hex_run(s: &[u16], start: usize, count: usize) -> Option<u64> {
    (start..start + count).try_fold(0, |acc, i| {
        Some((acc << 4) | u64::from(digit_at(s, i, 16)?))
    })
}

fn dash_at(s: &[u16], index: usize) -> Option<()> {
    (char_at(s, index) == u16::from(b'-')).then_some(())
}

/// `GUIDFromStringW`: parses `{XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX}` with hex
/// digits in either case. The NUL must follow the closing brace, and text after
/// that NUL is ignored. `None` is the null string, which gives the nil GUID.
///
/// Wine's `CLSIDFromString` also falls back to a ProgID lookup in the registry.
/// This port has no registry, so a ProgID returns `None`.
pub fn guid_from_string_w(s: Option<&[u16]>) -> Option<Uuid> {
    let Some(s) = s else {
        return Some(Uuid::default());
    };
    if char_at(s, 0) != u16::from(b'{') {
        return None;
    }
    let data1 = hex_run(s, 1, 8)? as u32;
    dash_at(s, 9)?;
    let data2 = hex_run(s, 10, 4)? as u16;
    dash_at(s, 14)?;
    let data3 = hex_run(s, 15, 4)? as u16;
    dash_at(s, 19)?;
    dash_at(s, 24)?;
    let mut data4 = [0u8; 8];
    for (k, byte) in data4.iter_mut().enumerate() {
        let at = 20 + 2 * k + usize::from(k >= 2);
        *byte = hex_run(s, at, 2)? as u8;
    }
    if char_at(s, 37) != u16::from(b'}') || s.get(38) != Some(&0) {
        return None;
    }
    Some(Uuid {
        data1,
        data2,
        data3,
        data4,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn w(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    #[test]
    fn unc_classifiers_match_server_and_share_shapes() {
        assert!(path_is_unc(&w("\\\\server\\share")));
        assert!(!path_is_unc(&w("\\server")));
        assert!(!path_is_unc(&w("")));
        assert!(path_is_unc_server(&w("\\\\server")));
        assert!(path_is_unc_server(&w("\\\\")));
        assert!(!path_is_unc_server(&w("\\\\server\\share")));
        assert!(!path_is_unc_server(&w("relative")));
        assert!(path_is_unc_server_share(&w("\\\\server\\share")));
        assert!(!path_is_unc_server_share(&w("\\\\server")));
        assert!(!path_is_unc_server_share(&w("\\\\server\\share\\dir")));
        assert!(!path_is_unc_server_share(&w("relative")));
    }

    #[test]
    fn root_detection_handles_drive_unc_and_volume_roots() {
        assert!(!path_is_root(&w("")));
        assert_eq!(get_root_end(&w("")), None);
        assert!(!path_is_root(&w("relative")));
        assert!(path_is_root(&w("\\")));
        assert!(path_is_root(&w("C:\\")));
        assert!(!path_is_root(&w("C:")));
        assert!(!path_is_root(&w("C:\\dir")));
        assert!(path_is_root(&w("\\\\")));
        assert!(path_is_root(&w("\\\\server")));
        assert!(!path_is_root(&w("\\\\server\\")));
        assert!(path_is_root(&w("\\\\server\\share")));
        assert!(!path_is_root(&w("\\\\server\\share\\dir")));
        assert!(path_is_root(&w("\\\\?\\C:\\")));
        assert_eq!(get_root_end(&w("\\\\?\\C:")), Some(5));
        assert!(path_is_root(&w("\\\\?\\UNC\\server\\share")));
        assert!(!path_is_root(&w("\\\\?\\UNC\\server\\share\\dir")));
        assert!(path_is_root(&w(
            "\\\\?\\Volume{12345678-1234-1234-1234-123456789abc}\\"
        )));
        assert_eq!(
            get_root_end(&w("\\\\?\\Volume{12345678-1234-1234-1234-123456789abc}")),
            Some(47)
        );
        assert!(!path_is_root(&w(
            "\\\\?\\Volume{12345678-1234-1234-1234-123456789abg}\\"
        )));
    }

    #[test]
    fn skip_root_requires_complete_drive_or_unc_root() {
        assert_eq!(path_skip_root(&w("C:\\dir")), Some(3));
        assert_eq!(path_skip_root(&w("\\\\server\\share\\dir")), Some(15));
        assert_eq!(path_skip_root(&w("\\\\server\\share")), None);
        assert_eq!(path_skip_root(&w("\\\\server")), None);
        assert_eq!(path_skip_root(&w("\\\\server\\")), None);
        assert_eq!(path_skip_root(&w("C:")), None);
        assert_eq!(path_skip_root(&w("C:relative")), None);
        assert_eq!(path_skip_root(&w("relative")), None);
        assert_eq!(path_skip_root(&w("")), None);
    }

    #[test]
    fn strip_to_root_preserves_roots_and_clears_invalid_paths() {
        let mut drive = w("C:\\dir\\file");
        assert!(path_strip_to_root(&mut drive));
        assert_eq!(drive, w("C:\\"));

        let mut unc = w("\\\\server\\share\\dir\\file");
        assert!(path_strip_to_root(&mut unc));
        assert_eq!(unc, w("\\\\server\\share"));

        let mut empty_share = w("\\\\\\\\name");
        assert!(path_strip_to_root(&mut empty_share));
        assert_eq!(empty_share, w("\\\\"));

        let mut prefixed_unc = w("\\\\?\\UNC\\server\\share\\dir");
        assert!(path_strip_to_root(&mut prefixed_unc));
        assert_eq!(prefixed_unc, w("\\\\?\\UNC\\server\\share"));

        let mut prefixed_drive = w("\\\\?\\C:\\dir");
        assert!(path_strip_to_root(&mut prefixed_drive));
        assert_eq!(prefixed_drive, w("\\\\?\\C:\\"));

        let mut prefixed_volume = w("\\\\?\\Volume{12345678-1234-1234-1234-123456789abc}\\dir");
        assert!(path_strip_to_root(&mut prefixed_volume));
        assert_eq!(
            prefixed_volume,
            w("\\\\?\\Volume{12345678-1234-1234-1234-123456789abc}\\")
        );

        let mut already_root = w("C:\\");
        assert!(!path_strip_to_root(&mut already_root));
        assert_eq!(already_root, w("C:\\"));

        let mut incomplete_unc = w("\\\\server");
        assert!(!path_strip_to_root(&mut incomplete_unc));
        assert_eq!(incomplete_unc, w("\\\\server"));

        let mut invalid_prefix = w("\\\\?x");
        assert!(!path_strip_to_root(&mut invalid_prefix));
        assert_eq!(invalid_prefix, w(""));

        let mut relative = w("relative");
        assert!(!path_strip_to_root(&mut relative));
        assert_eq!(relative, w(""));

        let mut empty = vec![0];
        assert!(!path_strip_to_root(&mut empty));
        assert_eq!(empty, vec![0]);

        let mut too_long_unc = vec![b'\\' as u16, b'\\' as u16];
        too_long_unc.extend(std::iter::repeat_n(b'a' as u16, PATHCCH_MAX_CCH));
        too_long_unc.push(0);
        let before = too_long_unc.clone();
        assert!(!path_strip_to_root(&mut too_long_unc));
        assert_eq!(too_long_unc, before);
    }

    #[test]
    fn remove_file_spec_handles_roots_prefixes_and_components() {
        let mut relative = w("dir\\file.txt");
        assert!(path_remove_file_spec(&mut relative));
        assert_eq!(relative, w("dir"));

        let mut file_only = w("file.txt");
        assert!(path_remove_file_spec(&mut file_only));
        assert_eq!(file_only, w(""));

        let mut drive = w("C:\\file.txt");
        assert!(path_remove_file_spec(&mut drive));
        assert_eq!(drive, w("C:\\"));

        let mut drive_root = w("C:\\");
        assert!(!path_remove_file_spec(&mut drive_root));
        assert_eq!(drive_root, w("C:\\"));

        let mut drive_relative = w("C:file");
        assert!(path_remove_file_spec(&mut drive_relative));
        assert_eq!(drive_relative, w("C:"));

        let mut disk_prefix = w("\\\\?\\C:\\file");
        assert!(path_remove_file_spec(&mut disk_prefix));
        assert_eq!(disk_prefix, w("\\\\?\\C:\\"));

        let mut volume_prefix = w("\\\\?\\Volume{12345678-1234-1234-1234-123456789abc}x");
        assert!(path_remove_file_spec(&mut volume_prefix));
        assert_eq!(
            volume_prefix,
            w("\\\\?\\Volume{12345678-1234-1234-1234-123456789abc}")
        );

        let mut doubled = w("\\\\\\\\");
        assert!(path_remove_file_spec(&mut doubled));
        assert_eq!(doubled, w("\\\\"));

        let mut drive_like_unc = w("\\\\C:\\file");
        assert!(path_remove_file_spec(&mut drive_like_unc));
        assert_eq!(drive_like_unc, w("\\\\C:\\"));

        let mut extended_prefix_only = w("\\\\?");
        assert!(path_remove_file_spec(&mut extended_prefix_only));
        assert_eq!(extended_prefix_only, w("\\"));

        let mut question_prefix = w("\\\\?foo\\bar");
        assert!(path_remove_file_spec(&mut question_prefix));
        assert_eq!(question_prefix, w("\\\\?foo"));

        let mut separator_at_root = w("C:\\\\foo");
        assert!(path_remove_file_spec(&mut separator_at_root));
        assert_eq!(separator_at_root, w("C:\\"));

        let mut empty_unc_server = w("\\\\\\\\server");
        assert!(path_remove_file_spec(&mut empty_unc_server));
        assert_eq!(empty_unc_server, w("\\\\"));

        let mut empty = vec![0];
        assert!(!path_remove_file_spec(&mut empty));
        assert_eq!(empty, vec![0]);
    }

    #[test]
    fn remove_extension_and_backslash_preserve_wine_boundaries() {
        let mut extension = w("file.txt");
        path_remove_extension(&mut extension);
        assert_eq!(extension, w("file"));
        let mut trailing_dot = w("file.");
        path_remove_extension(&mut trailing_dot);
        assert_eq!(trailing_dot, w("file"));
        let mut no_extension = w("dir\\file");
        path_remove_extension(&mut no_extension);
        assert_eq!(no_extension, w("dir\\file"));
        let mut empty = w("");
        path_remove_extension(&mut empty);
        assert_eq!(empty, w(""));

        let mut trailing = w("dir\\file\\");
        assert_eq!(path_remove_backslash(&mut trailing), 8);
        assert_eq!(trailing, w("dir\\file"));
        let mut root = w("C:\\");
        assert_eq!(path_remove_backslash(&mut root), 2);
        assert_eq!(root, w("C:\\"));
        let mut empty = w("");
        assert_eq!(path_remove_backslash(&mut empty), 0);
        assert_eq!(empty, w(""));
        let mut one_char = w("x");
        assert_eq!(path_remove_backslash(&mut one_char), 0);
        assert_eq!(one_char, w("x"));
    }

    #[test]
    fn drive_number_and_file_spec_follow_wide_path_rules() {
        assert_eq!(path_get_drive_number(&w("C:\\dir")), 2);
        assert_eq!(path_get_drive_number(&w("z:")), 25);
        assert_eq!(path_get_drive_number(&w("\\\\?\\D:\\dir")), 3);
        assert_eq!(path_get_drive_number(&w("\\\\?d:\\dir")), -1);
        assert_eq!(path_get_drive_number(&w("1:")), -1);
        assert_eq!(path_get_drive_number(&w("C")), -1);
        assert_eq!(path_get_drive_number(&w("")), -1);

        assert!(path_is_file_spec(&w("file.txt")));
        assert!(path_is_file_spec(&w("")));
        assert!(!path_is_file_spec(&w("dir\\file")));
        assert!(!path_is_file_spec(&w("name:stream")));
    }

    #[test]
    fn lfn_file_spec_detects_dos_name_limits() {
        assert!(!path_is_lfn_file_spec(&w("12345678.123")));
        assert!(!path_is_lfn_file_spec(&w("abc")));
        assert!(path_is_lfn_file_spec(&w("123456789")));
        assert!(path_is_lfn_file_spec(&w("a.bbbb")));
        assert!(path_is_lfn_file_spec(&w("a.b.c")));
        assert!(path_is_lfn_file_spec(&w("a b")));
        assert!(path_is_lfn_file_spec(&w("a.12345")));
    }

    #[test]
    fn remove_blanks_quotes_and_pretty_format_paths() {
        let mut blanks = w("  test  ");
        path_remove_blanks(&mut blanks);
        assert_eq!(blanks, w("test"));
        let mut trailing_blanks = w("test ");
        path_remove_blanks(&mut trailing_blanks);
        assert_eq!(trailing_blanks, w("test"));
        let mut only_blanks = w("   ");
        path_remove_blanks(&mut only_blanks);
        assert_eq!(only_blanks, w(""));
        let mut empty = w("");
        path_remove_blanks(&mut empty);
        assert_eq!(empty, w(""));

        let mut quote = w("a path");
        path_quote_spaces(&mut quote);
        assert_eq!(quote, w("\"a path\""));
        let mut no_spaces = w("path");
        path_quote_spaces(&mut no_spaces);
        assert_eq!(no_spaces, w("path"));
        let mut max_fit: Vec<u16> = vec![b'a' as u16; 255];
        max_fit[100] = SPACE;
        max_fit.push(0);
        path_quote_spaces(&mut max_fit);
        assert_eq!(max_fit[0], QUOTE);
        let mut too_long: Vec<u16> = vec![b'a' as u16; 257];
        too_long[100] = SPACE;
        too_long.push(0);
        let before = too_long.clone();
        path_quote_spaces(&mut too_long);
        assert_eq!(too_long, before);

        let mut unquote = w("\"a path\"");
        path_unquote_spaces(&mut unquote);
        assert_eq!(unquote, w("a path"));
        let mut unmatched = w("\"path");
        path_unquote_spaces(&mut unmatched);
        assert_eq!(unmatched, w("\"path"));
        let mut lone_quote = w("\"");
        path_unquote_spaces(&mut lone_quote);
        assert_eq!(lone_quote, w(""));
        let mut not_quoted = w("path");
        path_unquote_spaces(&mut not_quoted);
        assert_eq!(not_quoted, w("path"));

        let mut pretty = w("C:\\FOLDER");
        assert!(path_make_pretty(&mut pretty));
        assert_eq!(pretty, w("C:\\folder"));
        let mut mixed = w("C:\\Folder");
        let before = mixed.clone();
        assert!(!path_make_pretty(&mut mixed));
        assert_eq!(mixed, before);
        let mut empty_pretty = w("");
        assert!(path_make_pretty(&mut empty_pretty));
    }

    #[test]
    fn common_prefix_and_prefix_helpers_match_wine_quirks() {
        assert_eq!(path_common_prefix(&w("C:\\one\\a"), &w("c:\\one\\b")), 6);
        assert_eq!(path_common_prefix(&w("one\\.two"), &w("one\\.three")), 3);
        assert_eq!(path_common_prefix(&w("same"), &w("same")), 4);
        assert_eq!(path_common_prefix(&w("\\\\server"), &w("C:\\server")), 0);
        assert_eq!(path_common_prefix(&w("C:\\server"), &w("\\\\server")), 0);
        assert_eq!(path_common_prefix(&w("\\\\one"), &w("\\\\two")), 0);
        assert!(path_is_prefix(&w("C:\\"), &w("C:\\folder")));
        assert!(!path_is_prefix(&w("C:\\folder"), &w("C:\\foo")));
        assert!(path_is_prefix(&w(""), &w("anything")));
        assert!(path_is_same_root(&w("C:\\one"), &w("C:\\two")));
        assert!(!path_is_same_root(&w("C:\\one"), &w("D:\\two")));
        assert!(!path_is_same_root(&w("relative"), &w("relative")));
    }

    #[test]
    fn build_root_writes_drive_letter_root() {
        let mut p = w("aaaaaaaaa");
        path_build_root(&mut p, 0);
        assert_eq!(&p[..4], &w("A:\\")[..]);
        path_build_root(&mut p, 25);
        assert_eq!(&p[..4], &w("Z:\\")[..]);
    }

    #[test]
    fn build_root_ignores_invalid_drive_or_short_buffer() {
        let mut p = w("aaaaaaaaa");
        path_build_root(&mut p, -1);
        path_build_root(&mut p, 26);
        assert_eq!(p, w("aaaaaaaaa"));
        let mut short = [b'x' as u16; 3];
        path_build_root(&mut short, 0);
        assert_eq!(short, [b'x' as u16; 3]);
    }

    #[test]
    fn get_args_skips_unquoted_space_only() {
        assert_eq!(path_get_args(&w("prog arg")), 5);
        assert_eq!(path_get_args(&w("\"a b\" c")), 6);
        assert_eq!(path_get_args(&w("prog")), 4);
        assert_eq!(path_get_args(&[0]), 0);
    }

    #[test]
    fn remove_args_truncates_at_first_unquoted_space() {
        let mut p = w("prog arg1 arg2");
        path_remove_args(&mut p);
        assert_eq!(p, w("prog"));

        let mut quoted = w("\"C:\\a b.exe\" x");
        path_remove_args(&mut quoted);
        assert_eq!(quoted, w("\"C:\\a b.exe\""));

        let mut trailing = w("prog ");
        path_remove_args(&mut trailing);
        assert_eq!(trailing, w("prog"));

        let mut plain = w("prog");
        path_remove_args(&mut plain);
        assert_eq!(plain, w("prog"));

        let mut empty = vec![0u16];
        path_remove_args(&mut empty);
        assert_eq!(empty, vec![0u16]);
    }

    #[test]
    fn find_file_name_after_separators() {
        assert_eq!(path_find_file_name(&w("dir\\file.txt")), 4);
        assert_eq!(path_find_file_name(&w("dir/file.txt")), 4);
        assert_eq!(path_find_file_name(&w("C:file.txt")), 2);
        assert_eq!(path_find_file_name(&w("a//b")), 3);
        assert_eq!(path_find_file_name(&w("a\\\\b")), 3);
    }

    #[test]
    fn find_file_name_without_separator_is_zero() {
        assert_eq!(path_find_file_name(&w("file.txt")), 0);
        assert_eq!(path_find_file_name(&w("")), 0);
    }

    #[test]
    fn find_file_name_ignores_trailing_separator() {
        assert_eq!(path_find_file_name(&w("dir\\")), 0);
    }

    #[test]
    fn find_extension_cases() {
        assert_eq!(path_find_extension(&w("file")), 4);
        assert_eq!(path_find_extension(&w("file.txt")), 4);
        assert_eq!(path_find_extension(&w("file.txt.doc")), 8);
        assert_eq!(path_find_extension(&w("")), 0);
    }

    #[test]
    fn find_extension_reset_by_separator_or_space() {
        assert_eq!(path_find_extension(&w("dir.x\\file")), 10);
        assert_eq!(path_find_extension(&w("file.t x")), 8);
    }

    #[test]
    fn is_relative_cases() {
        assert!(path_is_relative(&[0]));
        assert!(path_is_relative(&w("test.txt")));
        assert!(path_is_relative(&w("file://folder/test.txt")));
        assert!(path_is_relative(&w("file:///C:/test.txt")));
        assert!(!path_is_relative(&w("\\\\folder\\test.txt")));
        assert!(!path_is_relative(&w("C:\\test.txt")));
    }

    #[test]
    fn add_backslash_appends_once() {
        let mut p = w("one\\two");
        assert_eq!(path_add_backslash(&mut p), Some(8));
        assert_eq!(p, w("one\\two\\"));
        assert_eq!(path_add_backslash(&mut p), Some(8));
        assert_eq!(p, w("one\\two\\"));
    }

    #[test]
    fn add_backslash_empty_stays_empty() {
        let mut p = vec![0u16];
        assert_eq!(path_add_backslash(&mut p), Some(0));
        assert_eq!(p, vec![0u16]);
    }

    #[test]
    fn add_backslash_rejects_long_path() {
        let mut p: Vec<u16> = vec![b'a' as u16; MAX_PATH];
        p.push(0);
        let before = p.clone();
        assert_eq!(path_add_backslash(&mut p), None);
        assert_eq!(p, before);
    }

    #[test]
    fn strip_path_keeps_file_name() {
        let mut p = w("short//path\\file.txt");
        path_strip_path(&mut p);
        assert_eq!(p, w("file.txt"));
    }

    #[test]
    fn strip_path_without_components_is_noop() {
        let mut p = w("test");
        path_strip_path(&mut p);
        assert_eq!(p, w("test"));
    }

    // Vectors from dlls/shlwapi/tests/string.c (test_StrChrNW, test_StrChrW,
    // test_StrStrW, test_StrStrNW) on "Testing String" and "DeAdBeEf".
    #[test]
    fn chr_matches_wcschr_and_respects_limit() {
        let s = w("Testing String");
        assert_eq!(str_chr(&s, b't' as u16), Some(3));
        assert_eq!(str_chr(&s, b'S' as u16), Some(8));
        assert_eq!(str_chr(&s, b'r' as u16), Some(10));
        assert_eq!(str_chr(&s, b'z' as u16), None);
        assert_eq!(str_chr(&s, 0), Some(14));
        assert_eq!(str_chr(&[b'a' as u16], b'b' as u16), None);
        assert_eq!(str_chr(&[b'a' as u16], b'a' as u16), Some(0));
        assert_eq!(str_chr_n(&s, b't' as u16, 10), Some(3));
        assert_eq!(str_chr_n(&s, b'S' as u16, 10), Some(8));
        assert_eq!(str_chr_n(&s, b'r' as u16, 10), None);
        assert_eq!(str_chr_n(&s, b'r' as u16, 11), Some(10));
        assert_eq!(str_chr_n(&s, b'r' as u16, 0), None);
        assert_eq!(str_chr_n(&w("ab"), 0, 5), None);
        assert_eq!(str_chr_n(&w("ab"), b'b' as u16, 100), Some(1));
    }

    #[test]
    fn str_str_matches_wcsstr() {
        let s = w("DeAdBeEf");
        assert_eq!(str_str(&s, &w("")), None);
        assert_eq!(str_str(&s, &w("DeAd")), Some(0));
        assert_eq!(str_str(&s, &w("dead")), None);
        assert_eq!(str_str(&s, &w("AdBe")), Some(2));
        assert_eq!(str_str(&s, &w("adbe")), None);
        assert_eq!(str_str(&s, &w("BeEf")), Some(4));
        assert_eq!(str_str(&s, &w("beef")), None);
        assert_eq!(str_str(&w(""), &w("")), None);
        assert_eq!(str_str(&w(""), &w("x")), None);
        assert_eq!(str_str(&w("ab"), &w("abc")), None);
        assert_eq!(str_str(&w("aab"), &w("ab")), Some(1));
    }

    #[test]
    fn str_str_n_counts_only_starting_positions() {
        let s = w("DeAdBeEf");
        let beef = w("BeEf");
        assert_eq!(str_str_n(&s, &w(""), 8), None);
        assert_eq!(str_str_n(&s, &w("DeAd"), 8), Some(0));
        assert_eq!(str_str_n(&s, &w("dead"), 8), None);
        assert_eq!(str_str_n(&s, &w("AdBe"), 8), Some(2));
        assert_eq!(str_str_n(&s, &w("adbe"), 8), None);
        assert_eq!(str_str_n(&s, &w("beef"), 8), None);
        assert_eq!(str_str_n(&s, &beef, 0), None);
        for count in 1..=4 {
            assert_eq!(str_str_n(&s, &beef, count), None, "count {count}");
        }
        for count in [5, 6, 7, 8, 9, 100] {
            assert_eq!(str_str_n(&s, &beef, count), Some(4), "count {count}");
        }
        assert_eq!(str_str_n(&w("ab"), &w("abc"), 5), None);
    }

    // Vectors from dlls/shlwapi/tests/string.c (test_StrCpyW), plus the
    // StrCpyNW body in dlls/kernelbase/string.c.
    #[test]
    fn cpy_n_truncates_and_terminates() {
        let mut dst = [9u16; 8];
        str_cpy_n(&mut dst, &w("abc"), 4);
        assert_eq!(&dst[..4], &w("abc")[..]);
        assert_eq!(dst[4], 9);

        let mut dst = [9u16; 8];
        str_cpy_n(&mut dst, &w("abc"), 3);
        assert_eq!(&dst[..3], &w("ab")[..]);
        assert_eq!(dst[3], 9);

        let mut dst = [9u16; 8];
        str_cpy_n(&mut dst, &w("abc"), 1);
        assert_eq!(dst[0], 0);

        let mut dst = [9u16; 8];
        str_cpy_n(&mut dst, &w("abc"), 0);
        assert_eq!(dst, [9u16; 8]);

        let mut dst = [9u16; 4];
        str_cpy_n(&mut dst, &w("abcdef"), 100);
        assert_eq!(dst, [b'a' as u16, b'b' as u16, b'c' as u16, 0]);
    }

    #[test]
    fn cpy_copies_when_it_fits() {
        let mut dst = [0u16; 8];
        assert!(str_cpy(&mut dst, &w("Test")));
        assert_eq!(&dst[..5], &w("Test")[..]);

        let mut small = [7u16; 4];
        assert!(!str_cpy(&mut small, &w("Test")));
        assert_eq!(small, [7u16; 4]);

        let mut exact = [7u16; 5];
        assert!(str_cpy(&mut exact, &w("Test")));
        assert_eq!(
            exact,
            [b'T' as u16, b'e' as u16, b's' as u16, b't' as u16, 0]
        );

        let mut empty = [1u16; 1];
        assert!(str_cpy(&mut empty, &w("")));
        assert_eq!(empty, [0u16]);
    }

    #[test]
    fn cat_appends_when_it_fits() {
        let mut dst = [0u16; 10];
        dst[..6].copy_from_slice(&w("Hello"));
        assert!(str_cat(&mut dst, &w(", W")));
        assert_eq!(&dst[..9], &w("Hello, W")[..]);

        let mut tight = [0u16; 8];
        tight[..6].copy_from_slice(&w("Hello"));
        assert!(!str_cat(&mut tight, &w(", W")));
        assert_eq!(&tight[..6], &w("Hello")[..]);
        assert_eq!(tight[6], 0);

        let mut exact = [0u16; 9];
        exact[..6].copy_from_slice(&w("Hello"));
        assert!(str_cat(&mut exact, &w(", W")));
        assert_eq!(&exact[..9], &w("Hello, W")[..]);

        let mut empty_src = [0u16; 3];
        empty_src[..3].copy_from_slice(&[b'a' as u16, b'b' as u16, 0]);
        assert!(str_cat(&mut empty_src, &w("")));
        assert_eq!(&empty_src[..3], &w("ab")[..]);
    }

    #[test]
    fn ncat_appends_at_most_count_minus_one() {
        let mut dst = [0u16; 16];
        dst[..6].copy_from_slice(&w("Hello"));
        str_ncat(&mut dst, &w("World"), 3);
        assert_eq!(&dst[..8], &w("HelloWo")[..]);

        let mut dst = [0u16; 16];
        dst[..6].copy_from_slice(&w("Hello"));
        str_ncat(&mut dst, &w("World"), 100);
        assert_eq!(&dst[..11], &w("HelloWorld")[..]);

        let mut dst = [0u16; 16];
        dst[..6].copy_from_slice(&w("Hello"));
        str_ncat(&mut dst, &w("World"), 0);
        assert_eq!(&dst[..6], &w("Hello")[..]);
        str_ncat(&mut dst, &w("World"), -1);
        assert_eq!(&dst[..6], &w("Hello")[..]);

        let mut dst = [0u16; 8];
        dst[..6].copy_from_slice(&w("Hello"));
        str_ncat(&mut dst, &w("World"), 100);
        assert_eq!(&dst[..8], &w("HelloWo")[..]);
    }

    // Expected results follow the body of SHStripMneumonicW in
    // dlls/shlwapi/string.c: the first `&` is removed, `&&` removes one and
    // marks nothing, and a trailing `&` is left in place.
    #[test]
    fn strip_mneumonic_removes_marker_and_returns_marked_char() {
        let mut s = w("&File");
        assert_eq!(sh_strip_mneumonic(&mut s), b'F' as u16);
        assert_eq!(s, w("File"));

        let mut s = w("Save &As");
        assert_eq!(sh_strip_mneumonic(&mut s), b'A' as u16);
        assert_eq!(s, w("Save As"));

        let mut s = w("&&Name");
        assert_eq!(sh_strip_mneumonic(&mut s), b'&' as u16);
        assert_eq!(s, w("&Name"));

        let mut s = w("A&&B&C");
        assert_eq!(sh_strip_mneumonic(&mut s), b'A' as u16);
        assert_eq!(s, w("A&B&C"));

        let mut s = w("Trail&");
        assert_eq!(sh_strip_mneumonic(&mut s), b'T' as u16);
        assert_eq!(s, w("Trail&"));

        let mut s = w("NoMarker");
        assert_eq!(sh_strip_mneumonic(&mut s), b'N' as u16);
        assert_eq!(s, w("NoMarker"));

        let mut s = w("");
        assert_eq!(sh_strip_mneumonic(&mut s), 0);
        assert_eq!(s, w(""));
    }

    // Rows from the StrToInt_results table in dlls/shlwapi/tests/string.c:
    // (input, StrToInt64ExW with flags 0, with STIF_SUPPORT_HEX, StrToIntW).
    // `None` marks the rows where Wine reports failure.
    #[test]
    fn str_to_int_matches_wine_table() {
        let rows: [(&str, Option<i64>, Option<i64>, i32); 30] = [
            ("1099", Some(1099), Some(1099), 1099),
            ("4294967319", Some(4294967319), Some(4294967319), 23),
            ("+88987", Some(88987), Some(88987), 0),
            ("012", Some(12), Some(12), 12),
            ("-55", Some(-55), Some(-55), -55),
            ("-0", Some(0), Some(0), 0),
            ("0x44ff", Some(0), Some(0x44ff), 0),
            ("0x2bdc546291f4b1", Some(0), Some(0x2bdc546291f4b1), 0),
            ("+0x44f4", Some(0), Some(0x44f4), 0),
            ("-0x44fd", Some(0), Some(0x44fd), 0),
            ("+ 88987", None, None, 0),
            ("- 55", None, None, 0),
            ("- 0", None, None, 0),
            ("+ 0x44f4", None, None, 0),
            ("--0x44fd", None, None, 0),
            (" 1999", Some(1999), Some(1999), 0),
            (" +88987", Some(88987), Some(88987), 0),
            (" 012", Some(12), Some(12), 0),
            (" -55", Some(-55), Some(-55), 0),
            (" 0x44ff", Some(0), Some(0x44ff), 0),
            (" +0x44f4", Some(0), Some(0x44f4), 0),
            (" -0x44fd", Some(0), Some(0x44fd), 0),
            ("\t\n +3", Some(3), Some(3), 0),
            ("\u{b}+4", None, None, 0),
            ("\u{c}+5", None, None, 0),
            ("\r+6", None, None, 0),
            ("", None, None, 0),
            ("-", None, None, 0),
            ("0x", Some(0), None, 0),
            ("0xg", Some(0), None, 0),
        ];
        for (input, dec, hex, to_int) in rows {
            let s = w(input);
            let dec_got = str_to_int64_ex(&s, 0);
            let hex_got = str_to_int64_ex(&s, STIF_SUPPORT_HEX);
            assert_eq!(dec_got, dec, "decimal {input:?}");
            assert_eq!(hex_got, hex, "hex {input:?}");
            assert_eq!(str_to_int(&s), to_int, "StrToIntW {input:?}");
        }
    }

    // The alphabet and `StrRChrW` vectors follow test_StrRChrW in
    // dlls/shlwapi/tests/string.c. The bounded and empty cases are derived from
    // the StrRChrW body in dlls/kernelbase/string.c.
    #[test]
    fn rchr_finds_last_match_before_bound() {
        let mut alphabet: Vec<u16> = (32u16..128).collect();
        alphabet.push(0);
        for ch in 32u16..128 {
            assert_eq!(str_rchr(&alphabet, None, ch), Some(usize::from(ch - 32)));
        }
        assert_eq!(str_rchr(&alphabet, None, 0), None);

        assert_eq!(str_rchr(&w("abcabc"), None, b'a' as u16), Some(3));
        assert_eq!(str_rchr(&w("abcabc"), Some(3), b'a' as u16), Some(0));
        assert_eq!(str_rchr(&w("abcabc"), Some(0), b'a' as u16), None);
        assert_eq!(str_rchr(&w("abcabc"), Some(4), b'b' as u16), Some(1));
        assert_eq!(str_rchr(&w("abc"), None, 0), None);
        assert_eq!(str_rchr(&w("abc"), Some(100), 0), Some(3));
        assert_eq!(str_rchr(&w(""), None, b'a' as u16), None);
    }

    // Expected results are derived by hand from the StrTrimW body in
    // dlls/kernelbase/string.c. Wine's own tests do not cover StrTrimW.
    #[test]
    fn trim_removes_leading_and_trailing_members() {
        let cases: [(&str, &str, &str, bool); 7] = [
            ("  hello  ", " ", "hello", true),
            ("hello", " ", "hello", false),
            ("xxhixx", "x", "hi", true),
            ("   ", " ", "", true),
            ("a\tb\n", "\t\n", "a\tb", true),
            ("--a", "-", "a", true),
            ("  a", "", "  a", false),
        ];
        for (input, trim, expected, changed) in cases {
            let mut s = w(input);
            assert_eq!(str_trim(&mut s, &w(trim)), changed, "{input:?}");
            let text = &s[..wlen(&s)];
            let want: Vec<u16> = expected.encode_utf16().collect();
            assert_eq!(text, &want[..], "{input:?}");
        }
    }

    #[test]
    fn str_to_int_ex_truncates_to_32_bits_and_keeps_failure_visible() {
        assert_eq!(str_to_int_ex(&w("4294967319"), 0), Some(23));
        assert_eq!(str_to_int_ex(&w("-55"), 0), Some(-55));
        assert_eq!(str_to_int_ex(&w("x"), 0), None);
    }

    #[test]
    fn str_to_int_ignores_non_ascii_digits() {
        assert_eq!(str_to_int64_ex(&[0x0661, 0x0662, 0], 0), None);
        assert_eq!(str_to_int64_ex(&[0x07c3, 0x07c4, 0], 0), None);
        assert_eq!(
            str_to_int64_ex(&[0x00a0, b'-' as u16, b'2' as u16, 0], 0),
            None
        );
    }

    #[test]
    fn str_to_int_stops_at_the_terminator() {
        let s = [b'4' as u16, b'2' as u16, 0, b'9' as u16];
        assert_eq!(str_to_int64_ex(&s, 0), Some(42));
    }

    #[test]
    fn str_to_int64_wraps_on_overflow() {
        assert_eq!(
            str_to_int64_ex(&w("9223372036854775808"), 0),
            Some(i64::MIN)
        );
    }

    // Vectors come from dlls/shlwapi/tests/string.c (test_StrCpyNXW and
    // test_StrCatChainW). Truncation and null-argument cases follow the
    // StrCpyNXW and StrCatChainW bodies in dlls/kernelbase/string.c.
    #[test]
    fn cpy_nx_copies_at_most_count_minus_one_units() {
        let mut dest = [u16::from(b'\n'); 8];
        assert_eq!(str_cpy_nx(&mut dest, Some(&w("hello")), 8), 5);
        let want: Vec<u16> = "hello\0\n\n".encode_utf16().collect();
        assert_eq!(dest[..], want[..]);

        let mut small = [0xffffu16; 4];
        assert_eq!(str_cpy_nx(&mut small, Some(&w("hello")), 100), 3);
        assert_eq!(
            small,
            [u16::from(b'h'), u16::from(b'e'), u16::from(b'l'), 0]
        );

        let mut one = [0xffffu16; 4];
        assert_eq!(str_cpy_nx(&mut one, Some(&w("hello")), 1), 0);
        assert_eq!(one[0], 0);
        assert_eq!(one[1], 0xffff);
    }

    #[test]
    fn cpy_nx_writes_nothing_for_null_source_or_bad_count() {
        let mut untouched = [0xffffu16; 4];
        assert_eq!(str_cpy_nx(&mut untouched, Some(&w("hello")), 0), 0);
        assert_eq!(str_cpy_nx(&mut untouched, Some(&w("hello")), -3), 0);
        assert_eq!(str_cpy_nx(&mut untouched, None, 4), 0);
        assert_eq!(str_cpy_nx(&mut [], Some(&w("hello")), 4), 0);
        assert_eq!(untouched, [0xffff; 4]);
    }

    #[test]
    fn cat_chain_appends_truncates_and_terminates() {
        let mut buf = [0x11u16; 33];
        buf[..9].copy_from_slice(&w("DeAdBeEf"));
        assert_eq!(str_cat_chain(&mut buf, 0, None, Some(&w("BeEf"))), 8);
        assert_eq!(buf[..9], w("DeAdBeEf")[..]);

        let mut buf = [0x11u16; 33];
        assert_eq!(str_cat_chain(&mut [], 0, Some(0), Some(&w("BeEf"))), 0);
        assert_eq!(str_cat_chain(&mut buf, 32, Some(0), Some(&w("DeAd"))), 4);
        assert_eq!(buf[..5], w("DeAd")[..]);
        assert_eq!(str_cat_chain(&mut buf, 32, None, Some(&w("BeEf"))), 8);
        assert_eq!(buf[..9], w("DeAdBeEf")[..]);

        let mut buf = [0x11u16; 33];
        assert_eq!(str_cat_chain(&mut buf, 32, Some(0), Some(&w("DeAd"))), 4);
        assert_eq!(str_cat_chain(&mut buf, 32, Some(4), Some(&w("BeEf"))), 8);
        assert_eq!(buf[..9], w("DeAdBeEf")[..]);

        let mut buf = [0x11u16; 33];
        assert_eq!(str_cat_chain(&mut buf, 5, Some(0), Some(&w("DeAd"))), 4);
        assert_eq!(buf[..5], w("DeAd")[..]);

        let mut buf = [0x11u16; 33];
        assert_eq!(str_cat_chain(&mut buf, 4, Some(0), Some(&w("DeAd"))), 3);
        assert_eq!(buf[..4], w("DeA")[..]);
        assert_eq!(buf[4], 0x11);
        assert_eq!(str_cat_chain(&mut buf, 4, Some(1), Some(&w("BeEf"))), 3);
        assert_eq!(buf[..4], w("DBe")[..]);
    }

    #[test]
    fn cat_chain_at_bounds_and_null_cat_write_nothing() {
        let mut buf = [0x11u16; 33];
        buf[..9].copy_from_slice(&w("DeAdBeEf"));
        buf[9..14].copy_from_slice(&w("DeAd"));
        assert_eq!(str_cat_chain(&mut buf, 9, Some(8), Some(&w("BeEf"))), 8);
        assert_eq!(str_cat_chain(&mut buf, 9, Some(9), Some(&w("BeEf"))), 8);
        assert_eq!(str_cat_chain(&mut buf, 9, Some(10), Some(&w("BeEf"))), 10);
        assert_eq!(str_cat_chain(&mut buf, 9, Some(2), None), 2);
        assert_eq!(str_cat_chain(&mut buf, 9, Some(9), None), 8);
        assert_eq!(buf[..9], w("DeAdBeEf")[..]);
        assert_eq!(buf[9..14], w("DeAd")[..]);
    }

    // Vectors and the length rule come from dlls/shlwapi/tests/ordinal.c and
    // dlls/shlwapi/tests/clsid.c (IID_Endianness). The no-match cases follow the
    // SHSearchMapInt body in dlls/shlwapi/ordinal.c.
    #[test]
    fn search_map_int_returns_first_matching_value_or_minus_one() {
        let keys = [1, 2, 2];
        let values = [10, 20, 30];
        assert_eq!(sh_search_map_int(&keys, &values, 1), 10);
        assert_eq!(sh_search_map_int(&keys, &values, 2), 20);
        assert_eq!(sh_search_map_int(&keys, &values, 100), -1);
        assert_eq!(sh_search_map_int(&keys[..1], &values, 2), -1);
        assert_eq!(sh_search_map_int(&keys[..0], &values, 1), -1);
        assert_eq!(sh_search_map_int(&[7, 8], &[1], 8), -1);
    }

    fn endianness() -> Uuid {
        Uuid {
            data1: 0x0102_0304,
            data2: 0x0506,
            data3: 0x0708,
            data4: [0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x0A],
        }
    }

    #[test]
    fn string_from_guid_writes_uppercase_braced_text_when_it_fits() {
        let mut dst = [0xffffu16; 256];
        assert_eq!(sh_string_from_guid(&endianness(), &mut dst), Some(39));
        assert_eq!(dst[..39], w("{01020304-0506-0708-090A-0B0C0D0E0F0A}")[..]);
        assert_eq!(dst[39], 0xffff);

        let mut exact = [0u16; 39];
        assert_eq!(sh_string_from_guid(&endianness(), &mut exact), Some(39));
        assert_eq!(exact[0], u16::from(b'{'));
    }

    #[test]
    fn string_from_guid_rejects_short_buffers_without_writing() {
        for len in [0, 38] {
            let mut dst = vec![7u16; len];
            assert_eq!(sh_string_from_guid(&endianness(), &mut dst), None);
            assert!(dst.iter().all(|&c| c == 7));
        }
    }

    #[test]
    fn guid_from_string_parses_wine_endianness_vector() {
        let text = w("{01020304-0506-0708-090A-0B0C0D0E0F0A}");
        assert_eq!(guid_from_string_w(Some(&text)), Some(endianness()));
    }

    #[test]
    fn guid_from_string_accepts_lowercase_and_round_trips_uppercase() {
        let text = w("{abcdef01-2345-6789-abcd-ef0123456789}");
        let guid = guid_from_string_w(Some(&text)).unwrap();
        assert_eq!(
            guid,
            Uuid {
                data1: 0xabcd_ef01,
                data2: 0x2345,
                data3: 0x6789,
                data4: [0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89],
            }
        );
        let mut dst = [0u16; 39];
        assert_eq!(sh_string_from_guid(&guid, &mut dst), Some(39));
        assert_eq!(dst[..], w("{ABCDEF01-2345-6789-ABCD-EF0123456789}")[..]);
    }

    #[test]
    fn guid_from_string_null_is_nil_guid() {
        assert_eq!(guid_from_string_w(None), Some(Uuid::default()));
    }

    // Each entry breaks one check in the guid_from_string body of
    // dlls/combase/combase.c: the brace, each hex field, each dash, and the
    // closing brace or the terminator.
    #[test]
    fn guid_from_string_rejects_each_malformed_field() {
        let cases = [
            "",
            "01020304-0506-0708-090A-0B0C0D0E0F0A}",
            "{0102030G-0506-0708-090A-0B0C0D0E0F0A}",
            "{01020304_0506-0708-090A-0B0C0D0E0F0A}",
            "{01020304-050G-0708-090A-0B0C0D0E0F0A}",
            "{01020304-0506_0708-090A-0B0C0D0E0F0A}",
            "{01020304-0506-070G-090A-0B0C0D0E0F0A}",
            "{01020304-0506-0708_090A-0B0C0D0E0F0A}",
            "{01020304-0506-0708-090A_0B0C0D0E0F0A}",
            "{01020304-0506-0708-G90A-0B0C0D0E0F0A}",
            "{01020304-0506-0708-090A-0B0C0D0E0F0G}",
            "{01020304-0506-0708-090A-0B0C0D0E0F0Ax",
            "{01020304-0506-0708-090A-0B0C0D0E0F0A",
        ];
        for input in cases {
            assert_eq!(guid_from_string_w(Some(&w(input))), None, "{input:?}");
        }
    }

    #[test]
    fn guid_from_string_needs_terminator_and_ignores_text_after_it() {
        let text = w("{01020304-0506-0708-090A-0B0C0D0E0F0A}");
        let mut trailing = text.clone();
        trailing.extend("junk".encode_utf16());
        assert_eq!(guid_from_string_w(Some(&trailing)), Some(endianness()));
        assert_eq!(guid_from_string_w(Some(&text[..38])), None);
    }

    #[test]
    fn path_undecorate_matches_wine_vectors() {
        // Vectors from Wine's dlls/shlwapi/tests/path.c test_PathUndecorate.
        let cases = [
            ("c:\\test\\a[123]", "c:\\test\\a"),
            ("c:\\test\\a[123].txt", "c:\\test\\a.txt"),
            ("c:\\test\\a.txt[123]", "c:\\test\\a.txt[123]"),
            ("c:\\test\\a[123a].txt", "c:\\test\\a[123a].txt"),
            ("c:\\test\\a[a123].txt", "c:\\test\\a[a123].txt"),
            ("c:\\test\\a[12\u{0660}].txt", "c:\\test\\a[12\u{0660}].txt"),
            ("c:\\test\\a[12]file", "c:\\test\\a[12]file"),
            ("c:\\test[123]\\a", "c:\\test[123]\\a"),
            ("c:\\test\\[123]", "c:\\test\\[123]"),
            ("a[123]", "a"),
            ("a[]", "a"),
            ("[123]", "[123]"),
        ];
        for (input, expected) in cases {
            let mut path = w(input);
            path_undecorate(&mut path);
            assert_eq!(path, w(expected), "{input:?}");
        }
    }

    #[test]
    fn path_undecorate_ignores_inputs_without_a_closing_bracket_or_digits() {
        // Hand-derived from the C body: empty and dot-led paths return early, a
        // lone `]` never reaches index 0, and a file name ending in `]` keeps its
        // text when the bracket is not preceded by digits.
        let cases = [
            ("", ""),
            (".txt", ".txt"),
            ("]", "]"),
            ("name]", "name]"),
            ("file.txt", "file.txt"),
            ("x[]", "x"),
            ("file[1]", "file"),
            ("file[1].txt", "file.txt"),
            ("[12].txt", "[12].txt"),
            ("a\\[1].txt", "a\\[1].txt"),
        ];
        for (input, expected) in cases {
            let mut path = w(input);
            path_undecorate(&mut path);
            assert_eq!(path, w(expected), "{input:?}");
        }
    }

    #[test]
    fn path_is_network_path_checks_unc_then_drive_type() {
        let remote_d = |drive: i32| drive == 3;
        assert!(!path_is_network_path(None, remote_d));
        assert!(path_is_network_path(
            Some(&w("\\\\server\\share")),
            remote_d
        ));
        assert!(path_is_network_path(Some(&w("D:\\data")), remote_d));
        assert!(!path_is_network_path(Some(&w("C:\\data")), remote_d));
        assert!(!path_is_network_path(Some(&w("relative\\path")), remote_d));
        assert!(!path_is_network_path(Some(&w("\\x")), remote_d));
        assert!(!path_is_network_path(Some(&w("")), remote_d));
    }

    #[test]
    fn fix_slashes_and_colon_rewrites_slashes_before_nul_only() {
        let mut path = w("C:/dir/sub");
        fix_slashes_and_colon(&mut path);
        assert_eq!(path, w("C:\\dir\\sub"));

        let mut tail = [u16::from(b'/'), 0, u16::from(b'/')];
        fix_slashes_and_colon(&mut tail);
        assert_eq!(tail, [u16::from(b'\\'), 0, u16::from(b'/')]);
    }

    #[test]
    fn str_cat_buff_counts_existing_text_against_buffer() {
        let mut buf = [0u16; 8];
        assert!(str_cpy(&mut buf, &w("abc")));
        str_cat_buff(Some(&mut buf), &w("defgh"), 8);
        assert_eq!(buf[..], w("abcdefg")[..]);

        let mut full = w("abcdefg");
        str_cat_buff(Some(&mut full), &w("X"), 8);
        assert_eq!(full, w("abcdefg"));

        let mut short = w("abc");
        str_cat_buff(Some(&mut short), &w("X"), 2);
        assert_eq!(short, w("abc"));

        str_cat_buff(None, &w("X"), 8);
    }

    #[test]
    fn str_span_helpers_stop_at_nul_and_handle_null() {
        assert_eq!(str_spn(Some(&w("abc1d")), Some(&w("cba"))), 3);
        assert_eq!(str_spn(Some(&w("xyz")), Some(&w("cba"))), 0);
        assert_eq!(str_spn(Some(&w("")), Some(&w("abc"))), 0);
        assert_eq!(str_spn(None, Some(&w("abc"))), 0);
        assert_eq!(str_spn(Some(&w("abc")), None), 0);
        let accept_after_nul = [u16::from(b'a'), 0, u16::from(b'b')];
        assert_eq!(str_spn(Some(&w("ab")), Some(&accept_after_nul)), 1);

        assert_eq!(str_cspn(Some(&w("abc1d")), Some(&w("1"))), 3);
        assert_eq!(str_cspn(Some(&w("abc")), Some(&w(""))), 3);
        assert_eq!(str_cspn(Some(&w("")), Some(&w("x"))), 0);
        assert_eq!(str_cspn(None, Some(&w("x"))), 0);
        assert_eq!(str_cspn(Some(&w("abc")), None), 0);
        let reject_after_nul = [u16::from(b'x'), 0, u16::from(b'a')];
        assert_eq!(str_cspn(Some(&w("ab")), Some(&reject_after_nul)), 2);

        assert_eq!(str_pbrk(Some(&w("hello")), Some(&w("lo"))), Some(2));
        assert_eq!(str_pbrk(Some(&w("hello")), Some(&w("xyz"))), None);
        assert_eq!(str_pbrk(Some(&w("")), Some(&w("a"))), None);
        assert_eq!(str_pbrk(None, Some(&w("a"))), None);
        assert_eq!(str_pbrk(Some(&w("a")), None), None);
        let break_after_nul = [u16::from(b'b'), 0, u16::from(b'a')];
        assert_eq!(str_pbrk(Some(&w("ab")), Some(&break_after_nul)), Some(1));
    }

    #[test]
    fn str_dup_copies_up_to_nul_and_null_gives_empty() {
        assert_eq!(str_dup(None), vec![0]);
        assert_eq!(str_dup(Some(&w("abc"))), w("abc"));
        assert_eq!(
            str_dup(Some(&[u16::from(b'a'), 0, u16::from(b'b')])),
            w("a")
        );
    }
}
