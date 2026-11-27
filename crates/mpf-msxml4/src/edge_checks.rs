//! Edge-case tests for the ported msxml4 tables.
//!
//! These cover the export lookup, the forward target, the coclass lookup, the CLSID table and
//! the manifest constants. Not every expectation is oracle-derived; the ones that aren't say so
//! in a comment.

use super::*;

#[test]
fn export_names_follow_spec_order() {
    // Not oracle-derived: the names in dlls/msxml4/msxml4.spec, in the order the spec lists them.
    let names: Vec<&str> = EXPORTS.iter().map(|entry| entry.name).collect();
    assert_eq!(
        names,
        [
            "DllCanUnloadNow",
            "DllGetClassObject",
            "DllRegisterServer",
            "DllUnregisterServer",
        ]
    );
}

#[test]
fn export_lookup_is_exact_and_case_sensitive() {
    // Not oracle-derived: the lookup rule. Names are case-sensitive. A
    // forward target such as `msxml3.DllGetClassObject` is not an export of this DLL.
    assert_eq!(export_entry("dllcanunloadnow"), None);
    assert_eq!(export_entry("DllCanUnloadNow "), None);
    assert_eq!(export_entry("msxml3.DllGetClassObject"), None);
    assert_eq!(export_entry(""), None);
    for entry in EXPORTS {
        assert_eq!(export_entry(entry.name), Some(entry));
    }
}

#[test]
fn only_dll_get_class_object_is_forwarded() {
    // Not oracle-derived: the spec's forward syntax is `name(args) module.function`. The other
    // three exports have no target, so the port gives them no behaviour.
    for entry in EXPORTS {
        let forwarded = matches!(entry.target, ExportTarget::Forward { .. });
        assert_eq!(
            forwarded,
            entry.name == "DllGetClassObject",
            "{}",
            entry.name
        );
    }
}

#[test]
fn coclass_lookup_is_exact_and_case_sensitive() {
    // Not oracle-derived: the lookup rule. Names are case-sensitive, and the library name MSXML2
    // is not a coclass.
    assert_eq!(coclass_clsid(""), None);
    assert_eq!(coclass_clsid("domdocument"), None);
    assert_eq!(coclass_clsid("DOMDocument "), None);
    assert_eq!(coclass_clsid("MSXML2"), None);
    assert_ne!(coclass_clsid("DOMDocument"), coclass_clsid("DOMDocument40"));
}

#[test]
fn coclass_names_and_clsids_are_unique() {
    // Not oracle-derived: a coclass name and its CLSID each identify one coclass, and the
    // library LIBID is not a CLSID.
    for (i, (name, guid)) in COCLASSES.iter().enumerate() {
        assert_ne!(*guid, LIBID_MSXML2, "{name}");
        for (other, other_guid) in &COCLASSES[i + 1..] {
            assert_ne!(name, other);
            assert_ne!(guid, other_guid, "{name} and {other} share a CLSID");
        }
    }
}

#[test]
fn rt_manifest_is_the_winuser_type() {
    // Not oracle-derived: RT_MANIFEST is 24 in include/winuser.h. The rsrc.rc row gives the
    // same value, which the oracle test checks.
    assert_eq!(RT_MANIFEST, 24);
}
