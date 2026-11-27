/// `MODULE` from `dlls/msctfp/Makefile.in`, which `winebuild` writes as the `LIBRARY` name.
pub const MODULE: &str = "msctfp.dll";

/// One line of `dlls/msctfp/msctfp.spec`. Every line is `@ stdcall -private`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Export {
    pub name: &'static str,
    /// The `@` ordinal. `winebuild` numbers the spec lines from 1 in spec order.
    pub ordinal: u16,
    /// Parameters in the spec line, for example `DllGetClassObject(ptr ptr ptr)` has 3.
    pub param_count: u32,
}

impl Export {
    /// The `@N` suffix that `winebuild --def -m32` gives a stdcall export: 4 bytes per parameter.
    pub fn i386_stack_bytes(&self) -> u32 {
        self.param_count * 4
    }
}

/// The `msctfp.spec` export table, in spec order. `-private` sets `FLAG_PRIVATE`, which
/// `tools/winebuild/build.h` describes as "function is private (cannot be imported)".
pub const EXPORTS: [Export; 5] = [
    Export {
        name: "DllCanUnloadNow",
        ordinal: 1,
        param_count: 0,
    },
    Export {
        name: "DllGetClassObject",
        ordinal: 2,
        param_count: 3,
    },
    Export {
        name: "DllRegisterServer",
        ordinal: 3,
        param_count: 0,
    },
    Export {
        name: "DllUnregisterServer",
        ordinal: 4,
        param_count: 0,
    },
    Export {
        name: "GetProxyDllInfo",
        ordinal: 5,
        param_count: 2,
    },
];

/// The export named `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn find_export(name: &str) -> Option<&'static Export> {
    EXPORTS.iter().find(|export| export.name == name)
}
