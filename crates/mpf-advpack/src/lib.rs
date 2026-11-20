//! Port of the portable helpers in Wine's `dlls/advpack`.
//!
//! Ported (5 helpers, all pure over strings):
//! - `strip_quotes` (`advpack.c`): drops one pair of single quotes from a token.
//! - `get_parameter` (`install.c`): sequential tokenizer over a parameter list.
//! - `is_full_path` (`install.c`): recognizes drive-letter and UNC paths.
//! - `convert_file_list` (`files.c`): normalizes a colon-separated file list.
//! - `file_in_list` (`files.c`): membership test over a double-NUL-terminated list.
//!
//! Skipped: all 81 entries of `advpack.spec`. Each export reads or writes an INF,
//! the registry, files, a cabinet, a window or a process, or is a `FIXME` stub.
//! `get_dest_dir` and `set_ldids` depend on setupapi and the registry.

const QUOTE: u16 = b'\'' as u16;
const DOUBLE_QUOTE: u16 = b'"' as u16;
const COLON: u16 = b':' as u16;
const BACKSLASH: u16 = b'\\' as u16;

/// Strips one pair of single quotes from a NUL-terminated token, as `strip_quotes` does.
///
/// `size` counts the terminating NUL, as the C callers pass it. `buffer` must hold at least
/// `max(*size, 1)` units.
pub fn strip_quotes(buffer: &mut [u16], size: &mut usize) {
    if buffer[0] == QUOTE && *size > 1 && buffer[*size - 2] == QUOTE {
        *size -= 2;
        buffer[*size] = 0;
        buffer.copy_within(1..=*size, 0);
    }
}

/// Returns the next parameter of the list at `params`, as `get_parameter` does.
///
/// `params` is the index of the next unread unit in `buf`, or `None` once the list is
/// exhausted. As in C, consumed quotes and separators are overwritten with NUL, so the
/// returned token is a sub-slice of `buf` up to its NUL. An empty parameter returns `None`.
/// `buf` must be NUL-terminated and `separator` must not be NUL.
pub fn get_parameter<'a>(
    buf: &'a mut [u16],
    params: &mut Option<usize>,
    separator: u16,
    quoted: bool,
) -> Option<&'a [u16]> {
    let start = (*params)?;
    let mut token = start;
    let mut next = start;
    if quoted && buf[start] == DOUBLE_QUOTE {
        if let Some(end) = find_unit(buf, start + 1, DOUBLE_QUOTE) {
            buf[end] = 0;
            token = start + 1;
            next = end + 1;
        }
    }
    *params = find_unit(buf, next, separator).map(|sep| {
        buf[sep] = 0;
        sep + 1
    });
    let len = buf[token..].iter().take_while(|&&unit| unit != 0).count();
    if len == 0 {
        None
    } else {
        Some(&buf[token..token + len])
    }
}

/// Reports whether `path` is a drive-letter (`X:\...`) or UNC (`\\...`) path, as `is_full_path`
/// does.
///
/// `path` is a wide string up to its NUL, or `None` for a NULL pointer. Strings shorter than
/// three units are never full paths.
pub fn is_full_path(path: Option<&[u16]>) -> bool {
    let Some(path) = path else {
        return false;
    };
    let len = path.iter().take_while(|&&unit| unit != 0).count();
    if len < 3 {
        return false;
    }
    (path[1] == COLON && path[2] == BACKSLASH) || (path[0] == BACKSLASH && path[1] == BACKSLASH)
}

/// Converts a colon-separated file list, as `convert_file_list` does.
///
/// `file_list` is the C string without its NUL. Leading and trailing spaces, tabs and colons
/// are trimmed. The result has each remaining colon replaced by NUL, the file count (one more
/// than the number of colons), and two trailing NULs. Returns `None` for an empty list, and
/// also when the trimmed list is a single unit: Wine's `first == last` check rejects it.
pub fn convert_file_list(file_list: &[u8]) -> Option<(Vec<u8>, u32)> {
    if file_list.is_empty() {
        return None;
    }
    let mut first = 0;
    let mut last = file_list.len() - 1;
    while first < last && is_list_trim(file_list[first]) {
        first += 1;
    }
    while last > first && is_list_trim(file_list[last]) {
        last -= 1;
    }
    if first == last {
        return None;
    }
    let mut list = file_list[first..=last].to_vec();
    let mut files = 1;
    for byte in &mut list {
        if *byte == b':' {
            *byte = 0;
            files += 1;
        }
    }
    list.extend_from_slice(&[0, 0]);
    Some((list, files))
}

/// Reports whether `file` is one of the names in `file_list`, as `file_in_list` does.
///
/// `file_list` holds NUL-terminated names and ends at the first empty name. Names match when
/// they are equal ignoring ASCII case. Wine's `lstrcmpiA` also folds non-ASCII bytes per
/// locale, which this port does not reproduce.
pub fn file_in_list(file: &[u8], file_list: &[u8]) -> bool {
    file_list
        .split(|&byte| byte == 0)
        .take_while(|name| !name.is_empty())
        .any(|name| name.eq_ignore_ascii_case(file))
}

fn is_list_trim(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b':')
}

/// Index of the first `wanted` unit in `buf[from..]` before its NUL, like `wcschr`.
fn find_unit(buf: &[u16], from: usize, wanted: u16) -> Option<usize> {
    buf[from..]
        .iter()
        .take_while(|&&unit| unit != 0)
        .position(|&unit| unit == wanted)
        .map(|index| from + index)
}

#[cfg(test)]
pub mod edge_checks;
#[cfg(test)]
pub mod oracle_checks;
