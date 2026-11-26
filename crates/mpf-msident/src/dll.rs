//! Port of `DllGetClassObject` in Wine's `dlls/msident/msident.c`, and of the default `DllCanUnloadNow` in
//! `libs/winecrt0/dll_canunload.c`, which `dlls/msident/msident.spec` exports.

use crate::class_factory::ClassFactory;
use crate::guid::{is_equal_guid, Guid, CLSID_USER_IDENTITY_MANAGER};
use crate::hresult::{CLASS_E_CLASSNOTAVAILABLE, HRESULT, S_FALSE};

/// `DllGetClassObject`. `Ok(())` is `S_OK`: the `UserIdentityManagerCF` class factory was returned for `riid`.
/// Any other CLSID gets `CLASS_E_CLASSNOTAVAILABLE`.
pub fn dll_get_class_object(rclsid: &Guid, riid: &Guid) -> Result<(), HRESULT> {
    if is_equal_guid(&CLSID_USER_IDENTITY_MANAGER, rclsid) {
        ClassFactory.query_interface(riid)
    } else {
        Err(CLASS_E_CLASSNOTAVAILABLE)
    }
}

/// `DllCanUnloadNow`: always `S_FALSE`.
pub fn dll_can_unload_now() -> HRESULT {
    S_FALSE
}
