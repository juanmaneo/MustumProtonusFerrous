//! Rows checked against the sanitizer-built Wine oracle in `target/canon/cryptui`.
//!
//! The verbatim `is_valid_oid`, `format_hex_string`, `check_context_type` and
//! `export_append_extension` bodies were compiled with
//! `gcc -fsanitize=address,undefined -g`, and the rows below were captured from
//! that run without sanitizer reports. `check_context_type` returns the masked
//! flag bit, so its truthy results are recorded as `true`. Inputs with bytes
//! above 0x7f are not used, because passing a negative plain `char` to
//! `isdigit` is undefined in C. The empty hex input is not used either, because
//! Wine returns an uninitialized buffer for it.
use super::*;

#[test]
fn oracle_oid_rows() {
    let rows: &[(&[u8], bool)] = &[
        (b"1.2", true),
        (b"2.1.10.200", true),
        (b"1", false),
        (b"", false),
        (b"3.1", false),
        (b"1..2", false),
        (b"1.2.", false),
        (b"1.2a3", false),
        (b"01.2", false),
    ];
    for (oid, expected) in rows {
        assert_eq!(oid::is_valid_oid(oid), *expected, "{oid:?}");
    }
}

#[test]
fn oracle_hex_rows() {
    let rows: &[(&[u8], &str)] = &[
        (&[0x0a, 0xff], "0a ff "),
        (&[0x00], "00 "),
        (&[0xff, 0xab, 0xcd, 0x01], "ff ab cd 01 "),
    ];
    for (bytes, expected) in rows {
        assert_eq!(format_hex_string(bytes), *expected);
    }
}

#[test]
fn oracle_import_rows() {
    let rows: &[(u32, u32, bool)] = &[
        (
            import::CRYPTUI_WIZ_IMPORT_ALLOW_CERT,
            import::CERT_QUERY_CONTENT_CERT,
            true,
        ),
        (
            import::CRYPTUI_WIZ_IMPORT_ALLOW_CERT,
            import::CERT_QUERY_CONTENT_CRL,
            false,
        ),
        (0x1, import::CERT_QUERY_CONTENT_CRL, true),
        (import::CRYPTUI_WIZ_IMPORT_ALLOW_CERT, 8, true),
        (
            import::CRYPTUI_WIZ_IMPORT_ALLOW_CTL,
            import::CERT_QUERY_CONTENT_SERIALIZED_CRL,
            false,
        ),
    ];
    for (flags, content_type, expected) in rows {
        assert_eq!(
            import::context_type_allowed(*flags, *content_type),
            *expected,
            "flags {flags:#x}, type {content_type}"
        );
    }
}

#[test]
fn oracle_export_rows() {
    let rows: &[(u32, u32, &str, &str)] = &[
        (3, 0, "cert", "cert.p7b"),
        (2, 0, "CERT.PFX", "CERT.PFX"),
        (1, 3, "list", "list.crl"),
        (4, 2, "chain.stl", "chain.stl.ctl"),
        (5, 4, "store", "store.sst"),
        (1, 4, "store.SST", "store.SST"),
        (1, 1, "a.b", "a.b.cer"),
    ];
    for (format, subject, name, expected) in rows {
        assert_eq!(export::export_file_name(name, *format, *subject), *expected);
    }
}
