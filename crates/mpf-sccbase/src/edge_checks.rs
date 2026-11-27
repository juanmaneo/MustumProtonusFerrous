//! Edge-case tests for the ported sccbase logic.
//!
//! These cover the constants, the declaration table, and the stub records' argument
//! pass-through. Not every expectation is oracle-derived; the ones that aren't say so in a
//! comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the 26 declaration lines of dlls/sccbase/sccbase.spec, in order.
    assert_eq!(
        EXPORTS,
        [
            ("CPAcquireContext", ExportKind::Stub),
            ("CPAcquireContextW", ExportKind::Stub),
            ("CPCreateHash", ExportKind::Stub),
            ("CPDecrypt", ExportKind::Stub),
            ("CPDeriveKey", ExportKind::Stub),
            ("CPDestroyHash", ExportKind::Stub),
            ("CPDestroyKey", ExportKind::Stub),
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
            ("CPSetHashParam", ExportKind::Stub),
            ("CPSetKeyParam", ExportKind::Stub),
            ("CPSetProvParam", ExportKind::Stub),
            ("CPSignHash", ExportKind::Stub),
            ("CPVerifySignature", ExportKind::Stub),
            ("DllRegisterServer", ExportKind::StdcallPrivate),
            ("DllUnregisterServer", ExportKind::StdcallPrivate),
        ]
    );
    assert_eq!(export_kind("CPAcquireContext"), Some(ExportKind::Stub));
    assert_eq!(export_kind("CPVerifySignature"), Some(ExportKind::Stub));
    assert_eq!(
        export_kind("DllRegisterServer"),
        Some(ExportKind::StdcallPrivate)
    );
    assert_eq!(export_kind("cpacquirecontext"), None);
    assert_eq!(export_kind("CPAcquireContextA"), None);
    assert_eq!(export_kind(""), None);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed names of sccbase.spec. The code and
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
