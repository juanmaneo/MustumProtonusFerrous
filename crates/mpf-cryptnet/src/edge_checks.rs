//! Edge-case tests for the ported cryptnet functions.
//!
//! These tests exercise boundary conditions and ensure the ported logic
//! behaves correctly under extreme inputs.

use super::*;

#[test]
fn oid_enum_variants() {
    // Test all enum variants are handled
    assert_eq!(url_oid_to_str(Oid::Int(0)), "0");
    assert_eq!(url_oid_to_str(Oid::Int(1)), "URL_OID_CERTIFICATE_ISSUER");
    assert_eq!(url_oid_to_str(Oid::Int(8)), "URL_OID_CROSS_CERT_DIST_POINT");
    assert_eq!(url_oid_to_str(Oid::Int(9)), "9");
    assert_eq!(url_oid_to_str(Oid::Int(12)), "12");
    assert_eq!(url_oid_to_str(Oid::Int(65535)), "65535");
    assert_eq!(url_oid_to_str(Oid::Str("test")), "test");
    assert_eq!(url_oid_to_str(Oid::Str("")), "");
}

#[test]
fn escape_path_empty_and_max() {
    // Empty input
    let (got, len) = escape_path(&[]);
    assert_eq!(got, "");
    assert_eq!(len, 0);

    // Max length input (theoretical)
    let max_len = 1000;
    let src = vec![b'a' as u16; max_len];
    let (got, len) = escape_path(&src);
    assert_eq!(len, max_len);
    assert_eq!(got.len(), max_len);
}

#[test]
fn escape_path_only_escaped_chars() {
    // Only '+' characters
    let src = &[b'+' as u16, b'+' as u16, b'+' as u16];
    let (got, len) = escape_path(src);
    assert_eq!(got, "%2B%2B%2B");
    assert_eq!(len, 9);
}

#[test]
fn escape_path_only_equal_chars() {
    let src = &[b'=' as u16, b'=' as u16];
    let (got, len) = escape_path(src);
    assert_eq!(got, "%3D%3D");
    assert_eq!(len, 6);
}

#[test]
fn escape_path_unicode_bmp() {
    // Test BMP characters beyond ASCII (e.g., accented letters)
    let src = &[0x00E9, 0x00F6, 0x00DF]; // é, ö, ß
    let (got, len) = escape_path(src);
    assert_eq!(got, "éöß");
    assert_eq!(len, 3);
}

#[test]
fn oid_str_large_values() {
    // Test large integer values that are not in the switch table
    assert_eq!(url_oid_to_str(Oid::Int(100)), "100");
    assert_eq!(url_oid_to_str(Oid::Int(12345)), "12345");
}

#[test]
fn escape_path_mixed() {
    // Mix of escaped and unescaped characters
    let src = &[0x61, 0x2b, 0x62, 0x2f, 0x63]; // "a+b/c"
    let (got, len) = escape_path(src);
    assert_eq!(got, "a%2Bb%2Fc");
    assert_eq!(len, 9);
}

#[test]
fn oid_str_null_pointer() {
    // This test is theoretical; in practice we pass valid pointers.
    // We simulate by using a string literal.
    assert_eq!(url_oid_to_str(Oid::Str("")), "");
}

#[test]
fn escape_path_surrogate_becomes_replacement_char() {
    // A lone surrogate WCHAR cannot be a Rust char; the port emits U+FFFD.
    // The C code copies the unit verbatim, so this is a documented divergence
    // for input that is not valid UTF-16.
    let src = &[0x61, 0xD800, 0x62];
    let (got, len) = escape_path(src);
    assert_eq!(got, "a\u{FFFD}b");
    assert_eq!(len, 3);
}
