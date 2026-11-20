//! The `Uri_CREATE` flags, the `ALLOW_NULL_TERM_*` extras that the parsers take,
//! and the flag checks that run before a URI is parsed.

use crate::has;

pub const URI_CREATE_ALLOW_RELATIVE: u32 = 0x1;
pub const URI_CREATE_ALLOW_IMPLICIT_WILDCARD_SCHEME: u32 = 0x2;
pub const URI_CREATE_ALLOW_IMPLICIT_FILE_SCHEME: u32 = 0x4;
pub const URI_CREATE_NO_CANONICALIZE: u32 = 0x10;
pub const URI_CREATE_FILE_USE_DOS_PATH: u32 = 0x20;
pub const URI_CREATE_DECODE_EXTRA_INFO: u32 = 0x40;
pub const URI_CREATE_NO_DECODE_EXTRA_INFO: u32 = 0x80;
pub const URI_CREATE_CANONICALIZE: u32 = 0x100;
pub const URI_CREATE_CRACK_UNKNOWN_SCHEMES: u32 = 0x200;
pub const URI_CREATE_NO_CRACK_UNKNOWN_SCHEMES: u32 = 0x400;
pub const URI_CREATE_PRE_PROCESS_HTML_URI: u32 = 0x800;
pub const URI_CREATE_NO_PRE_PROCESS_HTML_URI: u32 = 0x1000;
pub const URI_CREATE_IE_SETTINGS: u32 = 0x2000;
pub const URI_CREATE_NO_IE_SETTINGS: u32 = 0x4000;

pub const ALLOW_NULL_TERM_SCHEME: u32 = 0x01;
pub const ALLOW_NULL_TERM_USER_NAME: u32 = 0x02;
pub const ALLOW_NULL_TERM_PASSWORD: u32 = 0x04;

/// Whether `flags` sets both halves of a pair of opposite flags.
pub fn has_invalid_flag_combination(flags: u32) -> bool {
    (has(flags, URI_CREATE_DECODE_EXTRA_INFO) && has(flags, URI_CREATE_NO_DECODE_EXTRA_INFO))
        || (has(flags, URI_CREATE_CANONICALIZE) && has(flags, URI_CREATE_NO_CANONICALIZE))
        || (has(flags, URI_CREATE_CRACK_UNKNOWN_SCHEMES)
            && has(flags, URI_CREATE_NO_CRACK_UNKNOWN_SCHEMES))
        || (has(flags, URI_CREATE_PRE_PROCESS_HTML_URI)
            && has(flags, URI_CREATE_NO_PRE_PROCESS_HTML_URI))
        || (has(flags, URI_CREATE_IE_SETTINGS) && has(flags, URI_CREATE_NO_IE_SETTINGS))
}

/// Adds each default flag unless its opposite is set.
pub fn apply_default_flags(mut flags: u32) -> u32 {
    if !has(flags, URI_CREATE_NO_CANONICALIZE) {
        flags |= URI_CREATE_CANONICALIZE;
    }
    if !has(flags, URI_CREATE_NO_DECODE_EXTRA_INFO) {
        flags |= URI_CREATE_DECODE_EXTRA_INFO;
    }
    if !has(flags, URI_CREATE_NO_CRACK_UNKNOWN_SCHEMES) {
        flags |= URI_CREATE_CRACK_UNKNOWN_SCHEMES;
    }
    if !has(flags, URI_CREATE_NO_PRE_PROCESS_HTML_URI) {
        flags |= URI_CREATE_PRE_PROCESS_HTML_URI;
    }
    if !has(flags, URI_CREATE_IE_SETTINGS) {
        flags |= URI_CREATE_NO_IE_SETTINGS;
    }
    flags
}
