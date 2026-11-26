/// How `dlls/cryptdll/cryptdll.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
    /// `@ stdcall ... dll.name`: forwarded to the `dll.name` export of another DLL.
    Forward(&'static str),
}

/// The `cryptdll.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 14] = [
    ("CDBuildIntegrityVect", ExportKind::Stub),
    ("CDBuildVect", ExportKind::Stub),
    ("CDFindCommonCSystem", ExportKind::Stub),
    ("CDFindCommonCSystemWithKey", ExportKind::Stub),
    ("CDGenerateRandomBits", ExportKind::Stub),
    ("CDLocateCSystem", ExportKind::Stub),
    ("CDLocateCheckSum", ExportKind::Stub),
    ("CDLocateRng", ExportKind::Stub),
    ("CDRegisterCSystem", ExportKind::Stub),
    ("CDRegisterCheckSum", ExportKind::Stub),
    ("CDRegisterRng", ExportKind::Stub),
    ("MD5Final", ExportKind::Forward("advapi32.MD5Final")),
    ("MD5Init", ExportKind::Forward("advapi32.MD5Init")),
    ("MD5Update", ExportKind::Forward("advapi32.MD5Update")),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
