//! Boundary and branch tests for the msimtf port. An expectation marked "not oracle-derived" comes from
//! reading the C source or the spec, not from a row the harness printed.

use super::*;

#[test]
fn constructor_checks_the_outer_object_before_allocating() {
    // Not oracle-derived: `pUnkOuter` is tested before `HeapAlloc`, so an allocation failure never hides it.
    assert_eq!(
        active_imm_app_constructor(true, false),
        Err(CLASS_E_NOAGGREGATION)
    );
    // Not oracle-derived: a failed `HeapAlloc` gives `E_OUTOFMEMORY` and no object.
    assert_eq!(active_imm_app_constructor(false, false), Err(E_OUTOFMEMORY));
}

#[test]
fn query_interface_answers_iunknown_and_iactiveimmapp_with_the_app_pointer() {
    // Not oracle-derived: the IID branches of `ActiveIMMApp_QueryInterface`. Each success adds one reference.
    let mut app = ActiveImmApp { ref_count: 1 };
    assert_eq!(
        active_imm_app_query_interface(&mut app, &IID_UNKNOWN),
        Ok(ActiveImmInterface::ActiveImmApp)
    );
    assert_eq!(app.ref_count, 2);
    assert_eq!(
        active_imm_app_query_interface(&mut app, &IID_ACTIVE_IMM_APP),
        Ok(ActiveImmInterface::ActiveImmApp)
    );
    assert_eq!(app.ref_count, 3);
}

#[test]
fn query_interface_answers_the_pump_owner_and_refuses_other_iids_without_a_reference() {
    // Not oracle-derived: the pump IID selects the pump pointer, and an unknown IID adds no reference.
    let mut app = ActiveImmApp { ref_count: 1 };
    assert_eq!(
        active_imm_app_query_interface(&mut app, &IID_ACTIVE_IMM_MESSAGE_PUMP_OWNER),
        Ok(ActiveImmInterface::MessagePumpOwner)
    );
    assert_eq!(app.ref_count, 2);
    assert_eq!(
        active_imm_app_query_interface(&mut app, &IID_CLASS_FACTORY),
        Err(E_NOINTERFACE)
    );
    assert_eq!(app.ref_count, 2);
}

#[test]
fn add_ref_and_release_move_the_count_by_one() {
    // Not oracle-derived: the arithmetic of `InterlockedIncrement` and `InterlockedDecrement`.
    let mut app = ActiveImmApp { ref_count: 1 };
    assert_eq!(active_imm_app_add_ref(&mut app), 2);
    assert_eq!(active_imm_app_release(&mut app), 1);
    // The count reaches 0 here, which is the point where the C destructor runs. The port frees nothing.
    assert_eq!(active_imm_app_release(&mut app), 0);
}

#[test]
fn add_ref_and_release_wrap_at_the_limits_of_the_long_count() {
    // Not oracle-derived: the Interlocked functions wrap, so the unsigned results follow the two's
    // complement of the `LONG` count.
    let mut app = ActiveImmApp {
        ref_count: i32::MAX,
    };
    assert_eq!(active_imm_app_add_ref(&mut app), 0x8000_0000);
    assert_eq!(active_imm_app_release(&mut app), 0x7FFF_FFFF);
    let mut app = ActiveImmApp { ref_count: 0 };
    assert_eq!(active_imm_app_release(&mut app), u32::MAX);
}

#[test]
fn class_factory_query_interface_answers_iunknown_and_iclassfactory() {
    // Not oracle-derived: the IID branches of `ClassFactory_QueryInterface`. Row 16 covers the refusal of a
    // non-matching CLSID, and row 15 covers `IID_IClassFactory` through `DllGetClassObject`.
    assert_eq!(
        class_factory_query_interface(&IID_UNKNOWN),
        Ok(ClassFactory)
    );
    assert_eq!(
        class_factory_query_interface(&IID_CLASS_FACTORY),
        Ok(ClassFactory)
    );
    assert_eq!(
        class_factory_query_interface(&IID_ACTIVE_IMM_APP),
        Err(E_NOINTERFACE)
    );
}

#[test]
fn class_factory_create_instance_returns_each_selected_interface_with_one_reference() {
    // Not oracle-derived: row 25 covers `IID_IActiveIMMApp`. These cover the other two accepted IIDs.
    let (obj, interface) = class_factory_create_instance(&IID_UNKNOWN, false, true).unwrap();
    assert_eq!(
        (obj.ref_count, interface),
        (1, ActiveImmInterface::ActiveImmApp)
    );
    let (obj, interface) =
        class_factory_create_instance(&IID_ACTIVE_IMM_MESSAGE_PUMP_OWNER, false, true).unwrap();
    assert_eq!(
        (obj.ref_count, interface),
        (1, ActiveImmInterface::MessagePumpOwner)
    );
}

