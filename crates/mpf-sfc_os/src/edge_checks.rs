//! Edge-case tests for the ported sfc_os logic.
//!
//! These cover the NULL-handle branch, the restore point status write, the export table and
//! the stub helpers. Not every expectation is oracle-derived. The ones that aren't say so in a
//! comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/winternl.h and include/winnt.h.
    assert_eq!(ERROR_SUCCESS, 0);
    assert_eq!(ERROR_FILE_NOT_FOUND, 2);
    assert_eq!(ERROR_INVALID_HANDLE, 6);
    assert_eq!(ERROR_NO_MORE_FILES, 18);
    assert_eq!(ERROR_CALL_NOT_IMPLEMENTED, 120);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn sfc_is_key_protected_branches_on_null_handle() {
    // Not oracle-derived: the NULL and non-NULL branches of dlls/sfc_os/sfc_os.c, with the
    // values the oracle rows 06 and 07 show for each branch.
    assert_eq!(
        sfc_is_key_protected(false),
        Outcome {
            ret: false,
            last_error: Some(ERROR_INVALID_HANDLE)
        }
    );
    assert_eq!(
        sfc_is_key_protected(true),
        Outcome {
            ret: false,
            last_error: Some(ERROR_FILE_NOT_FOUND)
        }
    );
}

#[test]
fn set_restore_point_writes_only_n_status() {
    // Not oracle-derived: the C body writes nStatus only, so the port's single field is the
    // whole observable effect. Both entry points behave the same and are idempotent.
    for set in [sr_set_restore_point_a, sr_set_restore_point_w] {
        let mut status = StateMgrStatus { n_status: 7 };
        assert_eq!(
            set(&mut status),
            Outcome {
                ret: false,
                last_error: None
            }
        );
        assert_eq!(status.n_status, ERROR_SUCCESS);
        assert_eq!(
            set(&mut status),
            Outcome {
                ret: false,
                last_error: None
            }
        );
        assert_eq!(status.n_status, ERROR_SUCCESS);
    }
}

#[test]
fn outcome_is_copy_and_comparable() {
    // Not oracle-derived: checks the derived traits the other tests rely on.
    let outcome = sfp_verify_file();
    let copy = outcome;
    assert_eq!(outcome, copy);
    assert_ne!(
        outcome,
        Outcome {
            ret: true,
            last_error: None
        }
    );
}

#[test]
fn wine_spec_unimplemented_stub_keeps_its_arguments() {
    // Not oracle-derived: the record copies the module and function it is given. The oracle
    // rows only cover the names winebuild passes for sfc_os.dll.
    let record = wine_spec_unimplemented_stub("other.dll", "SomeExport");
    assert_eq!(
        record,
        StubException {
            code: EXCEPTION_WINE_STUB,
            flags: EXCEPTION_NONCONTINUABLE,
            number_parameters: 2,
            module: "other.dll",
            function: "SomeExport",
        }
    );
}

#[test]
fn export_table_matches_the_spec() {
    // Not oracle-derived: the table comes from dlls/sfc_os/sfc_os.spec. There are 11 `@ stub`
    // exports and 7 `@ stdcall` exports.
    assert_eq!(EXPORTS.len(), 18);
    let stdcall: Vec<&str> = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stdcall)
        .map(|(name, _)| *name)
        .collect();
    assert_eq!(
        stdcall,
        [
            "SRSetRestorePointA",
            "SRSetRestorePointW",
            "SfcConnectToServer",
            "SfcGetNextProtectedFile",
            "SfcIsFileProtected",
            "SfcIsKeyProtected",
            "SfpVerifyFile",
        ]
    );
    assert_eq!(export_kind("SfcIsFileProtected"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("SfcClose"), Some(ExportKind::Stub));
    assert_eq!(export_kind("sfcclose"), None);
    assert_eq!(export_kind("Missing"), None);
}

#[test]
fn stub_helpers_cover_every_spec_stub_export() {
    // Not oracle-derived: checks that the helpers name exactly the `@ stub` exports of the spec,
    // in spec order.
    let helpers = [
        begin_file_map_enumeration().function,
        close_file_map_enumeration().function,
        get_next_file_map_content().function,
        sfc_close().function,
        sfc_file_exception().function,
        sfc_init_prot().function,
        sfc_initiate_scan().function,
        sfc_install_protected_files().function,
        sfc_terminate_watcher_thread().function,
        sfp_delete_catalog().function,
        sfp_install_catalog().function,
    ];
    let spec_stubs: Vec<&str> = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .map(|(name, _)| *name)
        .collect();
    assert_eq!(helpers.as_slice(), spec_stubs.as_slice());
}
