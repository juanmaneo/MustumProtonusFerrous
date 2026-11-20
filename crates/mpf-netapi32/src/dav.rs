//! WebDAV path conversion from Wine's `netapi32.c`: `\\server@port\path` to `http://` URLs and
//! back.
//!
//! Each function returns the converted text with its terminating NUL, as the C code writes it. A
//! `capacity` below the length needed (the NUL included) gives `InsufficientBuffer` with the
//! length needed, as the C code reports it in `*buflen`.

use crate::wide::{at, eq_ci, is_ascii_digit, until_nul, wcstol_dec, AT, BACKSLASH, COLON, SLASH};
use crate::{ERROR_BAD_NET_NAME, ERROR_INSUFFICIENT_BUFFER, ERROR_INVALID_PARAMETER};

/// The failures of the two conversions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DavError {
    InvalidParameter,
    BadNetName,
    InsufficientBuffer { required: usize },
}

impl DavError {
    /// The Win32 error code the C function returns for this failure.
    pub const fn win32_code(self) -> u32 {
        match self {
            Self::InvalidParameter => ERROR_INVALID_PARAMETER,
            Self::BadNetName => ERROR_BAD_NET_NAME,
            Self::InsufficientBuffer { .. } => ERROR_INSUFFICIENT_BUFFER,
        }
    }
}

/// `DavGetHTTPFromUNCPath`: converts `\\server[@[SSL][@]port]\path` to `http://` or `https://`.
/// The port is kept unless it is the default for the scheme. Backslashes in the path become
/// slashes, and one trailing separator is dropped.
pub fn http_from_unc(unc_path: &[u16], capacity: usize) -> Result<Vec<u16>, DavError> {
    let unc = until_nul(unc_path);
    if at(unc, 0) != BACKSLASH || at(unc, 1) != BACKSLASH || at(unc, 2) == 0 {
        return Err(DavError::InvalidParameter);
    }
    let server_end = segment_end(unc, 2);
    let server = &unc[2..server_end];
    let mut q = server_end;
    let mut https = false;
    let mut port = 0;
    if at(unc, q) == AT {
        let start = q + 1;
        let end = segment_end(unc, start);
        if eq_ci(&unc[start..end], "SSL") {
            https = true;
            q = end;
        } else {
            (port, q) = port_at(unc, start)?;
        }
    }
    if at(unc, q) == AT {
        (port, q) = port_at(unc, q + 1)?;
    }
    if matches!(at(unc, q), BACKSLASH | SLASH) {
        q += 1;
    }
    let path = match &unc[q..] {
        [rest @ .., BACKSLASH | SLASH] => rest,
        path => path,
    };

    let (scheme, default_port) = if https {
        ("https://", 443)
    } else {
        ("http://", 80)
    };
    let mut out: Vec<u16> = scheme.encode_utf16().collect();
    out.extend_from_slice(server);
    if port != 0 && port != default_port {
        out.extend(format!(":{port}").encode_utf16());
    }
    if !path.is_empty() {
        out.push(SLASH);
        out.extend(path.iter().map(|&c| if c == BACKSLASH { SLASH } else { c }));
    }
    let required = out.len() + 1;
    if capacity < required {
        return Err(DavError::InsufficientBuffer { required });
    }
    out.push(0);
    Ok(out)
}

/// `DavGetUNCFromHTTPPath`: converts an `http://` or `https://` URL to `\\server@SSL@port\path`.
/// The port is kept unless it is the default for the scheme (80, or 443 for `https`). The `@SSL`
/// marker is written for `https`, and the path has its slashes turned into backslashes.
pub fn unc_from_http(http_path: &[u16], capacity: usize) -> Result<Vec<u16>, DavError> {
    let http = until_nul(http_path);
    let mut p = 0;
    while !matches!(at(http, p), 0 | COLON) {
        p += 1;
    }
    let scheme = &http[..p];
    let ssl = if eq_ci(scheme, "http") {
        false
    } else if eq_ci(scheme, "https") {
        true
    } else {
        return Err(DavError::InvalidParameter);
    };
    if at(http, p) != COLON || at(http, p + 1) != SLASH || at(http, p + 2) != SLASH {
        return Err(DavError::InvalidParameter);
    }
    let server_start = p + 3;
    let mut q = server_start;
    while !matches!(at(http, q), 0 | COLON | SLASH) {
        q += 1;
    }
    let server = &http[server_start..q];
    if server.is_empty() {
        return Err(DavError::BadNetName);
    }
    let mut port = None;
    let mut path_start = q;
    if at(http, q) == COLON {
        let digits_start = q + 1;
        let mut end = digits_start;
        while is_ascii_digit(at(http, end)) {
            end += 1;
        }
        let digits = &http[digits_start..end];
        let default_port = if ssl { "443" } else { "80" };
        if !eq_ci(digits, default_port) {
            port = Some(digits);
        }
        path_start = end;
    }
    let path = &http[path_start..];
    if path.windows(2).any(|pair| matches!(pair, [SLASH, SLASH])) {
        return Err(DavError::BadNetName);
    }
    let path = match path {
        [rest @ .., SLASH] => rest,
        path => path,
    };

    let mut out = vec![BACKSLASH, BACKSLASH];
    out.extend_from_slice(server);
    if ssl {
        out.extend("@SSL".encode_utf16());
    }
    if let Some(digits) = port {
        out.push(AT);
        out.extend_from_slice(digits);
    }
    out.extend("\\DavWWWRoot".encode_utf16());
    out.extend(path.iter().map(|&c| if c == SLASH { BACKSLASH } else { c }));
    let required = out.len() + 1;
    if capacity < required {
        return Err(DavError::InsufficientBuffer { required });
    }
    out.push(0);
    Ok(out)
}

/// The index of the first `\`, `/` or `@` at or after `start`, or the end of `s`.
fn segment_end(s: &[u16], start: usize) -> usize {
    let mut end = start;
    while !matches!(at(s, end), 0 | BACKSLASH | SLASH | AT) {
        end += 1;
    }
    end
}

/// The port number that `wcstol` reads at `start`, and the index after its digits. A port of 0,
/// or no digits at all, is an invalid parameter.
fn port_at(s: &[u16], start: usize) -> Result<(u32, usize), DavError> {
    match wcstol_dec(&s[start..]) {
        (0, _) => Err(DavError::InvalidParameter),
        (value, len) => Ok((value as u32, start + len)),
    }
}
