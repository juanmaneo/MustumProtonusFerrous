use crate::guid::{
    is_equal_guid, Guid, CLSID_WMA_DEC_MEDIA_OBJECT, IID_ICLASS_FACTORY, IID_IUNKNOWN,
};
use crate::hresult::{HResult, CLASS_E_CLASSNOTAVAILABLE, E_NOINTERFACE, S_OK};

/// `class_factory_QueryInterface`. `Ok(())` sets `*out` to the factory and returns `S_OK`.
/// `Err(hr)` sets `*out` to NULL and returns `hr`.
pub fn class_factory_query_interface(riid: &Guid) -> Result<(), HResult> {
    if is_equal_guid(riid, &IID_ICLASS_FACTORY) || is_equal_guid(riid, &IID_IUNKNOWN) {
        Ok(())
    } else {
        Err(E_NOINTERFACE)
    }
}

/// `class_factory_AddRef`. The factory is static, so the count is the constant 2.
pub fn class_factory_add_ref() -> u32 {
    2
}

/// `class_factory_Release`. The factory is static, so the count is the constant 1.
pub fn class_factory_release() -> u32 {
    1
}

/// `class_factory_LockServer`. `dolock` is ignored and the result is always `S_OK`.
pub fn class_factory_lock_server(_dolock: bool) -> HResult {
    S_OK
}

/// `DllGetClassObject`. Only `CLSID_WMADecMediaObject` is served, through
/// [`class_factory_query_interface`]. Any other clsid sets `*out` to NULL and returns
/// `CLASS_E_CLASSNOTAVAILABLE`.
pub fn dll_get_class_object(clsid: &Guid, riid: &Guid) -> Result<(), HResult> {
    if is_equal_guid(clsid, &CLSID_WMA_DEC_MEDIA_OBJECT) {
        return class_factory_query_interface(riid);
    }
    Err(CLASS_E_CLASSNOTAVAILABLE)
}
