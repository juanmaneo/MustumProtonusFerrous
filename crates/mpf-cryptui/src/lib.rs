//! Deterministic logic from Wine's `dlls/cryptui/main.c`.
//!
//! This port covers `format_hex_string`, the OID syntax check from
//! `is_valid_oid`, the import allow-list from `check_context_type`, and the
//! extension rules from `export_append_extension`. Certificate decoding, store
//! access, resources, windows, files and the `SetLastError` call in
//! `check_context_type` depend on Windows services and are left to callers.

use std::fmt::Write;

pub mod export;
pub mod import;
pub mod oid;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// Formats bytes as lowercase hexadecimal pairs, each followed by a space, as
/// Wine's `format_hex_string` does. Wine returns an uninitialized buffer for an
/// empty input, so this port returns an empty string instead.
pub fn format_hex_string(bytes: &[u8]) -> String {
    let mut text = String::with_capacity(bytes.len() * 3);
    for byte in bytes {
        write!(text, "{byte:02x} ").expect("writing to a String cannot fail");
    }
    text
}
