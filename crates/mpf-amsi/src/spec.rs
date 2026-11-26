//! Export declarations from `dlls/amsi/amsi.spec`.

/// Module name that `__wine_spec_unimplemented_stub` receives from `@ stub` exports.
pub const AMSI_DLL_NAME: &str = "amsi.dll";

/// How `amsi.spec` declares an export.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// Has a body in `dlls/amsi/main.c`.
    Implemented,
    /// `@ stub`: calls `__wine_spec_unimplemented_stub`.
    Stub,
    /// `@ stdcall -private` with no body in `dlls/amsi/main.c`.
    PrivateWithoutBody,
}

/// Returns how `amsi.spec` declares `name`, or `None` if it is not exported.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    match name {
        "AmsiCloseSession" | "AmsiInitialize" | "AmsiOpenSession" | "AmsiScanBuffer"
        | "AmsiScanString" | "AmsiUninitialize" => Some(ExportKind::Implemented),
        "AmsiUacInitialize" | "AmsiUacScan" | "AmsiUacUninitialize" | "DllGetClassObject" => {
            Some(ExportKind::Stub)
        }
        "DllCanUnloadNow" | "DllRegisterServer" | "DllUnregisterServer" => {
            Some(ExportKind::PrivateWithoutBody)
        }
        _ => None,
    }
}
