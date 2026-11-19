//! Proxy-bypass domain matching from Wine's `dlls/wininet/http.c` (`HTTP_DomainMatches`).

use crate::wide::{eq_ci, find, nicmp_eq, until_nul};

/// Whether the bypass entry `domain` matches `server`. An entry is `<local>` (a host with no dot),
/// a host name compared without case, or a `*.suffix` wildcard. A wildcard matches a host with at
/// least one label before the suffix, so `*.a.b` matches `www.foo.a.b` but not `a.b`.
pub fn domain_matches(server: &[u16], domain: &[u16]) -> bool {
    let server = until_nul(server);
    if eq_ci(domain, "<local>") && find(server, b'.').is_none() {
        return true;
    }
    if !domain.is_empty() && domain[0] != u16::from(b'*') {
        return nicmp_eq(server, domain);
    }
    if domain.get(1) != Some(&u16::from(b'.')) {
        return false;
    }
    let Some(first_dot) = find(server, b'.') else {
        return false;
    };
    let suffix = &domain[2..];
    if server.len() - (first_dot + 1) < suffix.len() {
        return false;
    }
    let start = server.len() - suffix.len();
    server[start - 1] == u16::from(b'.') && nicmp_eq(&server[start..], suffix)
}
