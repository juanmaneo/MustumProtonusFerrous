//! Oracle-derived test vectors for the ported apphelp logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `dlls/apphelp/apphelp.c`
//! (lines 26-125) and `__wine_spec_unimplemented_stub` from `libs/winecrt0/stub.c` (lines 34-41)
//! with `gcc -fsanitize=address,undefined -g` under `target/canon/apphelp/`, then running the
//! harness. The 25 printed rows (A1-A6, B1-B11, C1-C5, D1-D3) all ran clean under ASan and
//! UBSan, so none were excluded. The harness also raised the stub for all 155 `@ stub` names
//! (summary row S1, 0 mismatches).
//!
//! Row IDs match the harness output. A row's input is in the comment; the assertion is the
//! printed output. Rows without a pointer result print no such value, so they are not asserted.

use super::*;

#[test]
fn oracle_bool_rows() {
    // A1 ApphelpCheckInstallShieldPackage(NULL,NULL) BOOL=1 last_error=none
    assert!(apphelp_check_install_shield_package());
    // A2 ApphelpCheckMsiPackage(0x1,L"x.msi") BOOL=1 last_error=none
    assert!(apphelp_check_msi_package());
    // A6 ShimFlushCache(NULL,NULL,NULL,0) BOOL=1 last_error=120
    assert_eq!(
        shim_flush_cache(),
        StubResult {
            value: true,
            last_error: 120
        }
    );
    // C1 SetPermLayerState(L"p",L"l",0,1,1) BOOL=1 last_error=120
    // C2 SetPermLayerState(NULL,NULL,0xffffffff,0,0) BOOL=1 last_error=120
    assert_eq!(
        set_perm_layer_state(),
        StubResult {
            value: true,
            last_error: 120
        }
    );
    // C3 SdbSetPermLayerKeys(L"p",L"l",0) BOOL=1 last_error=120
    assert_eq!(
        sdb_set_perm_layer_keys(),
        StubResult {
            value: true,
            last_error: 120
        }
    );
    // C4 SdbGetPermLayerKeys(L"p",buf,&size=4,0) BOOL=1 size=4 last_error=120
    // C5 SdbGetPermLayerKeys(NULL,NULL,NULL,0) BOOL=1 last_error=120
    assert_eq!(
        sdb_get_perm_layer_keys(),
        StubResult {
            value: true,
            last_error: 120
        }
    );
}

#[test]
fn oracle_shell_object_rows() {
    // A3 ApphelpCheckShellObject(clsid,shim=1,flags=NULL) BOOL=1 last_error=none
    assert!(apphelp_check_shell_object(None));

    // (flags before, flags after) for A4 and A5; both return BOOL=1.
    // A4 ApphelpCheckShellObject(clsid,shim=0,flags=0xdeadbeefcafebabe) *flags=0x0
    // A5 ApphelpCheckShellObject(clsid,shim=1,flags=0xffffffffffffffff) *flags=0x0
    let rows: [u64; 2] = [0xdead_beef_cafe_babe, u64::MAX];
    for before in rows {
        let mut flags = before;
        assert!(apphelp_check_shell_object(Some(&mut flags)));
        assert_eq!(flags, 0x0);
    }
}

#[test]
fn oracle_database_rows() {
    // B1 SdbCreateDatabase(L"a",DOS_PATH) HANDLE=0x0 last_error=120
    assert_eq!(
        sdb_create_database(),
        StubResult {
            value: 0,
            last_error: 120
        }
    );
    // B2 SdbCreateDatabase(NULL,NT_PATH) HANDLE=0x0 last_error=120
    assert_eq!(
        sdb_create_database(),
        StubResult {
            value: 0,
            last_error: 120
        }
    );
    // B3 SdbInitDatabase(0,NULL) HANDLE=0x0 last_error=120
    // B4 SdbInitDatabase(0xffffffff,L"p") HANDLE=0x0 last_error=120
    for _ in 0..2 {
        assert_eq!(
            sdb_init_database(),
            StubResult {
                value: 0,
                last_error: 120
            }
        );
    }
    // B5 SdbOpenDatabase(L"p",NT_PATH) HANDLE=0x0 last_error=120
    assert_eq!(
        sdb_open_database(),
        StubResult {
            value: 0,
            last_error: 120
        }
    );
    // B6 SdbGetFirstChild(NULL,0) TAGID=0x0
    // B7 SdbGetFirstChild(0x1234,0x7) TAGID=0x0
    assert_eq!(sdb_get_first_child(), 0x0);
    assert_eq!(sdb_get_first_child(), 0x0);
    // B8 SdbCloseDatabase(0x1234) returned
    sdb_close_database();
}

#[test]
fn oracle_app_patch_dir_rows() {
    // B9 SdbGetAppPatchDir(NULL,buf="AB",2) buf[0]=0x0 buf[1]=0x42
    let mut buf = [0x41_u16, 0x42];
    sdb_get_app_patch_dir(Some(&mut buf[0]), 2);
    assert_eq!(buf, [0x0, 0x42]);

    // B10 SdbGetAppPatchDir(0x1,NULL,2) returned; no buffer is involved
    sdb_get_app_patch_dir(None, 2);

    // B11 SdbGetAppPatchDir(0x1,buf="AB",0) buf[0]=0x41 buf[1]=0x42
    let mut buf = [0x41_u16, 0x42];
    sdb_get_app_patch_dir(Some(&mut buf[0]), 0);
    assert_eq!(buf, [0x41, 0x42]);
}

#[test]
fn oracle_stub_exception_rows() {
    // D1 __wine_spec_unimplemented_stub("apphelp","SdbGetNextChild") code=0x80000100 flags=0x1 nargs=2
    assert_eq!(
        unimplemented_stub("apphelp", "SdbGetNextChild"),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            module: "apphelp",
            function: "SdbGetNextChild",
        }
    );
    // D2 __wine_spec_unimplemented_stub("apphelp","SetPermLayers") code=0x80000100 flags=0x1 nargs=2
    assert_eq!(
        unimplemented_stub("apphelp", "SetPermLayers"),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            module: "apphelp",
            function: "SetPermLayers",
        }
    );
    // D3 __wine_spec_unimplemented_stub("apphelp","SdbApphelpNotifyExSdbApphelpNotifyEx") code=0x80000100 flags=0x1 nargs=2
    assert_eq!(
        unimplemented_stub("apphelp", "SdbApphelpNotifyExSdbApphelpNotifyEx"),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            module: "apphelp",
            function: "SdbApphelpNotifyExSdbApphelpNotifyEx",
        }
    );
}

#[test]
fn oracle_stub_summary_row() {
    // S1 stub_exports checked=155 mismatches=0
    assert_eq!(STUB_EXPORTS.len(), 155);
    for name in STUB_EXPORTS {
        assert_eq!(
            unimplemented_stub("apphelp", name),
            StubException {
                code: 0x8000_0100,
                flags: 0x1,
                module: "apphelp",
                function: name,
            }
        );
    }
}
