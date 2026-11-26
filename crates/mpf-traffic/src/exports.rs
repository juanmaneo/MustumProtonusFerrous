/// How `dlls/traffic/traffic.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: a real C function, ported as [`crate::tc_register_client`] or
    /// [`crate::tc_deregister_client`].
    Stdcall,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `traffic.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 21] = [
    ("TcAddClassMap", ExportKind::Stub),
    ("TcAddFilter", ExportKind::Stub),
    ("TcAddFlow", ExportKind::Stub),
    ("TcCloseInterface", ExportKind::Stub),
    ("TcDeleteFilter", ExportKind::Stub),
    ("TcDeleteFlow", ExportKind::Stub),
    ("TcDeregisterClient", ExportKind::Stdcall),
    ("TcEnumerateFlows", ExportKind::Stub),
    ("TcEnumerateInterfaces", ExportKind::Stub),
    ("TcGetFlowNameA", ExportKind::Stub),
    ("TcGetFlowNameW", ExportKind::Stub),
    ("TcModifyFlow", ExportKind::Stub),
    ("TcOpenInterfaceA", ExportKind::Stub),
    ("TcOpenInterfaceW", ExportKind::Stub),
    ("TcQueryFlowA", ExportKind::Stub),
    ("TcQueryFlowW", ExportKind::Stub),
    ("TcQueryInterface", ExportKind::Stub),
    ("TcRegisterClient", ExportKind::Stdcall),
    ("TcSetFlowA", ExportKind::Stub),
    ("TcSetFlowW", ExportKind::Stub),
    ("TcSetInterface", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
