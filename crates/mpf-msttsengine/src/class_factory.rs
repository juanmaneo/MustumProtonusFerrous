//! The `IClassFactory` vtable of the static `ttsengine_cf` in `dlls/msttsengine/main.c`.

use crate::com::{
    is_equal_guid, ComPtr, Guid, Hresult, CLASS_E_NOAGGREGATION, E_NOINTERFACE, IID_CLASS_FACTORY,
    IID_UNKNOWN, S_OK,
};
use crate::tts_engine::{ttsengine_create, Interface, TtsEngine};

/// The static class factory. The C object holds only its vtable and the `create_instance` pointer, which is
/// always `ttsengine_create`, so the port has no state and calls `ttsengine_create` directly.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClassFactory;

/// `class_factory_QueryInterface`. `Ok` is `S_OK` with `*out` set to the factory. `Err` is `E_NOINTERFACE`
/// with `*out` set to NULL. The factory answers `IUnknown` and `IClassFactory` only.
pub fn class_factory_query_interface(riid: &Guid) -> Result<ClassFactory, Hresult> {
    if is_equal_guid(riid, &IID_UNKNOWN) || is_equal_guid(riid, &IID_CLASS_FACTORY) {
        Ok(ClassFactory)
    } else {
        Err(E_NOINTERFACE)
    }
}

/// `class_factory_AddRef`: always 2, because the factory is static.
pub fn class_factory_add_ref() -> u32 {
    2
}

/// `class_factory_Release`: always 1, because the factory is static.
pub fn class_factory_release() -> u32 {
    1
}

/// `class_factory_LockServer`: succeeds for either value of `lock` and changes nothing.
pub fn class_factory_lock_server(_lock: bool) -> Hresult {
    S_OK
}

/// `class_factory_CreateInstance`. `has_outer` is true when the caller passed a non-NULL `outer`, and the
/// factory rejects that with `CLASS_E_NOAGGREGATION` before it allocates. Otherwise it calls
/// `ttsengine_create` with `malloc_ok`, the result of the allocation.
pub fn class_factory_create_instance<T: ComPtr>(
    has_outer: bool,
    riid: &Guid,
    malloc_ok: bool,
) -> Result<(TtsEngine<T>, Interface), Hresult> {
    if has_outer {
        return Err(CLASS_E_NOAGGREGATION);
    }
    ttsengine_create(riid, malloc_ok)
}
