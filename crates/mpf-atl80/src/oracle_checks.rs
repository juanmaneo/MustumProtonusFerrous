//! Oracle-derived test vectors for the ported atl80 logic.
//!
//! Vectors were captured from `target/canon/atl80/harness.c`, which compiles the unmodified
//! `dlls/atl80/atl80.c` with Windows header shims, plus the verbatim
//! `__wine_spec_unimplemented_stub` body from `dlls/ntdll/exception.c`, with
//! `gcc -fsanitize=address,undefined -g`. The run printed 24 rows: 16 for
//! `AtlComModuleRegisterServer` and 8 for the `@ stub` exports. ASan and UBSan reported nothing,
//! so no vector is excluded.

use super::*;
use std::cell::RefCell;

/// `E_FAIL` from `include/winerror.h`, as a signed HRESULT.
const E_FAIL: Hresult = 0x8000_4005_u32 as Hresult;

const CLSID_A: Guid = Guid {
    data1: 0x41,
    data2: 0,
    data3: 0,
    data4: [0; 8],
};
const CLSID_B: Guid = Guid {
    data1: 0x42,
    data2: 0,
    data3: 0,
    data4: [0; 8],
};
const CATID_X: Guid = Guid {
    data1: 0x58,
    data2: 0,
    data3: 0,
    data4: [0; 8],
};
static CATMAP: [CatMapEntry; 1] = [CatMapEntry {
    i_type: 1,
    pcatid: CATID_X,
}];

thread_local! {
    static LOG: RefCell<String> = const { RefCell::new(String::new()) };
}

fn log(token: &str) {
    LOG.with(|log| log.borrow_mut().push_str(token));
}

fn take_log() -> String {
    LOG.with(|log| std::mem::take(&mut *log.borrow_mut()))
}

// The callbacks log the same tokens as the C harness: `a`, `b` or `f` for the update call,
// `c<letter>` for the category call, and `t<hinst>` or `T<hinst>` for the type library call.

fn upd_a(_: bool) -> Hresult {
    log("a");
    S_OK
}

fn upd_b(_: bool) -> Hresult {
    log("b");
    S_OK
}

fn upd_fail(_: bool) -> Hresult {
    log("f");
    E_FAIL
}

fn cats_none() -> Option<&'static [CatMapEntry]> {
    None
}

fn cats_some() -> Option<&'static [CatMapEntry]> {
    Some(&CATMAP)
}

fn reg_cats_ok(clsid: &Guid, _: &[CatMapEntry], _: bool) -> Hresult {
    log(&format!("c{}", char::from(clsid.data1 as u8)));
    S_OK
}

fn reg_cats_fail(clsid: &Guid, catmap: &[CatMapEntry], reg: bool) -> Hresult {
    reg_cats_ok(clsid, catmap, reg);
    E_FAIL
}

fn reg_tlb_ok(hinst: HInstance) -> Hresult {
    log(&format!("t{hinst}"));
    S_OK
}

fn reg_tlb_fail(hinst: HInstance) -> Hresult {
    log(&format!("T{hinst}"));
    E_FAIL
}

const A_NONE: Option<ObjMapEntry> = Some(ObjMapEntry {
    pclsid: CLSID_A,
    update_registry: upd_a,
    get_category_map: cats_none,
});
const A_SOME: Option<ObjMapEntry> = Some(ObjMapEntry {
    pclsid: CLSID_A,
    update_registry: upd_a,
    get_category_map: cats_some,
});
const A_FAIL_SOME: Option<ObjMapEntry> = Some(ObjMapEntry {
    pclsid: CLSID_A,
    update_registry: upd_fail,
    get_category_map: cats_some,
});
const B_NONE: Option<ObjMapEntry> = Some(ObjMapEntry {
    pclsid: CLSID_B,
    update_registry: upd_b,
    get_category_map: cats_none,
});
const B_SOME: Option<ObjMapEntry> = Some(ObjMapEntry {
    pclsid: CLSID_B,
    update_registry: upd_b,
    get_category_map: cats_some,
});
const B_FAIL_SOME: Option<ObjMapEntry> = Some(ObjMapEntry {
    pclsid: CLSID_B,
    update_registry: upd_fail,
    get_category_map: cats_some,
});

