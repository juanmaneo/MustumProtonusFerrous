//! Oracle-derived test vectors for the ported dhtmled.ocx logic.
//!
//! Vectors were captured by compiling the verbatim Wine bodies of `dlls/dhtmled.ocx/edit.c`,
//! `dlls/dhtmled.ocx/main.c` and `dlls/kernelbase/main.c` (`MulDiv`) with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/dhtmled.ocx/` and running the
//! harness. It printed 25 rows. Row R19 (`MulDiv(INT_MIN, 1, -1)`) is flagged by UBSan
//! (negation overflow), so it is excluded and 24 rows remain. The harness uses placeholder
//! GUIDs (only equality is observed) and counters for COM calls.

use super::*;

#[test]
fn oracle_qi_rows() {
    // R01 qi IUnknown -> S_OK, DHTMLEdit
    assert_eq!(dhtml_edit_qi(Iid::IUnknown), Some(Target::DHTMLEdit));
    // R02 qi IOleObject -> S_OK, OleObject
    assert_eq!(dhtml_edit_qi(Iid::IOleObject), Some(Target::OleObject));
    // R03 qi IProvideClassInfo -> S_OK, ProvideClassInfo2
    assert_eq!(
        dhtml_edit_qi(Iid::IProvideClassInfo),
        Some(Target::ProvideClassInfo2)
    );
    // R04 qi IViewObject2 -> S_OK, ViewObjectEx
    assert_eq!(dhtml_edit_qi(Iid::IViewObject2), Some(Target::ViewObjectEx));
    // R05 qi IClassFactory -> E_NOINTERFACE, NULL
    assert_eq!(dhtml_edit_qi(Iid::IClassFactory), None);
}

#[test]
fn oracle_create_rows() {
    // R06 create IOleObject -> S_OK, OleObject, ref 1
    assert_eq!(
        dhtml_edit_create(Iid::IOleObject),
        Ok((Target::OleObject, 1))
    );
    // R07 create IClassFactory -> E_NOINTERFACE
    assert_eq!(dhtml_edit_create(Iid::IClassFactory), Err(E_NOINTERFACE));
    // R20 create IDHTMLEdit with dpi 96x144 -> S_OK, extent 5080x3387, ref 1
    assert_eq!(
        dhtml_edit_create(Iid::IDHTMLEdit),
        Ok((Target::DHTMLEdit, 1))
    );
    assert_eq!(default_extent(96, 144), Sizel { cx: 5080, cy: 3387 });
}

#[test]
fn oracle_class_factory_rows() {
    // R08 ClassFactory QI IClassFactory -> S_OK
    assert_eq!(class_factory_query_interface(Iid::IClassFactory), S_OK);
    // R09 ClassFactory QI IDHTMLEdit -> E_NOINTERFACE
    assert_eq!(
        class_factory_query_interface(Iid::IDHTMLEdit),
        E_NOINTERFACE
    );
    // R10 CreateInstance with an outer -> CLASS_E_NOAGGREGATION
    assert_eq!(
        class_factory_create_instance(true, Iid::IDHTMLEdit),
        Err(CLASS_E_NOAGGREGATION)
    );
    // R11 CreateInstance without outer, IDHTMLEdit -> S_OK, ref 1
    assert_eq!(
        class_factory_create_instance(false, Iid::IDHTMLEdit),
        Ok((Target::DHTMLEdit, 1))
    );
}

#[test]
fn oracle_dll_rows() {
    // R12 DllGetClassObject(CLSID_DHTMLEdit, IClassFactory) -> S_OK
    assert_eq!(
        dll_get_class_object(Clsid::DHTMLEdit, Iid::IClassFactory),
        S_OK
    );
    // R13 DllGetClassObject(other CLSID, IClassFactory) -> CLASS_E_CLASSNOTAVAILABLE
    assert_eq!(
        dll_get_class_object(Clsid::Other, Iid::IClassFactory),
        CLASS_E_CLASSNOTAVAILABLE
    );
    // R14 DllMain(DLL_PROCESS_ATTACH) -> DisableThreadLibraryCalls
    assert_eq!(
        dll_main(DLL_PROCESS_ATTACH, false),
        DllMainAction::DisableThreadLibraryCalls
    );
    // R15 DllMain(DLL_PROCESS_DETACH, reserved NULL) -> release_typelib
    assert_eq!(
        dll_main(DLL_PROCESS_DETACH, false),
        DllMainAction::ReleaseTypelib
    );
    // R16 DllMain(DLL_PROCESS_DETACH, reserved set) -> nothing
    assert_eq!(dll_main(DLL_PROCESS_DETACH, true), DllMainAction::Nothing);
}

#[test]
fn oracle_mul_div_rows() {
    // R17 MulDiv(192, 2540, 96) = 5080
    assert_eq!(mul_div(192, 2540, 96), 5080);
    // R18 MulDiv(192, 2540, 0) = -1
    assert_eq!(mul_div(192, 2540, 0), -1);
    // R19 MulDiv(INT_MIN, 1, -1) is excluded: UBSan flags the negation.
}

#[test]
fn oracle_extent_and_ole_rows() {
    // R21 SetExtent aspect 4, size 1x2 -> DV_E_DVASPECT, extent unchanged (0x0)
    let mut extent = Sizel { cx: 0, cy: 0 };
    assert_eq!(
        object_set_extent(&mut extent, 4, Sizel { cx: 1, cy: 2 }),
        DV_E_DVASPECT
    );
    assert_eq!(extent, Sizel { cx: 0, cy: 0 });
    // R22 GetExtent aspect 4 -> E_FAIL, out unchanged (7x9)
    assert_eq!(object_get_extent(&Sizel { cx: 7, cy: 9 }, 4), Err(E_FAIL));
    // R23 DoVerb INPLACEACTIVATE -> S_OK
    assert_eq!(ole_object_do_verb(OLEIVERB_INPLACEACTIVATE), S_OK);
    // R24 DoVerb verb 0 -> E_NOTIMPL
    assert_eq!(ole_object_do_verb(0), E_NOTIMPL);
    // R25 GetViewStatus -> status 3
    assert_eq!(view_object_ex_get_view_status(), 3);
}
