/// How `dlls/bthprops.cpl/bthprops.cpl.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: a C function whose body would be in a C source. This reference has none, so it is not ported.
    Stdcall,
    /// `@ stdcall -import`: a thunk that jumps to the same function in `bluetoothapis`. Not ported.
    Import,
    /// `@ stdcall -private`: not exported by the DLL, and no body in this reference, so it is not ported.
    Private,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

use ExportKind::{Import, Private, Stdcall, Stub};

/// The `bthprops.cpl.spec` export table, in spec order. The commented-out `CPlApplet` line is not in it.
pub const EXPORTS: [(&str, ExportKind); 64] = [
    ("BluetoothAddressToString", Stub),
    ("BluetoothAuthenticateDevice", Stub),
    ("BluetoothAuthenticateDeviceEx", Import),
    ("BluetoothAuthenticateMultipleDevices", Stub),
    ("BluetoothAuthenticationAgent", Stub),
    ("BluetoothDisconnectDevice", Stub),
    ("BluetoothDisplayDeviceProperties", Stub),
    ("BluetoothEnableDiscovery", Import),
    ("BluetoothEnableIncomingConnections", Import),
    ("BluetoothEnumerateInstalledServices", Stub),
    ("BluetoothEnumerateInstalledServicesEx", Stub),
    ("BluetoothFindBrowseGroupClose", Stub),
    ("BluetoothFindClassIdClose", Stub),
    ("BluetoothFindDeviceClose", Import),
    ("BluetoothFindFirstBrowseGroup", Stub),
    ("BluetoothFindFirstClassId", Stub),
    ("BluetoothFindFirstDevice", Import),
    ("BluetoothFindFirstProfileDescriptor", Stub),
    ("BluetoothFindFirstProtocolDescriptorStack", Stub),
    ("BluetoothFindFirstProtocolEntry", Stub),
    ("BluetoothFindFirstRadio", Import),
    ("BluetoothFindFirstService", Stub),
    ("BluetoothFindFirstServiceEx", Stub),
    ("BluetoothFindNextBrowseGroup", Stub),
    ("BluetoothFindNextClassId", Stub),
    ("BluetoothFindNextDevice", Import),
    ("BluetoothFindNextProfileDescriptor", Stub),
    ("BluetoothFindNextProtocolDescriptorStack", Stub),
    ("BluetoothFindNextProtocolEntry", Stub),
    ("BluetoothFindNextRadio", Import),
    ("BluetoothFindNextService", Stub),
    ("BluetoothFindProfileDescriptorClose", Stub),
    ("BluetoothFindProtocolDescriptorStackClose", Stub),
    ("BluetoothFindProtocolEntryClose", Stub),
    ("BluetoothFindRadioClose", Import),
    ("BluetoothFindServiceClose", Stub),
    ("BluetoothGetDeviceInfo", Stdcall),
    ("BluetoothGetRadioInfo", Import),
    ("BluetoothIsConnectable", Import),
    ("BluetoothIsDiscoverable", Import),
    ("BluetoothIsVersionAvailable", Stub),
    ("BluetoothMapClassOfDeviceToImageIndex", Stub),
    ("BluetoothMapClassOfDeviceToString", Stub),
    ("BluetoothRegisterForAuthentication", Stub),
    ("BluetoothRegisterForAuthenticationEx", Import),
    ("BluetoothRemoveDevice", Import),
    ("BluetoothSdpEnumAttributes", Import),
    ("BluetoothSdpGetAttributeValue", Import),
    ("BluetoothSdpGetContainerElementData", Import),
    ("BluetoothSdpGetElementData", Import),
    ("BluetoothSdpGetString", Stub),
    ("BluetoothSelectDevices", Stub),
    ("BluetoothSelectDevicesFree", Stub),
    ("BluetoothSendAuthenticationResponse", Stub),
    ("BluetoothSendAuthenticationResponseEx", Import),
    ("BluetoothSetLocalServiceInfo", Stub),
    ("BluetoothSetServiceState", Stub),
    ("BluetoothUnregisterAuthentication", Import),
    ("BluetoothUpdateDeviceRecord", Stub),
    ("BthpEnableAllServices", Stub),
    ("BthpFindPnpInfo", Stub),
    ("BthpMapStatusToErr", Stub),
    ("DllCanUnloadNow", Private),
    ("DllGetClassObject", Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
