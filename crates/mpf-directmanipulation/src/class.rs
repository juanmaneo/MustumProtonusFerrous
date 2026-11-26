//! `DllGetClassObject` routing and the class factory's `QueryInterface`, `AddRef`, `Release` and `LockServer`.

use crate::guid::{classify_clsid, classify_iid, Clsid, Guid, Iid};
use crate::hresult::{HResult, CLASS_E_CLASSNOTAVAILABLE, E_NOINTERFACE, S_OK};
use crate::query::Iface;

/// The two class factories the DLL exports.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Factory {
    Manager,
    Compositor,
}

/// `ClassFactory_QueryInterface`: IUnknown and IClassFactory only, with no reference counting.
pub fn class_factory_query_interface(factory: Factory, iid: &Guid) -> Result<Iface, HResult> {
    match classify_iid(iid) {
        Some(Iid::Unknown | Iid::ClassFactory) => Ok(Iface::ClassFactory(factory)),
        _ => Err(E_NOINTERFACE),
    }
}

/// `ClassFactory_AddRef` always reports 2 and keeps no state.
pub const fn class_factory_add_ref() -> u32 {
    2
}

/// `ClassFactory_Release` always reports 1 and keeps no state.
pub const fn class_factory_release() -> u32 {
    1
}

/// `ClassFactory_LockServer` succeeds for both lock and unlock.
pub const fn class_factory_lock_server(_lock: bool) -> HResult {
    S_OK
}

/// `DllGetClassObject`: the manager and shared-manager CLSIDs share one factory.
pub fn dll_get_class_object(clsid: &Guid, iid: &Guid) -> Result<Iface, HResult> {
    let factory = match classify_clsid(clsid) {
        Some(Clsid::Manager | Clsid::Shared) => Factory::Manager,
        Some(Clsid::DCompCompositor) => Factory::Compositor,
        None => return Err(CLASS_E_CLASSNOTAVAILABLE),
    };
    class_factory_query_interface(factory, iid)
}
