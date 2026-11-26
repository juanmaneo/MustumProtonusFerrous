/// How `dlls/mtxdm/mtxdm.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ cdecl`: a real function, ported as [`crate::get_dispenser_manager`].
    Cdecl,
}

/// The `mtxdm.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 1] = [("GetDispenserManager", ExportKind::Cdecl)];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
