//! Oracle-derived test vectors for the ported rstrtmgr logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of the `Rm*` functions
//! (`dlls/rstrtmgr/main.c`, lines 29-119) and of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`, lines 504-516) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/rstrtmgr/`, and running the harness. All 18 printed rows are kept: ASan and
//! UBSan reported nothing, so no vector is excluded.

use super::*;

#[test]
fn oracle_explicit_function_rows() {
    // row 01 RmStartSession(handle=set, flags=0) -> ret=0 handle=deadbeef
    let mut handle = 0x1111_1111;
    assert_eq!(rm_start_session(Some(&mut handle)), ERROR_SUCCESS);
    assert_eq!(handle, 0xdead_beef);

    // row 02 RmStartSession(handle=NULL, flags=1) -> ret=0 handle_unchanged=22222222
    assert_eq!(rm_start_session(None), ERROR_SUCCESS);

    // row 03 RmGetList(1, needed=set, info=set) -> ret=0 needed=0 info=0
    let (mut needed, mut info) = (0xffff_ffff, 0xffff_fffe);
    assert_eq!(
        rm_get_list(Some(&mut needed), Some(&mut info)),
        ERROR_SUCCESS
    );
    assert_eq!((needed, info), (0, 0));

    // row 04 RmGetList(0, NULL, NULL, NULL, NULL) -> ret=0 needed_unchanged=4294967295 info_unchanged=4294967294
    assert_eq!(rm_get_list(None, None), ERROR_SUCCESS);

    // row 05 RmGetList(7, needed=5, info=NULL) -> ret=0 needed=0 info_unchanged=9
    let (mut needed, info) = (5, 9);
    assert_eq!(rm_get_list(Some(&mut needed), None), ERROR_SUCCESS);
    assert_eq!((needed, info), (0, 9));

    // row 06 RmGetList(7, needed=NULL, info=9) -> ret=0 needed_unchanged=5 info=0
    let (needed, mut info) = (5, 9);
    assert_eq!(rm_get_list(None, Some(&mut info)), ERROR_SUCCESS);
    assert_eq!((needed, info), (5, 0));

    // row 07 RmRegisterResources(1, nFiles=2, nApps=0, nServices=0) -> ret=0
    assert_eq!(rm_register_resources(), ERROR_SUCCESS);

    // row 08 RmRegisterResources(0, all empty) -> ret=0
    assert_eq!(rm_register_resources(), ERROR_SUCCESS);

    // row 09 RmEndSession(0xdeadbeef) -> ret=0
    assert_eq!(rm_end_session(), ERROR_SUCCESS);

    // row 10 RmEndSession(0) -> ret=0
    assert_eq!(rm_end_session(), ERROR_SUCCESS);

    // row 11 RmRestart(3, flags=1, status=NULL) -> ret=0
    assert_eq!(rm_restart(), ERROR_SUCCESS);

    // row 12 RmShutdown(3, flags=0xffffffff, status=NULL) -> ret=0
    assert_eq!(rm_shutdown(), ERROR_SUCCESS);

    // row 13 RmAddFilter(2, file, NULL, svc, action=1) -> ret=0
    assert_eq!(rm_add_filter(), ERROR_SUCCESS);

    // row 14 RmRemoveFilter(2, file, NULL, NULL) -> ret=0
    assert_eq!(rm_remove_filter(), ERROR_SUCCESS);
}

#[test]
fn oracle_stub_rows() {
    // row 15 RmCancelCurrentTask -> raise code=0x80000100 flags=0x1 nargs=2 p0=rstrtmgr.dll p1=RmCancelCurrentTask
    assert_eq!(
        rm_cancel_current_task(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "rstrtmgr.dll",
            function: "RmCancelCurrentTask",
        }
    );

    // row 16 RmGetFilterList -> raise code=0x80000100 flags=0x1 nargs=2 p0=rstrtmgr.dll p1=RmGetFilterList
    assert_eq!(
        rm_get_filter_list(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "rstrtmgr.dll",
            function: "RmGetFilterList",
        }
    );

    // row 17 RmJoinSession -> raise code=0x80000100 flags=0x1 nargs=2 p0=rstrtmgr.dll p1=RmJoinSession
    assert_eq!(
        rm_join_session(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "rstrtmgr.dll",
            function: "RmJoinSession",
        }
    );

    // row 18 RmReserveHeap -> raise code=0x80000100 flags=0x1 nargs=2 p0=rstrtmgr.dll p1=RmReserveHeap
    assert_eq!(
        rm_reserve_heap(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "rstrtmgr.dll",
            function: "RmReserveHeap",
        }
    );
}
