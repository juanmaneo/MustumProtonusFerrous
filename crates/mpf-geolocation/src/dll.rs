//! Port of the exports in `main.c`, and of the `DllCanUnloadNow` default from `libs/winecrt0`.

use crate::factory::{factory_query_interface, Factory};
use crate::heap::Iface;
use crate::hresult::{Hresult, CLASS_E_CLASSNOTAVAILABLE, S_FALSE};
use crate::iid::Iid;

/// The only runtime class this module serves.
const GEOLOCATOR_CLASS: &str = "Windows.Devices.Geolocation.Geolocator";

/// `DllCanUnloadNow`, the default from `libs/winecrt0/dll_canunload.c`.
pub fn dll_can_unload_now() -> Hresult {
    S_FALSE
}

/// `DllGetClassObject`: no class objects are served. The clsid, riid, and out parameters are unused.
pub fn dll_get_class_object() -> Hresult {
    CLASS_E_CLASSNOTAVAILABLE
}

/// `DllGetActivationFactory`. `class_id` is the HSTRING's raw buffer, and `None` (a NULL HSTRING)
/// reads as empty. The comparison stops at the first NUL, as `wcscmp` does.
pub fn dll_get_activation_factory(
    factory: &mut Factory,
    class_id: Option<&str>,
) -> (Hresult, Option<Iface>) {
    let name = class_id
        .unwrap_or_default()
        .split('\0')
        .next()
        .unwrap_or_default();
    if name != GEOLOCATOR_CLASS {
        return (CLASS_E_CLASSNOTAVAILABLE, None);
    }
    // The C code checks `*factory` after the QueryInterface. That QueryInterface cannot fail for
    // IActivationFactory, so its result is the export's result.
    factory_query_interface(factory, Iid::IActivationFactory)
}
