/// How `dlls/ctl3d.dll16/ctl3d.dll16.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `pascal`: a C function with a body in `dlls/ctl3d.dll16/ctl3d.c`.
    Pascal,
    /// `stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// One line of `dlls/ctl3d.dll16/ctl3d.dll16.spec`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Export {
    /// The ordinal at the start of the line.
    pub ordinal: u16,
    /// The exported name, which is the `function` argument `winebuild` passes to a stub.
    pub name: &'static str,
    pub kind: ExportKind,
}

/// The `ctl3d.dll16.spec` export table, in spec order. Ordinals 14 and 15 are not exported.
pub const EXPORTS: [Export; 25] = [
    Export {
        ordinal: 1,
        name: "Ctl3dGetVer",
        kind: ExportKind::Pascal,
    },
    Export {
        ordinal: 2,
        name: "Ctl3dSubclassDlg",
        kind: ExportKind::Pascal,
    },
    Export {
        ordinal: 3,
        name: "Ctl3dSubclassCtl",
        kind: ExportKind::Pascal,
    },
    Export {
        ordinal: 4,
        name: "Ctl3dCtlColor",
        kind: ExportKind::Pascal,
    },
    Export {
        ordinal: 5,
        name: "Ctl3dEnabled",
        kind: ExportKind::Pascal,
    },
    Export {
        ordinal: 6,
        name: "Ctl3dColorChange",
        kind: ExportKind::Pascal,
    },
    Export {
        ordinal: 7,
        name: "BtnWndProc3d",
        kind: ExportKind::Pascal,
    },
    Export {
        ordinal: 8,
        name: "EditWndProc3d",
        kind: ExportKind::Pascal,
    },
    Export {
        ordinal: 9,
        name: "ListWndProc3d",
        kind: ExportKind::Pascal,
    },
    Export {
        ordinal: 10,
        name: "ComboWndProc3d",
        kind: ExportKind::Pascal,
    },
    Export {
        ordinal: 11,
        name: "StaticWndProc3d",
        kind: ExportKind::Pascal,
    },
    Export {
        ordinal: 12,
        name: "Ctl3dRegister",
        kind: ExportKind::Pascal,
    },
    Export {
        ordinal: 13,
        name: "Ctl3dUnregister",
        kind: ExportKind::Pascal,
    },
    Export {
        ordinal: 16,
        name: "Ctl3dAutoSubclass",
        kind: ExportKind::Pascal,
    },
    Export {
        ordinal: 17,
        name: "Ctl3dDlgProc",
        kind: ExportKind::Pascal,
    },
    Export {
        ordinal: 18,
        name: "Ctl3dCtlColorEx",
        kind: ExportKind::Pascal,
    },
    Export {
        ordinal: 19,
        name: "Ctl3dSetStyle",
        kind: ExportKind::Stub,
    },
    Export {
        ordinal: 20,
        name: "Ctl3dDlgFramePaint",
        kind: ExportKind::Pascal,
    },
    Export {
        ordinal: 21,
        name: "Ctl3dSubclassDlgEx",
        kind: ExportKind::Pascal,
    },
    Export {
        ordinal: 22,
        name: "Ctl3dWinIniChange",
        kind: ExportKind::Pascal,
    },
    Export {
        ordinal: 23,
        name: "Ctl3dIsAutoSubclass",
        kind: ExportKind::Pascal,
    },
    Export {
        ordinal: 24,
        name: "Ctl3dUnAutoSubclass",
        kind: ExportKind::Pascal,
    },
    Export {
        ordinal: 25,
        name: "Ctl3dSubclassCtlEx",
        kind: ExportKind::Pascal,
    },
    Export {
        ordinal: 26,
        name: "Ctl3dUnsubclassCtl",
        kind: ExportKind::Pascal,
    },
    Export {
        ordinal: 27,
        name: "Ctl3dAutoSubclassEx",
        kind: ExportKind::Pascal,
    },
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|export| export.name == name)
        .map(|export| export.kind)
}
