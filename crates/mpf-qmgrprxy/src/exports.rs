/// One `@ stdcall -private` line of `dlls/qmgrprxy/qmgrprxy.spec`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Export {
    /// The export name.
    pub name: &'static str,
    /// The number of parameters in the spec prototype. Every parameter is `ptr`.
    pub params: u32,
    /// Whether the spec marks the export `-private`.
    pub private: bool,
}

/// The `qmgrprxy.spec` export table, in spec order.
pub const EXPORTS: [Export; 5] = [
    Export {
        name: "DllCanUnloadNow",
        params: 0,
        private: true,
    },
    Export {
        name: "DllGetClassObject",
        params: 3,
        private: true,
    },
    Export {
        name: "DllRegisterServer",
        params: 0,
        private: true,
    },
    Export {
        name: "DllUnregisterServer",
        params: 0,
        private: true,
    },
    Export {
        name: "GetProxyDllInfo",
        params: 2,
        private: true,
    },
];

/// The export named `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export(name: &str) -> Option<Export> {
    EXPORTS.iter().copied().find(|e| e.name == name)
}

/// The stack bytes that a `stdcall` export with `params` parameters takes, as `winebuild`'s
/// `get_args_size` computes them when every parameter is `ptr`. `ptr_size` is 4 on i386 and 8 on
/// x86_64.
pub fn args_size(params: u32, ptr_size: u32) -> u32 {
    params * ptr_size
}
