//! `ClassFactory_*` from `main.c`: the static factory that `DllGetClassObject` hands out.

use crate::hresult::{HResult, CLASS_E_NOAGGREGATION, E_NOINTERFACE, S_OK};
use crate::object::dhtml_edit_create;
use crate::qi::{Iid, Target};

/// `ClassFactory_QueryInterface`: IUnknown and IClassFactory resolve to the factory itself.
pub fn class_factory_query_interface(iid: Iid) -> HResult {
    match iid {
        Iid::IUnknown | Iid::IClassFactory => S_OK,
        _ => E_NOINTERFACE,
    }
}

/// `ClassFactory_AddRef`. The factory is static, so its count is a constant.
pub fn class_factory_add_ref() -> u32 {
    2
}

/// `ClassFactory_Release`. Constant for the same reason as [`class_factory_add_ref`].
pub fn class_factory_release() -> u32 {
    1
}

/// `ClassFactory_CreateInstance`: aggregation is refused, otherwise `dhtml_edit_create`.
pub fn class_factory_create_instance(has_outer: bool, iid: Iid) -> Result<(Target, i32), HResult> {
    if has_outer {
        return Err(CLASS_E_NOAGGREGATION);
    }
    dhtml_edit_create(iid)
}

/// `ClassFactory_LockServer`. Always succeeds; no server lock is kept.
pub fn class_factory_lock_server(_lock: bool) -> HResult {
    S_OK
}
