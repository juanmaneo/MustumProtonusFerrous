use crate::com::{is_equal_guid, Guid, Interface, IID_ICLASSFACTORY, IID_IUNKNOWN};
use crate::realtime_stylus::{realtime_stylus_create, RealTimeStylus};
use crate::{Hresult, E_NOINTERFACE, S_OK};

/// `class_factory_QueryInterface`. Only `IUnknown` and `IClassFactory` are answered.
pub fn class_factory_query_interface(iid: &Guid, obj: &mut Option<Interface>) -> Hresult {
    if is_equal_guid(iid, &IID_IUNKNOWN) || is_equal_guid(iid, &IID_ICLASSFACTORY) {
        *obj = Some(Interface::ClassFactory);
        return S_OK;
    }
    *obj = None;
    E_NOINTERFACE
}

/// `class_factory_AddRef`. The factory is static, so the count is always 2.
pub fn class_factory_add_ref() -> u32 {
    2
}

/// `class_factory_Release`. The factory is static, so the count is always 1.
pub fn class_factory_release() -> u32 {
    1
}

/// `class_factory_CreateInstance`. The C code sets `obj` to NULL and then calls
/// `realtime_stylus_create`, which ignores `outer`.
pub fn class_factory_create_instance(iid: &Guid) -> (Hresult, Option<RealTimeStylus>) {
    realtime_stylus_create(iid)
}

/// `class_factory_LockServer`. It is a `FIXME` stub that returns `S_OK` and has no effect.
pub fn class_factory_lock_server(_lock: bool) -> Hresult {
    S_OK
}
