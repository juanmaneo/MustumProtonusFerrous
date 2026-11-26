/// How `dlls/wkscli/wkscli.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: a real function, ported in [`crate::net_get_join_information`] or in the
    /// fixed-status functions of the `status` module.
    Stdcall,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `wkscli.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 23] = [
    ("NetAddAlternateComputerName", ExportKind::Stub),
    ("NetEnumerateComputerNames", ExportKind::Stub),
    ("NetGetJoinInformation", ExportKind::Stdcall),
    ("NetGetJoinableOUs", ExportKind::Stub),
    ("NetJoinDomain", ExportKind::Stub),
    ("NetRemoveAlternateComputerName", ExportKind::Stub),
    ("NetRenameMachineInDomain", ExportKind::Stub),
    ("NetSetPrimaryComputerName", ExportKind::Stub),
    ("NetUnjoinDomain", ExportKind::Stub),
    ("NetUseAdd", ExportKind::Stdcall),
    ("NetUseDel", ExportKind::Stdcall),
    ("NetUseEnum", ExportKind::Stdcall),
    ("NetUseGetInfo", ExportKind::Stdcall),
    ("NetValidateName", ExportKind::Stub),
    ("NetWkstaGetInfo", ExportKind::Stub),
    ("NetWkstaSetInfo", ExportKind::Stub),
    ("NetWkstaStatisticsGet", ExportKind::Stub),
    ("NetWkstaTransportAdd", ExportKind::Stub),
    ("NetWkstaTransportDel", ExportKind::Stub),
    ("NetWkstaTransportEnum", ExportKind::Stdcall),
    ("NetWkstaUserEnum", ExportKind::Stub),
    ("NetWkstaUserGetInfo", ExportKind::Stub),
    ("NetWkstaUserSetInfo", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
