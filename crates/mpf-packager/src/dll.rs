use crate::class_factory;
use crate::com::{Guid, CLASS_E_CLASSNOTAVAILABLE, CLSID_PACKAGE, HRESULT};

/// `DllGetClassObject`. Only `CLSID_Package` is served, by the static class factory. Any other
/// CLSID gets `CLASS_E_CLASSNOTAVAILABLE`.
pub fn dll_get_class_object(clsid: &Guid, iid: &Guid) -> HRESULT {
    if *clsid == CLSID_PACKAGE {
        class_factory::query_interface(iid)
    } else {
        CLASS_E_CLASSNOTAVAILABLE
    }
}
