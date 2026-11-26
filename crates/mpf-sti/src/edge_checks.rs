//! Boundary and branch tests for `mpf-sti`.
//!
//! The oracle rows in `oracle_checks.rs` cover the main paths. The tests here cover the
//! remaining branches. Each expectation that the oracle rows do not give is marked
//! "Not oracle-derived" and names its source.

use super::*;
use crate::oracle_checks::{hr, outer, wide};

#[test]
fn hresult_from_win32_boundaries() {
    // Not oracle-derived: boundary values of HRESULT_FROM_WIN32 (include/winerror.h). Only
    // ERROR_SUCCESS, 2, 5 and 0x80000001 are in the oracle.
    assert_eq!(hresult_from_win32(0), S_OK);
    assert_eq!(hresult_from_win32(1), hr(0x8007_0001));
    assert_eq!(hresult_from_win32(0x0001_0000), hr(0x8007_0000));
    assert_eq!(hresult_from_win32(0x7FFF_FFFF), hr(0x8007_FFFF));
    assert_eq!(hresult_from_win32(0x8000_0000), hr(0x8000_0000));
    assert_eq!(hresult_from_win32(0xFFFF_FFFF), hr(0xFFFF_FFFF));
}

#[test]
fn com_constants_match_winerror_and_sti_guids() {
    // Not oracle-derived: HRESULT values from include/winerror.h, and the three sti.h GUIDs
    // (target/canon/sti/guids_sti.h). IUnknown and IClassFactory are the standard OLE IIDs,
    // which this tree does not define.
    assert_eq!(S_OK, 0);
    assert_eq!(S_FALSE, 1);
    assert_eq!(E_NOINTERFACE, hr(0x8000_4002));
    assert_eq!(E_NOTIMPL, hr(0x8000_4001));
    assert_eq!(E_OUTOFMEMORY, hr(0x8007_000E));
    assert_eq!(CLASS_E_NOAGGREGATION, hr(0x8004_0110));
    assert_eq!(STG_E_UNIMPLEMENTEDFUNCTION, hr(0x8003_00FE));
    assert_eq!(
        CLSID_STI,
        Guid {
            data1: 0xB323_F8E0,
            data2: 0x2E68,
            data3: 0x11D0,
            data4: [0x90, 0xEA, 0x00, 0xAA, 0x00, 0x60, 0xF8, 0x6C],
        }
    );
    assert_eq!(
        IID_ISTILLIMAGEW,
        Guid {
            data1: 0x641B_D880,
            data2: 0x2DC8,
            data3: 0x11D0,
            data4: [0x90, 0xEA, 0x00, 0xAA, 0x00, 0x60, 0xF8, 0x6C],
        }
    );
    assert_eq!(
        IID_ISTILLIMAGEA,
        Guid {
            data1: 0xA7B1_F740,
            data2: 0x1D7F,
            data3: 0x11D1,
            data4: [0xAC, 0xA9, 0x00, 0xA0, 0x24, 0x38, 0xAD, 0x48],
        }
    );
    assert_eq!(
        IID_IUNKNOWN,
        Guid {
            data1: 0,
            data2: 0,
            data3: 0,
            data4: [0xC0, 0, 0, 0, 0, 0, 0, 0x46],
        }
    );
    assert_eq!(
        IID_ICLASSFACTORY,
        Guid {
            data1: 1,
            data2: 0,
            data3: 0,
            data4: [0xC0, 0, 0, 0, 0, 0, 0, 0x46],
        }
    );
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the seven lines of dlls/sti/sti.spec, in spec order.
    let names: Vec<&str> = EXPORTS.iter().map(|(name, _)| *name).collect();
    assert_eq!(
        names,
        [
            "DllCanUnloadNow",
            "DllGetClassObject",
            "DllRegisterServer",
            "DllUnregisterServer",
            "StiCreateInstance",
            "StiCreateInstanceA",
            "StiCreateInstanceW",
        ]
    );
}

#[test]
fn export_kind_classifies_each_name() {
    // Not oracle-derived: the kinds come from the spec lines. The alias and the name lookup are
    // case-sensitive, as in the spec.
    assert_eq!(
        export_kind("DllCanUnloadNow"),
        Some(ExportKind::Winecrt0Default)
    );
    assert_eq!(export_kind("DllGetClassObject"), Some(ExportKind::Function));
    assert_eq!(export_kind("DllRegisterServer"), Some(ExportKind::Function));
    assert_eq!(
        export_kind("DllUnregisterServer"),
        Some(ExportKind::Function)
    );
    assert_eq!(
        export_kind("StiCreateInstance"),
        Some(ExportKind::Alias("StiCreateInstanceW"))
    );
    assert_eq!(
        export_kind("StiCreateInstanceA"),
        Some(ExportKind::Function)
    );
    assert_eq!(
        export_kind("StiCreateInstanceW"),
        Some(ExportKind::Function)
    );
    assert_eq!(export_kind("StiCreateInstanceX"), None);
    assert_eq!(export_kind("dllcanunloadnow"), None);
    assert_eq!(
        format!("{:?}", ExportKind::Alias("StiCreateInstanceW").clone()),
        "Alias(\"StiCreateInstanceW\")"
    );
}

