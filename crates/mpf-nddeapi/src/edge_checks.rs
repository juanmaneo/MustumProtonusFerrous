//! Edge-case tests for the ported nddeapi logic.
//!
//! These cover the constants, the stub names and the export table. Not every expectation is
//! oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: NDDE_NOT_IMPLEMENTED is the `#define` in dlls/nddeapi/nddeapi.c. The
    // exception values come from include/winternl.h and include/winnt.h.
    assert_eq!(NDDE_NOT_IMPLEMENTED, 14);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the two fixed names of nddeapi.spec. The code
    // and flags are the same for any names.
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
    // Not oracle-derived: the 28 export lines of dlls/nddeapi/nddeapi.spec, in spec order. The
    // spec has no ordinals 508 and 608, so none appears here.
    assert_eq!(
        EXPORTS,
        [
            ("NDdeShareAddA", ExportKind::Stdcall),
            ("NDdeShareDelA", ExportKind::Stdcall),
            ("NDdeShareEnumA", ExportKind::Stdcall),
            ("NDdeShareGetInfoA", ExportKind::Stdcall),
            ("NDdeShareSetInfoA", ExportKind::Stdcall),
            ("NDdeGetErrorStringA", ExportKind::Stdcall),
            ("NDdeIsValidShareNameA", ExportKind::Stdcall),
            ("NDdeIsValidAppTopicListA", ExportKind::Stdcall),
            ("NDdeGetShareSecurityA", ExportKind::Stdcall),
            ("NDdeSetShareSecurityA", ExportKind::Stdcall),
            ("NDdeGetTrustedShareA", ExportKind::Stdcall),
            ("NDdeSetTrustedShareA", ExportKind::Stdcall),
            ("NDdeTrustedShareEnumA", ExportKind::Stdcall),
            ("NDdeShareAddW", ExportKind::Stdcall),
            ("NDdeShareDelW", ExportKind::Stdcall),
            ("NDdeShareEnumW", ExportKind::Stdcall),
            ("NDdeShareGetInfoW", ExportKind::Stdcall),
            ("NDdeShareSetInfoW", ExportKind::Stdcall),
            ("NDdeGetErrorStringW", ExportKind::Stdcall),
            ("NDdeIsValidShareNameW", ExportKind::Stdcall),
            ("NDdeIsValidAppTopicListW", ExportKind::Stdcall),
            ("NDdeGetShareSecurityW", ExportKind::Stdcall),
            ("NDdeSetShareSecurityW", ExportKind::Stdcall),
            ("NDdeGetTrustedShareW", ExportKind::Stdcall),
            ("NDdeSetTrustedShareW", ExportKind::Stdcall),
            ("NDdeTrustedShareEnumW", ExportKind::Stdcall),
            ("NDdeSpecialCommandA", ExportKind::Stub),
            ("NDdeSpecialCommandW", ExportKind::Stub),
        ]
    );
}

#[test]
fn export_kind_is_case_sensitive_and_rejects_unknown_names() {
    // Not oracle-derived: the lookup rules of `export_kind`, which the C loader applies by name.
    assert_eq!(export_kind("NDdeShareAddA"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("NDdeSpecialCommandW"), Some(ExportKind::Stub));
    assert_eq!(export_kind("ndDeShareAddA"), None);
    assert_eq!(export_kind("NDdeShareAdd"), None);
    assert_eq!(export_kind(""), None);
}
