/// How `dlls/regapi/regapi.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `regapi.spec` export table, in spec order. Every export is a `@ stub`.
pub const EXPORTS: [(&str, ExportKind); 69] = [
    ("RegBuildNumberQuery", ExportKind::Stub),
    ("RegCdCreateA", ExportKind::Stub),
    ("RegCdCreateW", ExportKind::Stub),
    ("RegCdDeleteA", ExportKind::Stub),
    ("RegCdDeleteW", ExportKind::Stub),
    ("RegCdEnumerateA", ExportKind::Stub),
    ("RegCdEnumerateW", ExportKind::Stub),
    ("RegCdQueryA", ExportKind::Stub),
    ("RegCdQueryW", ExportKind::Stub),
    ("RegCloseServer", ExportKind::Stub),
    ("RegConsoleShadowQueryA", ExportKind::Stub),
    ("RegConsoleShadowQueryW", ExportKind::Stub),
    ("RegDefaultUserConfigQueryA", ExportKind::Stub),
    ("RegDefaultUserConfigQueryW", ExportKind::Stub),
    ("RegDenyTSConnectionsPolicy", ExportKind::Stub),
    ("RegFreeUtilityCommandList", ExportKind::Stub),
    ("RegGetMachinePolicy", ExportKind::Stub),
    ("RegGetMachinePolicyEx", ExportKind::Stub),
    ("RegGetTServerVersion", ExportKind::Stub),
    ("RegGetUserConfigFromUserParameters", ExportKind::Stub),
    ("RegGetUserPolicy", ExportKind::Stub),
    ("RegIsMachineInHelpMode", ExportKind::Stub),
    ("RegIsMachinePolicyAllowHelp", ExportKind::Stub),
    ("RegIsTServer", ExportKind::Stub),
    ("RegMergeUserConfigWithUserParameters", ExportKind::Stub),
    ("RegOpenServerA", ExportKind::Stub),
    ("RegOpenServerW", ExportKind::Stub),
    ("RegPdCreateA", ExportKind::Stub),
    ("RegPdCreateW", ExportKind::Stub),
    ("RegPdDeleteA", ExportKind::Stub),
    ("RegPdDeleteW", ExportKind::Stub),
    ("RegPdEnumerateA", ExportKind::Stub),
    ("RegPdEnumerateW", ExportKind::Stub),
    ("RegPdQueryA", ExportKind::Stub),
    ("RegPdQueryW", ExportKind::Stub),
    ("RegQueryOEMId", ExportKind::Stub),
    ("RegQueryUtilityCommandList", ExportKind::Stub),
    ("RegSAMUserConfig", ExportKind::Stub),
    ("RegUserConfigDelete", ExportKind::Stub),
    ("RegUserConfigQuery", ExportKind::Stub),
    ("RegUserConfigRename", ExportKind::Stub),
    ("RegUserConfigSet", ExportKind::Stub),
    ("RegWdCreateA", ExportKind::Stub),
    ("RegWdCreateW", ExportKind::Stub),
    ("RegWdDeleteA", ExportKind::Stub),
    ("RegWdDeleteW", ExportKind::Stub),
    ("RegWdEnumerateA", ExportKind::Stub),
    ("RegWdEnumerateW", ExportKind::Stub),
    ("RegWdQueryA", ExportKind::Stub),
    ("RegWdQueryW", ExportKind::Stub),
    ("RegWinStationAccessCheck", ExportKind::Stub),
    ("RegWinStationCreateA", ExportKind::Stub),
    ("RegWinStationCreateW", ExportKind::Stub),
    ("RegWinStationDeleteA", ExportKind::Stub),
    ("RegWinStationDeleteW", ExportKind::Stub),
    ("RegWinStationEnumerateA", ExportKind::Stub),
    ("RegWinStationEnumerateW", ExportKind::Stub),
    ("RegWinStationQueryA", ExportKind::Stub),
    ("RegWinStationQueryDefaultSecurity", ExportKind::Stub),
    ("RegWinStationQueryEx", ExportKind::Stub),
    ("RegWinStationQueryNumValueW", ExportKind::Stub),
    ("RegWinStationQuerySecurityA", ExportKind::Stub),
    ("RegWinStationQuerySecurityW", ExportKind::Stub),
    ("RegWinStationQueryValueW", ExportKind::Stub),
    ("RegWinStationQueryW", ExportKind::Stub),
    ("RegWinStationSetNumValueW", ExportKind::Stub),
    ("RegWinStationSetSecurityA", ExportKind::Stub),
    ("RegWinStationSetSecurityW", ExportKind::Stub),
    ("WaitForTSConnectionsPolicyChanges", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
