//! `DllGetClassObject` and `DllGetActivationFactory` from Wine's `dlls/capabilityaccessmanagerclient/main.c`.

use crate::com::IID_IACTIVATIONFACTORY;
use crate::statics::CapabilityStatics;
use crate::{Hresult, Interface, CLASS_E_CLASSNOTAVAILABLE};

const RUNTIME_CLASS_APP_CAPABILITY: &str =
    "Windows.Security.Authorization.AppCapabilityAccess.AppCapability";

/// `DllGetClassObject`: no class objects are provided.
pub fn dll_get_class_object() -> Hresult {
    CLASS_E_CLASSNOTAVAILABLE
}

/// `DllGetActivationFactory`. `class_id` is the HSTRING's raw buffer, empty for NULL.
/// `statics` is the DLL's single factory object, so a successful lookup adds a reference to it.
pub fn dll_get_activation_factory(
    statics: &mut CapabilityStatics,
    class_id: &[u16],
) -> Result<Interface, Hresult> {
    if !wcscmp_eq(class_id, RUNTIME_CLASS_APP_CAPABILITY) {
        return Err(CLASS_E_CLASSNOTAVAILABLE);
    }
    statics
        .query_interface(&IID_IACTIVATIONFACTORY)
        .ok()
        .ok_or(CLASS_E_CLASSNOTAVAILABLE)
}

/// `wcscmp(buffer, expected) == 0`. The buffer ends at its first NUL.
fn wcscmp_eq(buffer: &[u16], expected: &str) -> bool {
    let end = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
    buffer[..end].iter().copied().eq(expected.encode_utf16())
}
