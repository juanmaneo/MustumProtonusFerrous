//! Port of the `DllGetClassObject` and `DllGetActivationFactory` bodies in Wine's `dlls/appxdeploymentclient/main.c`.

use crate::factory::ActivationFactory;
use crate::hresult::{CLASS_E_CLASSNOTAVAILABLE, HRESULT, S_OK};
use crate::query::Iid;

/// `RuntimeClass_Windows_Management_Deployment_PackageManager`.
pub const PACKAGE_MANAGER_CLASS: &str = "Windows.Management.Deployment.PackageManager";

/// `DllGetClassObject`: no class objects are provided, so the result is always `CLASS_E_CLASSNOTAVAILABLE`.
pub fn dll_get_class_object() -> HRESULT {
    CLASS_E_CLASSNOTAVAILABLE
}

/// `DllGetActivationFactory`. The class id is compared with `wcscmp`, so it ends at the first NUL.
/// On a match, the static factory is queried for `IActivationFactory`, which adds a reference.
pub fn dll_get_activation_factory(
    factory: &mut ActivationFactory,
    class_id: Option<&[u16]>,
) -> HRESULT {
    let handed_out = class_id_is_package_manager(class_id)
        && factory.query_interface(Iid::ActivationFactory).is_ok();
    if handed_out {
        S_OK
    } else {
        CLASS_E_CLASSNOTAVAILABLE
    }
}

/// `WindowsGetStringRawBuffer` followed by `wcscmp`. A NULL `HSTRING` reads as the empty string.
fn class_id_is_package_manager(class_id: Option<&[u16]>) -> bool {
    class_id.is_some_and(|buffer| {
        let end = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
        buffer[..end]
            .iter()
            .copied()
            .eq(PACKAGE_MANAGER_CLASS.encode_utf16())
    })
}
