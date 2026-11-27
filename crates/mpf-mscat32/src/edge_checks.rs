//! Edge-case tests for the ported mscat32 logic.
//!
//! These cover the export table shape, the lookup by name, the stub dispatch, and the stub record
//! fields. Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn export_table_numbers_its_rows_from_one_in_spec_order() {
    // Oracle-derived: winebuild assigns ordinals 1 to 35 in spec order.
    assert_eq!(EXPORTS.len(), 35);
    for (index, row) in EXPORTS.iter().enumerate() {
        assert_eq!(usize::from(row.ordinal), index + 1);
    }
}

#[test]
fn export_kinds_and_private_marks_are_counted() {
    // Oracle-derived: 30 forwards, 5 stubs, and 7 PRIVATE rows (every stub, plus the two
    // `-private` forwards DllRegisterServer and DllUnregisterServer).
    let forwards = EXPORTS
        .iter()
        .filter(|e| matches!(e.kind, ExportKind::Forwarded(_)))
        .count();
    let stubs = EXPORTS
        .iter()
        .filter(|e| e.kind == ExportKind::Stub)
        .count();
    let private = EXPORTS.iter().filter(|e| e.private).count();
    assert_eq!((forwards, stubs, private), (30, 5, 7));
}

#[test]
fn dll_registration_exports_forward_to_their_mscat32_names_in_wintrust() {
    // Oracle-derived: `DllRegisterServer=wintrust.mscat32DllRegisterServer @31 PRIVATE`, and the
    // matching Unregister row.
    assert_eq!(
        export("DllRegisterServer"),
        Some(Export {
            name: "DllRegisterServer",
            ordinal: 31,
            kind: ExportKind::Forwarded("wintrust.mscat32DllRegisterServer"),
            private: true,
        })
    );
    assert_eq!(
        export("DllUnregisterServer").map(|e| e.kind),
        Some(ExportKind::Forwarded("wintrust.mscat32DllUnregisterServer"))
    );
}

#[test]
fn a_public_forward_keeps_its_own_name_in_wintrust() {
    // Oracle-derived: `CryptCATOpen=wintrust.CryptCATOpen @25`, which has no PRIVATE mark.
    assert_eq!(
        export("CryptCATOpen"),
        Some(Export {
            name: "CryptCATOpen",
            ordinal: 25,
            kind: ExportKind::Forwarded("wintrust.CryptCATOpen"),
            private: false,
        })
    );
}

#[test]
fn export_lookup_is_case_sensitive_and_rejects_empty_names() {
    // Not oracle-derived: the name lookup is the port's own API, so there is no C oracle for it.
    assert_eq!(export("cryptcatverifymember"), None);
    assert_eq!(export(""), None);
}

#[test]
fn commented_out_stub_is_not_an_export() {
    // Oracle-derived: the spec line is `#@ stub CatalogCompactHashDatabase`, and the `.def` has no
    // row for it.
    assert_eq!(export("CatalogCompactHashDatabase"), None);
}

#[test]
fn stub_for_export_covers_every_stub_row_and_nothing_else() {
    // Not oracle-derived: checks the port's own dispatch against the `EXPORTS` kinds.
    for row in EXPORTS.iter() {
        match row.kind {
            ExportKind::Stub => {
                assert_eq!(
                    stub_for_export(row.name).map(|s| s.function),
                    Some(row.name)
                );
            }
            ExportKind::Forwarded(_) => assert_eq!(stub_for_export(row.name), None),
        }
    }
    assert_eq!(stub_for_export("CryptCATOpen"), None);
    assert_eq!(stub_for_export("nope"), None);
}

#[test]
fn each_stub_helper_names_its_own_export() {
    // Oracle-derived: the module is the `mscat32.dll` string in the winebuild assembly, and each
    // function is the export name from the spec.
    let cases = [
        (crypt_cat_verify_member(), "CryptCATVerifyMember"),
        (crypt_cat_cdf_enum_attributes(), "CryptCATCDFEnumAttributes"),
        (crypt_cat_cdf_enum_members(), "CryptCATCDFEnumMembers"),
        (ms_cat_construct_hash_tag(), "MsCatConstructHashTag"),
        (ms_cat_free_hash_tag(), "MsCatFreeHashTag"),
    ];
    for (stub, name) in cases {
        assert_eq!(stub.module, "mscat32.dll");
        assert_eq!(stub.function, name);
        assert_eq!(stub.number_parameters, 2);
    }
}

#[test]
fn wine_spec_unimplemented_stub_passes_any_names_through() {
    // Not oracle-derived: the C body copies its two pointers into the record without reading
    // them. The harness only ran the five fixed names, so these other inputs are the port's
    // own edge cases.
    let other = wine_spec_unimplemented_stub("other.dll", "Foo");
    assert_eq!(
        other,
        StubException {
            code: EXCEPTION_WINE_STUB,
            flags: EXCEPTION_NONCONTINUABLE,
            number_parameters: 2,
            module: "other.dll",
            function: "Foo",
        }
    );
    let empty = wine_spec_unimplemented_stub("", "");
    assert_eq!(empty.module, "");
    assert_eq!(empty.function, "");
}