/// Runs the registration with the type library handle 7, as the C harness does. The `_fails`
/// flags pick the failing category or type library callback. Returns the HRESULT and the log.
fn run(
    obj_map: &[Option<ObjMapEntry>],
    register_type_lib: bool,
    clsid: Option<&Guid>,
    cats_fails: bool,
    tlb_fails: bool,
) -> (Hresult, String) {
    let cats: fn(&Guid, &[CatMapEntry], bool) -> Hresult = if cats_fails {
        reg_cats_fail
    } else {
        reg_cats_ok
    };
    let tlb: fn(HInstance) -> Hresult = if tlb_fails { reg_tlb_fail } else { reg_tlb_ok };
    take_log();
    let ret = atl_com_module_register_server(obj_map, 7, register_type_lib, clsid, cats, tlb);
    (ret, take_log())
}

#[test]
fn oracle_atl_com_module_register_server_rows() {
    // row 01 [] clsid=- reg_tlb=0 cats=ok tlb=ok -> ret=0x00000000 log=""
    assert_eq!(run(&[], false, None, false, false), (S_OK, String::new()));

    // row 02 [] clsid=- reg_tlb=1 cats=ok tlb=ok -> ret=0x00000000 log="t7"
    assert_eq!(run(&[], true, None, false, false), (S_OK, "t7".into()));

    // row 03 [] clsid=- reg_tlb=1 cats=ok tlb=fail -> ret=0x80004005 log="T7"
    assert_eq!(run(&[], true, None, false, true), (E_FAIL, "T7".into()));

    // row 04 [A:none] clsid=- reg_tlb=0 cats=ok tlb=ok -> ret=0x00000000 log="a"
    assert_eq!(
        run(&[A_NONE], false, None, false, false),
        (S_OK, "a".into())
    );

    // row 05 [A:some] clsid=- reg_tlb=0 cats=ok tlb=ok -> ret=0x00000000 log="acA"
    assert_eq!(
        run(&[A_SOME], false, None, false, false),
        (S_OK, "acA".into())
    );

    // row 06 [A:some] clsid=- reg_tlb=1 cats=fail tlb=ok -> ret=0x80004005 log="acA"
    assert_eq!(
        run(&[A_SOME], true, None, true, false),
        (E_FAIL, "acA".into())
    );

    // row 07 [A:fail-update] clsid=- reg_tlb=0 cats=ok tlb=ok -> ret=0x80004005 log="f"
    assert_eq!(
        run(&[A_FAIL_SOME], false, None, false, false),
        (E_FAIL, "f".into())
    );

    // row 08 [A:some,B:some] clsid=- reg_tlb=1 cats=ok tlb=ok -> ret=0x00000000 log="acAbcBt7"
    assert_eq!(
        run(&[A_SOME, B_SOME], true, None, false, false),
        (S_OK, "acAbcBt7".into())
    );

    // row 09 [A:none,B:some] clsid=B reg_tlb=0 cats=ok tlb=ok -> ret=0x00000000 log="bcB"
    assert_eq!(
        run(&[A_NONE, B_SOME], false, Some(&CLSID_B), false, false),
        (S_OK, "bcB".into())
    );

    // row 10 [A:none,B:some] clsid=A reg_tlb=0 cats=ok tlb=ok -> ret=0x00000000 log="a"
    assert_eq!(
        run(&[A_NONE, B_SOME], false, Some(&CLSID_A), false, false),
        (S_OK, "a".into())
    );

    // row 11 [NULL,A:none] clsid=- reg_tlb=0 cats=ok tlb=ok -> ret=0x00000000 log="a"
    assert_eq!(
        run(&[None, A_NONE], false, None, false, false),
        (S_OK, "a".into())
    );

    // row 12 [A:some,B:fail-update] clsid=- reg_tlb=0 cats=ok tlb=ok -> ret=0x80004005 log="acAf"
    assert_eq!(
        run(&[A_SOME, B_FAIL_SOME], false, None, false, false),
        (E_FAIL, "acAf".into())
    );

    // row 13 [A:some] clsid=B reg_tlb=1 cats=ok tlb=fail -> ret=0x80004005 log="T7"
    assert_eq!(
        run(&[A_SOME], true, Some(&CLSID_B), false, true),
        (E_FAIL, "T7".into())
    );

    // row 14 [A:none,B:none] clsid=- reg_tlb=1 cats=ok tlb=fail -> ret=0x80004005 log="abT7"
    assert_eq!(
        run(&[A_NONE, B_NONE], true, None, false, true),
        (E_FAIL, "abT7".into())
    );

    // row 15 [A:some,B:some] clsid=B reg_tlb=1 cats=fail tlb=ok -> ret=0x80004005 log="bcB"
    assert_eq!(
        run(&[A_SOME, B_SOME], true, Some(&CLSID_B), true, false),
        (E_FAIL, "bcB".into())
    );

    // row 16 [NULL,NULL] clsid=- reg_tlb=1 cats=ok tlb=ok -> ret=0x00000000 log="t7"
    assert_eq!(
        run(&[None, None], true, None, false, false),
        (S_OK, "t7".into())
    );
}

