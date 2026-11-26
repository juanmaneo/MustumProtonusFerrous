//! The `DllGetClassObject` lookup from Wine's `dlls/qdvd/qdvd_main.c`, and the `winecrt0`
//! forwarders that `qdvd.spec` exports as `DllCanUnloadNow`, `DllRegisterServer` and
//! `DllUnregisterServer`.

use crate::class_factory::{class_factory_query_interface, ClassFactory};
use crate::guid::{is_equal_guid, Guid, CLSID_DVDGRAPHBUILDER, CLSID_DVDNAVIGATOR};
use crate::hresult::{succeeded, Hresult, CLASS_E_CLASSNOTAVAILABLE, S_FALSE};

/// `DllCanUnloadNow`, which always returns `S_FALSE`, so the DLL is never unloaded.
pub fn dll_can_unload_now() -> Hresult {
    S_FALSE
}

/// `DllGetClassObject`. An unknown CLSID fails before the IID is checked. A known CLSID returns
/// its factory when the factory answers the IID.
pub fn dll_get_class_object(clsid: &Guid, iid: &Guid) -> Result<ClassFactory, Hresult> {
    let factory = if is_equal_guid(clsid, &CLSID_DVDGRAPHBUILDER) {
        ClassFactory::GraphBuilder
    } else if is_equal_guid(clsid, &CLSID_DVDNAVIGATOR) {
        ClassFactory::Navigator
    } else {
        return Err(CLASS_E_CLASSNOTAVAILABLE);
    };
    let hr = class_factory_query_interface(iid);
    if succeeded(hr) {
        Ok(factory)
    } else {
        Err(hr)
    }
}

/// `DllRegisterServer`, which returns what `__wine_register_resources` returns. The caller
/// supplies that function.
pub fn dll_register_server(register_resources: impl FnOnce() -> Hresult) -> Hresult {
    register_resources()
}

/// `DllUnregisterServer`, which returns what `__wine_unregister_resources` returns. The caller
/// supplies that function.
pub fn dll_unregister_server(unregister_resources: impl FnOnce() -> Hresult) -> Hresult {
    unregister_resources()
}
