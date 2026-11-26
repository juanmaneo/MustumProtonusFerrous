/// The calling convention that `dlls/ia2comproxy/ia2comproxy.spec` gives an export.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CallingConvention {
    /// `stdcall`.
    Stdcall,
}

/// One line of `dlls/ia2comproxy/ia2comproxy.spec`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Export {
    pub name: &'static str,
    pub convention: CallingConvention,
    /// `-private`, which winebuild stores as `FLAG_PRIVATE`: "function is private (cannot be imported)".
    pub private: bool,
    /// The parameter types between the parentheses, in spec order. Empty for `()`.
    pub params: &'static [&'static str],
}

/// The `ia2comproxy.spec` export table, in spec order.
pub const EXPORTS: [Export; 4] = [
    Export {
        name: "DllCanUnloadNow",
        convention: CallingConvention::Stdcall,
        private: true,
        params: &[],
    },
    Export {
        name: "DllGetClassObject",
        convention: CallingConvention::Stdcall,
        private: true,
        params: &["ptr", "ptr", "ptr"],
    },
    Export {
        name: "DllRegisterServer",
        convention: CallingConvention::Stdcall,
        private: true,
        params: &[],
    },
    Export {
        name: "DllUnregisterServer",
        convention: CallingConvention::Stdcall,
        private: true,
        params: &[],
    },
];

/// The export named `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn find_export(name: &str) -> Option<Export> {
    EXPORTS.iter().find(|export| export.name == name).copied()
}
