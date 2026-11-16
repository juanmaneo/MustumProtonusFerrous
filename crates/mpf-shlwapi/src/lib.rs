//! shlwapi path helpers: the string-only subset of Wine's path routines.
//!
//! The shlwapi entry points forward to `dlls/kernelbase/path.c` in reference
//! Wine, and the logic here mirrors those bodies. Strings are NUL-terminated
//! UTF-16 slices, as the Win32 API uses. Functions take `&[u16]` and return
//! indices into that slice instead of raw pointers, so callers never handle
//! pointer arithmetic. Only the subset that needs no filesystem is ported.

/// `MAX_PATH`, the limit `PathAddBackslash` enforces.
pub const MAX_PATH: usize = 260;

const BACKSLASH: u16 = b'\\' as u16;
const SLASH: u16 = b'/' as u16;
const COLON: u16 = b':' as u16;
const DOT: u16 = b'.' as u16;
const SPACE: u16 = b' ' as u16;

/// Length of the UTF-16 string before its first NUL, or the slice length.
fn wlen(s: &[u16]) -> usize {
    s.iter().position(|&c| c == 0).unwrap_or(s.len())
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

#[cfg(test)]
mod tests {
    use super::*;

    fn w(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
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
}
