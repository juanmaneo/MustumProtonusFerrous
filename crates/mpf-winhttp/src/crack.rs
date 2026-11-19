//! `WinHttpCrackUrl`: splits a URL into its components the way Wine's `url.c` does, including
//! the order in which errors are reported.

use crate::escape::{decode_url, escape_url};
use crate::{
    at, c_str, AT, COLON, ERROR_INSUFFICIENT_BUFFER, ERROR_INVALID_PARAMETER, ERROR_SUCCESS,
    ERROR_WINHTTP_INVALID_URL, ERROR_WINHTTP_UNRECOGNIZED_SCHEME, ICU_DECODE, ICU_ESCAPE,
    INTERNET_DEFAULT_HTTPS_PORT, INTERNET_DEFAULT_HTTP_PORT, INTERNET_SCHEME_HTTP,
    INTERNET_SCHEME_HTTPS, QUESTION, SLASH,
};

/// How the caller asked for one component: not at all, as a zero-copy view of the input, or as
/// a copy into a buffer that holds `usize` WCHARs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Slot {
    Unrequested,
    View,
    Buffer(usize),
}

impl Slot {
    /// `WinHttpCrackUrl` reads a non-zero length as a request for the component.
    fn requested(self) -> bool {
        !matches!(self, Slot::Unrequested | Slot::Buffer(0))
    }
}

/// The slots for the six components of a URL.
pub struct Slots {
    pub scheme: Slot,
    pub user: Slot,
    pub password: Slot,
    pub host: Slot,
    pub path: Slot,
    pub extra: Slot,
}

/// One cracked component. `Overflow` holds the WCHAR count the buffer needs, terminator included.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Piece {
    Absent,
    Text(Vec<u16>),
    Overflow(usize),
}

/// A cracked URL. `status` reports `ERROR_INSUFFICIENT_BUFFER` when a buffer was too small; the
/// other fields are still filled in, as Wine does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrackedUrl {
    pub n_scheme: u32,
    pub port: u16,
    pub scheme: Piece,
    pub user: Piece,
    pub password: Piece,
    pub host: Piece,
    pub path: Piece,
    pub extra: Piece,
}

impl CrackedUrl {
    pub fn status(&self) -> u32 {
        let pieces = [
            &self.scheme,
            &self.user,
            &self.password,
            &self.host,
            &self.path,
            &self.extra,
        ];
        if pieces
            .iter()
            .any(|piece| matches!(piece, Piece::Overflow(_)))
        {
            ERROR_INSUFFICIENT_BUFFER
        } else {
            ERROR_SUCCESS
        }
    }
}

