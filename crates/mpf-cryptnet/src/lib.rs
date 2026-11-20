//! Port of the portable logic in Wine's `dlls/cryptnet/cryptnet_main.c`.
//!
//! Ported:
//! - `url_oid_to_str`: [`url_oid_to_str`]
//! - `escape_path`: [`escape_path`]
//! - `map_ocsp_status`: [`map_ocsp_status`]
//!
//! Skipped:
//! - `CRYPT_CrackUrl`: calls `InternetCrackUrlW` (wininet), needs the OS.
//! - `decode_base64_blob`: calls `CryptStringToBinaryA` (crypt32), needs the OS.
//! - `build_request_path` / `build_request_url`: call `CryptBinaryToStringW` (crypt32), needs the OS.
//! - `match_cert_id`: calls `sha1_hash`, which uses CryptoAPI, needs the OS.
//! - `CRYPT_CreateBlob`: calls `CryptMemAlloc`, needs the allocator.
//! - `CRYPT_GetExpiration`: reads CERT_CONTEXT / CRL_CONTEXT / CTL_CONTEXT fields, needs the OS types.
//! - All exports needing OS, registry, files, network, COM or handles.

/// OID passed either as an integer (IS_INTOID) or as a string pointer.
#[derive(Clone)]
pub enum Oid<'a> {
    /// Integer OID, passed as a pointer whose value fits in 16 bits.
    Int(u16),
    /// String OID pointer, returned verbatim.
    Str(&'a str),
}

/// `url_oid_to_str`: maps an integer OID to its name string, or formats it
/// as decimal (`snprintf(buf, "%d", LOWORD(oid))` in the C default arm).
/// String OIDs are returned verbatim.
pub fn url_oid_to_str(oid: Oid<'_>) -> String {
    match oid {
        Oid::Int(val) => match val {
            1 => "URL_OID_CERTIFICATE_ISSUER".into(),
            2 => "URL_OID_CERTIFICATE_CRL_DIST_POINT".into(),
            3 => "URL_OID_CTL_ISSUER".into(),
            4 => "URL_OID_CTL_NEXT_UPDATE".into(),
            5 => "URL_OID_CRL_ISSUER".into(),
            6 => "URL_OID_CERTIFICATE_FRESHEST_CRL".into(),
            7 => "URL_OID_CRL_FRESHEST_CRL".into(),
            8 => "URL_OID_CROSS_CERT_DIST_POINT".into(),
            _ => val.to_string(),
        },
        Oid::Str(s) => s.to_string(),
    }
}

/// `escape_path`: percent-encodes `+`, `/`, `=` in a WCHAR slice, using the
/// uppercase hex table from the C code.
///
/// Returns the escaped string and the output length (`*dst_len`: it starts at
/// `src_len` and grows by 2 for each escaped character).
pub fn escape_path(src: &[u16]) -> (String, usize) {
    let mut result = String::with_capacity(src.len());
    let mut len = 0usize;
    for &c in src {
        match c {
            0x2b => {
                result.push('%');
                result.push('2');
                result.push('B');
                len += 3;
            }
            0x2f => {
                result.push('%');
                result.push('2');
                result.push('F');
                len += 3;
            }
            0x3d => {
                result.push('%');
                result.push('3');
                result.push('D');
                len += 3;
            }
            _ => {
                result.push(char::from_u32(c as u32).unwrap_or('\u{FFFD}'));
                len += 1;
            }
        }
    }
    (result, len)
}

/// `map_ocsp_status`: maps OCSP basic response status codes to Win32 error
/// codes.
pub fn map_ocsp_status(status: u32) -> u32 {
    match status {
        0 => 0,
        1 => 0x8009_2010,
        2 => 0x8009_2013,
        _ => 0x8009_2013,
    }
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
