/// One line of `dlls/ieproxy/ieproxy.spec`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Export {
    /// The exported name.
    pub name: &'static str,
    /// The ordinal written in the spec, or `None` for `@`, which winebuild assigns.
    pub ordinal: Option<u32>,
    /// The `stdcall` convention.
    pub stdcall: bool,
    /// The `-private` flag.
    pub private: bool,
    /// The parameter types in the spec. Every one is `ptr`.
    pub params: &'static [&'static str],
    /// The callee that the `WINE_REGISTER_DLL` wrapper in `include/rpcproxy.h` returns, or `None`
    /// when the wrapper calls none.
    pub delegate: Option<&'static str>,
}

/// The DLL file name, from `MODULE = ieproxy.dll` in `dlls/ieproxy/Makefile.in`.
pub const MODULE: &str = "ieproxy.dll";

/// The `ieproxy.spec` export table, in spec order.
pub const EXPORTS: [Export; 5] = [
    Export {
        name: "GetProxyDllInfo",
        ordinal: Some(3),
        stdcall: true,
        private: true,
        params: &["ptr", "ptr"],
        delegate: None,
    },
    Export {
        name: "DllCanUnloadNow",
        ordinal: None,
        stdcall: true,
        private: true,
        params: &[],
        delegate: Some("NdrDllCanUnloadNow"),
    },
    Export {
        name: "DllGetClassObject",
        ordinal: None,
        stdcall: true,
        private: true,
        params: &["ptr", "ptr", "ptr"],
        delegate: Some("NdrDllGetClassObject"),
    },
    Export {
        name: "DllRegisterServer",
        ordinal: None,
        stdcall: true,
        private: true,
        params: &[],
        delegate: Some("__wine_register_resources"),
    },
    Export {
        name: "DllUnregisterServer",
        ordinal: None,
        stdcall: true,
        private: true,
        params: &[],
        delegate: Some("__wine_unregister_resources"),
    },
];

/// winebuild's ordinal assignment, `assign_ordinals` in `tools/winebuild/parser.c`.
///
/// The base is the lowest explicit ordinal, or 1 when no entry is explicit. Each `@` entry, in spec
/// order, takes the lowest ordinal at or above the base that no other entry uses.
pub fn assign_ordinals(exports: &[Export]) -> Vec<u32> {
    let mut used: Vec<u32> = exports.iter().filter_map(|e| e.ordinal).collect();
    let base = used.iter().copied().min().unwrap_or(1);
    exports
        .iter()
        .map(|export| match export.ordinal {
            Some(ordinal) => ordinal,
            None => {
                let mut ordinal = base;
                while used.contains(&ordinal) {
                    ordinal += 1;
                }
                used.push(ordinal);
                ordinal
            }
        })
        .collect()
}

/// The ordinal that `name` exports from `ieproxy.spec`, or `None` when the spec does not export it.
/// Names are case-sensitive.
pub fn export_ordinal(name: &str) -> Option<u32> {
    EXPORTS
        .iter()
        .zip(assign_ordinals(&EXPORTS))
        .find(|(export, _)| export.name == name)
        .map(|(_, ordinal)| ordinal)
}

/// The name that `ordinal` exports from `ieproxy.spec`, or `None` when no entry has it.
pub fn export_name(ordinal: u32) -> Option<&'static str> {
    EXPORTS
        .iter()
        .zip(assign_ordinals(&EXPORTS))
        .find(|(_, assigned)| *assigned == ordinal)
        .map(|(export, _)| export.name)
}

/// The entry that `name` is in `ieproxy.spec`, or `None` when the spec does not export it.
pub fn find_export(name: &str) -> Option<Export> {
    EXPORTS.iter().find(|export| export.name == name).copied()
}

/// The `@N` suffix that winebuild adds to a stdcall export for i386: four bytes per parameter.
/// Every parameter in `ieproxy.spec` is `ptr`, which is four bytes on i386.
pub fn i386_stdcall_bytes(export: &Export) -> Option<u32> {
    export.stdcall.then(|| export.params.len() as u32 * 4)
}
