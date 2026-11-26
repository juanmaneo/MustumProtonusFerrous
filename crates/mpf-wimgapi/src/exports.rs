/// How `dlls/wimgapi/wimgapi.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: a C function in `main.c`, ported as a [`crate::Wimgapi`] method.
    Stdcall,
    /// `@ stdcall -private`: no body in `main.c`, so it is not ported.
    Private,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

use ExportKind::{Private, Stdcall, Stub};

/// The `wimgapi.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 42] = [
    ("DllCanUnloadNow", Private),
    ("DllMain", Private),
    ("WIMApplyImage", Stub),
    ("WIMCaptureImage", Stub),
    ("WIMCloseHandle", Stub),
    ("WIMCommitImageHandle", Stub),
    ("WIMCopyFile", Stub),
    ("WIMCreateFile", Stdcall),
    ("WIMCreateImageFile", Stub),
    ("WIMDeleteImage", Stub),
    ("WIMDeleteImageMounts", Stub),
    ("WIMEnumImageFiles", Stub),
    ("WIMExportImage", Stub),
    ("WIMExtractImagePath", Stub),
    ("WIMFindFirstImageFile", Stub),
    ("WIMFindNextImageFile", Stub),
    ("WIMGetAttributes", Stub),
    ("WIMGetImageCount", Stub),
    ("WIMGetImageInformation", Stub),
    ("WIMGetMessageCallbackCount", Stub),
    ("WIMGetMountedImageHandle", Stub),
    ("WIMGetMountedImageInfo", Stub),
    ("WIMGetMountedImageInfoFromFile", Stub),
    ("WIMGetMountedImages", Stdcall),
    ("WIMInitFileIOCallbacks", Stub),
    ("WIMLoadImage", Stub),
    ("WIMMountImage", Stub),
    ("WIMMountImageHandle", Stub),
    ("WIMReadImageFile", Stub),
    ("WIMRegisterLogFile", Stub),
    ("WIMRegisterMessageCallback", Stdcall),
    ("WIMRemountImage", Stub),
    ("WIMSetBootImage", Stub),
    ("WIMSetFileIOCallbackTemporaryPath", Stub),
    ("WIMSetImageInformation", Stub),
    ("WIMSetReferenceFile", Stub),
    ("WIMSetTemporaryFile", Stub),
    ("WIMSplitFile", Stub),
    ("WIMUmountImage", Stub),
    ("WIMUmountImageHandle", Stub),
    ("WIMUnregisterLogFile", Stub),
    ("WIMUnregisterMessageCallback", Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
