//! Oracle-derived test vectors for the ported srclient export table and stub model.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`, lines 504-516) and `assign_ordinals` (`tools/winebuild/parser.c`,
//! lines 882-933) with `gcc -fsanitize=address,undefined -g` under `target/canon/srclient/`, and
//! running the harness on `dlls/srclient/srclient.spec` and `dlls/srclient/Makefile.in`. The harness
//! printed 24 rows with no ASan or UBSan reports, so none are excluded. Rows 01-13 are the export
//! table: the ordinal from `assign_ordinals`, and the stub or forward kind read from the spec text.
//! Rows 14-24 are the `@ stub` exports raised through the verbatim stub body. The forwards have no
//! raise row because srclient has no C body for them.

use super::*;

#[test]
fn oracle_export_rows_01_to_13() {
    // row 01 export ord=1 name=SysprepCleanup kind=stub
    // row 02 export ord=2 name=SysprepGeneralize kind=stub
    // row 03 export ord=3 name=DisableSR kind=stub
    // row 04 export ord=4 name=DisableSRInternal kind=stub
    // row 05 export ord=5 name=EnableSR kind=stub
    // row 06 export ord=6 name=EnableSREx kind=stub
    // row 07 export ord=7 name=EnableSRInternal kind=stub
    // row 08 export ord=8 name=SRNewSystemId kind=stub
    // row 09 export ord=9 name=SRRemoveRestorePoint kind=stub
    // row 10 export ord=10 name=SRSetRestorePointA kind=forward sfc_os.SRSetRestorePointA
    // row 11 export ord=11 name=SRSetRestorePointInternal kind=stub
    // row 12 export ord=12 name=SRSetRestorePointW kind=forward sfc_os.SRSetRestorePointW
    // row 13 export ord=13 name=SetSRStateAfterSetup kind=stub
    assert_eq!(
        EXPORTS,
        [
            (1, "SysprepCleanup", ExportKind::Stub),
            (2, "SysprepGeneralize", ExportKind::Stub),
            (3, "DisableSR", ExportKind::Stub),
            (4, "DisableSRInternal", ExportKind::Stub),
            (5, "EnableSR", ExportKind::Stub),
            (6, "EnableSREx", ExportKind::Stub),
            (7, "EnableSRInternal", ExportKind::Stub),
            (8, "SRNewSystemId", ExportKind::Stub),
            (9, "SRRemoveRestorePoint", ExportKind::Stub),
            (
                10,
                "SRSetRestorePointA",
                ExportKind::Forward("sfc_os.SRSetRestorePointA"),
            ),
            (11, "SRSetRestorePointInternal", ExportKind::Stub),
            (
                12,
                "SRSetRestorePointW",
                ExportKind::Forward("sfc_os.SRSetRestorePointW"),
            ),
            (13, "SetSRStateAfterSetup", ExportKind::Stub),
        ]
    );
}

#[test]
fn oracle_stub_rows_14_to_24() {
    // row 14 SysprepCleanup -> raise code=0x80000100 flags=0x1 nargs=2 p0=srclient.dll p1=SysprepCleanup
    assert_eq!(
        sysprep_cleanup(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "srclient.dll",
            function: "SysprepCleanup",
        }
    );

    // row 15 SysprepGeneralize -> raise code=0x80000100 flags=0x1 nargs=2 p0=srclient.dll p1=SysprepGeneralize
    assert_eq!(
        sysprep_generalize(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "srclient.dll",
            function: "SysprepGeneralize",
        }
    );

    // row 16 DisableSR -> raise code=0x80000100 flags=0x1 nargs=2 p0=srclient.dll p1=DisableSR
    assert_eq!(
        disable_sr(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "srclient.dll",
            function: "DisableSR",
        }
    );

    // row 17 DisableSRInternal -> raise code=0x80000100 flags=0x1 nargs=2 p0=srclient.dll p1=DisableSRInternal
    assert_eq!(
        disable_sr_internal(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "srclient.dll",
            function: "DisableSRInternal",
        }
    );

    // row 18 EnableSR -> raise code=0x80000100 flags=0x1 nargs=2 p0=srclient.dll p1=EnableSR
    assert_eq!(
        enable_sr(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "srclient.dll",
            function: "EnableSR",
        }
    );

    // row 19 EnableSREx -> raise code=0x80000100 flags=0x1 nargs=2 p0=srclient.dll p1=EnableSREx
    assert_eq!(
        enable_sr_ex(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "srclient.dll",
            function: "EnableSREx",
        }
    );

    // row 20 EnableSRInternal -> raise code=0x80000100 flags=0x1 nargs=2 p0=srclient.dll p1=EnableSRInternal
    assert_eq!(
        enable_sr_internal(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "srclient.dll",
            function: "EnableSRInternal",
        }
    );

    // row 21 SRNewSystemId -> raise code=0x80000100 flags=0x1 nargs=2 p0=srclient.dll p1=SRNewSystemId
    assert_eq!(
        sr_new_system_id(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "srclient.dll",
            function: "SRNewSystemId",
        }
    );

    // row 22 SRRemoveRestorePoint -> raise code=0x80000100 flags=0x1 nargs=2 p0=srclient.dll p1=SRRemoveRestorePoint
    assert_eq!(
        sr_remove_restore_point(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "srclient.dll",
            function: "SRRemoveRestorePoint",
        }
    );

    // row 23 SRSetRestorePointInternal -> raise code=0x80000100 flags=0x1 nargs=2 p0=srclient.dll p1=SRSetRestorePointInternal
    assert_eq!(
        sr_set_restore_point_internal(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "srclient.dll",
            function: "SRSetRestorePointInternal",
        }
    );

    // row 24 SetSRStateAfterSetup -> raise code=0x80000100 flags=0x1 nargs=2 p0=srclient.dll p1=SetSRStateAfterSetup
    assert_eq!(
        set_sr_state_after_setup(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "srclient.dll",
            function: "SetSRStateAfterSetup",
        }
    );
}
