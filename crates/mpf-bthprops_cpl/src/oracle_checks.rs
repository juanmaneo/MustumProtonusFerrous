//! Rows printed by `target/canon/bthprops.cpl/harness.c`. The harness copies the
//! `__wine_spec_unimplemented_stub` body from `dlls/ntdll/exception.c` and runs it once for each
//! `@ stub` name in `bthprops.cpl.spec`, with the module `bthprops.cpl`. Its recorder replaces
//! `RtlRaiseException` and jumps out of the stub's `for (;;)` loop. ASan and UBSan reported
//! nothing, so no row was excluded.

use super::*;

/// Rows 01 to 42. Every row has `code=0x80000100 flags=0x1 nparams=2` and the module
/// `bthprops.cpl`, with the export name as the function. Each tuple is the row number, the
/// function name, and the helper that must return the record the harness printed.
#[test]
fn oracle_stub_rows() {
    type Row = (usize, &'static str, fn() -> StubException);
    let rows: [Row; 42] = [
        (1, "BluetoothAddressToString", bluetooth_address_to_string),
        (
            2,
            "BluetoothAuthenticateDevice",
            bluetooth_authenticate_device,
        ),
        (
            3,
            "BluetoothAuthenticateMultipleDevices",
            bluetooth_authenticate_multiple_devices,
        ),
        (
            4,
            "BluetoothAuthenticationAgent",
            bluetooth_authentication_agent,
        ),
        (5, "BluetoothDisconnectDevice", bluetooth_disconnect_device),
        (
            6,
            "BluetoothDisplayDeviceProperties",
            bluetooth_display_device_properties,
        ),
        (
            7,
            "BluetoothEnumerateInstalledServices",
            bluetooth_enumerate_installed_services,
        ),
        (
            8,
            "BluetoothEnumerateInstalledServicesEx",
            bluetooth_enumerate_installed_services_ex,
        ),
        (
            9,
            "BluetoothFindBrowseGroupClose",
            bluetooth_find_browse_group_close,
        ),
        (
            10,
            "BluetoothFindClassIdClose",
            bluetooth_find_class_id_close,
        ),
        (
            11,
            "BluetoothFindFirstBrowseGroup",
            bluetooth_find_first_browse_group,
        ),
        (
            12,
            "BluetoothFindFirstClassId",
            bluetooth_find_first_class_id,
        ),
        (
            13,
            "BluetoothFindFirstProfileDescriptor",
            bluetooth_find_first_profile_descriptor,
        ),
        (
            14,
            "BluetoothFindFirstProtocolDescriptorStack",
            bluetooth_find_first_protocol_descriptor_stack,
        ),
        (
            15,
            "BluetoothFindFirstProtocolEntry",
            bluetooth_find_first_protocol_entry,
        ),
        (
            16,
            "BluetoothFindFirstService",
            bluetooth_find_first_service,
        ),
        (
            17,
            "BluetoothFindFirstServiceEx",
            bluetooth_find_first_service_ex,
        ),
        (
            18,
            "BluetoothFindNextBrowseGroup",
            bluetooth_find_next_browse_group,
        ),
        (19, "BluetoothFindNextClassId", bluetooth_find_next_class_id),
        (
            20,
            "BluetoothFindNextProfileDescriptor",
            bluetooth_find_next_profile_descriptor,
        ),
        (
            21,
            "BluetoothFindNextProtocolDescriptorStack",
            bluetooth_find_next_protocol_descriptor_stack,
        ),
        (
            22,
            "BluetoothFindNextProtocolEntry",
            bluetooth_find_next_protocol_entry,
        ),
        (23, "BluetoothFindNextService", bluetooth_find_next_service),
        (
            24,
            "BluetoothFindProfileDescriptorClose",
            bluetooth_find_profile_descriptor_close,
        ),
        (
            25,
            "BluetoothFindProtocolDescriptorStackClose",
            bluetooth_find_protocol_descriptor_stack_close,
        ),
        (
            26,
            "BluetoothFindProtocolEntryClose",
            bluetooth_find_protocol_entry_close,
        ),
        (
            27,
            "BluetoothFindServiceClose",
            bluetooth_find_service_close,
        ),
        (
            28,
            "BluetoothIsVersionAvailable",
            bluetooth_is_version_available,
        ),
        (
            29,
            "BluetoothMapClassOfDeviceToImageIndex",
            bluetooth_map_class_of_device_to_image_index,
        ),
        (
            30,
            "BluetoothMapClassOfDeviceToString",
            bluetooth_map_class_of_device_to_string,
        ),
        (
            31,
            "BluetoothRegisterForAuthentication",
            bluetooth_register_for_authentication,
        ),
        (32, "BluetoothSdpGetString", bluetooth_sdp_get_string),
        (33, "BluetoothSelectDevices", bluetooth_select_devices),
        (
            34,
            "BluetoothSelectDevicesFree",
            bluetooth_select_devices_free,
        ),
        (
            35,
            "BluetoothSendAuthenticationResponse",
            bluetooth_send_authentication_response,
        ),
        (
            36,
            "BluetoothSetLocalServiceInfo",
            bluetooth_set_local_service_info,
        ),
        (37, "BluetoothSetServiceState", bluetooth_set_service_state),
        (
            38,
            "BluetoothUpdateDeviceRecord",
            bluetooth_update_device_record,
        ),
        (39, "BthpEnableAllServices", bthp_enable_all_services),
        (40, "BthpFindPnpInfo", bthp_find_pnp_info),
        (41, "BthpMapStatusToErr", bthp_map_status_to_err),
        (42, "DllGetClassObject", dll_get_class_object),
    ];
    for (row, function, helper) in rows {
        assert_eq!(
            helper(),
            StubException {
                code: 0x8000_0100,
                flags: 0x1,
                number_parameters: 2,
                module: "bthprops.cpl",
                function,
            },
            "row {row:02}"
        );
    }
}
