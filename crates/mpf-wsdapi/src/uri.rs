//! Port of `trim_trailing_slash` and `build_uri_list` from Wine's
//! `dlls/wsdapi/soap.c`: the URI helpers of the WS-Discovery messages.

use crate::{until_nul, Error};

/// `trim_trailing_slash`: the URI up to its first NUL, without one trailing `/`.
pub fn trim_trailing_slash(uri: &[u16]) -> &[u16] {
    let uri = until_nul(uri);
    match uri {
        [head @ .., last] if *last == u16::from(b'/') => head,
        _ => uri,
    }
}

/// `build_uri_list`: joins `uris` with single spaces and ends the result with a
/// NUL, which is the buffer that Wine fills.
///
/// `buffer_size` is the count that Wine passes, and Wine compares it in UTF-16
/// units. A URI therefore needs `2 * (len + 1)` units of room after the text so
/// far, which is more than it writes. An empty list fails with `InvalidArg`.
pub fn build_uri_list(uris: &[&[u16]], buffer_size: usize) -> Result<Vec<u16>, Error> {
    if uris.is_empty() {
        return Err(Error::InvalidArg);
    }

    let mut out = Vec::new();
    for (i, uri) in uris.iter().enumerate() {
        let uri = until_nul(uri);
        if out.len() + 2 * (uri.len() + 1) > buffer_size {
            return Err(Error::InvalidArg);
        }
        if i > 0 {
            out.push(u16::from(b' '));
        }
        out.extend_from_slice(uri);
    }
    out.push(0);
    Ok(out)
}
