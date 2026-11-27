//! Edge-case tests for the `msvcr120_app` export table and stub records.
//!
//! Not every expectation is oracle-derived. The ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h, include/winnt.h and dlls/msvcr120_app/Makefile.in.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    assert_eq!(MSVCR120_APP_DLL_NAME, "msvcr120_app.dll");
}

#[test]
fn wine_spec_unimplemented_stub_passes_its_arguments_through() {
    // Not oracle-derived: empty and unusual inputs, to show the record copies them unchanged.
    let record = wine_spec_unimplemented_stub("", "");
    assert_eq!(record.code, EXCEPTION_WINE_STUB);
    assert_eq!(record.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(record.number_parameters, 2);
    assert_eq!(record.module, "");
    assert_eq!(record.function, "");
    let record = wine_spec_unimplemented_stub("other.dll", "#42");
    assert_eq!(record.module, "other.dll");
    assert_eq!(record.function, "#42");
}

#[test]
fn find_export_rejects_names_the_spec_does_not_export() {
    // Not oracle-derived: lookups the spec cannot match. Names are case-sensitive and exact.
    assert!(find_export("").is_none());
    assert!(find_export("atexit ").is_none());
    assert!(find_export("ATEXIT").is_none());
    assert!(find_export("_NLG_Dispatch").is_none());
    // The spec comments this line out (`# extern __lconv`), so winebuild never exports it.
    assert!(find_export("__lconv").is_none());
}

#[test]
fn find_export_returns_the_spec_entry() {
    // Oracle-derived: the spec lines for these names.
    let atexit = find_export("atexit").unwrap();
    assert_eq!(atexit.kind, ExportKind::Forward);
    assert!(atexit.private);
    assert!(atexit.arch.is_empty());
    let setjmp = find_export("setjmp").unwrap();
    assert!(setjmp.private);
    assert_eq!(setjmp.arch, &[Arch::Arm, Arch::X86_64]);
    let fpe = find_export("_FPE_Raise").unwrap();
    assert_eq!(fpe.kind, ExportKind::Stub);
    assert_eq!(fpe.arch, &[Arch::Arm]);
}

#[test]
fn arch_tokens_use_winebuild_spelling() {
    // Not oracle-derived: the spellings that tools/winebuild/parser.c (parse_spec_flags) accepts.
    assert_eq!(Arch::Arm.token(), "arm");
    assert_eq!(Arch::Arm64.token(), "arm64");
    assert_eq!(Arch::I386.token(), "i386");
    assert_eq!(Arch::NotI386.token(), "!i386");
    assert_eq!(Arch::Win32.token(), "win32");
    assert_eq!(Arch::Win64.token(), "win64");
    assert_eq!(Arch::X86_64.token(), "x86_64");
}

#[test]
fn colliding_helper_names_keep_their_underscores() {
    // Not oracle-derived: checks the naming fallback documented in stubs.rs.
    assert_eq!(stub__nlg_dispatch2().function, "_NLG_Dispatch2");
    assert_eq!(stub___nlg_dispatch2().function, "__NLG_Dispatch2");
    assert_eq!(stub__nlg_return2().function, "_NLG_Return2");
    assert_eq!(stub___nlg_return2().function, "__NLG_Return2");
}
