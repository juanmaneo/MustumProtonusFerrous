//! Boundary cases outside the sanitizer rows: NUL truncation, empty and long
//! inputs, and every allow-list and extension branch.
use super::*;

use crate::export::{
    export_extension, export_file_name, CRYPTUI_WIZ_EXPORT_CERT_STORE,
    CRYPTUI_WIZ_EXPORT_CRL_CONTEXT, CRYPTUI_WIZ_EXPORT_CTL_CONTEXT, CRYPTUI_WIZ_EXPORT_FORMAT_PFX,
    CRYPTUI_WIZ_EXPORT_FORMAT_PKCS7,
};
use crate::import::{
    context_type_allowed, CERT_QUERY_CONTENT_CERT, CERT_QUERY_CONTENT_CRL, CERT_QUERY_CONTENT_CTL,
    CERT_QUERY_CONTENT_SERIALIZED_CERT, CERT_QUERY_CONTENT_SERIALIZED_CRL,
    CERT_QUERY_CONTENT_SERIALIZED_CTL, CRYPTUI_WIZ_IMPORT_ALLOW_CERT, CRYPTUI_WIZ_IMPORT_ALLOW_CRL,
    CRYPTUI_WIZ_IMPORT_ALLOW_CTL,
};

#[test]
fn oid_accepts_well_formed_arcs_and_rejects_bad_separators() {
    assert!(oid::is_valid_oid(b"0.0"));
    assert!(oid::is_valid_oid(b"1.2.3"));
    assert!(oid::is_valid_oid(b"1.2.34567890"));
    assert!(oid::is_valid_oid(b"2.0.0.0.0.0"));
    assert!(!oid::is_valid_oid(b"3.1"));
    assert!(!oid::is_valid_oid(b" 1.2"));
    assert!(!oid::is_valid_oid(b"1.2 "));
    assert!(!oid::is_valid_oid(b"1.2\t"));
    assert!(!oid::is_valid_oid(b"1.2..3"));
    assert!(!oid::is_valid_oid(b"1.2.3."));
    assert!(!oid::is_valid_oid(b"1.."));
    assert!(!oid::is_valid_oid(b"0."));
    assert!(!oid::is_valid_oid(b"2"));
    assert!(!oid::is_valid_oid(b"."));
    assert!(!oid::is_valid_oid(b"1.a"));
}

#[test]
fn oid_ends_at_the_first_nul() {
    assert!(oid::is_valid_oid(b"1.2\0junk"));
    assert!(!oid::is_valid_oid(b"1\0.2"));
    assert!(!oid::is_valid_oid(b"\0"));
    assert!(!oid::is_valid_oid(b"1.\0"));
}

#[test]
fn hex_terminates_each_byte_with_a_space() {
    assert_eq!(format_hex_string(&[0x00, 0x7f, 0x80, 0xff]), "00 7f 80 ff ");
    assert_eq!(format_hex_string(&[]), "");

    let long = vec![0xab; 256];
    let text = format_hex_string(&long);
    assert_eq!(text.len(), 256 * 3);
    assert!(text.starts_with("ab ab "));
    assert!(text.ends_with("ab "));
}

#[test]
fn import_without_allow_flags_accepts_every_type() {
    for content_type in [
        CERT_QUERY_CONTENT_CERT,
        CERT_QUERY_CONTENT_CRL,
        CERT_QUERY_CONTENT_CTL,
        CERT_QUERY_CONTENT_SERIALIZED_CERT,
        CERT_QUERY_CONTENT_SERIALIZED_CRL,
        CERT_QUERY_CONTENT_SERIALIZED_CTL,
        8,
    ] {
        assert!(context_type_allowed(0, content_type));
        assert!(context_type_allowed(0x1 | 0x10000, content_type));
    }
}

#[test]
fn import_matches_each_content_type_to_its_own_flag() {
    let groups = [
        (
            CRYPTUI_WIZ_IMPORT_ALLOW_CERT,
            [CERT_QUERY_CONTENT_CERT, CERT_QUERY_CONTENT_SERIALIZED_CERT],
        ),
        (
            CRYPTUI_WIZ_IMPORT_ALLOW_CRL,
            [CERT_QUERY_CONTENT_CRL, CERT_QUERY_CONTENT_SERIALIZED_CRL],
        ),
        (
            CRYPTUI_WIZ_IMPORT_ALLOW_CTL,
            [CERT_QUERY_CONTENT_CTL, CERT_QUERY_CONTENT_SERIALIZED_CTL],
        ),
    ];
    for (flag, contents) in groups {
        for (other_flag, _) in groups {
            for content_type in contents {
                assert_eq!(
                    context_type_allowed(other_flag, content_type),
                    other_flag == flag
                );
            }
        }
    }
}

#[test]
fn import_accepts_multi_object_types_when_any_flag_is_set() {
    assert!(context_type_allowed(CRYPTUI_WIZ_IMPORT_ALLOW_CRL, 8));
    let all =
        CRYPTUI_WIZ_IMPORT_ALLOW_CERT | CRYPTUI_WIZ_IMPORT_ALLOW_CRL | CRYPTUI_WIZ_IMPORT_ALLOW_CTL;
    for content_type in [CERT_QUERY_CONTENT_CRL, CERT_QUERY_CONTENT_SERIALIZED_CTL, 8] {
        assert!(context_type_allowed(all, content_type));
    }
}

#[test]
fn export_extension_prefers_format_over_subject() {
    assert_eq!(
        export_extension(
            CRYPTUI_WIZ_EXPORT_FORMAT_PKCS7,
            CRYPTUI_WIZ_EXPORT_CRL_CONTEXT
        ),
        ".p7b"
    );
    assert_eq!(
        export_extension(CRYPTUI_WIZ_EXPORT_FORMAT_PFX, CRYPTUI_WIZ_EXPORT_CERT_STORE),
        ".pfx"
    );
    assert_eq!(export_extension(1, CRYPTUI_WIZ_EXPORT_CRL_CONTEXT), ".crl");
    assert_eq!(export_extension(4, CRYPTUI_WIZ_EXPORT_CTL_CONTEXT), ".ctl");
    assert_eq!(export_extension(5, CRYPTUI_WIZ_EXPORT_CERT_STORE), ".sst");
    assert_eq!(export_extension(1, 1), ".cer");
    assert_eq!(export_extension(0, 0), ".cer");
}

#[test]
fn export_name_boundaries() {
    assert_eq!(export_file_name("", 1, 1), ".cer");
    assert_eq!(export_file_name(".", 1, 1), "..cer");
    assert_eq!(export_file_name("a.cer.x", 1, 1), "a.cer.x.cer");
    assert_eq!(export_file_name("dir.x\\file", 1, 1), "dir.x\\file.cer");
    assert_eq!(export_file_name("MyCert.Cer", 1, 1), "MyCert.Cer");
    assert_eq!(export_file_name("a", 2, 1), "a.pfx");
    assert_eq!(export_file_name("\u{e9}", 3, 1), "\u{e9}.p7b");
    assert_eq!(export_file_name("a.\u{e9}", 2, 1), "a.\u{e9}.pfx");
    assert_eq!(export_file_name("a.P7B", 3, 1), "a.P7B");
    assert_eq!(export_file_name("chain", 1, 4), "chain.sst");
    assert_eq!(export_file_name("chain.sst", 1, 4), "chain.sst");
}
