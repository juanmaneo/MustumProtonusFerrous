//! Oracle-derived test vectors for the ported sfc_os logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `dlls/sfc_os/sfc_os.c` and
//! `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`) with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/sfc_os/`, and running the harness.
//! The 27 printed rows all ran clean under ASan and UBSan, so none were excluded. The stub rows
//! come from a recorder that longjmps out of the verbatim `for (;;)` loop, because the C body
//! never returns.

use super::*;

#[test]
fn oracle_explicit_rows() {
    // row 01 SfcGetNextProtectedFile handle=NULL data=NULL ret=0 last_error=18
    assert_eq!(
        sfc_get_next_protected_file(),
        Outcome {
            ret: false,
            last_error: Some(18)
        }
    );
    // row 02 SfcGetNextProtectedFile handle=0x1234 data=set ret=0 last_error=18
    assert_eq!(
        sfc_get_next_protected_file(),
        Outcome {
            ret: false,
            last_error: Some(18)
        }
    );
    // row 03 SfcIsFileProtected handle=NULL name=set ret=0 last_error=2
    assert_eq!(
        sfc_is_file_protected(),
        Outcome {
            ret: false,
            last_error: Some(2)
        }
    );
    // row 04 SfcIsFileProtected handle=0x1234 name=NULL ret=0 last_error=2
    assert_eq!(
        sfc_is_file_protected(),
        Outcome {
            ret: false,
            last_error: Some(2)
        }
    );
    // row 05 SfcIsFileProtected handle=NULL name=set repeat ret=0 last_error=2
    assert_eq!(
        sfc_is_file_protected(),
        Outcome {
            ret: false,
            last_error: Some(2)
        }
    );
    // row 06 SfcIsKeyProtected hkey=NULL subkey=NULL sam=0 ret=0 last_error=6
    assert_eq!(
        sfc_is_key_protected(false),
        Outcome {
            ret: false,
            last_error: Some(6)
        }
    );
    // row 07 SfcIsKeyProtected hkey=0x80000002 subkey=set sam=0x100 ret=0 last_error=2
    assert_eq!(
        sfc_is_key_protected(true),
        Outcome {
            ret: false,
            last_error: Some(2)
        }
    );
    // row 08 SfcIsKeyProtected hkey=NULL subkey=set sam=0x200 ret=0 last_error=6
    assert_eq!(
        sfc_is_key_protected(false),
        Outcome {
            ret: false,
            last_error: Some(6)
        }
    );
    // row 09 SfcConnectToServer unknown=0 ret=0 last_error=none
    assert_eq!(sfc_connect_to_server(), 0);
    // row 10 SfcConnectToServer unknown=0xdeadbeef ret=0 last_error=none
    assert_eq!(sfc_connect_to_server(), 0);
    // row 11 SfpVerifyFile filename=set error=set size=8 ret=0 last_error=120
    assert_eq!(
        sfp_verify_file(),
        Outcome {
            ret: false,
            last_error: Some(120)
        }
    );
    // row 12 SfpVerifyFile filename=NULL error=NULL size=0 ret=0 last_error=120
    assert_eq!(
        sfp_verify_file(),
        Outcome {
            ret: false,
            last_error: Some(120)
        }
    );

    // row 13 SRSetRestorePointA restorepoint=NULL ret=0 nStatus=0 last_error=none
    let mut status = StateMgrStatus {
        n_status: 0xdead_beef,
    };
    assert_eq!(
        sr_set_restore_point_a(&mut status),
        Outcome {
            ret: false,
            last_error: None
        }
    );
    assert_eq!(status.n_status, 0);

    // row 14 SRSetRestorePointA restorepoint=set ret=0 nStatus=0 last_error=none
    let mut status = StateMgrStatus {
        n_status: 0xdead_beef,
    };
    assert_eq!(
        sr_set_restore_point_a(&mut status),
        Outcome {
            ret: false,
            last_error: None
        }
    );
    assert_eq!(status.n_status, 0);

    // row 15 SRSetRestorePointW restorepoint=NULL ret=0 nStatus=0 last_error=none
    let mut status = StateMgrStatus {
        n_status: 0xdead_beef,
    };
    assert_eq!(
        sr_set_restore_point_w(&mut status),
        Outcome {
            ret: false,
            last_error: None
        }
    );
    assert_eq!(status.n_status, 0);

    // row 16 SRSetRestorePointW restorepoint=set ret=0 nStatus=0 last_error=none
    let mut status = StateMgrStatus {
        n_status: 0xdead_beef,
    };
    assert_eq!(
        sr_set_restore_point_w(&mut status),
        Outcome {
            ret: false,
            last_error: None
        }
    );
    assert_eq!(status.n_status, 0);
}

#[test]
fn oracle_stub_rows() {
    let rows = [
        // row 17
        (begin_file_map_enumeration(), "BeginFileMapEnumeration"),
        // row 18
        (close_file_map_enumeration(), "CloseFileMapEnumeration"),
        // row 19
        (get_next_file_map_content(), "GetNextFileMapContent"),
        // row 20
        (sfc_close(), "SfcClose"),
        // row 21
        (sfc_file_exception(), "SfcFileException"),
        // row 22
        (sfc_init_prot(), "SfcInitProt"),
        // row 23
        (sfc_initiate_scan(), "SfcInitiateScan"),
        // row 24
        (sfc_install_protected_files(), "SfcInstallProtectedFiles"),
        // row 25
        (sfc_terminate_watcher_thread(), "SfcTerminateWatcherThread"),
        // row 26
        (sfp_delete_catalog(), "SfpDeleteCatalog"),
        // row 27
        (sfp_install_catalog(), "SfpInstallCatalog"),
    ];
    for (got, function) in rows {
        assert_eq!(
            got,
            StubException {
                code: 0x8000_0100,
                flags: 0x1,
                number_parameters: 2,
                module: "sfc_os.dll",
                function,
            }
        );
    }
}
