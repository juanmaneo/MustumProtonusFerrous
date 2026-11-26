//! Port of the `netcfgcf` class factory in `dlls/netcfgx/netcfg.c`.

use crate::com::{
    is_equal_guid, Guid, HResult, CLASS_E_NOAGGREGATION, E_NOINTERFACE, IID_ICLASSFACTORY,
    IID_IUNKNOWN, S_OK,
};
use crate::netcfg::{inet_cfg_create_instance, Iface};

/// The class factory `DllGetClassObject` returns for `CLSID_CNetCfg`. The C struct also holds a
/// `fnCreateInstance` pointer; the port always creates a `CNetCfg`, so it needs no fields.
pub struct ClassFactory;

impl ClassFactory {
    /// `netcfgcf_QueryInterface`. The C code also adds a reference to the factory here, but
    /// `netcfgcf_AddRef` is fixed (see [`ClassFactory::add_ref`]), so the port does not count it.
    pub fn query_interface(&self, riid: &Guid, ppobj: &mut Option<Iface>) -> HResult {
        if is_equal_guid(riid, &IID_IUNKNOWN) || is_equal_guid(riid, &IID_ICLASSFACTORY) {
            *ppobj = Some(Iface::IClassFactory);
            return S_OK;
        }
        E_NOINTERFACE
    }

    /// `netcfgcf_AddRef`, which always returns 2.
    pub fn add_ref(&self) -> u32 {
        2
    }

    /// `netcfgcf_Release`, which always returns 1.
    pub fn release(&self) -> u32 {
        1
    }

    /// `netcfgcf_CreateInstance`. `p_outer` is whether `pOuter` is non-NULL, and `heap_alloc_ok`
    /// is whether the `HeapAlloc` in `INetCfg_CreateInstance` succeeds.
    pub fn create_instance(
        &self,
        p_outer: bool,
        riid: &Guid,
        ppobj: &mut Option<Iface>,
        heap_alloc_ok: bool,
    ) -> HResult {
        *ppobj = None;
        if p_outer {
            return CLASS_E_NOAGGREGATION;
        }
        match inet_cfg_create_instance(heap_alloc_ok) {
            Ok(mut punk) => {
                let hr = punk.query_interface(riid, ppobj);
                punk.release();
                hr
            }
            Err(hr) => hr,
        }
    }

    /// `netcfgcf_LockServer`, a stub that always succeeds.
    pub fn lock_server(&self, _do_lock: bool) -> HResult {
        S_OK
    }
}