/// `WinHttpCrackUrl`. An `Err` carries the Win32 error code; an `Ok` may still report
/// `ERROR_INSUFFICIENT_BUFFER` through [`CrackedUrl::status`].
pub fn crack_url(url: &[u16], flags: u32, slots: &Slots) -> Result<CrackedUrl, u32> {
    let transformed: Vec<u16>;
    let url = if (flags & ICU_ESCAPE) != 0 {
        transformed = escape_url(url)?;
        transformed.as_slice()
    } else if (flags & ICU_DECODE) != 0 {
        transformed = decode_url(url);
        transformed.as_slice()
    } else {
        url
    };

    let colon = find(c_str(url), COLON).ok_or(ERROR_WINHTTP_UNRECOGNIZED_SCHEME)?;
    let (n_scheme, default_port) =
        crack_scheme(&url[..colon]).ok_or(ERROR_WINHTTP_UNRECOGNIZED_SCHEME)?;
    let scheme = set_component(slots.scheme, Some(&url[..colon]), flags)?;

    let mut p = colon + 1;
    if at(url, p) != SLASH || at(url, p + 1) != SLASH {
        return Err(ERROR_WINHTTP_INVALID_URL);
    }
    p += 2;
    if at(url, p) == 0 {
        return Err(ERROR_WINHTTP_INVALID_URL);
    }

    let (user, password) = match find(&url[p..], AT) {
        Some(at_rel) if find(&url[p..p + at_rel], SLASH).is_none() => {
            let at_pos = p + at_rel;
            let pair = match find(&url[p..at_pos], COLON) {
                Some(colon_rel) => {
                    let colon_pos = p + colon_rel;
                    let user = set_component(slots.user, Some(&url[p..colon_pos]), flags)?;
                    let password =
                        set_component(slots.password, Some(&url[colon_pos + 1..at_pos]), flags)?;
                    (user, password)
                }
                None => {
                    let user = set_component(slots.user, Some(&url[p..at_pos]), flags)?;
                    (user, set_component(slots.password, None, flags)?)
                }
            };
            p = at_pos + 1;
            pair
        }
        _ => (
            set_component(slots.user, None, flags)?,
            set_component(slots.password, None, flags)?,
        ),
    };

    let (host, port, path, extra) = match find(&url[p..], SLASH) {
        Some(slash_rel) => {
            let slash = p + slash_rel;
            let (host, port) = match find(&url[p..slash], COLON) {
                Some(colon_rel) => {
                    let colon_pos = p + colon_rel;
                    let host = set_component(slots.host, Some(&url[p..colon_pos]), flags)?;
                    let port = if colon_pos + 1 == slash {
                        default_port
                    } else {
                        parse_port(&url[colon_pos + 1..slash])?
                    };
                    (host, port)
                }
                None => (
                    set_component(slots.host, Some(&url[p..slash]), flags)?,
                    default_port,
                ),
            };
            let (path, extra) = match find(&url[slash..], QUESTION) {
                Some(question_rel) if slots.extra.requested() => {
                    let question = slash + question_rel;
                    let path = set_component(slots.path, Some(&url[slash..question]), flags)?;
                    (
                        path,
                        set_component(slots.extra, Some(&url[question..]), flags)?,
                    )
                }
                Some(_) => (
                    set_component(slots.path, Some(&url[slash..]), flags)?,
                    Piece::Absent,
                ),
                None => {
                    let path = set_component(slots.path, Some(&url[slash..]), flags)?;
                    (path, set_component(slots.extra, Some(&[]), flags)?)
                }
            };
            (host, port, path, extra)
        }
        None => {
            let (host, port) = match find(&url[p..], COLON) {
                Some(colon_rel) => {
                    let colon_pos = p + colon_rel;
                    let host = set_component(slots.host, Some(&url[p..colon_pos]), flags)?;
                    let port = if at(url, colon_pos + 1) == 0 {
                        default_port
                    } else {
                        parse_port(&url[colon_pos + 1..])?
                    };
                    (host, port)
                }
                None => (
                    set_component(slots.host, Some(&url[p..]), flags)?,
                    default_port,
                ),
            };
            let path = set_component(slots.path, Some(&[]), flags)?;
            let extra = set_component(slots.extra, Some(&[]), flags)?;
            (host, port, path, extra)
        }
    };

    Ok(CrackedUrl {
        n_scheme,
        port,
        scheme,
        user,
        password,
        host,
        path,
        extra,
    })
}

/// `set_component`: stores one component. `value` is `None` for a NULL pointer.
fn set_component(slot: Slot, value: Option<&[u16]>, flags: u32) -> Result<Piece, u32> {
    match slot {
        Slot::Unrequested => Ok(Piece::Absent),
        Slot::View => match value {
            None => Ok(Piece::Absent),
            Some(text) if !text.is_empty() && (flags & (ICU_DECODE | ICU_ESCAPE)) != 0 => {
                Err(ERROR_INVALID_PARAMETER)
            }
            Some(text) => Ok(Piece::Text(text.to_vec())),
        },
        Slot::Buffer(0) => Err(ERROR_INVALID_PARAMETER),
        Slot::Buffer(capacity) => {
            let text = value.unwrap_or(&[]);
            if text.len() >= capacity {
                Ok(Piece::Overflow(text.len() + 1))
            } else {
                Ok(Piece::Text(text.to_vec()))
            }
        }
    }
}

/// The scheme number and its default port for `http` and `https`, ignoring ASCII case.
fn crack_scheme(text: &[u16]) -> Option<(u32, u16)> {
    if equals_ignore_case(text, b"http") {
        Some((INTERNET_SCHEME_HTTP, INTERNET_DEFAULT_HTTP_PORT))
    } else if equals_ignore_case(text, b"https") {
        Some((INTERNET_SCHEME_HTTPS, INTERNET_DEFAULT_HTTPS_PORT))
    } else {
        None
    }
}

fn equals_ignore_case(text: &[u16], word: &[u8]) -> bool {
    text.len() == word.len()
        && text.iter().zip(word).all(|(&unit, &letter)| {
            unit == u16::from(letter) || unit == u16::from(letter.to_ascii_uppercase())
        })
}

/// `parse_port`: reads leading decimal digits and stops at the first other unit.
fn parse_port(text: &[u16]) -> Result<u16, u32> {
    let mut port: u16 = 0;
    for &unit in text {
        let Some(digit) = decimal_digit(unit) else {
            break;
        };
        port = port
            .checked_mul(10)
            .and_then(|scaled| scaled.checked_add(digit))
            .ok_or(ERROR_WINHTTP_INVALID_URL)?;
    }
    Ok(port)
}

fn decimal_digit(unit: u16) -> Option<u16> {
    match unit {
        0x30..=0x39 => Some(unit - 0x30),
        _ => None,
    }
}

fn find(text: &[u16], unit: u16) -> Option<usize> {
    text.iter().position(|&candidate| candidate == unit)
}
