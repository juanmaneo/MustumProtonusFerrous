//! Oracle-derived test vectors for the ported irprops.cpl logic.
//!
//! The stub rows were captured by compiling the verbatim `__wine_spec_unimplemented_stub` body
//! from `dlls/ntdll/exception.c` with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/irprops.cpl/` and running the harness. The harness replaces `RtlRaiseException`
//! with a recorder that longjmps out, because the C loop never returns. ASan and UBSan flagged no
//! row, so none is excluded. The export rows are the 51 lines of
//! `dlls/irprops.cpl/irprops.cpl.spec`, extracted with sed; winebuild was not run.

use super::*;

type StubHelper = fn() -> StubException;

const ORACLE_STUB_ROWS: [(&str, StubHelper); 33] = [
    ("BluetoothAuthenticateDevice", bluetooth_authenticate_device),
    (
        "BluetoothAuthenticateMultipleDevices",
        bluetooth_authenticate_multiple_devices,
    ),
    (
        "BluetoothAuthenticationAgent",
        bluetooth_authentication_agent,
    ),
    ("BluetoothDisconnectDevice", bluetooth_disconnect_device),
    (
        "BluetoothDisplayDeviceProperties",
        bluetooth_display_device_properties,
    ),
    (
        "BluetoothEnumerateInstalledServices",
        bluetooth_enumerate_installed_services,
    ),
    (
        "BluetoothFindBrowseGroupClose",
        bluetooth_find_browse_group_close,
    ),
    ("BluetoothFindClassIdClose", bluetooth_find_class_id_close),
    (
        "BluetoothFindFirstBrowseGroup",
        bluetooth_find_first_browse_group,
    ),
    ("BluetoothFindFirstClassId", bluetooth_find_first_class_id),
    (
        "BluetoothFindFirstProfileDescriptor",
        bluetooth_find_first_profile_descriptor,
    ),
    (
        "BluetoothFindFirstProtocolDescriptorStack",
        bluetooth_find_first_protocol_descriptor_stack,
    ),
    (
        "BluetoothFindFirstProtocolEntry",
        bluetooth_find_first_protocol_entry,
    ),
    ("BluetoothFindFirstService", bluetooth_find_first_service),
    (
        "BluetoothFindNextBrowseGroup",
        bluetooth_find_next_browse_group,
    ),
    ("BluetoothFindNextClassId", bluetooth_find_next_class_id),
    (
        "BluetoothFindNextProfileDescriptor",
        bluetooth_find_next_profile_descriptor,
    ),
    (
        "BluetoothFindNextProtocolDescriptorStack",
        bluetooth_find_next_protocol_descriptor_stack,
    ),
    (
        "BluetoothFindNextProtocolEntry",
        bluetooth_find_next_protocol_entry,
    ),
    ("BluetoothFindNextService", bluetooth_find_next_service),
    (
        "BluetoothFindProfileDescriptorClose",
        bluetooth_find_profile_descriptor_close,
    ),
    (
        "BluetoothFindProtocolDescriptorStackClose",
        bluetooth_find_protocol_descriptor_stack_close,
    ),
    (
        "BluetoothFindProtocolEntryClose",
        bluetooth_find_protocol_entry_close,
    ),
    ("BluetoothFindServiceClose", bluetooth_find_service_close),
    (
        "BluetoothMapClassOfDeviceToImageIndex",
        bluetooth_map_class_of_device_to_image_index,
    ),
    (
        "BluetoothMapClassOfDeviceToString",
        bluetooth_map_class_of_device_to_string,
    ),
    (
        "BluetoothRegisterForAuthentication",
        bluetooth_register_for_authentication,
    ),
    ("BluetoothSdpGetString", bluetooth_sdp_get_string),
    ("BluetoothSelectDevices", bluetooth_select_devices),
    ("BluetoothSelectDevicesFree", bluetooth_select_devices_free),
    (
        "BluetoothSendAuthenticationResponse",
        bluetooth_send_authentication_response,
    ),
    ("BluetoothSetServiceState", bluetooth_set_service_state),
    (
        "BluetoothUpdateDeviceRecord",
        bluetooth_update_device_record,
    ),
];

const ORACLE_EXPORT_ROWS: [(&str, ExportKind); 51] = [
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

#[test]
fn oracle_stub_rows() {
    // rows 01-33: raise code=0x80000100 flags=0x1 nargs=2 p0=irprops.cpl p1=<export name>
    for (name, helper) in ORACLE_STUB_ROWS {
        assert_eq!(
            helper(),
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "irprops.cpl",
                function: name,
            }
        );
    }
}

#[test]
fn oracle_export_rows() {
    // rows 01-51: one line of irprops.cpl.spec each, in spec order.
    assert_eq!(EXPORTS, ORACLE_EXPORT_ROWS);
}

#[test]
fn oracle_stub_rows_are_the_stub_exports() {
    let stub_exports: Vec<&str> = ORACLE_EXPORT_ROWS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .map(|(name, _)| *name)
        .collect();
    let stub_rows: Vec<&str> = ORACLE_STUB_ROWS.iter().map(|(name, _)| *name).collect();
    assert_eq!(stub_exports, stub_rows);
}
