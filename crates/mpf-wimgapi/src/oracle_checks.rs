//! Oracle-derived test vectors for the ported wimgapi logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `WIMRegisterMessageCallback`,
//! `WIMGetMountedImages` and `WIMCreateFile` (`dlls/wimgapi/main.c`) and
//! `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`) with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/wimgapi/`, and running the harness.
//! The harness replaces `RtlRaiseException` with a recorder that jumps out of the stub's loop.
//! The 43 printed rows (6 explicit-function rows, 37 `@ stub` rows) produced no ASan or UBSan
//! report, so none is excluded.

use super::*;

/// Last-error sentinel the C harness starts each explicit call with.
const SENTINEL: u32 = 0xdead_beef;

#[test]
fn oracle_explicit_function_rows() {
    // row 01 WIMRegisterMessageCallback(NULL,NULL,NULL) -> ret=0 lasterr=0xdeadbeef
    let mut w = Wimgapi::with_last_error(SENTINEL);
    assert_eq!(w.wim_register_message_callback(0, 0, 0), 0);
    assert_eq!(w.last_error(), 0xdead_beef);

    // row 02 WIMRegisterMessageCallback(0x1234,0x5678,0x9abc) -> ret=0 lasterr=0xdeadbeef
    let mut w = Wimgapi::with_last_error(SENTINEL);
    assert_eq!(w.wim_register_message_callback(0x1234, 0x5678, 0x9abc), 0);
    assert_eq!(w.last_error(), 0xdead_beef);

    // row 03 WIMGetMountedImages(NULL,NULL) -> ret=0 lasterr=120
    let mut w = Wimgapi::with_last_error(SENTINEL);
    assert_eq!(w.wim_get_mounted_images(0, None), 0);
    assert_eq!(w.last_error(), 120);

    // row 04 WIMGetMountedImages(list,&len=7) -> ret=0 lasterr=120 len=7
    let mut w = Wimgapi::with_last_error(SENTINEL);
    let mut len = 7;
    assert_eq!(w.wim_get_mounted_images(0x1000, Some(&mut len)), 0);
    assert_eq!(w.last_error(), 120);
    assert_eq!(len, 7);

    // row 05 WIMCreateFile(x.wim,access=0x80000000,creation=4,flags=0,compression=0,&res=9)
    //   -> ret=(nil) lasterr=120 res=9
    let mut w = Wimgapi::with_last_error(SENTINEL);
    let path: Vec<u16> = "x.wim".encode_utf16().collect();
    let mut res = 9;
    assert_eq!(
        w.wim_create_file(Some(&path), 0x8000_0000, 4, 0, 0, Some(&mut res)),
        NULL_HANDLE
    );
    assert_eq!(w.last_error(), 120);
    assert_eq!(res, 9);

    // row 06 WIMCreateFile(NULL,0,0,0,0,NULL) -> ret=(nil) lasterr=120
    let mut w = Wimgapi::with_last_error(SENTINEL);
    assert_eq!(w.wim_create_file(None, 0, 0, 0, 0, None), NULL_HANDLE);
    assert_eq!(w.last_error(), 120);
}

