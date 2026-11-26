/// How `dlls/npmshtml/npmshtml.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `stdcall`: a function defined in `dlls/npmshtml/main.c`, ported in [`crate::np_initialize`]
    /// and [`crate::np_shutdown`].
    Stdcall,
    /// `name(args) module.function` in the spec: the loader resolves the export in `module`.
    Forward {
        module: &'static str,
        function: &'static str,
    },
}

/// The `npmshtml.spec` export table, in spec order (ordinals 1 to 3).
pub const EXPORTS: [(&str, ExportKind); 3] = [
    (
        "NP_GetEntryPoints",
        ExportKind::Forward {
            module: "mshtml",
            function: "NP_GetEntryPoints",
        },
    ),
    ("NP_Initialize", ExportKind::Stdcall),
    ("NP_Shutdown", ExportKind::Stdcall),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
