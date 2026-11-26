//! Edge-case tests for the ported url logic.
//!
//! These cover the `HRESULT` boundary of the `FileProtocolHandlerA` target choice, the
//! export table, and the stub record's pass-through of its arguments. Not every expectation
//! is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/winternl.h and include/winnt.h.
    assert_eq!(S_OK, 0);
    assert_eq!(E_NOTIMPL, 0x8000_4001);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn file_protocol_handler_target_only_s_ok_selects_the_path() {
    // Not oracle-derived: the C ternary tests `createpath==S_OK`, so every other HRESULT
    // (including the negative failures and `S_FALSE`) takes the URL.
    let url = "file:///C:/a.txt";
    assert_eq!(file_protocol_handler_a_target(S_OK, "", url), "");
    assert_eq!(file_protocol_handler_a_target(S_OK + 1, "p", url), url);
    assert_eq!(file_protocol_handler_a_target(u32::MAX, "p", url), url);
    assert_eq!(file_protocol_handler_a_target(0x8000_0000, "p", url), url);
    assert_eq!(file_protocol_handler_a_target(0x8000_4001, "p", url), url);
    assert_eq!(file_protocol_handler_a_target(0, "", ""), "");
}

#[test]
fn stub_exception_passes_names_through() {
    // Not oracle-derived: the stub record copies its arguments unchanged. The url.spec exports
    // are all named, so no ordinal function name is needed.
    assert_eq!(
        wine_spec_unimplemented_stub("x.dll", ""),
        StubException {
            code: EXCEPTION_WINE_STUB,
            flags: EXCEPTION_NONCONTINUABLE,
            number_parameters: 2,
            module: "x.dll",
            function: "",
        }
    );
}

#[test]
fn export_table_shape() {
    // Not oracle-derived: counts taken from dlls/url/url.spec (21 exports: 5 stdcall with
    // bodies, 3 aliases, 13 stubs).
    assert_eq!(EXPORTS.len(), 21);
    let count = |kind: fn(&ExportKind) -> bool| EXPORTS.iter().filter(|(_, k)| kind(k)).count();
    assert_eq!(count(|k| *k == ExportKind::Stdcall), 5);
    assert_eq!(count(|k| matches!(k, ExportKind::Alias(_))), 3);
    assert_eq!(count(|k| *k == ExportKind::Stub), 13);
}

#[test]
fn every_alias_points_at_a_stdcall_export() {
    // Not oracle-derived: the alias targets are the `@ stdcall NAME(...) TARGET` forms in
    // dlls/url/url.spec.
    for (name, kind) in EXPORTS {
        if let ExportKind::Alias(target) = kind {
            assert_eq!(
                export_kind(target),
                Some(ExportKind::Stdcall),
                "alias {name}"
            );
        }
    }
    assert_eq!(
        export_kind("FileProtocolHandler"),
        Some(ExportKind::Alias("FileProtocolHandlerA"))
    );
    assert_eq!(export_kind("OpenURL"), Some(ExportKind::Alias("OpenURLA")));
    assert_eq!(
        export_kind("TelnetProtocolHandler"),
        Some(ExportKind::Alias("TelnetProtocolHandlerA"))
    );
}

#[test]
fn export_kind_is_case_sensitive_and_exact() {
    // Not oracle-derived: lookup is by exact name, as the spec file defines the names.
    assert_eq!(export_kind("OpenURLA"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("openurla"), None);
    assert_eq!(export_kind("OpenURL "), None);
    assert_eq!(export_kind("DllMain"), None);
    assert_eq!(export_kind(""), None);
}
