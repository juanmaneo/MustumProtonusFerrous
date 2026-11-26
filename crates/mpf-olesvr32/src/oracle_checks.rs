//! Oracle-derived test vectors for the ported olesvr32 logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of the nine explicit exports in
//! `dlls/olesvr32/olesvr_main.c` and `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`)
//! with `gcc -fsanitize=address,undefined -g` under `target/canon/olesvr32/`, and running the
//! harness. The 17 printed rows include one row (row 17: `OleRegisterServer` with the counter at
//! `INT32_MAX`) that UBSan flags as a signed integer overflow at `++OLE_current_handle`. It is
//! excluded here, so 16 rows remain.

use super::*;

#[test]
fn oracle_register_rows() {
    // The C static counter starts at 0 and is shared by both register exports.
    let mut counter = HandleCounter::default();
    let mut h = -999;

    // row 01 OleRegisterServer(svr=Svr,osu=multi) -> ret=0 hRet=1
    assert_eq!(ole_register_server(&mut counter, &mut h), OLE_OK);
    assert_eq!(h, 1);

    // row 02 OleRegisterServer(svr=Svr2,osu=single) -> ret=0 hRet=2
    assert_eq!(ole_register_server(&mut counter, &mut h), OLE_OK);
    assert_eq!(h, 2);

    // row 03 OleRegisterServerDoc(h=2,doc=Doc1) -> ret=0 hRet=3
    assert_eq!(ole_register_server_doc(&mut counter, &mut h), OLE_OK);
    assert_eq!(h, 3);

    // row 04 OleRegisterServer(svr=Svr3,osu=multi) -> ret=0 hRet=4
    assert_eq!(ole_register_server(&mut counter, &mut h), OLE_OK);
    assert_eq!(h, 4);

    // row 05 OleRegisterServerDoc(h=4,doc=Doc2) -> ret=0 hRet=5
    assert_eq!(ole_register_server_doc(&mut counter, &mut h), OLE_OK);
    assert_eq!(h, 5);
}

#[test]
fn oracle_block_unblock_rows() {
    // row 06 OleBlockServer(h=5) -> ret=0
    assert_eq!(ole_block_server(), OLE_OK);

    // row 07 OleUnblockServer(h=5,block_in=1) -> ret=0 block_out=0
    let mut block = 1;
    assert_eq!(ole_unblock_server(&mut block), OLE_OK);
    assert_eq!(block, 0);

    // row 08 OleUnblockServer(h=5,block_in=-7) -> ret=0 block_out=0
    let mut block = -7;
    assert_eq!(ole_unblock_server(&mut block), OLE_OK);
    assert_eq!(block, 0);
}

#[test]
fn oracle_document_and_revoke_rows() {
    // row 09 OleRevokeServerDoc(h=5) -> ret=0
    assert_eq!(ole_revoke_server_doc(), OLE_OK);

    // row 10 OleRenameServerDoc(h=5,new=New) -> ret=0
    assert_eq!(ole_rename_server_doc(), OLE_OK);

    // row 11 OleRevertServerDoc(h=5) -> ret=0
    assert_eq!(ole_revert_server_doc(), OLE_OK);

    // row 12 OleSavedServerDoc(h=5) -> ret=0
    assert_eq!(ole_saved_server_doc(), OLE_OK);

    // row 13 OleRevokeServer(h=2) -> ret=0
    assert_eq!(ole_revoke_server(), OLE_OK);
}

#[test]
fn oracle_stub_rows() {
    // row 14 WEP() -> raise code=0x80000100 flags=0x1 nargs=2 p0=olesvr32.dll p1=WEP raises=1
    assert_eq!(
        wep(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "olesvr32.dll",
            function: "WEP",
        }
    );

    // row 15 OleRevokeObject() -> raise code=0x80000100 flags=0x1 nargs=2 p0=olesvr32.dll p1=OleRevokeObject raises=1
    assert_eq!(
        ole_revoke_object(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "olesvr32.dll",
            function: "OleRevokeObject",
        }
    );

    // row 16 OleQueryServerVersion() -> raise code=0x80000100 flags=0x1 nargs=2 p0=olesvr32.dll p1=OleQueryServerVersion raises=1
    assert_eq!(
        ole_query_server_version(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "olesvr32.dll",
            function: "OleQueryServerVersion",
        }
    );
}
