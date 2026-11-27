//! Edge-case tests for the ported atl80 logic.
//!
//! These cover the HRESULT sign boundary, the arguments that the C code passes to its callbacks,
//! the early returns, and the stub table. Not every expectation is oracle-derived; the ones that
//! aren't say so in a comment.

use super::*;
use std::cell::RefCell;

thread_local! {
    static LOG: RefCell<String> = const { RefCell::new(String::new()) };
}

fn log(token: &str) {
    LOG.with(|log| log.borrow_mut().push_str(token));
}

fn take_log() -> String {
    LOG.with(|log| std::mem::take(&mut *log.borrow_mut()))
}

const CLSID: Guid = Guid {
    data1: 0x41,
    data2: 0,
    data3: 0,
    data4: [0; 8],
};

fn upd_record(reg: bool) -> Hresult {
    log(&reg.to_string());
    S_OK
}

fn upd_s_false(_: bool) -> Hresult {
    log("s");
    1
}

fn upd_out_of_memory(_: bool) -> Hresult {
    0x8007_000E_u32 as Hresult
}

fn cats_some() -> Option<&'static [CatMapEntry]> {
    static CATMAP: [CatMapEntry; 0] = [];
    Some(&CATMAP)
}

fn reg_cats_record(_: &Guid, _: &[CatMapEntry], reg: bool) -> Hresult {
    log(&reg.to_string());
    S_OK
}

fn reg_tlb_record(hinst: HInstance) -> Hresult {
    log(&format!("t{hinst}"));
    S_OK
}

fn reg_tlb_out_of_memory(_: HInstance) -> Hresult {
    0x8007_000E_u32 as Hresult
}

fn entry(update_registry: fn(bool) -> Hresult) -> Option<ObjMapEntry> {
    Some(ObjMapEntry {
        pclsid: CLSID,
        update_registry,
        get_category_map: || None,
    })
}

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/winternl.h and include/winnt.h.
    assert_eq!(S_OK, 0);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn failed_is_the_sign_bit() {
    // Not oracle-derived: FAILED(hr) is hr < 0, so the boundary is 0 and -1.
    assert!(failed(-1));
    assert!(!failed(0));
    assert!(!failed(1));
    assert!(failed(i32::MIN));
    assert!(!failed(i32::MAX));
    assert!(failed(0x8000_4005_u32 as Hresult));
}

#[test]
fn update_registry_is_called_with_true_and_category_with_true() {
    // Not oracle-derived: the oracle log does not show the bool arguments. The C code passes
    // TRUE to pfnUpdateRegistry and to AtlRegisterClassCategoriesHelper.
    let obj_map = [Some(ObjMapEntry {
        pclsid: CLSID,
        update_registry: upd_record,
        get_category_map: cats_some,
    })];
    take_log();
    let ret =
        atl_com_module_register_server(&obj_map, 0, false, None, reg_cats_record, reg_tlb_record);
    assert_eq!(ret, S_OK);
    assert_eq!(take_log(), "truetrue");
}

#[test]
fn non_failing_positive_hresult_keeps_the_walk_going() {
    // Not oracle-derived: S_FALSE (1) is not FAILED, so the walk continues to the next entry.
    let obj_map = [entry(upd_s_false), entry(upd_record)];
    take_log();
    let ret =
        atl_com_module_register_server(&obj_map, 0, false, None, reg_cats_record, reg_tlb_record);
    assert_eq!(ret, S_OK);
    assert_eq!(take_log(), "strue");
}

#[test]
fn failure_hresult_is_returned_unchanged() {
    // Not oracle-derived: the oracle failures are all E_FAIL. The port returns the callback's
    // HRESULT as is, whatever its value.
    let obj_map = [entry(upd_out_of_memory)];
    let ret =
        atl_com_module_register_server(&obj_map, 0, false, None, reg_cats_record, reg_tlb_record);
    assert_eq!(ret, 0x8007_000E_u32 as Hresult);

    let ret =
        atl_com_module_register_server(&[], 0, true, None, reg_cats_record, reg_tlb_out_of_memory);
    assert_eq!(ret, 0x8007_000E_u32 as Hresult);
}

#[test]
fn type_library_handle_is_passed_through() {
    // Not oracle-derived: the oracle always uses hinst 7. This checks another handle value.
    take_log();
    let ret =
        atl_com_module_register_server(&[], 0x1234, true, None, reg_cats_record, reg_tlb_record);
    assert_eq!(ret, S_OK);
    assert_eq!(take_log(), "t4660");
}

#[test]
fn empty_category_map_still_calls_the_helper() {
    // Not oracle-derived: a present but empty category map is not the NULL case, so the helper
    // runs with an empty slice.
    fn empty_map() -> Option<&'static [CatMapEntry]> {
        Some(&[])
    }
    let obj_map = [Some(ObjMapEntry {
        pclsid: CLSID,
        update_registry: upd_record,
        get_category_map: empty_map,
    })];
    take_log();
    let ret =
        atl_com_module_register_server(&obj_map, 0, false, None, reg_cats_record, reg_tlb_record);
    assert_eq!(ret, S_OK);
    assert_eq!(take_log(), "truetrue");
}

#[test]
fn stubs_name_their_module_and_export() {
    // Not oracle-derived: checks each helper's function name against the export name in
    // dlls/atl80/atl80.spec and the module name from MODULE in dlls/atl80/Makefile.in.
    let cases = [
        (atl_set_error_info(), "AtlSetErrorInfo"),
        (atl_dev_mode_w2a(), "AtlDevModeW2A"),
        (
            atl_win_module_register_wnd_class_info_w(),
            "AtlWinModuleRegisterWndClassInfoW",
        ),
        (
            atl_win_module_register_wnd_class_info_a(),
            "AtlWinModuleRegisterWndClassInfoA",
        ),
        (atl_un_register_type_lib(), "AtlUnRegisterTypeLib"),
        (
            atl_win_module_register_class_ex_w(),
            "AtlWinModuleRegisterClassExW",
        ),
        (
            atl_win_module_register_class_ex_a(),
            "AtlWinModuleRegisterClassExA",
        ),
        (atl_win_module_term(), "AtlWinModuleTerm"),
    ];
    for (stub, function) in cases {
        assert_eq!(stub.module, "atl80.dll");
        assert_eq!(stub.function, function);
        assert_eq!(stub.code, EXCEPTION_WINE_STUB);
        assert_eq!(stub.flags, EXCEPTION_NONCONTINUABLE);
        assert_eq!(stub.number_parameters, 2);
    }
}

#[test]
fn wine_spec_unimplemented_stub_copies_its_arguments() {
    // Not oracle-derived: the oracle only used the atl80 names. This checks that the module and
    // function arguments are passed through unchanged.
    assert_eq!(
        wine_spec_unimplemented_stub("other.dll", "Ordinal"),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "other.dll",
            function: "Ordinal",
        }
    );
}
