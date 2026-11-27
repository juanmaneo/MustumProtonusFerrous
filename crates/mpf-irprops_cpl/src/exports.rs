/// How `dlls/irprops.cpl/irprops.cpl.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
    /// `@ stdcall` with a `dll.name` target: forwards to that export of another DLL.
    Forward(&'static str),
    /// `@ stdcall -import`: jumps through `__imp_` to the same name in `bluetoothapis`, the DLL
    /// in `IMPORTS` of `dlls/irprops.cpl/Makefile.in`.
    Import,
    /// `@ stdcall` with no target and no `-import`. `dlls/irprops.cpl` has no body for it, so it
    /// is listed and not modelled.
    StdcallNoBody,
}

/// The `irprops.cpl.spec` export table, in spec order. Each entry is one spec line.
pub const EXPORTS: [(&str, ExportKind); 51] = [
    ("BluetoothAuthenticateDevice", ExportKind::Stub),
    ("BluetoothAuthenticateMultipleDevices", ExportKind::Stub),
    ("BluetoothAuthenticationAgent", ExportKind::Stub),
    ("BluetoothDisconnectDevice", ExportKind::Stub),
    ("BluetoothDisplayDeviceProperties", ExportKind::Stub),
    (
        "BluetoothEnableDiscovery",
        ExportKind::Forward("bthprops.cpl.BluetoothEnableDiscovery"),
    ),
    (
        "BluetoothEnableIncomingConnections",
        ExportKind::Forward("bthprops.cpl.BluetoothEnableIncomingConnections"),
    ),
    ("BluetoothEnumerateInstalledServices", ExportKind::Stub),
    ("BluetoothFindBrowseGroupClose", ExportKind::Stub),
    ("BluetoothFindClassIdClose", ExportKind::Stub),
    ("BluetoothFindDeviceClose", ExportKind::Import),
    ("BluetoothFindFirstBrowseGroup", ExportKind::Stub),
    ("BluetoothFindFirstClassId", ExportKind::Stub),
    ("BluetoothFindFirstDevice", ExportKind::Import),
    ("BluetoothFindFirstProfileDescriptor", ExportKind::Stub),
    (
        "BluetoothFindFirstProtocolDescriptorStack",
        ExportKind::Stub,
    ),
    ("BluetoothFindFirstProtocolEntry", ExportKind::Stub),
    ("BluetoothFindFirstRadio", ExportKind::Import),
    ("BluetoothFindFirstService", ExportKind::Stub),
    ("BluetoothFindNextBrowseGroup", ExportKind::Stub),
    ("BluetoothFindNextClassId", ExportKind::Stub),
    ("BluetoothFindNextDevice", ExportKind::Import),
    ("BluetoothFindNextProfileDescriptor", ExportKind::Stub),
    ("BluetoothFindNextProtocolDescriptorStack", ExportKind::Stub),
    ("BluetoothFindNextProtocolEntry", ExportKind::Stub),
    ("BluetoothFindNextRadio", ExportKind::Import),
    ("BluetoothFindNextService", ExportKind::Stub),
    ("BluetoothFindProfileDescriptorClose", ExportKind::Stub),
    (
        "BluetoothFindProtocolDescriptorStackClose",
        ExportKind::Stub,
    ),
    ("BluetoothFindProtocolEntryClose", ExportKind::Stub),
    ("BluetoothFindRadioClose", ExportKind::Import),
    ("BluetoothFindServiceClose", ExportKind::Stub),
    ("BluetoothGetDeviceInfo", ExportKind::StdcallNoBody),
    ("BluetoothGetRadioInfo", ExportKind::Import),
    (
        "BluetoothIsConnectable",
        ExportKind::Forward("bthprops.cpl.BluetoothIsConnectable"),
    ),
    (
        "BluetoothIsDiscoverable",
        ExportKind::Forward("bthprops.cpl.BluetoothIsDiscoverable"),
    ),
    ("BluetoothMapClassOfDeviceToImageIndex", ExportKind::Stub),
    ("BluetoothMapClassOfDeviceToString", ExportKind::Stub),
    ("BluetoothRegisterForAuthentication", ExportKind::Stub),
    ("BluetoothRemoveDevice", ExportKind::Import),
    ("BluetoothSdpEnumAttributes", ExportKind::Import),
    ("BluetoothSdpGetAttributeValue", ExportKind::Import),
    ("BluetoothSdpGetContainerElementData", ExportKind::Import),
    ("BluetoothSdpGetElementData", ExportKind::Import),
    ("BluetoothSdpGetString", ExportKind::Stub),
    ("BluetoothSelectDevices", ExportKind::Stub),
    ("BluetoothSelectDevicesFree", ExportKind::Stub),
    ("BluetoothSendAuthenticationResponse", ExportKind::Stub),
    ("BluetoothSetServiceState", ExportKind::Stub),
    ("BluetoothUnregisterAuthentication", ExportKind::Import),
    ("BluetoothUpdateDeviceRecord", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
