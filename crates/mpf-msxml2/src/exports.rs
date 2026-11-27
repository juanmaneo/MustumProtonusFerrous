/// How `dlls/msxml2/msxml2.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// A forwarder to an export of another DLL, with the `dll.export` text of the spec line.
    Forward(&'static str),
    /// `@ stdcall -private` with no C body in `dlls/msxml2`. winebuild reports
    /// `function 'NAME' not defined` for it unless `--external-symbols` is given, and the
    /// build files in this tree do not give that flag.
    Undefined,
}

impl ExportKind {
    /// The `dll.export` target of a forwarder, or `None` for an export that has no forwarder.
    pub fn forward_target(self) -> Option<&'static str> {
        match self {
            Self::Forward(target) => Some(target),
            Self::Undefined => None,
        }
    }
}

/// The `msxml2.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 4] = [
    ("DllCanUnloadNow", ExportKind::Undefined),
    (
        "DllGetClassObject",
        ExportKind::Forward("msxml3.DllGetClassObject"),
    ),
    ("DllRegisterServer", ExportKind::Undefined),
    ("DllUnregisterServer", ExportKind::Undefined),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
