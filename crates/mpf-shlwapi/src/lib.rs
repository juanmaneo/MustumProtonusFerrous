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
const QUOTE: u16 = b'"' as u16;

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

#[cfg(test)]
mod tests {
    use super::*;

    fn w(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
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
}
