//! Edge-case tests for the ported slbcsp logic.
//!
//! These cover the stub record fields, the helper-to-export mapping, and the export table. Not
//! every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: the values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the C body copies both arguments into the record unchanged and always
    // sets NumberParameters to 2. These inputs are not among the oracle rows.
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
fn helpers_pass_the_slbcsp_module_and_their_export_name() {
    // Not oracle-derived: every helper passes MODULE = slbcsp.dll and its own export name. The
    // helpers also appear in the same order as the `@ stub` entries of the export table.
    let helpers: [fn() -> StubException; 25] = [
        cp_acquire_context,
        cp_set_hash_param,
        cp_set_key_param,
        cp_set_prov_param,
        cp_create_hash,
        cp_decrypt,
        cp_derive_key,
        cp_destroy_hash,
        cp_destroy_key,
        cp_duplicate_hash,
        cp_duplicate_key,
        cp_encrypt,
        cp_export_key,
        cp_gen_key,
        cp_gen_random,
        cp_get_hash_param,
        cp_get_key_param,
        cp_get_prov_param,
        cp_get_user_key,
        cp_hash_data,
        cp_hash_session_key,
        cp_import_key,
        cp_release_context,
        cp_sign_hash,
        cp_verify_signature,
    ];
    for helper in helpers {
        assert_eq!(helper().module, "slbcsp.dll");
    }
    let names: Vec<&str> = helpers.iter().map(|helper| helper().function).collect();
    let stubs: Vec<&str> = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .map(|(name, _)| *name)
        .collect();
    assert_eq!(names, stubs);
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the names and kinds of dlls/slbcsp/slbcsp.spec, in spec order.
    assert_eq!(
        EXPORTS,
        [
            ("CPAcquireContext", ExportKind::Stub),
            ("CPSetHashParam", ExportKind::Stub),
            ("CPSetKeyParam", ExportKind::Stub),
            ("CPSetProvParam", ExportKind::Stub),
            ("CPCreateHash", ExportKind::Stub),
            ("CPDecrypt", ExportKind::Stub),
            ("CPDeriveKey", ExportKind::Stub),
            ("CPDestroyHash", ExportKind::Stub),
            ("CPDestroyKey", ExportKind::Stub),
            ("CPDuplicateHash", ExportKind::Stub),
            ("CPDuplicateKey", ExportKind::Stub),
            ("CPEncrypt", ExportKind::Stub),
            ("CPExportKey", ExportKind::Stub),
            ("CPGenKey", ExportKind::Stub),
            ("CPGenRandom", ExportKind::Stub),
            ("CPGetHashParam", ExportKind::Stub),
            ("CPGetKeyParam", ExportKind::Stub),
            ("CPGetProvParam", ExportKind::Stub),
            ("CPGetUserKey", ExportKind::Stub),
            ("CPHashData", ExportKind::Stub),
            ("CPHashSessionKey", ExportKind::Stub),
            ("CPImportKey", ExportKind::Stub),
            ("CPReleaseContext", ExportKind::Stub),
            ("CPSignHash", ExportKind::Stub),
            ("CPVerifySignature", ExportKind::Stub),
            ("DllRegisterServer", ExportKind::StdcallPrivate),
            ("DllUnregisterServer", ExportKind::StdcallPrivate),
        ]
    );
    assert_eq!(export_kind("CPGenRandom"), Some(ExportKind::Stub));
    assert_eq!(
        export_kind("DllRegisterServer"),
        Some(ExportKind::StdcallPrivate)
    );
    assert_eq!(export_kind("cpgenrandom"), None);
    assert_eq!(export_kind("CPGenRandomX"), None);
}
