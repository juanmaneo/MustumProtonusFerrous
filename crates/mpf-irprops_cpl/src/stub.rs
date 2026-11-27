/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE` in `dlls/irprops.cpl/Makefile.in`.
const IRPROPS_DLL_NAME: &str = "irprops.cpl";

/// The record that `__wine_spec_unimplemented_stub` raises.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`. The C function raises
/// the record in a loop and never returns, so the port returns the record instead.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        // The module and the function name.
        number_parameters: 2,
        module,
        function,
    }
}

/// `@ stub` export `BluetoothAuthenticateDevice` of `irprops.cpl.spec`.
pub fn bluetooth_authenticate_device() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothAuthenticateDevice")
}

/// `@ stub` export `BluetoothAuthenticateMultipleDevices` of `irprops.cpl.spec`.
pub fn bluetooth_authenticate_multiple_devices() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothAuthenticateMultipleDevices")
}

/// `@ stub` export `BluetoothAuthenticationAgent` of `irprops.cpl.spec`.
pub fn bluetooth_authentication_agent() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothAuthenticationAgent")
}

/// `@ stub` export `BluetoothDisconnectDevice` of `irprops.cpl.spec`.
pub fn bluetooth_disconnect_device() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothDisconnectDevice")
}

/// `@ stub` export `BluetoothDisplayDeviceProperties` of `irprops.cpl.spec`.
pub fn bluetooth_display_device_properties() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothDisplayDeviceProperties")
}

/// `@ stub` export `BluetoothEnumerateInstalledServices` of `irprops.cpl.spec`.
pub fn bluetooth_enumerate_installed_services() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothEnumerateInstalledServices")
}

/// `@ stub` export `BluetoothFindBrowseGroupClose` of `irprops.cpl.spec`.
pub fn bluetooth_find_browse_group_close() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothFindBrowseGroupClose")
}

/// `@ stub` export `BluetoothFindClassIdClose` of `irprops.cpl.spec`.
pub fn bluetooth_find_class_id_close() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothFindClassIdClose")
}

/// `@ stub` export `BluetoothFindFirstBrowseGroup` of `irprops.cpl.spec`.
pub fn bluetooth_find_first_browse_group() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothFindFirstBrowseGroup")
}

/// `@ stub` export `BluetoothFindFirstClassId` of `irprops.cpl.spec`.
pub fn bluetooth_find_first_class_id() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothFindFirstClassId")
}

/// `@ stub` export `BluetoothFindFirstProfileDescriptor` of `irprops.cpl.spec`.
pub fn bluetooth_find_first_profile_descriptor() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothFindFirstProfileDescriptor")
}

/// `@ stub` export `BluetoothFindFirstProtocolDescriptorStack` of `irprops.cpl.spec`.
pub fn bluetooth_find_first_protocol_descriptor_stack() -> StubException {
    wine_spec_unimplemented_stub(
        IRPROPS_DLL_NAME,
        "BluetoothFindFirstProtocolDescriptorStack",
    )
}

/// `@ stub` export `BluetoothFindFirstProtocolEntry` of `irprops.cpl.spec`.
pub fn bluetooth_find_first_protocol_entry() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothFindFirstProtocolEntry")
}

/// `@ stub` export `BluetoothFindFirstService` of `irprops.cpl.spec`.
pub fn bluetooth_find_first_service() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothFindFirstService")
}

/// `@ stub` export `BluetoothFindNextBrowseGroup` of `irprops.cpl.spec`.
pub fn bluetooth_find_next_browse_group() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothFindNextBrowseGroup")
}

/// `@ stub` export `BluetoothFindNextClassId` of `irprops.cpl.spec`.
pub fn bluetooth_find_next_class_id() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothFindNextClassId")
}

/// `@ stub` export `BluetoothFindNextProfileDescriptor` of `irprops.cpl.spec`.
pub fn bluetooth_find_next_profile_descriptor() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothFindNextProfileDescriptor")
}

/// `@ stub` export `BluetoothFindNextProtocolDescriptorStack` of `irprops.cpl.spec`.
pub fn bluetooth_find_next_protocol_descriptor_stack() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothFindNextProtocolDescriptorStack")
}

/// `@ stub` export `BluetoothFindNextProtocolEntry` of `irprops.cpl.spec`.
pub fn bluetooth_find_next_protocol_entry() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothFindNextProtocolEntry")
}

/// `@ stub` export `BluetoothFindNextService` of `irprops.cpl.spec`.
pub fn bluetooth_find_next_service() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothFindNextService")
}

/// `@ stub` export `BluetoothFindProfileDescriptorClose` of `irprops.cpl.spec`.
pub fn bluetooth_find_profile_descriptor_close() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothFindProfileDescriptorClose")
}

/// `@ stub` export `BluetoothFindProtocolDescriptorStackClose` of `irprops.cpl.spec`.
pub fn bluetooth_find_protocol_descriptor_stack_close() -> StubException {
    wine_spec_unimplemented_stub(
        IRPROPS_DLL_NAME,
        "BluetoothFindProtocolDescriptorStackClose",
    )
}

/// `@ stub` export `BluetoothFindProtocolEntryClose` of `irprops.cpl.spec`.
pub fn bluetooth_find_protocol_entry_close() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothFindProtocolEntryClose")
}

/// `@ stub` export `BluetoothFindServiceClose` of `irprops.cpl.spec`.
pub fn bluetooth_find_service_close() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothFindServiceClose")
}

/// `@ stub` export `BluetoothMapClassOfDeviceToImageIndex` of `irprops.cpl.spec`.
pub fn bluetooth_map_class_of_device_to_image_index() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothMapClassOfDeviceToImageIndex")
}

/// `@ stub` export `BluetoothMapClassOfDeviceToString` of `irprops.cpl.spec`.
pub fn bluetooth_map_class_of_device_to_string() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothMapClassOfDeviceToString")
}

/// `@ stub` export `BluetoothRegisterForAuthentication` of `irprops.cpl.spec`.
pub fn bluetooth_register_for_authentication() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothRegisterForAuthentication")
}

/// `@ stub` export `BluetoothSdpGetString` of `irprops.cpl.spec`.
pub fn bluetooth_sdp_get_string() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothSdpGetString")
}

/// `@ stub` export `BluetoothSelectDevices` of `irprops.cpl.spec`.
pub fn bluetooth_select_devices() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothSelectDevices")
}

/// `@ stub` export `BluetoothSelectDevicesFree` of `irprops.cpl.spec`.
pub fn bluetooth_select_devices_free() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothSelectDevicesFree")
}

/// `@ stub` export `BluetoothSendAuthenticationResponse` of `irprops.cpl.spec`.
pub fn bluetooth_send_authentication_response() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothSendAuthenticationResponse")
}

/// `@ stub` export `BluetoothSetServiceState` of `irprops.cpl.spec`.
pub fn bluetooth_set_service_state() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothSetServiceState")
}

/// `@ stub` export `BluetoothUpdateDeviceRecord` of `irprops.cpl.spec`.
pub fn bluetooth_update_device_record() -> StubException {
    wine_spec_unimplemented_stub(IRPROPS_DLL_NAME, "BluetoothUpdateDeviceRecord")
}
