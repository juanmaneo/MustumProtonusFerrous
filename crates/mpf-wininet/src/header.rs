//! Header checks from Wine's `dlls/wininet/http.c`: whether a response keeps the connection open,
//! and the token list that `Connection` values use.

use crate::wide::{eq_ci, until_nul};

/// `has_keep_alive`: whether the connection stays open after a response of HTTP version
/// `version`, given the `Connection` header value `connection`. HTTP/1.1 is persistent unless the
/// header lists `Close`. Any other version is persistent only when the header lists `Keep-Alive`.
pub fn has_keep_alive(version: &[u16], connection: &[u16]) -> bool {
    if eq_ci(until_nul(version), "HTTP/1.1") {
        !has_token(connection, "Close")
    } else {
        has_token(connection, "Keep-Alive")
    }
}

/// `has_token`: whether `token` is one of the words of `list`. Words are separated by spaces or
/// commas, and the comparison ignores case.
fn has_token(list: &[u16], token: &str) -> bool {
    until_nul(list)
        .split(|&c| c == u16::from(b' ') || c == u16::from(b','))
        .any(|word| eq_ci(word, token))
}
