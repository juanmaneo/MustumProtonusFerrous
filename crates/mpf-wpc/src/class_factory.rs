use crate::guid::{is_equal_guid, Guid, CLSID_WINDOWS_PARENTAL_CONTROLS};
use crate::hresult::{query_hresult, Hresult, CLASS_E_CLASSNOTAVAILABLE, S_OK};
use crate::interface::{interface_for_iid, Interface};
use crate::object::{self, Object};

/// `ClassFactory_QueryInterface`: `ppv` becomes the factory on success and `None` otherwise.
pub fn query_interface(riid: &Guid, ppv: &mut Option<Object>) -> Hresult {
    let matched = matches!(
        interface_for_iid(riid),
        Some(Interface::Unknown | Interface::ClassFactory)
    );
    *ppv = matched.then_some(Object::ClassFactory);
    query_hresult(matched)
}

/// `ClassFactory_AddRef`: the count is always 2.
pub fn add_ref() -> u32 {
    2
}

/// `ClassFactory_Release`: the count is always 1.
pub fn release() -> u32 {
    1
}

/// `ClassFactory_LockServer`: `fLock` is only traced, so the call always succeeds.
pub fn lock_server() -> Hresult {
    S_OK
}

/// `WindowsParentalControls_CreateInstance`. The `outer` argument is ignored by the C code, and
/// the static object answers the query.
pub fn create_instance(riid: &Guid, ppv: &mut Option<Object>) -> Hresult {
    object::query_interface(riid, ppv)
}

/// `DllGetClassObject`. `ppv` is left untouched when the CLSID is not the one this DLL serves.
pub fn dll_get_class_object(rclsid: &Guid, riid: &Guid, ppv: &mut Option<Object>) -> Hresult {
    if is_equal_guid(&CLSID_WINDOWS_PARENTAL_CONTROLS, rclsid) {
        query_interface(riid, ppv)
    } else {
        CLASS_E_CLASSNOTAVAILABLE
    }
}
