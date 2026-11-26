//! Port of `struct package_manager` and its `IPackageManager` methods in Wine's `dlls/appxdeploymentclient/package.c`.

use crate::hresult::{E_INVALIDARG, E_NOINTERFACE, E_NOTIMPL, HRESULT};
use crate::query::{package_manager_view, Iid, View};
use crate::refcount::RefCount;

/// One `struct package_manager`. Its `IPackageManager` and `IPackageManager2` views share `ref_count`.
#[derive(Debug)]
pub struct PackageManager {
    ref_count: RefCount,
}

impl PackageManager {
    /// The object that `factory_ActivateInstance` returns, with `ref = 1`.
    pub(crate) fn activated() -> Self {
        Self {
            ref_count: RefCount::initial(),
        }
    }

    /// `package_manager_QueryInterface`. A successful query adds a reference, as `IInspectable_AddRef` does in C.
    pub fn query_interface(&mut self, iid: Iid) -> Result<View, HRESULT> {
        let view = package_manager_view(iid).ok_or(E_NOINTERFACE)?;
        self.add_ref();
        Ok(view)
    }

    /// `package_manager_AddRef`: returns the new count.
    pub fn add_ref(&mut self) -> u32 {
        self.ref_count.add_ref()
    }

    /// `package_manager_Release`: returns the new count. Zero means C calls `free`; the caller drops the object.
    pub fn release(&mut self) -> u32 {
        self.ref_count.release()
    }

    pub fn ref_count(&self) -> i32 {
        self.ref_count.get()
    }
}

/// The argument check of `package_manager_FindPackagesByNamePublisher`. A NULL `HSTRING` gives
/// `E_INVALIDARG`; otherwise the body is `E_NOTIMPL`.
pub fn find_packages_by_name_publisher(name: Option<&[u16]>, publisher: Option<&[u16]>) -> HRESULT {
    if name.is_none() || publisher.is_none() {
        return E_INVALIDARG;
    }
    E_NOTIMPL
}
