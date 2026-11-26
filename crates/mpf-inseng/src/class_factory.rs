//! The static `InstallEngineCF` class factory and `DllGetClassObject` from `inseng_main.c`.

use crate::com::{
    is_equal_guid, Guid, HResult, CLASS_E_CLASSNOTAVAILABLE, CLSID_INSTALL_ENGINE, E_NOINTERFACE,
    E_OUTOFMEMORY, IID_CLASS_FACTORY, IID_UNKNOWN, S_OK,
};
use crate::install_engine::InstallEngine;

/// `ClassFactory_QueryInterface`: accepts `IID_IUnknown` and `IID_IClassFactory`.
pub fn class_factory_query_interface(riid: &Guid) -> Result<(), HResult> {
    if is_equal_guid(&IID_UNKNOWN, riid) || is_equal_guid(&IID_CLASS_FACTORY, riid) {
        Ok(())
    } else {
        Err(E_NOINTERFACE)
    }
}

/// `ClassFactory_AddRef`: the factory is static, so the count is always 2.
pub fn class_factory_add_ref() -> u32 {
    2
}

/// `ClassFactory_Release`: the factory is static, so the count is always 1.
pub fn class_factory_release() -> u32 {
    1
}

/// `ClassFactory_LockServer`: does nothing and succeeds.
pub fn class_factory_lock_server(_lock: bool) -> HResult {
    S_OK
}

/// `InstallEngineCF_CreateInstance`. `alloc_ok` stands in for the `malloc` result.
///
/// The new object starts with a count of 1, the query runs on it, and then one release runs.
/// On success the caller gets the object with a count of 1. On failure the object is dropped,
/// as the C code frees it when the count reaches 0.
pub fn class_factory_create_instance(
    riid: &Guid,
    alloc_ok: bool,
) -> Result<InstallEngine, HResult> {
    if !alloc_ok {
        return Err(E_OUTOFMEMORY);
    }
    let mut engine = InstallEngine::created();
    let hres = engine.query_interface(riid);
    engine.release();
    hres.map(|()| engine)
}

/// `DllGetClassObject`: only `CLSID_InstallEngine` has a class, and the IID is checked by the factory.
pub fn dll_get_class_object(rclsid: &Guid, riid: &Guid) -> Result<(), HResult> {
    if is_equal_guid(rclsid, &CLSID_INSTALL_ENGINE) {
        class_factory_query_interface(riid)
    } else {
        Err(CLASS_E_CLASSNOTAVAILABLE)
    }
}
