/// How `dlls/inkobj/inkobj.spec` declares an export.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// Has a body in `dlls/inkobj/inkobj.c`.
    Implemented,
    /// `@ stdcall -private` with no body in `dlls/inkobj/inkobj.c`.
    PrivateWithoutBody,
}

/// Returns how `inkobj.spec` declares `name`, or `None` if it is not exported.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    match name {
        "DllGetClassObject" => Some(ExportKind::Implemented),
        "DllCanUnloadNow" | "DllRegisterServer" | "DllUnregisterServer" => {
            Some(ExportKind::PrivateWithoutBody)
        }
        _ => None,
    }
}
