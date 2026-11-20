//! Host list helpers from `dlls/wldap32/init.c`, which build the server URL list.
//!
//! Wine cuts a host list at its first NUL, as C strings do. The inputs here are byte slices.

const SCHEMES: [&[u8]; 4] = [b"ldap://", b"ldaps://", b"ldapi://", b"cldap://"];

/// C `isspace` in the "C" locale: space, tab, newline, vertical tab, form feed and carriage return.
fn is_space(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}

/// Splits a whitespace-separated host list, as `split_hostnames` does.
///
/// Runs of whitespace separate names, and leading whitespace is skipped. Wine drops a
/// one-character name that follows a separator, and this port keeps that: `"a b c"` gives
/// `["a", "b"]`.
///
/// Deviation: when the list ends in whitespace, Wine reads past its end, which is undefined. Here
/// a trailing space ends the last name, so `"a "` gives `["a"]`.
pub fn split_hostnames(hostnames: &[u8]) -> Vec<&[u8]> {
    let end = hostnames
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(hostnames.len());
    let list = &hostnames[..end];
    // Past the end, every read gives NUL, as the terminator does in C.
    let at = |i: usize| list.get(i).copied().unwrap_or(0);

    let mut names = Vec::new();
    let mut p = 0;
    while is_space(at(p)) {
        p += 1;
    }
    let mut q = p;
    while at(p) != 0 {
        if at(p + 1) != 0 {
            if is_space(at(p)) {
                names.push(&list[q..p]);
                p += 1;
                while is_space(at(p)) {
                    p += 1;
                }
                q = p;
            }
        } else {
            let name_end = if is_space(at(p)) { p } else { p + 1 };
            names.push(&list[q..name_end]);
        }
        p += 1;
    }
    names
}

/// Tells whether `url` starts with `ldap://`, `ldaps://`, `ldapi://` or `cldap://`, ignoring ASCII
/// case, as `has_ldap_scheme` does.
pub fn has_ldap_scheme(url: &[u8]) -> bool {
    SCHEMES.iter().any(|&scheme| {
        url.get(..scheme.len())
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case(scheme))
    })
}

/// Tells whether a host already has a port, as `join_hostnames` checks. A host with a scheme is
/// checked from its first slash on, so the colon in the scheme does not count.
fn has_port(host: &[u8]) -> bool {
    let rest = if has_ldap_scheme(host) {
        host.iter()
            .position(|&byte| byte == b'/')
            .map_or(host, |slash| &host[slash..])
    } else {
        host
    };
    rest.contains(&b':')
}

/// Joins hosts into a space-separated URL list, as `join_hostnames` does. A host without a scheme
/// gets `scheme` in front, and a host without a port gets `:port` appended.
///
/// Deviations: Wine returns an unterminated zero-byte buffer for an empty host list, which is
/// undefined, so this port returns an empty list. Wine also formats the port into a 7-byte buffer,
/// so a port of 100000 or more overflows it. This port prints every digit.
pub fn join_hostnames(scheme: &[u8], hostnames: &[&[u8]], port: u32) -> Vec<u8> {
    let port = format!(":{port}");
    let mut out = Vec::new();
    for (i, &host) in hostnames.iter().enumerate() {
        if i > 0 {
            out.push(b' ');
        }
        if !has_ldap_scheme(host) {
            out.extend_from_slice(scheme);
        }
        out.extend_from_slice(host);
        if !has_port(host) {
            out.extend_from_slice(port.as_bytes());
        }
    }
    out
}

/// Splits `hostnames` and joins the names into a URL list, as `urlify_hostnames` does.
pub fn urlify_hostnames(scheme: &[u8], hostnames: &[u8], port: u32) -> Vec<u8> {
    join_hostnames(scheme, &split_hostnames(hostnames), port)
}
