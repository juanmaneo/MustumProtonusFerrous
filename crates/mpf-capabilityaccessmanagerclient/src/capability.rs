//! `AppCapability` (`struct capability` in Wine's `dlls/capabilityaccessmanagerclient/capability.c`).

use crate::com::{is_base_interface, Guid, IID_IAPPCAPABILITY};
use crate::{Hresult, Interface, E_NOINTERFACE, E_NOTIMPL, S_OK};

/// `AppCapabilityAccessStatus` from `windows.security.authorization.appcapabilityaccess.idl`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppCapabilityAccessStatus {
    DeniedBySystem = 0,
    NotDeclaredByApp = 1,
    DeniedByUser = 2,
    UserPromptRequired = 3,
    Allowed = 4,
}

/// `EventRegistrationToken`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EventRegistrationToken {
    pub value: i64,
}

/// `dummy_cookie`, which every `add_AccessChanged` call returns.
const DUMMY_COOKIE: EventRegistrationToken = EventRegistrationToken { value: 0xdead_beef };

/// `struct capability`. An empty `name` stands for the NULL HSTRING.
pub struct AppCapability {
    pub(crate) name: Vec<u16>,
    pub(crate) ref_count: i32,
}

impl AppCapability {
    /// `cap_create`: duplicates the name and starts with one reference.
    pub fn create(cap_name: &[u16]) -> Result<Self, Hresult> {
        Ok(Self {
            name: cap_name.to_vec(),
            ref_count: 1,
        })
    }

    /// `cap_QueryInterface`. Success adds a reference.
    pub fn query_interface(&mut self, iid: &Guid) -> Result<Interface, Hresult> {
        if is_base_interface(iid) || *iid == IID_IAPPCAPABILITY {
            self.add_ref();
            Ok(Interface::AppCapability)
        } else {
            Err(E_NOINTERFACE)
        }
    }

    /// `cap_AddRef`. Returns the new count.
    pub fn add_ref(&mut self) -> u32 {
        self.ref_count = self.ref_count.wrapping_add(1);
        self.ref_count as u32
    }

    /// `cap_Release`. Returns the new count. At 0 the C code frees the object; here the
    /// owner does that by dropping it.
    pub fn release(&mut self) -> u32 {
        self.ref_count = self.ref_count.wrapping_sub(1);
        self.ref_count as u32
    }

    // The `FIXME` stubs. Each returns `E_NOTIMPL` and writes no output.
    pub fn get_iids(&self) -> Hresult {
        E_NOTIMPL
    }

    pub fn get_runtime_class_name(&self) -> Hresult {
        E_NOTIMPL
    }

    pub fn get_trust_level(&self) -> Hresult {
        E_NOTIMPL
    }

    pub fn get_user(&self) -> Hresult {
        E_NOTIMPL
    }

    /// `cap_RequestAccessAsync`. The C code also sets `*operation` to NULL; the port has no out pointer.
    pub fn request_access_async(&self) -> Hresult {
        E_NOTIMPL
    }

    /// `cap_get_CapabilityName`: the duplicated name.
    pub fn get_capability_name(&self) -> Result<&[u16], Hresult> {
        Ok(self.name.as_slice())
    }

    /// `cap_CheckAccess`, a semi-stub in C: always `Allowed`.
    pub fn check_access(&self) -> Result<AppCapabilityAccessStatus, Hresult> {
        Ok(AppCapabilityAccessStatus::Allowed)
    }

    /// `cap_add_AccessChanged`. The handler is not stored, so every call returns the dummy cookie.
    pub fn add_access_changed(&self) -> Result<EventRegistrationToken, Hresult> {
        Ok(DUMMY_COOKIE)
    }

    /// `cap_remove_AccessChanged`. Ignores the cookie.
    pub fn remove_access_changed(&self, _cookie: EventRegistrationToken) -> Hresult {
        S_OK
    }
}
