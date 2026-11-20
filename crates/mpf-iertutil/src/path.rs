//! The path, query and fragment parsers of `uri.c`, and the searches for dot
//! segments and for a file extension.

use crate::chars::{check_pct_encoded, is_forbidden_dos_path_char, is_path_delim, is_slash};
use crate::flags::{
    URI_CREATE_FILE_USE_DOS_PATH, URI_CREATE_NO_CANONICALIZE, URI_CREATE_NO_DECODE_EXTRA_INFO,
};
use crate::scheme::{URL_SCHEME_FILE, URL_SCHEME_MAILTO, URL_SCHEME_UNKNOWN, URL_SCHEME_WILDCARD};
use crate::{at, has, ParseData, BACKSLASH, DOT, HASH, NUL, PERCENT, QUESTION, SLASH};

const ROOT_PATH: &[u16] = &[SLASH];

/// Parses the path of a hierarchical URI at `pos`. An empty path is `/`
/// unless the wildcard scheme needs no path or `NO_CANONICALIZE` is set.
pub fn parse_path_hierarchical<'a>(
    s: &'a [u16],
    pos: &mut usize,
    data: &mut ParseData<'a>,
    flags: u32,
) -> bool {
    let start = *pos;
    let scheme = data.scheme_type;
    let is_file = scheme == URL_SCHEME_FILE;
    if is_path_delim(scheme, at(s, start)) {
        if scheme == URL_SCHEME_WILDCARD && !data.must_have_path {
            data.path = None;
        } else if !has(flags, URI_CREATE_NO_CANONICALIZE) {
            data.path = Some(ROOT_PATH);
        }
        return true;
    }
    let mut p = start;
    while !is_path_delim(scheme, at(s, p)) {
        let c = at(s, p);
        if c == PERCENT && scheme != URL_SCHEME_UNKNOWN && !is_file {
            if !check_pct_encoded(s, &mut p) {
                return false;
            }
            continue;
        }
        if is_forbidden_dos_path_char(c) && is_file && has(flags, URI_CREATE_FILE_USE_DOS_PATH) {
            return false;
        }
        if c == BACKSLASH
            && has(flags, URI_CREATE_NO_CANONICALIZE)
            && scheme != URL_SCHEME_FILE
            && scheme != URL_SCHEME_UNKNOWN
        {
            return false;
        }
        p += 1;
    }
    data.path = Some(&s[start..p]);
    *pos = p;
    true
}

/// Parses the path of an opaque URI at `pos`. For mailto, a leading `//` is
/// not part of the path. On failure `pos` moves to the start of the path, and
/// the path is `None`.
pub fn parse_path_opaque<'a>(
    s: &'a [u16],
    pos: &mut usize,
    data: &mut ParseData<'a>,
    flags: u32,
) -> bool {
    let begin = *pos;
    let scheme = data.scheme_type;
    let known = scheme != URL_SCHEME_UNKNOWN;
    let is_file = scheme == URL_SCHEME_FILE;
    let mailto_slashes =
        scheme == URL_SCHEME_MAILTO && at(s, begin) == SLASH && at(s, begin + 1) == SLASH;
    let path_start = if mailto_slashes { begin + 2 } else { begin };
    let has_path = !mailto_slashes || at(s, begin + 2) != NUL;
    let mut p = begin;
    while !is_path_delim(scheme, at(s, p)) {
        let c = at(s, p);
        if c == PERCENT && known {
            if !check_pct_encoded(s, &mut p) {
                *pos = path_start;
                data.path = None;
                return false;
            }
            continue;
        }
        if is_forbidden_dos_path_char(c) && is_file && has(flags, URI_CREATE_FILE_USE_DOS_PATH) {
            *pos = path_start;
            data.path = None;
            return false;
        }
        p += 1;
    }
    data.path = if has_path {
        Some(&s[path_start..p])
    } else {
        None
    };
    *pos = p;
    true
}

/// Scans a query or fragment from `start`, which holds its introducer, up to
/// NUL or one of `stops`. Returns the end, or `None` on a bad escape.
fn scan_extra(s: &[u16], start: usize, known: bool, flags: u32, stops: &[u16]) -> Option<usize> {
    let decode = known && !has(flags, URI_CREATE_NO_DECODE_EXTRA_INFO);
    let mut p = start + 1;
    while at(s, p) != NUL && !stops.contains(&at(s, p)) {
        if at(s, p) == PERCENT && decode {
            if !check_pct_encoded(s, &mut p) {
                return None;
            }
            continue;
        }
        p += 1;
    }
    Some(p)
}

/// Parses the query at `pos`, which runs from `?` up to `#` or the end.
pub fn parse_query<'a>(
    s: &'a [u16],
    pos: &mut usize,
    data: &mut ParseData<'a>,
    flags: u32,
) -> bool {
    let start = *pos;
    if at(s, start) != QUESTION {
        return true;
    }
    let known = data.scheme_type != URL_SCHEME_UNKNOWN;
    let Some(end) = scan_extra(s, start, known, flags, &[HASH]) else {
        data.query = None;
        return false;
    };
    data.query = Some(&s[start..end]);
    *pos = end;
    true
}

/// Parses the fragment at `pos`, which runs from `#` to the end.
pub fn parse_fragment<'a>(
    s: &'a [u16],
    pos: &mut usize,
    data: &mut ParseData<'a>,
    flags: u32,
) -> bool {
    let start = *pos;
    if at(s, start) != HASH {
        return true;
    }
    let known = data.scheme_type != URL_SCHEME_UNKNOWN;
    let Some(end) = scan_extra(s, start, known, flags, &[]) else {
        data.fragment = None;
        return false;
    };
    data.fragment = Some(&s[start..end]);
    *pos = end;
    true
}

/// Removes the `.` and `..` segments of `path` in place, as RFC 3986 section
/// 5.2.4 describes, and returns the length of the result.
pub fn remove_dot_segments(path: &mut [u16]) -> usize {
    let end = path.len();
    let mut out = 0;
    let mut inp = 0;
    while inp < end {
        while inp < end && !is_slash(path[inp]) {
            path[out] = path[inp];
            out += 1;
            inp += 1;
        }
        if inp == end {
            break;
        }
        path[out] = path[inp];
        out += 1;
        inp += 1;
        while inp < end {
            if path[inp] != DOT {
                break;
            }
            if inp + 1 == end {
                inp += 1;
                break;
            }
            if is_slash(path[inp + 1]) {
                inp += 2;
                continue;
            }
            if path[inp + 1] != DOT || (inp + 2 != end && !is_slash(path[inp + 2])) {
                break;
            }
            // `out > 1` means `path[out - 1]` closes the previous segment, so the
            // C's conditional second step always applies and is folded in here.
            if out > 1 {
                out -= 2;
            }
            while out > 0 {
                out -= 1;
                if is_slash(path[out]) {
                    break;
                }
            }
            if is_slash(path[out]) {
                out += 1;
            }
            inp += 2;
            if inp != end {
                inp += 1;
            }
        }
    }
    out
}

/// The index of the last `.` in the final segment of `path`, or -1.
pub fn find_file_extension(path: &[u16]) -> i32 {
    for (i, &c) in path.iter().enumerate().rev() {
        if is_slash(c) {
            break;
        }
        if c == DOT {
            return i as i32;
        }
    }
    -1
}
