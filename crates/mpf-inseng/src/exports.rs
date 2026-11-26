//! The exports of `dlls/inseng/inseng.spec` that are real functions, and the spec's export table.

use crate::com::{HResult, S_FALSE, S_OK};

/// `TRUE` from `include/windef.h`.
pub const TRUE: i32 = 1;

/// `CheckTrustEx`: traces its five arguments and returns `TRUE`.
pub fn check_trust_ex() -> i32 {
    TRUE
}

/// `DllInstall`: traces its arguments and returns `S_OK`.
pub fn dll_install() -> HResult {
    S_OK
}

/// `DllCanUnloadNow`, the default from `libs/winecrt0/dll_canunload.c`: always `S_FALSE`.
pub fn dll_can_unload_now() -> HResult {
    S_FALSE
}

/// How `dlls/inseng/inseng.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`.
    Stdcall,
    /// `@ stdcall -private`.
    StdcallPrivate,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `inseng.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 12] = [
    ("CheckForVersionConflict", ExportKind::Stub),
    ("CheckTrust", ExportKind::Stub),
    ("CheckTrustEx", ExportKind::Stdcall),
    ("DllCanUnloadNow", ExportKind::StdcallPrivate),
    ("DllGetClassObject", ExportKind::StdcallPrivate),
    ("DllInstall", ExportKind::Stdcall),
    ("DllRegisterServer", ExportKind::StdcallPrivate),
    ("DllUnregisterServer", ExportKind::StdcallPrivate),
    ("DownloadFile", ExportKind::Stub),
    ("GetICifFileFromFile", ExportKind::Stub),
    ("GetICifRWFileFromFile", ExportKind::Stub),
    ("PurgeDownloadDirectory", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
