//! Boundary and branch tests for the msctfp port.
//!
//! Expectations marked "not oracle-derived" have no harness row. They follow the reference
//! control flow or the spec layout directly.

use super::*;

const E_FAIL: Hresult = 0x8000_4005_u32 as Hresult;
const E_OUTOFMEMORY: Hresult = 0x8007_000e_u32 as Hresult;
const S_FALSE: Hresult = 1;

#[test]
fn module_name_is_the_makefile_module() {
    // Not oracle-derived: `MODULE = msctfp.dll` in dlls/msctfp/Makefile.in.
    assert_eq!(MODULE, "msctfp.dll");
}

#[test]
fn export_ordinals_follow_spec_order() {
    // Not oracle-derived: checks the table layout. winebuild's ordinals are in oracle_checks.
    for (index, export) in EXPORTS.iter().enumerate() {
        assert_eq!(usize::from(export.ordinal), index + 1);
    }
}

#[test]
fn find_export_is_exact_and_case_sensitive() {
    // Not oracle-derived: lookup semantics. The hit is oracle row 23.
    assert_eq!(find_export("GetProxyDllInfo").map(|e| e.ordinal), Some(5));
    assert!(find_export("getproxydllinfo").is_none());
    assert!(find_export("GetProxyDllInfo ").is_none());
    assert!(find_export("").is_none());
}

#[test]
fn dll_main_other_reasons_and_second_attach() {
    // Not oracle-derived: reason 4, and a second attach that overwrites the instance. Only the C
    // branch `if (fdwReason == DLL_PROCESS_ATTACH)` decides these.
    let mut state = ProxyDll {
        hinstance: 0x2000,
        thread_library_calls_disabled: false,
    };
    assert!(dll_main(&mut state, 0x1000, 4));
    assert_eq!(
        state,
        ProxyDll {
            hinstance: 0x2000,
            thread_library_calls_disabled: false,
        }
    );

    assert!(dll_main(&mut state, 0x3000, DLL_PROCESS_ATTACH));
    assert_eq!(
        state,
        ProxyDll {
            hinstance: 0x3000,
            thread_library_calls_disabled: true,
        }
    );
}

#[test]
fn enumeration_keeps_the_last_success_code() {
    // Not oracle-derived: a success code other than S_OK is kept, and the loop reads past it.
    use CallbackOutcome::{Set, Untouched};

    assert_eq!(
        enumerate_registry_resources(&[Set(S_FALSE), Set(S_OK)]),
        S_OK
    );
    assert_eq!(
        enumerate_registry_resources(&[Set(S_OK), Set(S_FALSE)]),
        S_FALSE
    );
    assert_eq!(
        enumerate_registry_resources(&[Set(S_FALSE), Untouched, Set(E_FAIL)]),
        S_FALSE
    );
}

#[test]
fn first_failure_decides_and_later_entries_are_not_read() {
    // Not oracle-derived: the loop stops at the first failed HRESULT, so a later failure
    // never replaces it.
    use CallbackOutcome::Set;

    assert_eq!(
        enumerate_registry_resources(&[Set(E_FAIL), Set(E_OUTOFMEMORY)]),
        E_FAIL
    );
    assert_eq!(
        enumerate_registry_resources(&[Set(E_OUTOFMEMORY), Set(E_FAIL)]),
        E_OUTOFMEMORY
    );
}
