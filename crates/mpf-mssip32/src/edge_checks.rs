//! Edge-case tests for the ported mssip32 logic.
//!
//! These cover the stub constants, the stub record's pass-through, the helper-to-spec mapping,
//! and the export table. Not every expectation is oracle-derived; the ones that aren't say so in
//! a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed names of mssip32.spec. The code and
    // flags are the same for any names.
    let other = wine_spec_unimplemented_stub("other.dll", "Foo");
    assert_eq!(other.module, "other.dll");
    assert_eq!(other.function, "Foo");
    assert_eq!(other.code, EXCEPTION_WINE_STUB);
    assert_eq!(other.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(other.number_parameters, 2);

    let empty = wine_spec_unimplemented_stub("", "");
    assert_eq!(empty.module, "");
    assert_eq!(empty.function, "");
}

#[test]
fn every_stub_export_has_a_helper() {
    // Not oracle-derived: checks the seven helpers against the Stub entries of the spec table.
    let helpers: [fn() -> StubException; 7] = [
        crypt_sip_get_info,
        crypt_sip_get_reg_working_flags,
        crypt_sip_create_indirect_data,
        crypt_sip_get_signed_data_msg,
        crypt_sip_put_signed_data_msg,
        crypt_sip_remove_signed_data_msg,
        crypt_sip_verify_indirect_data,
    ];
    let stubs: Vec<&str> = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .map(|(name, _)| *name)
        .collect();
    assert_eq!(helpers.len(), stubs.len());
    for (helper, name) in helpers.iter().zip(stubs) {
        let record = helper();
        assert_eq!(record.module, "mssip32.dll");
        assert_eq!(record.function, name);
        assert_eq!(export_kind(name), Some(ExportKind::Stub));
    }
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the nine lines of dlls/mssip32/mssip32.spec, in order.
    assert_eq!(
        EXPORTS,
        [
            ("CryptSIPGetInfo", ExportKind::Stub),
            ("CryptSIPGetRegWorkingFlags", ExportKind::Stub),
            ("CryptSIPCreateIndirectData", ExportKind::Stub),
            ("CryptSIPGetSignedDataMsg", ExportKind::Stub),
            ("CryptSIPPutSignedDataMsg", ExportKind::Stub),
            ("CryptSIPRemoveSignedDataMsg", ExportKind::Stub),
            ("CryptSIPVerifyIndirectData", ExportKind::Stub),
            ("DllRegisterServer", ExportKind::StdcallPrivate),
            ("DllUnregisterServer", ExportKind::StdcallPrivate),
        ]
    );
    assert_eq!(export_kind("CryptSIPGetInfo"), Some(ExportKind::Stub));
    assert_eq!(
        export_kind("DllRegisterServer"),
        Some(ExportKind::StdcallPrivate)
    );
    assert_eq!(
        export_kind("DllUnregisterServer"),
        Some(ExportKind::StdcallPrivate)
    );
    assert_eq!(export_kind("cryptsipgetinfo"), None);
    assert_eq!(export_kind(""), None);
}
