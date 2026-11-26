//! The static `IClassFactory` object and `DllGetClassObject` from `dlls/tiptsf/tiptsf_main.c`.
//! The factory's `CreateInstance` slot is [`crate::panelinput_create_instance`].

use crate::com::{
    is_equal_guid, Guid, CLASS_E_CLASSNOTAVAILABLE, CLSID_TEXT_INPUT_PANEL, E_NOINTERFACE, HRESULT,
    IID_CLASS_FACTORY, IID_UNKNOWN, S_OK,
};

/// `ClassFactory_QueryInterface`: accepts `IID_IUnknown` and `IID_IClassFactory`.
pub fn class_factory_query_interface(riid: &Guid) -> HRESULT {
    if is_equal_guid(riid, &IID_UNKNOWN) || is_equal_guid(riid, &IID_CLASS_FACTORY) {
        S_OK
    } else {
        E_NOINTERFACE
    }
}

/// `ClassFactory_AddRef`: the static factory always reports 2.
pub fn class_factory_add_ref() -> u32 {
    2
}

/// `ClassFactory_Release`: the static factory always reports 1.
pub fn class_factory_release() -> u32 {
    1
}

/// `ClassFactory_LockServer`: the lock flag is traced and ignored.
pub fn class_factory_lock_server(_f_lock: bool) -> HRESULT {
    S_OK
}

/// `DllGetClassObject`. Only `CLSID_TextInputPanel` has a class object.
pub fn dll_get_class_object(rclsid: &Guid, riid: &Guid) -> HRESULT {
    if is_equal_guid(&CLSID_TEXT_INPUT_PANEL, rclsid) {
        class_factory_query_interface(riid)
    } else {
        CLASS_E_CLASSNOTAVAILABLE
    }
}
