//! Port of the `IClassFactory` methods in Wine's `dlls/msdaps/main.c` that return fixed values
//! or decide on an interface from its IID.

use crate::com::{
    is_equal_guid, Guid, E_NOINTERFACE, HRESULT, IID_ICLASSFACTORY, IID_IUNKNOWN, S_OK,
};

/// `CF_QueryInterface` without the interface pointer. `S_OK` means the C code calls
/// `IClassFactory_AddRef` and stores the interface in `*obj`. `E_NOINTERFACE` means `*obj` is
/// left alone.
pub fn class_factory_query_interface(riid: &Guid) -> HRESULT {
    if is_equal_guid(riid, &IID_IUNKNOWN) || is_equal_guid(riid, &IID_ICLASSFACTORY) {
        S_OK
    } else {
        E_NOINTERFACE
    }
}

/// `CF_AddRef` always returns 2.
pub fn class_factory_add_ref() -> u32 {
    2
}

/// `CF_Release` always returns 1.
pub fn class_factory_release() -> u32 {
    1
}

/// `CF_LockServer` ignores `dolock` and always returns `S_OK`.
pub fn class_factory_lock_server(_dolock: bool) -> HRESULT {
    S_OK
}
