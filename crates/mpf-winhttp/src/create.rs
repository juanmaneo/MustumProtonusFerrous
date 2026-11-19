//! `WinHttpCreateUrl`: joins URL components into one UTF-16 string, as Wine's `url.c` does.

use crate::escape::escape_string;
use crate::{
    AT, COLON, ERROR_INVALID_PARAMETER, ICU_ESCAPE, INTERNET_DEFAULT_HTTPS_PORT,
    INTERNET_DEFAULT_HTTP_PORT, INTERNET_SCHEME_HTTP, INTERNET_SCHEME_HTTPS, SLASH,
};

/// The components of a URL to build. `None` is a NULL pointer, which leaves the part out.
pub struct UrlParts<'a> {
    pub scheme: Option<&'a [u16]>,
    pub n_scheme: u32,
    pub user: Option<&'a [u16]>,
    pub password: Option<&'a [u16]>,
    pub host: Option<&'a [u16]>,
    pub port: u16,
    pub path: Option<&'a [u16]>,
    pub extra: Option<&'a [u16]>,
}

/// Why `create_url` failed. `Insufficient` carries the WCHAR count the buffer needs, terminator
/// included.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateError {
    Win32(u32),
    Insufficient { required: usize },
}

/// `WinHttpCreateUrl`. Wine accepts a `capacity` equal to the URL length and then writes the
/// terminator past the end of the buffer; this port reports `Insufficient` for that case.
pub fn create_url(parts: &UrlParts, flags: u32, capacity: usize) -> Result<Vec<u16>, CreateError> {
    if parts.password.is_some() && parts.user.is_none() {
        return Err(CreateError::Win32(ERROR_INVALID_PARAMETER));
    }
    let default_scheme = match parts.n_scheme {
        0 => INTERNET_SCHEME_HTTP,
        n => n,
    };
    let scheme_text: Vec<u16> = match parts.scheme {
        Some(text) => text.to_vec(),
        None => scheme_string(default_scheme)
            .ok_or(CreateError::Win32(ERROR_INVALID_PARAMETER))?
            .bytes()
            .map(u16::from)
            .collect(),
    };
    let scheme_id = match parts.scheme {
        Some(text) => get_scheme(text),
        None => default_scheme,
    };

    let (url, escapes_ok) = assemble(parts, flags, scheme_id, &scheme_text);
    if capacity <= url.len() {
        return Err(CreateError::Insufficient {
            required: url.len() + 1,
        });
    }
    if !escapes_ok {
        return Err(CreateError::Win32(ERROR_INVALID_PARAMETER));
    }
    Ok(url)
}

/// Lays the URL out. The flag is false when escaping met a unit above 0xff.
fn assemble(parts: &UrlParts, flags: u32, scheme_id: u32, scheme_text: &[u16]) -> (Vec<u16>, bool) {
    let escape = (flags & ICU_ESCAPE) != 0;
    let mut out = Vec::new();
    let mut escapes_ok = true;

    out.extend_from_slice(scheme_text);
    out.extend_from_slice(&[COLON, SLASH, SLASH]);
    if let Some(user) = parts.user {
        out.extend_from_slice(user);
        if let Some(password) = parts.password {
            out.push(COLON);
            out.extend_from_slice(password);
        }
        out.push(AT);
    }
    if let Some(host) = parts.host {
        out.extend_from_slice(host);
        if !uses_default_port(scheme_id, parts.port) {
            out.push(COLON);
            out.extend(parts.port.to_string().bytes().map(u16::from));
        }
        if parts.path.is_some_and(|path| path.first() != Some(&SLASH)) {
            out.push(SLASH);
        }
    }
    if let Some(path) = parts.path {
        escapes_ok &= push_component(&mut out, path, escape);
    }
    if let Some(extra) = parts.extra {
        escapes_ok &= push_component(&mut out, extra, escape);
    }
    (out, escapes_ok)
}

fn push_component(out: &mut Vec<u16>, text: &[u16], escape: bool) -> bool {
    if escape {
        let (escaped, ok) = escape_string(text);
        out.extend(escaped);
        ok
    } else {
        out.extend_from_slice(text);
        true
    }
}

/// `get_scheme`: a case-sensitive test of the text's own length against `http` and `https`.
/// Any prefix of `http` matches, and an empty text matches too.
fn get_scheme(text: &[u16]) -> u32 {
    if prefix_matches(text, b"http") {
        INTERNET_SCHEME_HTTP
    } else if prefix_matches(text, b"https") {
        INTERNET_SCHEME_HTTPS
    } else {
        0
    }
}

/// `wcsncmp(text, word, text.len()) == 0`, where `word` is followed by a NUL.
fn prefix_matches(text: &[u16], word: &[u8]) -> bool {
    for (i, &unit) in text.iter().enumerate() {
        if unit != u16::from(word.get(i).copied().unwrap_or(0)) {
            return false;
        }
        if unit == 0 {
            return true;
        }
    }
    true
}

fn scheme_string(scheme: u32) -> Option<&'static str> {
    match scheme {
        INTERNET_SCHEME_HTTP => Some("http"),
        INTERNET_SCHEME_HTTPS => Some("https"),
        _ => None,
    }
}

fn uses_default_port(scheme: u32, port: u16) -> bool {
    matches!(
        (scheme, port),
        (INTERNET_SCHEME_HTTP, INTERNET_DEFAULT_HTTP_PORT)
            | (INTERNET_SCHEME_HTTPS, INTERNET_DEFAULT_HTTPS_PORT)
    )
}
