use crate::class_factory::ClassFactory;
use crate::com::{is_equal_guid, Guid, CLSID_UIRIBBONFRAMEWORK, IID_ICLASSFACTORY, IID_IUNKNOWN};
use crate::{Hresult, CLASS_E_CLASSNOTAVAILABLE, E_NOINTERFACE, S_FALSE, S_OK};

/// `DllGetClassObject`. The IID is checked first, then the CLSID is looked up in
/// `object_creation`. On success the C code allocates a class factory with one reference. On
/// failure it leaves `ppv` untouched, which the port reports as `None`.
pub fn dll_get_class_object(clsid: &Guid, riid: &Guid) -> (Hresult, Option<ClassFactory>) {
    if !is_equal_guid(riid, &IID_ICLASSFACTORY) && !is_equal_guid(riid, &IID_IUNKNOWN) {
        return (E_NOINTERFACE, None);
    }
    if !is_equal_guid(clsid, &CLSID_UIRIBBONFRAMEWORK) {
        return (CLASS_E_CLASSNOTAVAILABLE, None);
    }
    (S_OK, Some(ClassFactory::new()))
}

/// `DllCanUnloadNow`. `uiribbon/main.c` has no body for it, so winecrt0's default applies, which
/// always returns `S_FALSE`.
pub fn dll_can_unload_now() -> Hresult {
    S_FALSE
}
