//! Deterministic part of Wine's `dlls/winhttp/url.c` (commit `e2aa6cd`): `WinHttpCrackUrl` and
//! `WinHttpCreateUrl` over UTF-16 slices.
//!
//! Ported, checked against the verbatim C source by `oracle_checks.rs`:
//! - [`crack`]: `WinHttpCrackUrl`, with `set_component`, `get_scheme`, `parse_port` and the order
//!   in which errors are reported.
//! - [`create`]: `WinHttpCreateUrl`, with `get_scheme`, `get_scheme_string`, `uses_default_port`,
//!   `get_url_length` and `get_comp_length` folded into one assembly step.
//! - [`escape`]: `need_escape`, `escape_string`, `escape_url` and `decode_url`, the `ICU_ESCAPE`
//!   and `ICU_DECODE` transforms.
//!
//! Skipped: the `URL_COMPONENTSW` pointers and the `BOOL` and `SetLastError` return convention.
//! Errors come back as `Err(u32)`, and buffers become slices, `Vec`s and [`crack::Piece`].
//!
//! Deviations, each where Wine is undefined and the sanitizers flag it (X01 to X03 in
//! `oracle_checks.rs`):
//! - A capacity equal to the URL length is `Insufficient`. Wine writes the terminator one unit
//!   past the buffer.
//! - A NULL user or password copied into a buffer slot with no userinfo is an empty component.
//!   Wine passes NULL to `memcpy`.
//! - An unknown scheme number with no scheme text is `ERROR_INVALID_PARAMETER`. Wine
//!   dereferences NULL.
//! - A NULL source is treated as empty, and decoding and searching stop at the end of the text.

pub mod crack;
pub mod create;
pub mod escape;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub const ERROR_SUCCESS: u32 = 0;
pub const ERROR_INVALID_PARAMETER: u32 = 87;
pub const ERROR_INSUFFICIENT_BUFFER: u32 = 122;
pub const ERROR_WINHTTP_INVALID_URL: u32 = 12005;
pub const ERROR_WINHTTP_UNRECOGNIZED_SCHEME: u32 = 12006;

pub const ICU_ESCAPE: u32 = 0x8000_0000;
pub const ICU_DECODE: u32 = 0x1000_0000;

pub const INTERNET_SCHEME_HTTP: u32 = 1;
pub const INTERNET_SCHEME_HTTPS: u32 = 2;
pub const INTERNET_DEFAULT_HTTP_PORT: u16 = 80;
pub const INTERNET_DEFAULT_HTTPS_PORT: u16 = 443;

const COLON: u16 = b':' as u16;
const SLASH: u16 = b'/' as u16;
const AT: u16 = b'@' as u16;
const QUESTION: u16 = b'?' as u16;

/// The unit at `index`, or NUL past the end, which is the terminator the C code reads.
fn at(text: &[u16], index: usize) -> u16 {
    text.get(index).copied().unwrap_or(0)
}

/// The part of `text` before its first NUL, as `wcschr` and `wcsrchr` see a C string.
fn c_str(text: &[u16]) -> &[u16] {
    let end = text
        .iter()
        .position(|&unit| unit == 0)
        .unwrap_or(text.len());
    &text[..end]
}

#[cfg(test)]
fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}
