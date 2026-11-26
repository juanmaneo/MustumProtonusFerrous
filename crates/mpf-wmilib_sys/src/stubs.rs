use crate::stub::{wine_spec_unimplemented_stub, StubException, MODULE_NAME};

/// `DllInitialize`, exported as `@ stub` in `dlls/wmilib.sys/wmilib.sys.spec`.
pub fn dll_initialize() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "DllInitialize")
}

/// `DllUnload`, exported as `@ stub` in `dlls/wmilib.sys/wmilib.sys.spec`.
pub fn dll_unload() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "DllUnload")
}

/// `WmiCompleteRequest`, exported as `@ stub` in `dlls/wmilib.sys/wmilib.sys.spec`.
pub fn wmi_complete_request() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "WmiCompleteRequest")
}

/// `WmiFireEvent`, exported as `@ stub` in `dlls/wmilib.sys/wmilib.sys.spec`.
pub fn wmi_fire_event() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "WmiFireEvent")
}

/// `WmiSystemControl`, exported as `@ stub` in `dlls/wmilib.sys/wmilib.sys.spec`.
pub fn wmi_system_control() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "WmiSystemControl")
}
