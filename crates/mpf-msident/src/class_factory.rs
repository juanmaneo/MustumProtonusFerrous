//! Port of the `IClassFactory` methods of the static `UserIdentityManagerCF` object in Wine's `dlls/msident/msident.c`.

use crate::guid::{is_equal_guid, Guid, IID_ICLASSFACTORY, IID_IUNKNOWN};
use crate::hresult::{E_NOINTERFACE, HRESULT, S_OK};
use crate::user_identity_manager::UserIdentityManager;

/// The static class factory `UserIdentityManagerCF`. Like `UserIdentityManager`, it has no state.
#[derive(Debug)]
pub struct ClassFactory;

impl ClassFactory {
    /// `ClassFactory_QueryInterface`. `IUnknown` and `IClassFactory` succeed.
    pub fn query_interface(&self, riid: &Guid) -> Result<(), HRESULT> {
        if is_equal_guid(&IID_IUNKNOWN, riid) || is_equal_guid(&IID_ICLASSFACTORY, riid) {
            Ok(())
        } else {
            Err(E_NOINTERFACE)
        }
    }

    /// `ClassFactory_AddRef`: always 2.
    pub fn add_ref(&self) -> u32 {
        2
    }

    /// `ClassFactory_Release`: always 1.
    pub fn release(&self) -> u32 {
        1
    }

    /// `ClassFactory_LockServer`: always `S_OK`. The `fLock` argument is not recorded.
    pub fn lock_server(&self) -> HRESULT {
        S_OK
    }

    /// `UserIdentityManager_CreateInstance`. The `outer` argument is ignored, and the result is the
    /// `UserIdentityManager` `QueryInterface` result for `riid`.
    pub fn create_instance(&self, riid: &Guid) -> Result<(), HRESULT> {
        UserIdentityManager.query_interface(riid)
    }
}
