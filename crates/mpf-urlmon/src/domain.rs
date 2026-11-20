//! `find_domain_name` from Wine's `dlls/urlmon/uri.c`.

use crate::{fold, DOT};

const RECOGNIZED_TLDS: [&str; 7] = ["com", "edu", "gov", "int", "mil", "net", "org"];

/// Finds the offset in `host` where the domain name starts, or `None` when the
/// host has no domain name. The domain name runs to the end of `host`.
///
/// The two-label checks of `uri.c` need a `.` followed by a three-character TLD,
/// so hosts shorter than four code units have no domain name.
pub fn find_domain_name(host: &[u16]) -> Option<usize> {
    if host.len() < 4 {
        return None;
    }
    let mut last = None;
    let mut sec_last = None;
    for (i, &c) in host.iter().enumerate() {
        if c == DOT {
            sec_last = last;
            last = Some(i);
        }
    }
    let last = last?;
    let Some(sec_last) = sec_last else {
        return single_dot_domain(host, last);
    };

    let after_sec = sec_last + 1;
    if host.len() - last > 3 {
        Some(after_sec)
    } else if last - after_sec < 4 {
        if last - after_sec == 3 {
            if is_recognized_tld(&host[after_sec..last]) {
                Some(label_start(host, sec_last))
            } else {
                Some(after_sec)
            }
        } else {
            Some(label_start(host, sec_last))
        }
    } else {
        Some(after_sec)
    }
}

/// The host has one `.`, at `last`. Wine's check that a leading `.` needs a
/// three-character TLD is unreachable once the host is at least four code units
/// long, so it is not ported.
fn single_dot_domain(host: &[u16], last: usize) -> Option<usize> {
    if last == 3 && is_recognized_tld(&host[..3]) {
        return None;
    }
    if (1..3).contains(&last) && host[..last].iter().all(|&c| c < 0x80) {
        return None;
    }
    Some(0)
}

/// Whether the three code units `name` are a recognized TLD, ignoring ASCII case.
fn is_recognized_tld(name: &[u16]) -> bool {
    RECOGNIZED_TLDS.iter().any(|tld| {
        name.iter()
            .zip(tld.bytes())
            .all(|(&c, b)| fold(c) == fold(u16::from(b)))
    })
}

/// The start of the label that ends at the `.` at `dot`, or 0 when no `.` comes before it.
fn label_start(host: &[u16], dot: usize) -> usize {
    let mut p = dot;
    while p > 0 && host[p - 1] != DOT {
        p -= 1;
    }
    p
}
