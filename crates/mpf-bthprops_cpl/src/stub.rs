/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = bthprops.cpl` in `dlls/bthprops.cpl/Makefile.in`.
pub const BTHPROPS_DLL_NAME: &str = "bthprops.cpl";

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

/// Defines one helper per `@ stub` export, and the table of them in spec order.
macro_rules! stub_exports {
    ($($helper:ident => $export:literal,)*) => {
        $(
            #[doc = concat!("`", $export, "`, exported as `@ stub` in `dlls/bthprops.cpl/bthprops.cpl.spec`.")]
            pub fn $helper() -> StubException {
                wine_spec_unimplemented_stub(BTHPROPS_DLL_NAME, $export)
            }
        )*

        /// The `@ stub` exports of `bthprops.cpl.spec`, in spec order, with their helpers.
        pub const STUB_EXPORTS: &[(&str, fn() -> StubException)] = &[$(($export, $helper)),*];
    };
}

stub_exports! {
    bluetooth_address_to_string => "BluetoothAddressToString",
    bluetooth_authenticate_device => "BluetoothAuthenticateDevice",
    bluetooth_authenticate_multiple_devices => "BluetoothAuthenticateMultipleDevices",
    bluetooth_authentication_agent => "BluetoothAuthenticationAgent",
    bluetooth_disconnect_device => "BluetoothDisconnectDevice",
    bluetooth_display_device_properties => "BluetoothDisplayDeviceProperties",
    bluetooth_enumerate_installed_services => "BluetoothEnumerateInstalledServices",
    bluetooth_enumerate_installed_services_ex => "BluetoothEnumerateInstalledServicesEx",
    bluetooth_find_browse_group_close => "BluetoothFindBrowseGroupClose",
    bluetooth_find_class_id_close => "BluetoothFindClassIdClose",
    bluetooth_find_first_browse_group => "BluetoothFindFirstBrowseGroup",
    bluetooth_find_first_class_id => "BluetoothFindFirstClassId",
    bluetooth_find_first_profile_descriptor => "BluetoothFindFirstProfileDescriptor",
    bluetooth_find_first_protocol_descriptor_stack => "BluetoothFindFirstProtocolDescriptorStack",
    bluetooth_find_first_protocol_entry => "BluetoothFindFirstProtocolEntry",
    bluetooth_find_first_service => "BluetoothFindFirstService",
    bluetooth_find_first_service_ex => "BluetoothFindFirstServiceEx",
    bluetooth_find_next_browse_group => "BluetoothFindNextBrowseGroup",
    bluetooth_find_next_class_id => "BluetoothFindNextClassId",
    bluetooth_find_next_profile_descriptor => "BluetoothFindNextProfileDescriptor",
    bluetooth_find_next_protocol_descriptor_stack => "BluetoothFindNextProtocolDescriptorStack",
    bluetooth_find_next_protocol_entry => "BluetoothFindNextProtocolEntry",
    bluetooth_find_next_service => "BluetoothFindNextService",
    bluetooth_find_profile_descriptor_close => "BluetoothFindProfileDescriptorClose",
    bluetooth_find_protocol_descriptor_stack_close => "BluetoothFindProtocolDescriptorStackClose",
    bluetooth_find_protocol_entry_close => "BluetoothFindProtocolEntryClose",
    bluetooth_find_service_close => "BluetoothFindServiceClose",
    bluetooth_is_version_available => "BluetoothIsVersionAvailable",
    bluetooth_map_class_of_device_to_image_index => "BluetoothMapClassOfDeviceToImageIndex",
    bluetooth_map_class_of_device_to_string => "BluetoothMapClassOfDeviceToString",
    bluetooth_register_for_authentication => "BluetoothRegisterForAuthentication",
    bluetooth_sdp_get_string => "BluetoothSdpGetString",
    bluetooth_select_devices => "BluetoothSelectDevices",
    bluetooth_select_devices_free => "BluetoothSelectDevicesFree",
    bluetooth_send_authentication_response => "BluetoothSendAuthenticationResponse",
    bluetooth_set_local_service_info => "BluetoothSetLocalServiceInfo",
    bluetooth_set_service_state => "BluetoothSetServiceState",
    bluetooth_update_device_record => "BluetoothUpdateDeviceRecord",
    bthp_enable_all_services => "BthpEnableAllServices",
    bthp_find_pnp_info => "BthpFindPnpInfo",
    bthp_map_status_to_err => "BthpMapStatusToErr",
    dll_get_class_object => "DllGetClassObject",
}
