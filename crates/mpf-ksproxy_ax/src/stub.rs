/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = ksproxy.ax` in `dlls/ksproxy.ax/Makefile.in`.
const KSPROXY_DLL_NAME: &str = "ksproxy.ax";

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

/// `DllGetClassObject`, exported as `@ stub` in `dlls/ksproxy.ax/ksproxy.ax.spec`.
pub fn dll_get_class_object() -> StubException {
    wine_spec_unimplemented_stub(KSPROXY_DLL_NAME, "DllGetClassObject")
}

/// `KsGetMediaType`, exported as `@ stub` in `dlls/ksproxy.ax/ksproxy.ax.spec`.
pub fn ks_get_media_type() -> StubException {
    wine_spec_unimplemented_stub(KSPROXY_DLL_NAME, "KsGetMediaType")
}

/// `KsGetMediaTypeCount`, exported as `@ stub` in `dlls/ksproxy.ax/ksproxy.ax.spec`.
pub fn ks_get_media_type_count() -> StubException {
    wine_spec_unimplemented_stub(KSPROXY_DLL_NAME, "KsGetMediaTypeCount")
}

/// `KsGetMultiplePinFactoryItems`, exported as `@ stub` in `dlls/ksproxy.ax/ksproxy.ax.spec`.
pub fn ks_get_multiple_pin_factory_items() -> StubException {
    wine_spec_unimplemented_stub(KSPROXY_DLL_NAME, "KsGetMultiplePinFactoryItems")
}

/// `KsOpenDefaultDevice`, exported as `@ stub` in `dlls/ksproxy.ax/ksproxy.ax.spec`.
pub fn ks_open_default_device() -> StubException {
    wine_spec_unimplemented_stub(KSPROXY_DLL_NAME, "KsOpenDefaultDevice")
}

/// `KsResolveRequiredAttributes`, exported as `@ stub` in `dlls/ksproxy.ax/ksproxy.ax.spec`.
pub fn ks_resolve_required_attributes() -> StubException {
    wine_spec_unimplemented_stub(KSPROXY_DLL_NAME, "KsResolveRequiredAttributes")
}

/// `KsSynchronousDeviceControl`, exported as `@ stub` in `dlls/ksproxy.ax/ksproxy.ax.spec`.
pub fn ks_synchronous_device_control() -> StubException {
    wine_spec_unimplemented_stub(KSPROXY_DLL_NAME, "KsSynchronousDeviceControl")
}
