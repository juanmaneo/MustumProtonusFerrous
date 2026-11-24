//! Import allow-list from Wine's `check_context_type` in `dlls/cryptui/main.c`.
//!
//! Wine also sets `E_INVALIDARG` when the check fails; that side effect belongs
//! to the caller.

pub const CRYPTUI_WIZ_IMPORT_ALLOW_CERT: u32 = 0x0002_0000;
pub const CRYPTUI_WIZ_IMPORT_ALLOW_CRL: u32 = 0x0004_0000;
pub const CRYPTUI_WIZ_IMPORT_ALLOW_CTL: u32 = 0x0008_0000;

pub const CERT_QUERY_CONTENT_CERT: u32 = 1;
pub const CERT_QUERY_CONTENT_CTL: u32 = 2;
pub const CERT_QUERY_CONTENT_CRL: u32 = 3;
pub const CERT_QUERY_CONTENT_SERIALIZED_CERT: u32 = 5;
pub const CERT_QUERY_CONTENT_SERIALIZED_CTL: u32 = 6;
pub const CERT_QUERY_CONTENT_SERIALIZED_CRL: u32 = 7;

/// Reports whether an import with `flags` accepts content of `content_type`.
/// With no `ALLOW_*` flag set, every content type is accepted; content types
/// that can hold several kinds of object are accepted whenever any flag is set.
pub fn context_type_allowed(flags: u32, content_type: u32) -> bool {
    let allow_mask =
        CRYPTUI_WIZ_IMPORT_ALLOW_CERT | CRYPTUI_WIZ_IMPORT_ALLOW_CRL | CRYPTUI_WIZ_IMPORT_ALLOW_CTL;
    if (flags & allow_mask) == 0 {
        return true;
    }
    match content_type {
        CERT_QUERY_CONTENT_CERT | CERT_QUERY_CONTENT_SERIALIZED_CERT => {
            (flags & CRYPTUI_WIZ_IMPORT_ALLOW_CERT) != 0
        }
        CERT_QUERY_CONTENT_CRL | CERT_QUERY_CONTENT_SERIALIZED_CRL => {
            (flags & CRYPTUI_WIZ_IMPORT_ALLOW_CRL) != 0
        }
        CERT_QUERY_CONTENT_CTL | CERT_QUERY_CONTENT_SERIALIZED_CTL => {
            (flags & CRYPTUI_WIZ_IMPORT_ALLOW_CTL) != 0
        }
        _ => true,
    }
}
