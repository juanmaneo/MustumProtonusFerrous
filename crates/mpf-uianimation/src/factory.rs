use crate::class::{class_iid, ClassKind};
use crate::com::{query_interface, Guid, Out, HRESULT, IID_ICLASSFACTORY, S_OK};

/// `class_factory_AddRef`: the factories are static, so the count is always 2.
pub fn class_factory_add_ref() -> u32 {
    2
}

/// `class_factory_Release`: the factories are static, so the count is always 1.
pub fn class_factory_release() -> u32 {
    1
}

/// `class_factory_LockServer`: accepts the lock and does nothing.
pub fn class_factory_lock_server(_lock: bool) -> HRESULT {
    S_OK
}

/// `class_factory_QueryInterface` of the factory for `factory`.
pub fn class_factory_query_interface(factory: ClassKind, iid: Guid) -> (HRESULT, Out<ClassKind>) {
    query_interface(factory, iid, IID_ICLASSFACTORY)
}

/// `class_factory_CreateInstance`. The C code clears `*obj` and then runs the creator, which
/// queries the new object for `iid`, so the result is `Null` or `Set(factory)`, never `Untouched`.
/// The `outer` argument is ignored, as in the C code.
pub fn class_factory_create_instance(factory: ClassKind, iid: Guid) -> (HRESULT, Out<ClassKind>) {
    query_interface(factory, iid, class_iid(factory))
}
