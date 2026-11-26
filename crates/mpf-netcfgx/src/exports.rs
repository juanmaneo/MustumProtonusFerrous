/// How `dlls/netcfgx/netcfgx.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall -private` with a body in `dlls/netcfgx`, ported as [`crate::dll_get_class_object`].
    Ported,
    /// `@ stdcall -private` with no body in `dlls/netcfgx`: winecrt0 supplies the default, which is
    /// not modelled.
    WinecrtDefault,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `netcfgx.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 16] = [
    ("DllCanUnloadNow", ExportKind::WinecrtDefault),
    ("DllGetClassObject", ExportKind::Ported),
    ("DllRegisterServer", ExportKind::WinecrtDefault),
    ("DllUnregisterServer", ExportKind::WinecrtDefault),
    ("HrDiAddComponentToINetCfg", ExportKind::Stub),
    ("LanaCfgFromCommandArgs", ExportKind::Stub),
    ("ModemClassCoInstaller", ExportKind::Stub),
    ("NetCfgDiagFromCommandArgs", ExportKind::Stub),
    ("NetCfgDiagRepairRegistryBindings", ExportKind::Stub),
    ("NetClassInstaller", ExportKind::Stub),
    ("NetPropPageProvider", ExportKind::Stub),
    ("RasAddBindings", ExportKind::Stub),
    ("RasCountBindings", ExportKind::Stub),
    ("RasRemoveBindings", ExportKind::Stub),
    ("SvchostChangeSvchostGroup", ExportKind::Stub),
    ("UpdateLanaConfigUsingAnswerfile", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
