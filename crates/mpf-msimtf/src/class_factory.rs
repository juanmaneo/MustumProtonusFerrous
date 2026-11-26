//! The `ClassFactory` object of `dlls/msimtf/main.c`: its `IClassFactory` methods, and the
//! `CreateInstance` that calls `ActiveIMMApp_Constructor`.

use crate::active_imm_app::{
    active_imm_app_constructor, active_imm_app_query_interface, active_imm_app_release,
    ActiveImmApp, ActiveImmInterface,
};
use crate::guid::{is_equal_guid, Guid, IID_CLASS_FACTORY, IID_UNKNOWN};
use crate::hresult::{Hresult, E_NOINTERFACE, S_OK};

/// The static factory object. The C object's `cf` member always holds `ActiveIMMApp_Constructor`,
/// so the port has no state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClassFactory;

/// `ClassFactory_QueryInterface`. `IID_IUnknown` and `IID_IClassFactory` return the factory. Any
/// other IID gives `E_NOINTERFACE`.
pub fn class_factory_query_interface(riid: &Guid) -> Result<ClassFactory, Hresult> {
    if is_equal_guid(riid, &IID_UNKNOWN) || is_equal_guid(riid, &IID_CLASS_FACTORY) {
        Ok(ClassFactory)
    } else {
        Err(E_NOINTERFACE)
    }
}

/// `ClassFactory_AddRef`: always reports 2, because the factory is static.
pub fn class_factory_add_ref() -> u32 {
    2
}

/// `ClassFactory_Release`: always reports 1, because the factory is static.
pub fn class_factory_release() -> u32 {
    1
}

/// `ClassFactory_LockServer`: succeeds for either value of `dolock` and changes nothing.
pub fn class_factory_lock_server(_dolock: bool) -> Hresult {
    S_OK
}

/// `ClassFactory_CreateInstance`. A constructor failure is returned as is. Otherwise `riid` is
/// queried on the new object, and the constructor's reference is released, so a success returns the
/// object with a count of 1. A failed query releases the object and returns the error.
pub fn class_factory_create_instance(
    riid: &Guid,
    has_outer: bool,
    alloc_succeeds: bool,
) -> Result<(ActiveImmApp, ActiveImmInterface), Hresult> {
    let mut obj = active_imm_app_constructor(has_outer, alloc_succeeds)?;
    let interface = active_imm_app_query_interface(&mut obj, riid);
    active_imm_app_release(&mut obj);
    interface.map(|interface| (obj, interface))
}