#[test]
fn class_factory_create_instance_reports_each_failure() {
    // Not oracle-derived: the three failure paths of `ClassFactory_CreateInstance`, in C order.
    assert_eq!(
        class_factory_create_instance(&IID_ACTIVE_IMM_APP, true, true).unwrap_err(),
        CLASS_E_NOAGGREGATION
    );
    assert_eq!(
        class_factory_create_instance(&IID_ACTIVE_IMM_APP, false, false).unwrap_err(),
        E_OUTOFMEMORY
    );
    // The object is created, the query fails, and the object is released.
    assert_eq!(
        class_factory_create_instance(&IID_CLASS_FACTORY, false, true).unwrap_err(),
        E_NOINTERFACE
    );
}

#[test]
fn dll_get_class_object_queries_the_factory_only_for_the_matching_clsid() {
    // Not oracle-derived: the matching CLSID reaches the factory's `QueryInterface`, and its result is returned.
    assert_eq!(
        dll_get_class_object(&CLSID_CACTIVE_IMM, &IID_UNKNOWN),
        Ok(ClassFactory)
    );
    assert_eq!(
        dll_get_class_object(&CLSID_CACTIVE_IMM, &IID_ACTIVE_IMM_APP),
        Err(E_NOINTERFACE)
    );
    assert_eq!(
        dll_get_class_object(&IID_CLASS_FACTORY, &IID_CLASS_FACTORY),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
}

#[test]
fn export_table_lists_the_spec_names_in_spec_order() {
    // Not oracle-derived: the names and their order come from `dlls/msimtf/msimtf.spec`.
    let names: Vec<&str> = EXPORTS.iter().map(|(name, _)| *name).collect();
    assert_eq!(
        names,
        [
            "DllCanUnloadNow",
            "DllGetClassObject",
            "DllRegisterServer",
            "DllUnregisterServer",
            "MsimtfIsGuidMapEnable",
            "MsimtfIsWindowFiltered",
        ]
    );
}

#[test]
fn export_kind_finds_names_case_sensitively() {
    // Not oracle-derived: the lookup rules of `export_kind`.
    assert_eq!(export_kind("DllGetClassObject"), Some(ExportKind::Private));
    assert_eq!(
        export_kind("MsimtfIsWindowFiltered"),
        Some(ExportKind::Stub)
    );
    assert_eq!(export_kind("dllgetclassobject"), None);
    assert_eq!(export_kind("MsimtfIsWindowFiltered "), None);
    assert_eq!(export_kind("MergeFontPackage"), None);
}

#[test]
fn failed_is_a_sign_test() {
    // Not oracle-derived: `FAILED` from `include/winerror.h`.
    assert!(!failed(S_OK));
    assert!(!failed(S_FALSE));
    assert!(failed(E_NOTIMPL));
    assert!(failed(i32::MIN));
    assert!(!failed(i32::MAX));
}

#[test]
fn hresult_constants_have_the_winerror_values() {
    // Not oracle-derived: the values from `include/winerror.h`. Rows 01 to 25 print the same values.
    assert_eq!(S_OK as u32, 0x0000_0000);
    assert_eq!(S_FALSE as u32, 0x0000_0001);
    assert_eq!(E_FAIL as u32, 0x8000_4005);
    assert_eq!(E_NOTIMPL as u32, 0x8000_4001);
    assert_eq!(E_NOINTERFACE as u32, 0x8000_4002);
    assert_eq!(E_OUTOFMEMORY as u32, 0x8007_000E);
    assert_eq!(CLASS_E_NOAGGREGATION as u32, 0x8004_0110);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE as u32, 0x8004_0111);
}

#[test]
fn is_equal_guid_compares_every_field() {
    // Not oracle-derived: `IsEqualGUID` compares the sixteen bytes.
    assert!(is_equal_guid(&IID_UNKNOWN, &IID_UNKNOWN));
    assert!(!is_equal_guid(&IID_UNKNOWN, &IID_CLASS_FACTORY));
    let mut near = IID_UNKNOWN;
    near.data4[7] = 0x47;
    assert!(!is_equal_guid(&IID_UNKNOWN, &near));
}

#[test]
fn wine_spec_unimplemented_stub_records_the_module_and_the_function() {
    // Not oracle-derived: the record shape, with a module and function that are not msimtf exports.
    assert_eq!(
        wine_spec_unimplemented_stub("fake.dll", "Fake"),
        StubException {
            code: EXCEPTION_WINE_STUB,
            flags: EXCEPTION_NONCONTINUABLE,
            number_parameters: 2,
            module: "fake.dll",
            function: "Fake",
        }
    );
}

#[test]
fn stub_exception_constants_match_the_wine_headers() {
    // Not oracle-derived: from `include/winternl.h` and `include/winnt.h`. Rows 21 and 22 print the same values.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x1);
}
