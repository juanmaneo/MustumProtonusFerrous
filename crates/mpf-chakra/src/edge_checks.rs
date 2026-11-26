//! Edge-case tests for the ported chakra logic.
//!
//! The oracle covers the observed inputs. These tests cover the export table (its
//! shape, the lookup boundaries, and the exports that are not stubs) and the
//! `StubRaise` arguments. Expectations that are not oracle-derived say so in a comment.

use super::*;

#[test]
fn stub_export_names_are_unique_identifiers() {
    // Not oracle-derived: the table was generated from chakra.spec, so a duplicate
    // or a non-identifier name would mean the copy went wrong.
    for (i, name) in STUB_EXPORTS.iter().enumerate() {
        assert!(!name.is_empty(), "entry {i}");
        assert!(name.bytes().all(|b| b.is_ascii_alphanumeric()), "{name}");
        assert!(!STUB_EXPORTS[..i].contains(name), "{name}");
    }
}

#[test]
fn stub_lookup_boundaries() {
    // Not oracle-derived: the first and last `@ stub` lines of chakra.spec.
    assert!(is_stub_export("JsProjectWinRTNamespace"));
    assert!(is_stub_export("JsVariantToValue"));
    // Not oracle-derived: export names are compared exactly, so prefixes, suffixes,
    // other case and empty names are not stubs.
    assert!(!is_stub_export(""));
    assert!(!is_stub_export("JsAddR"));
    assert!(!is_stub_export("JsAddRefX"));
    assert!(!is_stub_export("jsaddref"));
    assert!(!is_stub_export("JsAddRef "));
}

#[test]
fn non_stub_exports_are_not_stubs() {
    // Not oracle-derived: these four are `stdcall -private` in chakra.spec, not `@ stub`.
    for name in [
        "DllGetClassObject",
        "DllCanUnloadNow",
        "DllRegisterServer",
        "DllUnregisterServer",
    ] {
        assert!(!is_stub_export(name), "{name}");
    }
}

#[test]
fn unimplemented_stub_passes_names_through() {
    // Not oracle-derived: the C stub copies both pointers into the raise arguments
    // without inspecting them, so empty names are passed through unchanged.
    assert_eq!(
        unimplemented_stub("", ""),
        StubRaise {
            code: EXCEPTION_WINE_STUB,
            flags: EXCEPTION_NONCONTINUABLE,
            module: "",
            function: "",
        }
    );
    let raise = unimplemented_stub(MODULE_NAME, "DumpDiagInfo");
    assert_eq!(raise.module, MODULE_NAME);
    assert_eq!(raise.function, "DumpDiagInfo");
    // Not oracle-derived: derived Debug output names the export, which helps when the raise is logged.
    assert!(format!("{raise:?}").contains("DumpDiagInfo"));
}

#[test]
fn class_object_leaves_non_integer_slot_untouched() {
    // Not oracle-derived: the out-slot is never written for any pointee type.
    let mut slot: Option<Box<u8>> = Some(Box::new(7));
    assert_eq!(
        dll_get_class_object(Some(&mut slot)),
        CLASS_E_CLASSNOTAVAILABLE
    );
    assert_eq!(slot.as_deref(), Some(&7));
}

#[test]
fn hresult_severity_matches_the_bit() {
    // Not oracle-derived: the severity bit makes CLASS_E_CLASSNOTAVAILABLE a failure
    // and the S_OK and S_FALSE results successes.
    assert!(dll_get_class_object::<u8>(None) < 0);
    assert!(dll_can_unload_now() >= 0);
    assert!(dll_register_server() >= 0);
    assert!(dll_unregister_server() >= 0);
}
