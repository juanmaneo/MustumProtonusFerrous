//! `matches_domain_pattern` from Wine's `dlls/urlmon/sec_mgr.c`.

use crate::{fold_eq, wide_str, DOT, STAR};

/// Matches `host` against a domain `pattern` and returns the offset in `host`
/// where the matched part starts, or `None` when it does not match.
///
/// A pattern that starts with `*.` matches only hosts whose extra labels are
/// absent, unless `implicit_wildcard` is set. Any other pattern matches a host
/// that ends with `.` plus the pattern if `implicit_wildcard` is set, and
/// otherwise only an identical host. Both strings are read up to their first NUL.
pub fn matches_domain_pattern(
    pattern: &[u16],
    host: &[u16],
    implicit_wildcard: bool,
) -> Option<usize> {
    let pattern = wide_str(pattern);
    let host = wide_str(host);
    let (pattern_len, host_len) = (pattern.len(), host.len());
    if host_len < pattern_len {
        return None;
    }

    if pattern.first() == Some(&STAR) && pattern.get(1) == Some(&DOT) {
        if host_len > pattern_len && fold_eq(&host[host_len - pattern_len + 1..], &pattern[1..]) {
            let mut start = host_len - pattern_len;
            while start > 0 && host[start - 1] != DOT {
                start -= 1;
            }
            if start == 0 || implicit_wildcard {
                return Some(start);
            }
        }
        None
    } else if implicit_wildcard && host_len > pattern_len {
        if host[host_len - pattern_len - 1] == DOT
            && fold_eq(&host[host_len - pattern_len..], pattern)
        {
            return Some(host_len - pattern_len);
        }
        None
    } else if fold_eq(host, pattern) {
        Some(0)
    } else {
        None
    }
}
