//! The object that is both `IActivationFactory` and `IAppCapabilityStatics`
//! (`struct capability_statics` in `capability.c`).

use crate::capability::AppCapability;
use crate::com::{is_base_interface, Guid, IID_IACTIVATIONFACTORY, IID_IAPPCAPABILITYSTATICS};
use crate::{Hresult, Interface, E_NOINTERFACE, E_NOTIMPL};

/// Both faces share `ref_count`. The static object starts with 1, which the DLL holds.
pub struct CapabilityStatics {
    pub(crate) ref_count: i32,
}

impl Default for CapabilityStatics {
    fn default() -> Self {
        Self { ref_count: 1 }
    }
}

impl CapabilityStatics {
    /// `factory_QueryInterface`. The statics face forwards here too, so base IIDs answer
    /// with the factory face. Success adds a reference.
    pub fn query_interface(&mut self, iid: &Guid) -> Result<Interface, Hresult> {
        let interface = if is_base_interface(iid) || *iid == IID_IACTIVATIONFACTORY {
            Interface::ActivationFactory
        } else if *iid == IID_IAPPCAPABILITYSTATICS {
            Interface::AppCapabilityStatics
        } else {
            return Err(E_NOINTERFACE);
        };
        self.add_ref();
        Ok(interface)
    }

    /// `factory_AddRef` and `capability_statics_AddRef`. Returns the new count.
    pub fn add_ref(&mut self) -> u32 {
        self.ref_count = self.ref_count.wrapping_add(1);
        self.ref_count as u32
    }

    /// `factory_Release`. Returns the new count. The object is static, so nothing is freed at 0.
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

    pub fn activate_instance(&self) -> Hresult {
        E_NOTIMPL
    }

    pub fn request_access_for_capabilities_async(&self) -> Hresult {
        E_NOTIMPL
    }

    pub fn request_access_for_capabilities_for_user_async(&self) -> Hresult {
        E_NOTIMPL
    }

    pub fn create_with_process_id_for_user(&self) -> Hresult {
        E_NOTIMPL
    }

    /// `capability_statics_Create`, which is `cap_create`.
    pub fn create(&self, cap_name: &[u16]) -> Result<AppCapability, Hresult> {
        AppCapability::create(cap_name)
    }
}
