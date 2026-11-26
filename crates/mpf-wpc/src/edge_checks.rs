//! Edge-case tests for the ported wpc logic.
//!
//! These cover the IID and CLSID boundaries, the factory and object QueryInterface branches,
//! the export and stub tables, and `LockServer`, which the oracle rows do not print. Not every
//! expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;
use crate::guid::*;

fn other_clsid() -> Guid {
    Guid {
        data1: 0x1234_5678,
        data2: 0x1234,
        data3: 0x1234,
        data4: [1, 2, 3, 4, 5, 6, 7, 8],
    }
}

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/winternl.h and include/winnt.h.
    assert_eq!(S_OK, 0x0000_0000);
    assert_eq!(E_NOTIMPL, 0x8000_4001);
    assert_eq!(E_NOINTERFACE, 0x8000_4002);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE, 0x8004_0111);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn guids_match_the_idl_uuids() {
    // Not oracle-derived: the uuid strings of include/wpcapi.idl and include/unknwn.idl, split
    // into the GUID fields by hand.
    assert_eq!(
        CLSID_WINDOWS_PARENTAL_CONTROLS,
        Guid {
            data1: 0xe77c_c89b,
            data2: 0x7401,
            data3: 0x4c04,
            data4: [0x8c, 0xed, 0x14, 0x9d, 0xb3, 0x5a, 0xdd, 0x04],
        }
    );
    assert_eq!(
        IID_WINDOWS_PARENTAL_CONTROLS_CORE,
        Guid {
            data1: 0x4ff4_0a0f,
            data2: 0x3f3b,
            data3: 0x4d7c,
            data4: [0xa4, 0x1b, 0x4f, 0x39, 0xd7, 0xb4, 0x4d, 0x05],
        }
    );
    assert_eq!(
        IID_WINDOWS_PARENTAL_CONTROLS,
        Guid {
            data1: 0x28b4_d88b,
            data2: 0xe072,
            data3: 0x49e6,
            data4: [0x80, 0x4d, 0x26, 0xed, 0xbe, 0x21, 0xa7, 0xb9],
        }
    );
    assert_eq!(IID_UNKNOWN.data4[7], 0x46);
    assert_eq!(IID_CLASS_FACTORY.data1, 1);
}

#[test]
fn is_equal_guid_compares_every_field() {
    // Not oracle-derived: IsEqualGUID must fail when any single field differs.
    let base = IID_WINDOWS_PARENTAL_CONTROLS;
    assert!(is_equal_guid(&base, &base));
    let mut changed = base;
    changed.data1 ^= 1;
    assert!(!is_equal_guid(&base, &changed));
    let mut changed = base;
    changed.data2 ^= 1;
    assert!(!is_equal_guid(&base, &changed));
    let mut changed = base;
    changed.data3 ^= 1;
    assert!(!is_equal_guid(&base, &changed));
    let mut changed = base;
    changed.data4[7] ^= 1;
    assert!(!is_equal_guid(&base, &changed));
}

#[test]
fn interface_for_iid_maps_only_the_four_known_iids() {
    // Not oracle-derived: each IID in the C comparisons maps to its interface, and every other
    // GUID, including the CLSID and a one-byte change of IID_IUnknown, maps to nothing.
    use crate::interface::{interface_for_iid, Interface};
    assert_eq!(interface_for_iid(&IID_UNKNOWN), Some(Interface::Unknown));
    assert_eq!(
        interface_for_iid(&IID_CLASS_FACTORY),
        Some(Interface::ClassFactory)
    );
    assert_eq!(
        interface_for_iid(&IID_WINDOWS_PARENTAL_CONTROLS_CORE),
        Some(Interface::WindowsParentalControlsCore)
    );
    assert_eq!(
        interface_for_iid(&IID_WINDOWS_PARENTAL_CONTROLS),
        Some(Interface::WindowsParentalControls)
    );
    assert_eq!(interface_for_iid(&CLSID_WINDOWS_PARENTAL_CONTROLS), None);
    assert_eq!(interface_for_iid(&other_clsid()), None);
    let mut near = IID_UNKNOWN;
    near.data4[7] = 0x47;
    assert_eq!(interface_for_iid(&near), None);
}

