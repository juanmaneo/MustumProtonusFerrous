//! The domain name search of `uri.c`, which finds where the domain name starts
//! in a host name.

use crate::chars::is_ascii;
use crate::{eq_ignore_ascii_case, DOT};

const RECOGNIZED_TLDS: [&str; 7] = ["com", "edu", "gov", "int", "mil", "net", "org"];

/// The index in `host` where its domain name starts, or -1 when it has none.
pub fn find_domain_name(host: &[u16]) -> i32 {
    domain_start(host).map_or(-1, |start| start as i32)
}

fn domain_start(host: &[u16]) -> Option<usize> {
    let len = host.len();
    if len < 4 {
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
    let Some(sec) = sec_last else {
        return single_dot_domain(host, last);
    };
    let after = sec + 1;
    if len - last > 3 {
        Some(after)
    } else if last - after == 3 {
        if is_recognized_tld(&host[after..last]) {
            Some(label_start(host, sec))
        } else {
            Some(after)
        }
    } else if last - after < 4 {
        Some(label_start(host, sec))
    } else {
        Some(after)
    }
}

/// The domain for a host with one dot, which is at `last`. A short first label
/// is part of the TLD, and so is a recognized three-letter first label.
fn single_dot_domain(host: &[u16], last: usize) -> Option<usize> {
    let short_tld = (1..3).contains(&last) && host[..last].iter().all(|&c| is_ascii(c));
    let recognized = last == 3 && is_recognized_tld(&host[..3]);
    if short_tld || recognized {
        None
    } else {
        Some(0)
    }
}

/// The index of the first unit of the label that ends at the dot `dot`.
fn label_start(host: &[u16], dot: usize) -> usize {
    let mut p = dot;
    while p > 0 && host[p - 1] != DOT {
        p -= 1;
    }
    p
}

fn is_recognized_tld(name: &[u16]) -> bool {
    RECOGNIZED_TLDS
        .iter()
        .any(|&tld| eq_ignore_ascii_case(tld, name))
}
