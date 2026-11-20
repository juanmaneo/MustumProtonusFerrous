//! Port of the deterministic logic in Wine's `dlls/urlmon`: the domain name
//! search of `uri.c` ([`domain`]), the schema split of `internet.c`
//! ([`schema`]), the domain pattern match of `sec_mgr.c` ([`pattern`]), the
//! zone check of `zone_id.c` ([`zone`]) and the hierarchical scheme list of
//! `sec_mgr.c` ([`scheme`]).
//!
//! Skipped: the IUri, IUriBuilder, binding, protocol, session, zone manager
//! and security manager objects, the registry reads, the COM interfaces and
//! their marshalling, the file, HTTP, FTP and gopher protocols, the MIME
//! filter and the feature-flag lookups in `internet.c`, which read the
//! registry. They call the OS, the registry, the network or COM.
//!
//! Deviations: strings are UTF-16 code units (`&[u16]`). Case-insensitive
//! comparisons fold ASCII letters only, while Wine's `wcsicmp` and
//! `CompareStringW` also fold non-ASCII letters. The pattern and schema
//! functions stop at the first NUL, as `lstrlenW` and `wcschr` do.
//! `find_domain_name` takes the host length as the slice length, as Wine does.
//! `parse_schema` ignores its `flags`, which Wine only logs.

pub mod domain;
pub mod pattern;
pub mod schema;
pub mod scheme;
pub mod zone;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

const DOT: u16 = b'.' as u16;
const STAR: u16 = b'*' as u16;

/// Folds an ASCII upper-case letter to lower case, as `wcsicmp` does.
fn fold(c: u16) -> u16 {
    if (u16::from(b'A')..=u16::from(b'Z')).contains(&c) {
        c + 32
    } else {
        c
    }
}

/// Compares two strings of code units ignoring ASCII case, as `wcsicmp` does
/// for two NUL-terminated strings of the same length.
fn fold_eq(a: &[u16], b: &[u16]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(&x, &y)| fold(x) == fold(y))
}

/// The prefix of `s` before its first NUL, as `lstrlenW` measures it.
fn wide_str(s: &[u16]) -> &[u16] {
    match s.iter().position(|&c| c == 0) {
        Some(n) => &s[..n],
        None => s,
    }
}
