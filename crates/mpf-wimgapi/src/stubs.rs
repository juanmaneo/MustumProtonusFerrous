/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = wimgapi.dll` in `dlls/wimgapi/Makefile.in`.
pub const WIMGAPI_DLL_NAME: &str = "wimgapi.dll";

/// The record that `__wine_spec_unimplemented_stub` raises.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`. The C function raises
/// the record in a loop and never returns, so the port returns the record instead.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        // The module and the function name.
        number_parameters: 2,
        module,
        function,
    }
}

/// Defines one helper per `@ stub` export, and the table of them in spec order.
macro_rules! stub_exports {
    ($($helper:ident => $export:literal,)*) => {
        $(
            #[doc = concat!("`", $export, "`, exported as `@ stub` in `dlls/wimgapi/wimgapi.spec`.")]
            pub fn $helper() -> StubException {
                wine_spec_unimplemented_stub(WIMGAPI_DLL_NAME, $export)
            }
        )*

        /// The `@ stub` exports of `wimgapi.spec`, in spec order, with their helpers.
        pub const STUB_EXPORTS: &[(&str, fn() -> StubException)] = &[$(($export, $helper)),*];
    };
}

stub_exports! {
    wim_apply_image => "WIMApplyImage",
    wim_capture_image => "WIMCaptureImage",
    wim_close_handle => "WIMCloseHandle",
    wim_commit_image_handle => "WIMCommitImageHandle",
    wim_copy_file => "WIMCopyFile",
    wim_create_image_file => "WIMCreateImageFile",
    wim_delete_image => "WIMDeleteImage",
    wim_delete_image_mounts => "WIMDeleteImageMounts",
    wim_enum_image_files => "WIMEnumImageFiles",
    wim_export_image => "WIMExportImage",
    wim_extract_image_path => "WIMExtractImagePath",
    wim_find_first_image_file => "WIMFindFirstImageFile",
    wim_find_next_image_file => "WIMFindNextImageFile",
    wim_get_attributes => "WIMGetAttributes",
    wim_get_image_count => "WIMGetImageCount",
    wim_get_image_information => "WIMGetImageInformation",
    wim_get_message_callback_count => "WIMGetMessageCallbackCount",
    wim_get_mounted_image_handle => "WIMGetMountedImageHandle",
    wim_get_mounted_image_info => "WIMGetMountedImageInfo",
    wim_get_mounted_image_info_from_file => "WIMGetMountedImageInfoFromFile",
    wim_init_file_io_callbacks => "WIMInitFileIOCallbacks",
    wim_load_image => "WIMLoadImage",
    wim_mount_image => "WIMMountImage",
    wim_mount_image_handle => "WIMMountImageHandle",
    wim_read_image_file => "WIMReadImageFile",
    wim_register_log_file => "WIMRegisterLogFile",
    wim_remount_image => "WIMRemountImage",
    wim_set_boot_image => "WIMSetBootImage",
    wim_set_file_io_callback_temporary_path => "WIMSetFileIOCallbackTemporaryPath",
    wim_set_image_information => "WIMSetImageInformation",
    wim_set_reference_file => "WIMSetReferenceFile",
    wim_set_temporary_file => "WIMSetTemporaryFile",
    wim_split_file => "WIMSplitFile",
    wim_umount_image => "WIMUmountImage",
    wim_umount_image_handle => "WIMUmountImageHandle",
    wim_unregister_log_file => "WIMUnregisterLogFile",
    wim_unregister_message_callback => "WIMUnregisterMessageCallback",
}
