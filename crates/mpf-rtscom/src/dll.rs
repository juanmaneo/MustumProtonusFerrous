use crate::class_factory::class_factory_query_interface;
use crate::com::{is_equal_guid, Guid, Interface, CLSID_REALTIMESTYLUS};
use crate::{Hresult, CLASS_E_CLASSNOTAVAILABLE, S_FALSE};

/// `DllGetClassObject`. Only `CLSID_RealTimeStylus` has a class factory. For any other CLSID,
/// `obj` is left untouched.
pub fn dll_get_class_object(clsid: &Guid, iid: &Guid, obj: &mut Option<Interface>) -> Hresult {
    if !is_equal_guid(clsid, &CLSID_REALTIMESTYLUS) {
        return CLASS_E_CLASSNOTAVAILABLE;
    }
    class_factory_query_interface(iid, obj)
}

/// `DllCanUnloadNow`. `rtscom_main.c` has no body for it, so winecrt0's default applies, which
/// always returns `S_FALSE`.
pub fn dll_can_unload_now() -> Hresult {
    S_FALSE
}
