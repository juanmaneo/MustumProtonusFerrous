//! `is_hierarchical_scheme` from Wine's `dlls/urlmon/sec_mgr.c`, with the
//! `URL_SCHEME` values of `include/shlwapi.h`.

pub const URL_SCHEME_FTP: i32 = 1;
pub const URL_SCHEME_HTTP: i32 = 2;
pub const URL_SCHEME_GOPHER: i32 = 3;
pub const URL_SCHEME_NNTP: i32 = 6;
pub const URL_SCHEME_TELNET: i32 = 7;
pub const URL_SCHEME_WAIS: i32 = 8;
pub const URL_SCHEME_FILE: i32 = 9;
pub const URL_SCHEME_HTTPS: i32 = 11;
pub const URL_SCHEME_RES: i32 = 18;

/// Whether `scheme` is one of the schemes that Wine expects to be hierarchical.
pub fn is_hierarchical_scheme(scheme: i32) -> bool {
    matches!(
        scheme,
        URL_SCHEME_FTP
            | URL_SCHEME_HTTP
            | URL_SCHEME_GOPHER
            | URL_SCHEME_NNTP
            | URL_SCHEME_TELNET
            | URL_SCHEME_WAIS
            | URL_SCHEME_FILE
            | URL_SCHEME_HTTPS
            | URL_SCHEME_RES
    )
}
