//! Edge-case tests for the ported cdosys logic.
//!
//! Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn class_not_available_constant_matches_winerror() {
    // Oracle-derived: CLASS_E_CLASSNOTAVAILABLE is 0x80040111 in the harness.
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE as u32, 0x8004_0111);
    // Not oracle-derived: an HRESULT failure has the severity bit set.
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE as u32 >> 31, 1);
}

#[test]
fn dll_get_class_object_ignores_every_argument_combination() {
    // Not oracle-derived: the Wine body never inspects its inputs, so every
    // combination of NULL/non-NULL pointers returns the same HRESULT.
    let guid = [0xffu8; 16];
    for clsid in [None, Some(&guid)] {
        for riid in [None, Some(&guid)] {
            for out in [false, true] {
                assert_eq!(
                    dll_get_class_object(clsid, riid, out),
                    CLASS_E_CLASSNOTAVAILABLE
                );
            }
        }
    }
}

#[test]
fn success_codes_have_clear_severity_bit() {
    // Not oracle-derived: SUCCEEDED(hr) is hr >= 0, so the severity bit is clear
    // for S_OK and S_FALSE. The class error sets it, so it reads as negative.
    assert_eq!(S_OK, 0);
    assert_eq!(S_FALSE, 1);
    assert!(dll_get_class_object(None, None, false) < 0);
}

#[test]
fn can_unload_now_never_reports_unloadable() {
    // Not oracle-derived: COM reads S_OK as "may unload" and S_FALSE as "stay
    // loaded". Wine's constant S_FALSE therefore pins the DLL in memory.
    assert_ne!(dll_can_unload_now(), S_OK);
    assert_eq!(dll_can_unload_now(), S_FALSE);
}

#[test]
fn register_and_unregister_agree_on_empty_resource_set() {
    // Not oracle-derived: with no WINE_REGISTRY resources, both paths return
    // the initial S_OK of __wine_(un)register_resources. The oracle rows cover
    // the same thing, so this is a consistency check.
    assert_eq!(dll_register_server(), S_OK);
    assert_eq!(dll_unregister_server(), dll_register_server());
}
