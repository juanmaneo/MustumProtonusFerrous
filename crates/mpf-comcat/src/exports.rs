/// One record of the `dlls/comcat/comcat.spec` export table, as winebuild resolves it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Export {
    /// The export name, which is also the C name.
    pub name: &'static str,
    /// The ordinal winebuild assigns in spec order, starting at 1.
    pub ordinal: u32,
    /// The stdcall argument bytes on i386. Every parameter in `comcat.spec` is a `ptr`, 4 bytes each.
    pub arg_bytes: u32,
    /// The `module.function` target of a forwarded export.
    pub forward: Option<&'static str>,
    /// `-private`: other DLLs cannot import the export, only `GetProcAddress` can reach it.
    pub private: bool,
}

/// The `comcat.spec` export table, in spec order.
pub const EXPORTS: [Export; 4] = [
    Export {
        name: "DllCanUnloadNow",
        ordinal: 1,
        arg_bytes: 0,
        forward: None,
        private: true,
    },
    Export {
        name: "DllGetClassObject",
        ordinal: 2,
        arg_bytes: 12,
        forward: Some("ole32.DllGetClassObject"),
        private: true,
    },
    Export {
        name: "DllRegisterServer",
        ordinal: 3,
        arg_bytes: 0,
        forward: None,
        private: true,
    },
    Export {
        name: "DllUnregisterServer",
        ordinal: 4,
        arg_bytes: 0,
        forward: None,
        private: true,
    },
];

/// The record for `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export(name: &str) -> Option<Export> {
    EXPORTS.iter().find(|e| e.name == name).copied()
}
