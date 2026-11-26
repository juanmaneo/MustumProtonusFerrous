//! `DllGetClassObject` from `dlls/msimtf/main.c`, and `DllCanUnloadNow`, which the spec exports from
//! the default in `libs/winecrt0/dll_canunload.c`.

use crate::class_factory::{class_factory_query_interface, ClassFactory};
use crate::guid::{is_equal_guid, Guid, CLSID_CACTIVE_IMM};
use crate::hresult::{Hresult, CLASS_E_CLASSNOTAVAILABLE, S_FALSE};

/// `DllGetClassObject`. For `CLSID_CActiveIMM` it returns the class factory's `QueryInterface`
/// result. For any other CLSID it returns `CLASS_E_CLASSNOTAVAILABLE`, and `*ppv` is NULL.
pub fn dll_get_class_object(rclsid: &Guid, riid: &Guid) -> Result<ClassFactory, Hresult> {
    if is_equal_guid(rclsid, &CLSID_CACTIVE_IMM) {
        return class_factory_query_interface(riid);
    }
    Err(CLASS_E_CLASSNOTAVAILABLE)
}

/// `DllCanUnloadNow`: always `S_FALSE`, so the DLL is never reported as unloadable.
pub fn dll_can_unload_now() -> Hresult {
    S_FALSE
}