#[test]
fn class_factory_accepts_only_unknown_and_class_factory() {
    // Not oracle-derived: the factory QueryInterface rejects the two IIDs that only the
    // WindowsParentalControls object answers. Row 08 covers IID_IWindowsParentalControls.
    let mut ppv = Some(Object::ParentalControls);
    assert_eq!(
        class_factory::query_interface(&IID_WINDOWS_PARENTAL_CONTROLS_CORE, &mut ppv),
        E_NOINTERFACE
    );
    assert_eq!(ppv, None);
}

#[test]
fn windows_parental_controls_accepts_its_three_iids_only() {
    // Not oracle-derived: the IIDs that the rows 15 and 16 do not print directly. Core is also
    // reached by row 13 through CreateInstance; the IClassFactory rejection is row 16.
    let mut ppv = None;
    assert_eq!(
        object::query_interface(&IID_WINDOWS_PARENTAL_CONTROLS, &mut ppv),
        S_OK
    );
    assert_eq!(ppv, Some(Object::ParentalControls));

    let mut ppv = Some(Object::ClassFactory);
    assert_eq!(
        object::query_interface(&IID_WINDOWS_PARENTAL_CONTROLS_CORE, &mut ppv),
        S_OK
    );
    assert_eq!(ppv, Some(Object::ParentalControls));

    let mut ppv = Some(Object::ClassFactory);
    assert_eq!(
        object::query_interface(&CLSID_WINDOWS_PARENTAL_CONTROLS, &mut ppv),
        E_NOINTERFACE
    );
    assert_eq!(ppv, None);
}

#[test]
fn dll_get_class_object_leaves_ppv_for_any_other_clsid() {
    // Not oracle-derived: a CLSID one field away from the served CLSID is not served, and the
    // out-parameter is untouched. Row 05 covers a different CLSID.
    let mut near = CLSID_WINDOWS_PARENTAL_CONTROLS;
    near.data1 ^= 1;
    let mut ppv = Some(Object::ClassFactory);
    assert_eq!(
        dll_get_class_object(&near, &IID_UNKNOWN, &mut ppv),
        CLASS_E_CLASSNOTAVAILABLE
    );
    assert_eq!(ppv, Some(Object::ClassFactory));
}

#[test]
fn lock_server_always_succeeds() {
    // Not oracle-derived: LockServer only traces fLock and returns S_OK. The oracle rows did
    // not print this call, so the expectation comes from the C body in dlls/wpc/wpc.c.
    assert_eq!(class_factory::lock_server(), S_OK);
}

#[test]
fn query_hresult_maps_a_match_to_s_ok() {
    // Not oracle-derived: the QueryInterface result mapping in dlls/wpc/wpc.c.
    use crate::hresult::query_hresult;
    assert_eq!(query_hresult(true), S_OK);
    assert_eq!(query_hresult(false), E_NOINTERFACE);
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the five lines of dlls/wpc/wpc.spec, in order.
    assert_eq!(
        EXPORTS,
        [
            ("ApplyParentalControlsW", ExportKind::Stub),
            ("DllCanUnloadNow", ExportKind::Generated),
            ("DllGetClassObject", ExportKind::Ported),
            ("DllRegisterServer", ExportKind::Generated),
            ("DllUnregisterServer", ExportKind::Generated),
        ]
    );
    assert_eq!(
        export_kind("ApplyParentalControlsW"),
        Some(ExportKind::Stub)
    );
    assert_eq!(export_kind("DllCanUnloadNow"), Some(ExportKind::Generated));
    assert_eq!(export_kind("DllGetClassObject"), Some(ExportKind::Ported));
    assert_eq!(
        export_kind("DllRegisterServer"),
        Some(ExportKind::Generated)
    );
    assert_eq!(
        export_kind("DllUnregisterServer"),
        Some(ExportKind::Generated)
    );
    assert_eq!(export_kind("dllgetclassobject"), None);
    assert_eq!(export_kind(""), None);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed names of wpc.spec. The code and
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
