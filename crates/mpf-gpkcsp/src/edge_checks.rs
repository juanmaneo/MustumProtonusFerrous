//! Boundary and branch checks for the gpkcsp port.
//!
//! Not every expectation here is oracle-derived. The ones that are not say so in a comment: they
//! come from `dlls/gpkcsp/gpkcsp.spec`, `include/winternl.h` and `include/winnt.h`.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: `include/winternl.h` and `include/winnt.h` in the reference.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed names of gpkcsp.spec. The code and
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
fn export_table_matches_spec_order() {
    // Not oracle-derived: the 26 lines of dlls/gpkcsp/gpkcsp.spec, in order.
    assert_eq!(
        EXPORTS,
        [
            ("CPAcquireContext", ExportKind::Stub),
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
            ("DllMain", ExportKind::PrivateStdcall),
            ("DllRegisterServer", ExportKind::PrivateStdcall),
            ("DllUnregisterServer", ExportKind::PrivateStdcall),
        ]
    );
    // Not oracle-derived: the count is the number of `@ stub` lines in the spec.
    assert_eq!(
        EXPORTS
            .iter()
            .filter(|(_, kind)| *kind == ExportKind::Stub)
            .count(),
        23
    );
    assert_eq!(export_kind("CPGenRandom"), Some(ExportKind::Stub));
    assert_eq!(export_kind("DllMain"), Some(ExportKind::PrivateStdcall));
    assert_eq!(export_kind("cpgenrandom"), None);
    assert_eq!(export_kind(""), None);
}
