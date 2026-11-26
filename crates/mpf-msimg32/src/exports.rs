/// How `dlls/msimg32/msimg32.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// An ordinary `stdcall` function from `msimg32_main.c`, ported as [`crate::v_set_ddrawflag`].
    CFunction,
    /// `@ stdcall` forwarder: the loader resolves the name in another module, so msimg32 has no code.
    Forward {
        module: &'static str,
        function: &'static str,
    },
    /// `@ stdcall -private`: the name is implemented by the module's `DllMain`, ported as
    /// [`crate::dll_main`].
    Private { implementation: &'static str },
}

/// The `msimg32.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 5] = [
    ("vSetDdrawflag", ExportKind::CFunction),
    (
        "AlphaBlend",
        ExportKind::Forward {
            module: "gdi32",
            function: "GdiAlphaBlend",
        },
    ),
    (
        "DllInitialize",
        ExportKind::Private {
            implementation: "DllMain",
        },
    ),
    (
        "GradientFill",
        ExportKind::Forward {
            module: "gdi32",
            function: "GdiGradientFill",
        },
    ),
    (
        "TransparentBlt",
        ExportKind::Forward {
            module: "gdi32",
            function: "GdiTransparentBlt",
        },
    ),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
