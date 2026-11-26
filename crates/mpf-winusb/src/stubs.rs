/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = winusb.dll` in `dlls/winusb/Makefile.in`.
pub const WINUSB_DLL_NAME: &str = "winusb.dll";

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
            #[doc = concat!("`", $export, "`, exported as `@ stub` in `dlls/winusb/winusb.spec`.")]
            pub fn $helper() -> StubException {
                wine_spec_unimplemented_stub(WINUSB_DLL_NAME, $export)
            }
        )*

        /// The `@ stub` exports of `winusb.spec`, in spec order, with their helpers.
        pub const STUB_EXPORTS: &[(&str, fn() -> StubException)] = &[$(($export, $helper)),*];
    };
}

stub_exports! {
    winusb_abort_pipe => "WinUsb_AbortPipe",
    winusb_control_transfer => "WinUsb_ControlTransfer",
    winusb_flush_pipe => "WinUsb_FlushPipe",
    winusb_get_associated_interface => "WinUsb_GetAssociatedInterface",
    winusb_get_current_alternate_setting => "WinUsb_GetCurrentAlternateSetting",
    winusb_get_descriptor => "WinUsb_GetDescriptor",
    winusb_get_overlapped_result => "WinUsb_GetOverlappedResult",
    winusb_get_pipe_policy => "WinUsb_GetPipePolicy",
    winusb_get_power_policy => "WinUsb_GetPowerPolicy",
    winusb_initialize => "WinUsb_Initialize",
    winusb_parse_configuration_descriptor => "WinUsb_ParseConfigurationDescriptor",
    winusb_parse_descriptors => "WinUsb_ParseDescriptors",
    winusb_query_device_information => "WinUsb_QueryDeviceInformation",
    winusb_query_interface_settings => "WinUsb_QueryInterfaceSettings",
    winusb_query_pipe => "WinUsb_QueryPipe",
    winusb_read_pipe => "WinUsb_ReadPipe",
    winusb_reset_pipe => "WinUsb_ResetPipe",
    winusb_set_current_alternate_setting => "WinUsb_SetCurrentAlternateSetting",
    winusb_set_pipe_policy => "WinUsb_SetPipePolicy",
    winusb_set_power_policy => "WinUsb_SetPowerPolicy",
    winusb_write_pipe => "WinUsb_WritePipe",
}
