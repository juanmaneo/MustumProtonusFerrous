//! Port of the portable logic in Wine's `dlls/hvsimanagementapi/main.c`: the `DllGetClassObject`
//! and `DllGetActivationFactory` entry points.

use crate::host::{Hresult, Iid, Interface, IsolatedHostStatics};

/// `CLASS_E_CLASSNOTAVAILABLE` from `include/winerror.h`.
pub const CLASS_E_CLASSNOTAVAILABLE: Hresult = 0x8004_0111u32 as i32;

const ISOLATED_WINDOWS_ENVIRONMENT_HOST: &str =
    "Windows.Security.Isolation.IsolatedWindowsEnvironmentHost";

/// `DllGetClassObject`. The DLL has no class objects, so it always returns
/// `CLASS_E_CLASSNOTAVAILABLE` and never writes `out`.
pub fn dll_get_class_object() -> Hresult {
    CLASS_E_CLASSNOTAVAILABLE
}

/// `DllGetActivationFactory`. Only `IsolatedWindowsEnvironmentHost` can be activated. It returns
/// the statics object's `IActivationFactory` view, and that takes a reference. Like `wcscmp`, the
/// name ends at the first NUL. An empty slice models a NULL HSTRING. Any other name, or a failed
/// `QueryInterface`, gives `CLASS_E_CLASSNOTAVAILABLE`, and no reference is taken.
pub fn dll_get_activation_factory(
    host: &mut IsolatedHostStatics,
    class_id: &[u16],
) -> Result<Interface, Hresult> {
    let end = class_id
        .iter()
        .position(|&c| c == 0)
        .unwrap_or(class_id.len());
    if class_id[..end]
        .iter()
        .copied()
        .eq(ISOLATED_WINDOWS_ENVIRONMENT_HOST.encode_utf16())
    {
        return host
            .query_interface(Iid::IActivationFactory)
            .ok()
            .ok_or(CLASS_E_CLASSNOTAVAILABLE);
    }
    Err(CLASS_E_CLASSNOTAVAILABLE)
}
