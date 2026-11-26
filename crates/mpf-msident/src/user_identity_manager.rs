//! Port of the `IUserIdentityManager` methods of the static `UserIdentityManager` object in Wine's `dlls/msident/msident.c`.

use crate::enum_user_identity::EnumUserIdentity;
use crate::guid::{is_equal_guid, Guid, IID_IUNKNOWN, IID_IUSER_IDENTITY_MANAGER};
use crate::hresult::{E_NOINTERFACE, E_NOTIMPL, E_OUTOFMEMORY, E_USER_CANCELLED, HRESULT};

/// The static `UserIdentityManager` object. It has no state: its `AddRef` and `Release` return constants,
/// and it is never freed.
#[derive(Debug)]
pub struct UserIdentityManager;

impl UserIdentityManager {
    /// `UserIdentityManager_QueryInterface`. `IUnknown` and `IUserIdentityManager` succeed. The static
    /// object's `AddRef` has no effect, so nothing is counted.
    pub fn query_interface(&self, riid: &Guid) -> Result<(), HRESULT> {
        if is_equal_guid(&IID_IUNKNOWN, riid) || is_equal_guid(&IID_IUSER_IDENTITY_MANAGER, riid) {
            Ok(())
        } else {
            Err(E_NOINTERFACE)
        }
    }

    /// `UserIdentityManager_AddRef`: always 2.
    pub fn add_ref(&self) -> u32 {
        2
    }

    /// `UserIdentityManager_Release`: always 1.
    pub fn release(&self) -> u32 {
        1
    }

    /// `UserIdentityManager_EnumIdentities`. `alloc_succeeds` models whether `malloc` returns memory.
    /// On failure the C code does not write `*ppEnumUser`.
    pub fn enum_identities(&self, alloc_succeeds: bool) -> Result<EnumUserIdentity, HRESULT> {
        if alloc_succeeds {
            Ok(EnumUserIdentity::new())
        } else {
            Err(E_OUTOFMEMORY)
        }
    }

    /// `UserIdentityManager_ManageIdentities`: always `E_NOTIMPL`.
    pub fn manage_identities(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `UserIdentityManager_Logon`: always `E_USER_CANCELLED`, and it does not write `*ppIdentity`.
    pub fn logon(&self) -> HRESULT {
        E_USER_CANCELLED
    }

    /// `UserIdentityManager_Logoff`: always `E_NOTIMPL`.
    pub fn logoff(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `UserIdentityManager_GetIdentityByCookie`: always `E_NOTIMPL`, and it does not write `*ppIdentity`.
    pub fn get_identity_by_cookie(&self) -> HRESULT {
        E_NOTIMPL
    }
}
