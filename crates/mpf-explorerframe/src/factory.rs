//! Module and class-object logic from `dlls/explorerframe/explorerframe_main.c`,
//! plus the aggregation check and module lock in `taskbarlist.c`.

use crate::hresult::{Hresult, CLASS_E_CLASSNOTAVAILABLE, CLASS_E_NOAGGREGATION, S_FALSE, S_OK};

/// The class IDs `DllGetClassObject` distinguishes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClassId {
    NamespaceTreeControl,
    TaskbarList,
    Other,
}

/// The class factory `DllGetClassObject` hands out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClassFactory {
    NamespaceTreeControl,
    TaskbarList,
}

/// `DllGetClassObject`'s class selection. `Err` is `CLASS_E_CLASSNOTAVAILABLE`.
/// The `IClassFactory::QueryInterface` call that follows is not modelled.
pub fn get_class_object(class: ClassId) -> Result<ClassFactory, Hresult> {
    match class {
        ClassId::NamespaceTreeControl => Ok(ClassFactory::NamespaceTreeControl),
        ClassId::TaskbarList => Ok(ClassFactory::TaskbarList),
        ClassId::Other => Err(CLASS_E_CLASSNOTAVAILABLE),
    }
}

/// `DllCanUnloadNow`: any nonzero module reference count keeps the DLL loaded.
pub fn can_unload_now(ref_count: i32) -> Hresult {
    if ref_count != 0 {
        S_FALSE
    } else {
        S_OK
    }
}

/// The checks at the top of `TaskbarList_Constructor`. An aggregated call fails
/// with `CLASS_E_NOAGGREGATION` before the module lock is taken. Otherwise
/// `EFRAME_LockModule` runs and the new lock count is returned. The
/// `QueryInterface` and `Release` that follow are not modelled.
pub fn taskbar_list_constructor(has_outer: bool, lock_count: i32) -> Result<i32, Hresult> {
    if has_outer {
        return Err(CLASS_E_NOAGGREGATION);
    }
    Ok(lock_count.wrapping_add(1))
}
