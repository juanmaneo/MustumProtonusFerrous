//! The class factory and `DllGetClassObject` from Wine's `dlls/mmcndmgr/mmcndmgr.c`
//! (`ClassFactory_*`).

use crate::version_info::mmcversion_query_interface;
use crate::{
    is_equal_guid, Guid, Hresult, Object, CLASS_E_CLASSNOTAVAILABLE, CLSID_MMCVERSIONINFO,
    E_NOINTERFACE, IID_ICLASSFACTORY, IID_IUNKNOWN, S_OK,
};

/// `ClassFactory_QueryInterface`. Accepts `IID_IClassFactory` and `IID_IUnknown`. For any other
/// IID the C returns `E_NOINTERFACE` and sets `*ppvObject = NULL`, which is the `Err` case.
pub fn class_factory_query_interface(riid: &Guid) -> Result<Object, Hresult> {
    if is_equal_guid(&IID_ICLASSFACTORY, riid) || is_equal_guid(&IID_IUNKNOWN, riid) {
        Ok(Object::ClassFactory)
    } else {
        Err(E_NOINTERFACE)
    }
}

/// `ClassFactory_AddRef`. The factory is static, so the count is always 2.
pub fn class_factory_add_ref() -> u32 {
    2
}

/// `ClassFactory_Release`. The factory is static, so the count is always 1.
pub fn class_factory_release() -> u32 {
    1
}

/// `ClassFactory_CreateInstance`. The C ignores `outer` and asks the `IMMCVersionInfo` object
/// for `riid`, so the result is that object's `QueryInterface` result.
pub fn class_factory_create_instance(riid: &Guid) -> Result<Object, Hresult> {
    mmcversion_query_interface(riid)
}

/// `ClassFactory_LockServer`. Always `S_OK`; the C keeps no lock count.
pub fn class_factory_lock_server() -> Hresult {
    S_OK
}

/// `DllGetClassObject`. Only `CLSID_MMCVersionInfo` is served, through the class factory's
/// `QueryInterface` on `iid`. Any other class id gives `CLASS_E_CLASSNOTAVAILABLE`, and the C
/// leaves `*ppv` unchanged, so the caller keeps its own value.
pub fn dll_get_class_object(rclsid: &Guid, iid: &Guid) -> Result<Object, Hresult> {
    if is_equal_guid(rclsid, &CLSID_MMCVERSIONINFO) {
        class_factory_query_interface(iid)
    } else {
        Err(CLASS_E_CLASSNOTAVAILABLE)
    }
}
