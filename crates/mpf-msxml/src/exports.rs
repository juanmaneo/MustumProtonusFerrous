/// `MODULE` from `dlls/msxml/Makefile.in`: the module name `winebuild` passes to stub records.
pub const MODULE_NAME: &str = "msxml.dll";

/// One entry of `dlls/msxml/msxml.spec`. Every entry is `@ stdcall -private`: `winebuild` sets
/// `FLAG_PRIVATE` on it, so the export cannot be imported.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Export {
    /// The exported name.
    pub name: &'static str,
    /// The number of entries in the spec's argument list, for example 3 for `(ptr ptr ptr)`.
    pub arg_count: usize,
    /// The implementation name when the export is forwarded, for example `msxml3.DllGetClassObject`.
    pub forward: Option<&'static str>,
}

/// The `msxml.spec` export table, in spec order. Only `DllGetClassObject` is forwarded. The other
/// three bind to functions of the same name, which `dlls/msxml` has no C sources to define.
pub const EXPORTS: [Export; 4] = [
    Export {
        name: "DllCanUnloadNow",
        arg_count: 0,
        forward: None,
    },
    Export {
        name: "DllGetClassObject",
        arg_count: 3,
        forward: Some("msxml3.DllGetClassObject"),
    },
    Export {
        name: "DllRegisterServer",
        arg_count: 0,
        forward: None,
    },
    Export {
        name: "DllUnregisterServer",
        arg_count: 0,
        forward: None,
    },
];

/// The export named `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export(name: &str) -> Option<Export> {
    EXPORTS.iter().find(|entry| entry.name == name).copied()
}
