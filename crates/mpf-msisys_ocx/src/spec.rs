//! Export declarations from `dlls/msisys.ocx/msisys.ocx.spec`.

/// How `msisys.ocx.spec` declares an export.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// Has a body in `dlls/msisys.ocx/msisys.c`.
    Implemented,
    /// `@ stdcall -private` with no body in `dlls/msisys.ocx/msisys.c`.
    PrivateWithoutBody,
}

/// Returns how `msisys.ocx.spec` declares `name`, or `None` if it is not exported.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    match name {
        "DllGetClassObject" => Some(ExportKind::Implemented),
        "DllCanUnloadNow" | "DllRegisterServer" | "DllUnregisterServer" => {
            Some(ExportKind::PrivateWithoutBody)
        }
        _ => None,
    }
}
