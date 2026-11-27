//! The dsound.spec export table, in spec order.

/// How dlls/dsound/dsound.spec exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `stdcall`: a real function with an ordinal.
    Stdcall,
    /// `stdcall -private`: exported by name, with no ordinal in the spec.
    StdcallPrivate,
}

/// One line of dsound.spec.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Export {
    /// The spec ordinal, or None when the spec gives none.
    pub ordinal: Option<u16>,
    pub name: &'static str,
    pub kind: ExportKind,
}

/// The dsound.spec export table. Ordinals 4 and 5 are absent from the spec.
pub static EXPORTS: [Export; 14] = [
    Export {
        ordinal: Some(1),
        name: "DirectSoundCreate",
        kind: ExportKind::Stdcall,
    },
    Export {
        ordinal: Some(2),
        name: "DirectSoundEnumerateA",
        kind: ExportKind::Stdcall,
    },
    Export {
        ordinal: Some(3),
        name: "DirectSoundEnumerateW",
        kind: ExportKind::Stdcall,
    },
    Export {
        ordinal: Some(6),
        name: "DirectSoundCaptureCreate",
        kind: ExportKind::Stdcall,
    },
    Export {
        ordinal: Some(7),
        name: "DirectSoundCaptureEnumerateA",
        kind: ExportKind::Stdcall,
    },
    Export {
        ordinal: Some(8),
        name: "DirectSoundCaptureEnumerateW",
        kind: ExportKind::Stdcall,
    },
    Export {
        ordinal: Some(9),
        name: "GetDeviceID",
        kind: ExportKind::Stdcall,
    },
    Export {
        ordinal: Some(10),
        name: "DirectSoundFullDuplexCreate",
        kind: ExportKind::Stdcall,
    },
    Export {
        ordinal: Some(11),
        name: "DirectSoundCreate8",
        kind: ExportKind::Stdcall,
    },
    Export {
        ordinal: Some(12),
        name: "DirectSoundCaptureCreate8",
        kind: ExportKind::Stdcall,
    },
    Export {
        ordinal: None,
        name: "DllCanUnloadNow",
        kind: ExportKind::StdcallPrivate,
    },
    Export {
        ordinal: None,
        name: "DllGetClassObject",
        kind: ExportKind::StdcallPrivate,
    },
    Export {
        ordinal: None,
        name: "DllRegisterServer",
        kind: ExportKind::StdcallPrivate,
    },
    Export {
        ordinal: None,
        name: "DllUnregisterServer",
        kind: ExportKind::StdcallPrivate,
    },
];

/// The export named `name`, or None when the spec does not export it. Names are case-sensitive.
pub fn export(name: &str) -> Option<&'static Export> {
    EXPORTS.iter().find(|export| export.name == name)
}

/// The export with spec ordinal `ordinal`, or None when no export has it.
pub fn export_by_ordinal(ordinal: u16) -> Option<&'static Export> {
    EXPORTS
        .iter()
        .find(|export| export.ordinal == Some(ordinal))
}
