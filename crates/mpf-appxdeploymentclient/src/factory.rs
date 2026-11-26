//! Port of `struct package_manager_statics` and the `IActivationFactory` methods in Wine's `dlls/appxdeploymentclient/package.c`.

use crate::hresult::{E_NOINTERFACE, E_OUTOFMEMORY, HRESULT};
use crate::package_manager::PackageManager;
use crate::query::{factory_accepts, Iid};
use crate::refcount::RefCount;

/// The static `IActivationFactory` of `Windows.Management.Deployment.PackageManager`. It starts with `ref = 1`
/// and is never freed: `factory_Release` does not call `free`.
#[derive(Debug)]
pub struct ActivationFactory {
    ref_count: RefCount,
}

impl Default for ActivationFactory {
    fn default() -> Self {
        Self {
            ref_count: RefCount::initial(),
        }
    }
}

impl ActivationFactory {
    /// `factory_QueryInterface`. A successful query adds a reference.
    pub fn query_interface(&mut self, iid: Iid) -> Result<(), HRESULT> {
        if !factory_accepts(iid) {
            return Err(E_NOINTERFACE);
        }
        self.add_ref();
        Ok(())
    }

    /// `factory_AddRef`: returns the new count.
    pub fn add_ref(&mut self) -> u32 {
        self.ref_count.add_ref()
    }

    /// `factory_Release`: returns the new count. The static object is never freed.
    pub fn release(&mut self) -> u32 {
        self.ref_count.release()
    }

    pub fn ref_count(&self) -> i32 {
        self.ref_count.get()
    }

    /// `factory_ActivateInstance`. `alloc_succeeds` models whether `calloc` returns memory.
    /// The static factory's count is not changed.
    pub fn activate_instance(&self, alloc_succeeds: bool) -> Result<PackageManager, HRESULT> {
        if !alloc_succeeds {
            return Err(E_OUTOFMEMORY);
        }
        Ok(PackageManager::activated())
    }
}
