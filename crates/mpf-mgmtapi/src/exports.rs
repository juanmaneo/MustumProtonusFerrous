/// How `dlls/mgmtapi/mgmtapi.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: a real function, ported as [`crate::snmp_mgr_open`] or [`crate::snmp_mgr_trap_listen`].
    Stdcall,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `mgmtapi.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 9] = [
    ("SnmpMgrClose", ExportKind::Stub),
    ("SnmpMgrCtl", ExportKind::Stub),
    ("SnmpMgrGetTrap", ExportKind::Stub),
    ("SnmpMgrGetTrapEx", ExportKind::Stub),
    ("SnmpMgrOidToStr", ExportKind::Stub),
    ("SnmpMgrOpen", ExportKind::Stdcall),
    ("SnmpMgrRequest", ExportKind::Stub),
    ("SnmpMgrStrToOid", ExportKind::Stub),
    ("SnmpMgrTrapListen", ExportKind::Stdcall),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
