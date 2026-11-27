/// Where `dlls/msxml4/msxml4.spec` sends an export.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportTarget {
    /// The spec gives no target, and `dlls/msxml4` has no body for the export.
    Unbound,
    /// `name(args) module.function`: the export is forwarded to another DLL.
    Forward {
        module: &'static str,
        function: &'static str,
    },
}

/// One line of `dlls/msxml4/msxml4.spec`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExportEntry {
    pub name: &'static str,
    pub convention: &'static str,
    pub private: bool,
    pub params: &'static [&'static str],
    pub target: ExportTarget,
}

/// The `msxml4.spec` export table, in spec order.
pub const EXPORTS: [ExportEntry; 4] = [
    ExportEntry {
        name: "DllCanUnloadNow",
        convention: "stdcall",
        private: true,
        params: &[],
        target: ExportTarget::Unbound,
    },
    ExportEntry {
        name: "DllGetClassObject",
        convention: "stdcall",
        private: true,
        params: &["ptr", "ptr", "ptr"],
        target: ExportTarget::Forward {
            module: "msxml3",
            function: "DllGetClassObject",
        },
    },
    ExportEntry {
        name: "DllRegisterServer",
        convention: "stdcall",
        private: true,
        params: &[],
        target: ExportTarget::Unbound,
    },
    ExportEntry {
        name: "DllUnregisterServer",
        convention: "stdcall",
        private: true,
        params: &[],
        target: ExportTarget::Unbound,
    },
];

/// The entry for `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_entry(name: &str) -> Option<ExportEntry> {
    EXPORTS.iter().find(|entry| entry.name == name).copied()
}
