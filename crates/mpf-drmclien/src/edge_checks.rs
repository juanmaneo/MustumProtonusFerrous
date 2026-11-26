//! Edge-case tests for the ported drmclien logic.
//!
//! These cover the export table, the kind lookup, and the stub record shape. Not every
//! expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the 31 entry lines of dlls/drmclien/drmclien.spec, in order.
    assert_eq!(EXPORTS.len(), 31);
    assert_eq!(EXPORTS[0], ("DllMain", ExportKind::Private));
    assert_eq!(EXPORTS[1], ("CanDecrypt", ExportKind::Stub));
    assert_eq!(EXPORTS[10], ("??0CDRMLiteCrypto@@QAE@XZ", ExportKind::Stub));
    assert_eq!(
        EXPORTS[28],
        (
            "?RestoreLicenses@CDRMLiteCrypto@@QAEJKPAEPAGPAUIUnknown@@PAHPAX@Z",
            ExportKind::Stub
        )
    );
    assert_eq!(EXPORTS[29], ("DllRegisterServer", ExportKind::Private));
    assert_eq!(EXPORTS[30], ("DllUnregisterServer", ExportKind::Private));
}

#[test]
fn stub_and_private_counts_match_spec() {
    // Not oracle-derived: 28 `@ stub` lines and 3 `-private` lines in drmclien.spec.
    let stubs = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .count();
    let private = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Private)
        .count();
    assert_eq!(stubs, 28);
    assert_eq!(private, 3);
}

#[test]
fn export_kind_is_case_sensitive_and_exact() {
    // Not oracle-derived: lookup semantics of the spec names. Names are matched byte for byte.
    assert_eq!(export_kind("DllMain"), Some(ExportKind::Private));
    assert_eq!(export_kind("CanDecrypt"), Some(ExportKind::Stub));
    assert_eq!(export_kind("candecrypt"), None);
    assert_eq!(export_kind("CanDecrypt "), None);
    assert_eq!(export_kind(""), None);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the C function raises the same record for any names. The harness only
    // passed the names of drmclien.spec, so these other values are not from the oracle.
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
    assert_eq!(empty.number_parameters, 2);
}

#[test]
fn helpers_use_the_makefile_module_name() {
    // Not oracle-derived: the module name is MODULE = drmclien.dll from dlls/drmclien/Makefile.in.
    // The oracle rows also print this module, so this only pins the name in one place.
    assert_eq!(can_decrypt().module, "drmclien.dll");
    assert_eq!(cdrmlite_crypto_dtor().module, "drmclien.dll");
    assert_eq!(cdrmlite_crypto_dtor().function, "??1CDRMLiteCrypto@@QAE@XZ");
}