/// Rows 07 to 43: `code=0x80000100 flags=0x1 nparams=2` for every row, then the module and the
/// function. Each tuple is the row number, the module, the function, and the helper's result.
#[test]
fn oracle_stub_rows() {
    let rows: [(u32, &str, &str, StubException); 37] = [
        (7, "wimgapi.dll", "WIMApplyImage", wim_apply_image()),
        (8, "wimgapi.dll", "WIMCaptureImage", wim_capture_image()),
        (9, "wimgapi.dll", "WIMCloseHandle", wim_close_handle()),
        (
            10,
            "wimgapi.dll",
            "WIMCommitImageHandle",
            wim_commit_image_handle(),
        ),
        (11, "wimgapi.dll", "WIMCopyFile", wim_copy_file()),
        (
            12,
            "wimgapi.dll",
            "WIMCreateImageFile",
            wim_create_image_file(),
        ),
        (13, "wimgapi.dll", "WIMDeleteImage", wim_delete_image()),
        (
            14,
            "wimgapi.dll",
            "WIMDeleteImageMounts",
            wim_delete_image_mounts(),
        ),
        (
            15,
            "wimgapi.dll",
            "WIMEnumImageFiles",
            wim_enum_image_files(),
        ),
        (16, "wimgapi.dll", "WIMExportImage", wim_export_image()),
        (
            17,
            "wimgapi.dll",
            "WIMExtractImagePath",
            wim_extract_image_path(),
        ),
        (
            18,
            "wimgapi.dll",
            "WIMFindFirstImageFile",
            wim_find_first_image_file(),
        ),
        (
            19,
            "wimgapi.dll",
            "WIMFindNextImageFile",
            wim_find_next_image_file(),
        ),
        (20, "wimgapi.dll", "WIMGetAttributes", wim_get_attributes()),
        (21, "wimgapi.dll", "WIMGetImageCount", wim_get_image_count()),
        (
            22,
            "wimgapi.dll",
            "WIMGetImageInformation",
            wim_get_image_information(),
        ),
        (
            23,
            "wimgapi.dll",
            "WIMGetMessageCallbackCount",
            wim_get_message_callback_count(),
        ),
        (
            24,
            "wimgapi.dll",
            "WIMGetMountedImageHandle",
            wim_get_mounted_image_handle(),
        ),
        (
            25,
            "wimgapi.dll",
            "WIMGetMountedImageInfo",
            wim_get_mounted_image_info(),
        ),
        (
            26,
            "wimgapi.dll",
            "WIMGetMountedImageInfoFromFile",
            wim_get_mounted_image_info_from_file(),
        ),
        (
            27,
            "wimgapi.dll",
            "WIMInitFileIOCallbacks",
            wim_init_file_io_callbacks(),
        ),
        (28, "wimgapi.dll", "WIMLoadImage", wim_load_image()),
        (29, "wimgapi.dll", "WIMMountImage", wim_mount_image()),
        (
            30,
            "wimgapi.dll",
            "WIMMountImageHandle",
            wim_mount_image_handle(),
        ),
        (31, "wimgapi.dll", "WIMReadImageFile", wim_read_image_file()),
        (
            32,
            "wimgapi.dll",
            "WIMRegisterLogFile",
            wim_register_log_file(),
        ),
        (33, "wimgapi.dll", "WIMRemountImage", wim_remount_image()),
        (34, "wimgapi.dll", "WIMSetBootImage", wim_set_boot_image()),
        (
            35,
            "wimgapi.dll",
            "WIMSetFileIOCallbackTemporaryPath",
            wim_set_file_io_callback_temporary_path(),
        ),
        (
            36,
            "wimgapi.dll",
            "WIMSetImageInformation",
            wim_set_image_information(),
        ),
        (
            37,
            "wimgapi.dll",
            "WIMSetReferenceFile",
            wim_set_reference_file(),
        ),
        (
            38,
            "wimgapi.dll",
            "WIMSetTemporaryFile",
            wim_set_temporary_file(),
        ),
        (39, "wimgapi.dll", "WIMSplitFile", wim_split_file()),
        (40, "wimgapi.dll", "WIMUmountImage", wim_umount_image()),
        (
            41,
            "wimgapi.dll",
            "WIMUmountImageHandle",
            wim_umount_image_handle(),
        ),
        (
            42,
            "wimgapi.dll",
            "WIMUnregisterLogFile",
            wim_unregister_log_file(),
        ),
        (
            43,
            "wimgapi.dll",
            "WIMUnregisterMessageCallback",
            wim_unregister_message_callback(),
        ),
    ];
    for (row, module, function, got) in rows {
        let want = StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module,
            function,
        };
        assert_eq!(got, want, "row {row:02}");
    }
}
