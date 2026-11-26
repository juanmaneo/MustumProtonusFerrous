/// How `dlls/thumbcache/thumbcache.spec` lists a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `DllGetClassObject`: has a C body, ported as [`crate::dll_get_class_object`].
    Ported,
    /// Not defined in `thumbcache.c` and not `@ stub`, so there is no body to port.
    NotInSource,
}

/// The `thumbcache.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 4] = [
    ("DllCanUnloadNow", ExportKind::NotInSource),
    ("DllGetClassObject", ExportKind::Ported),
    ("DllRegisterServer", ExportKind::NotInSource),
    ("DllUnregisterServer", ExportKind::NotInSource),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
