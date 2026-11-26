//! The class factories for the two DVD classes, from Wine's `dlls/qdvd/qdvd_main.c`.

use crate::guid::{is_equal_guid, Guid, IID_ICLASSFACTORY, IID_IUNKNOWN};
use crate::hresult::{Hresult, E_NOINTERFACE, S_OK};

/// The class a factory creates: `graph_builder_cf` or `navigator_cf` in the C code.
#[derive(PartialEq, Eq)]
pub enum ClassFactory {
    GraphBuilder,
    Navigator,
}

/// `class_factory_QueryInterface`. The factory answers `IUnknown` and `IClassFactory` only. The
/// C code's `*out` write is represented by the returned `HRESULT`.
pub fn class_factory_query_interface(iid: &Guid) -> Hresult {
    if is_equal_guid(iid, &IID_IUNKNOWN) || is_equal_guid(iid, &IID_ICLASSFACTORY) {
        S_OK
    } else {
        E_NOINTERFACE
    }
}

/// `class_factory_AddRef`, which always returns 2.
pub fn class_factory_add_ref() -> u32 {
    2
}

/// `class_factory_Release`, which always returns 1.
pub fn class_factory_release() -> u32 {
    1
}

/// `class_factory_LockServer`. The C code only logs the flag, so both values return `S_OK`.
pub fn class_factory_lock_server(_lock: bool) -> Hresult {
    S_OK
}

/// The check at the top of `class_factory_CreateInstance`. `Some` is the `HRESULT` the C code
/// returns before it creates an object: an aggregate request for any interface but `IUnknown`.
pub fn class_factory_create_instance_gate(has_outer: bool, iid: &Guid) -> Option<Hresult> {
    if has_outer && !is_equal_guid(iid, &IID_IUNKNOWN) {
        Some(E_NOINTERFACE)
    } else {
        None
    }
}