#[test]
fn derived_traits_are_exercised() {
    // Not oracle-derived: the Debug and Clone impls of the public value types, which the
    // coverage gate counts as functions.
    let face = Face::StillImageW;
    assert_eq!(face.clone(), Face::StillImageW);
    assert_eq!(format!("{face:?}"), "StillImageW");
    assert_eq!(format!("{:?}", Face::Inner), "Inner");
    assert_eq!(IID_IUNKNOWN.clone(), IID_IUNKNOWN);
    assert!(format!("{:?}", IID_IUNKNOWN).starts_with("Guid {"));
}

#[test]
fn launch_command_data_with_empty_command_keeps_the_separator() {
    // Not oracle-derived: the swprintf of L"%s %s" with an empty first %s gives " " and the
    // suffix. A command line that starts with NUL is empty as well.
    let mut expected = wide(" /StiDevice:%1 /StiEvent:%2");
    expected.push(0);
    assert_eq!(launch_command_data(&[]), expected);
    assert_eq!(launch_command_data(&[0, u16::from(b'x')]), expected);
}

#[test]
fn registered_apps_launch_path_literal() {
    // Not oracle-derived: the registeredAppsLaunchPath literal in dlls/sti/sti.c.
    assert_eq!(
        REGISTERED_APPS_LAUNCH_PATH,
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\StillImage\\Registered Applications"
    );
}

#[test]
fn inner_face_ref_counting_does_not_touch_outer() {
    // Not oracle-derived: the oracle rows with an outer object only AddRef or Release through
    // the IStillImageW face. Internal_AddRef and Internal_Release in sti.c change the inner
    // count only.
    let (o, state) = outer(S_OK);
    let (mut obj, _) = sti_create_instance_w(o, true).unwrap();
    assert_eq!(obj.add_ref(Face::Inner), 2);
    assert_eq!(obj.release(Face::Inner), 1);
    assert_eq!(state.borrow().refs, 5);
}

#[test]
fn inner_query_interface_rejects_other_iids_without_addref() {
    // Not oracle-derived: with an outer object, Internal_QueryInterface refuses an IID other
    // than IUnknown and IStillImageW without an AddRef and without asking the outer object.
    // The oracle covers this refusal only without an outer object (R16).
    let (o, state) = outer(S_OK);
    let (mut obj, _) = sti_create_instance_w(o, true).unwrap();
    assert_eq!(
        obj.query_interface(Face::Inner, &IID_ISTILLIMAGEA),
        Err(E_NOINTERFACE)
    );
    assert_eq!(obj.ref_count(), 1);
    assert_eq!(state.borrow().refs, 5);
    assert_eq!(state.borrow().qi_calls, 0);
}

#[test]
fn stillimage_face_forwards_every_iid_to_outer() {
    // Not oracle-derived: stillimagew_QueryInterface passes riid to the outer object without
    // checking it. An IID that the inner object refuses still reaches the outer object.
    let (o, state) = outer(E_NOINTERFACE);
    let (mut obj, _) = sti_create_instance_w(o, true).unwrap();
    assert_eq!(
        obj.query_interface(Face::Inner, &IID_ISTILLIMAGEW),
        Ok(Some(Face::StillImageW))
    );
    assert_eq!(
        obj.query_interface(Face::StillImageW, &IID_ISTILLIMAGEA),
        Err(E_NOINTERFACE)
    );
    assert_eq!(state.borrow().qi_calls, 1);
}

#[test]
fn class_factory_refuses_aggregation_before_checking_the_iid() {
    // Not oracle-derived: sti_create tests pUnkOuter before riid, so every IID except IUnknown
    // gives CLASS_E_NOAGGREGATION with an outer object. The oracle covers only IStillImageW (R29).
    for riid in [&IID_ISTILLIMAGEA, &IID_ICLASSFACTORY, &IID_ISTILLIMAGEW] {
        let (o, _) = outer(S_OK);
        assert_eq!(
            class_factory_create_instance(riid, o, true).map(|(_, f)| f),
            Err(CLASS_E_NOAGGREGATION)
        );
    }
}

#[test]
fn class_factory_iunknown_with_outer_reports_alloc_failure() {
    // Not oracle-derived: the oracle has an allocation failure only without an outer object
    // (R34). StiCreateInstanceW fails before it uses pUnkOuter, so the outer count stays put.
    let (o, state) = outer(S_OK);
    assert_eq!(
        class_factory_create_instance(&IID_IUNKNOWN, o, false).map(|(_, f)| f),
        Err(E_OUTOFMEMORY)
    );
    assert_eq!(state.borrow().refs, 5);
}

#[test]
fn class_factory_stillimage_w_reports_alloc_failure() {
    // Not oracle-derived: the oracle's allocation failure (R34) uses IUnknown. sti_create fails
    // before it reads pUnkOuter, so the IStillImageW path reports E_OUTOFMEMORY as well.
    assert_eq!(
        class_factory_create_instance(&IID_ISTILLIMAGEW, None, false).map(|(_, f)| f),
        Err(E_OUTOFMEMORY)
    );
}