#[test]
fn oracle_stub_rows() {
    // row 17 AtlSetErrorInfo -> code=0x80000100 flags=0x1 nparams=2 module="atl80.dll" function="AtlSetErrorInfo"
    assert_eq!(
        atl_set_error_info(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "atl80.dll",
            function: "AtlSetErrorInfo",
        }
    );

    // row 18 AtlDevModeW2A -> code=0x80000100 flags=0x1 nparams=2 module="atl80.dll" function="AtlDevModeW2A"
    assert_eq!(
        atl_dev_mode_w2a(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "atl80.dll",
            function: "AtlDevModeW2A",
        }
    );

    // row 19 AtlWinModuleRegisterWndClassInfoW -> code=0x80000100 flags=0x1 nparams=2 module="atl80.dll" function="AtlWinModuleRegisterWndClassInfoW"
    assert_eq!(
        atl_win_module_register_wnd_class_info_w(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "atl80.dll",
            function: "AtlWinModuleRegisterWndClassInfoW",
        }
    );

    // row 20 AtlWinModuleRegisterWndClassInfoA -> code=0x80000100 flags=0x1 nparams=2 module="atl80.dll" function="AtlWinModuleRegisterWndClassInfoA"
    assert_eq!(
        atl_win_module_register_wnd_class_info_a(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "atl80.dll",
            function: "AtlWinModuleRegisterWndClassInfoA",
        }
    );

    // row 21 AtlUnRegisterTypeLib -> code=0x80000100 flags=0x1 nparams=2 module="atl80.dll" function="AtlUnRegisterTypeLib"
    assert_eq!(
        atl_un_register_type_lib(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "atl80.dll",
            function: "AtlUnRegisterTypeLib",
        }
    );

    // row 22 AtlWinModuleRegisterClassExW -> code=0x80000100 flags=0x1 nparams=2 module="atl80.dll" function="AtlWinModuleRegisterClassExW"
    assert_eq!(
        atl_win_module_register_class_ex_w(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "atl80.dll",
            function: "AtlWinModuleRegisterClassExW",
        }
    );

    // row 23 AtlWinModuleRegisterClassExA -> code=0x80000100 flags=0x1 nparams=2 module="atl80.dll" function="AtlWinModuleRegisterClassExA"
    assert_eq!(
        atl_win_module_register_class_ex_a(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "atl80.dll",
            function: "AtlWinModuleRegisterClassExA",
        }
    );

    // row 24 AtlWinModuleTerm -> code=0x80000100 flags=0x1 nparams=2 module="atl80.dll" function="AtlWinModuleTerm"
    assert_eq!(
        atl_win_module_term(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "atl80.dll",
            function: "AtlWinModuleTerm",
        }
    );
}
