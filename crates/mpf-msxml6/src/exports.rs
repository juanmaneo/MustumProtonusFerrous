/// How a line of dlls/msxml6/msxml6.spec declares its export. Every line is stdcall -private.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// No forward target. This reference has no C body for the function.
    Private,
    /// A forward to function in module, written module.function after the parameters.
    Forward {
        module: &'static str,
        function: &'static str,
    },
}

/// One line of the msxml6.spec export table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Export {
    pub name: &'static str,
    /// The parameter types between the parentheses, in order.
    pub params: &'static [&'static str],
    pub kind: ExportKind,
}

/// The msxml6.spec export table, in spec order.
pub const EXPORTS: [Export; 4] = [
    Export {
        name: "DllCanUnloadNow",
        params: &[],
        kind: ExportKind::Private,
    },
    Export {
        name: "DllGetClassObject",
        params: &["ptr", "ptr", "ptr"],
        kind: ExportKind::Forward {
            module: "msxml3",
            function: "DllGetClassObject",
        },
    },
    Export {
        name: "DllRegisterServer",
        params: &[],
        kind: ExportKind::Private,
    },
    Export {
        name: "DllUnregisterServer",
        params: &[],
        kind: ExportKind::Private,
    },
];

/// The export named name, or None when the spec does not export it. Names are case-sensitive.
pub fn export(name: &str) -> Option<Export> {
    EXPORTS.iter().find(|entry| entry.name == name).copied()
}
