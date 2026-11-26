//! Port of the `CNetCfg` object in `dlls/netcfgx/netcfg.c`: `INetCfg_CreateInstance`, its
//! reference counting, and the fixed return values of its methods.

use crate::com::{
    is_equal_guid, Guid, HResult, E_NOINTERFACE, E_NOTIMPL, E_OUTOFMEMORY, IID_INETCFG,
    IID_INETCFGLOCK, IID_IUNKNOWN, S_OK,
};

/// The interfaces a `QueryInterface` call can return.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iface {
    IClassFactory,
    INetCfg,
    INetCfgLock,
}

/// `struct NetConfiguration`. The C struct also stores the two vtable pointers, which the port
/// does not need. `ref_count` is the C `LONG ref` field.
pub struct NetConfiguration {
    pub ref_count: i32,
}

impl NetConfiguration {
    /// `netcfg_QueryInterface`, which also serves `INetCfgLock` because both interfaces share the
    /// object. A successful call adds a reference for the interface it returns and leaves `out`
    /// unchanged on failure.
    pub fn query_interface(&mut self, riid: &Guid, out: &mut Option<Iface>) -> HResult {
        let iface = if is_equal_guid(riid, &IID_INETCFG) || is_equal_guid(riid, &IID_IUNKNOWN) {
            Iface::INetCfg
        } else if is_equal_guid(riid, &IID_INETCFGLOCK) {
            Iface::INetCfgLock
        } else {
            return E_NOINTERFACE;
        };
        *out = Some(iface);
        self.add_ref();
        S_OK
    }

    /// `netcfg_AddRef`. `InterlockedIncrement` wraps the signed count, so the port wraps too.
    pub fn add_ref(&mut self) -> u32 {
        self.ref_count = self.ref_count.wrapping_add(1);
        self.ref_count as u32
    }

    /// `netcfg_Release`. The C code frees the object when the count reaches 0; in Rust the owner
    /// drops the value instead, so this only returns the new count.
    pub fn release(&mut self) -> u32 {
        self.ref_count = self.ref_count.wrapping_sub(1);
        self.ref_count as u32
    }

    /// `netcfg_Initialize`.
    pub fn initialize(&self) -> HResult {
        S_OK
    }

    /// `netcfg_Uninitialize`.
    pub fn uninitialize(&self) -> HResult {
        S_OK
    }

    /// `netcfg_Apply`.
    pub fn apply(&self) -> HResult {
        E_NOTIMPL
    }

    /// `netcfg_Cancel`.
    pub fn cancel(&self) -> HResult {
        E_NOTIMPL
    }

    /// `netcfg_EnumComponents`.
    pub fn enum_components(&self) -> HResult {
        E_NOTIMPL
    }

    /// `netcfg_FindComponent`.
    pub fn find_component(&self) -> HResult {
        E_NOTIMPL
    }

    /// `netcfg_QueryNetCfgClass`.
    pub fn query_net_cfg_class(&self) -> HResult {
        E_NOTIMPL
    }

    /// `netcfglock_AcquireWriteLock`. It succeeds without recording any lock state.
    pub fn acquire_write_lock(&self) -> HResult {
        S_OK
    }

    /// `netcfglock_ReleaseWriteLock`.
    pub fn release_write_lock(&self) -> HResult {
        S_OK
    }

    /// `netcfglock_IsWriteLocked`.
    pub fn is_write_locked(&self) -> HResult {
        E_NOTIMPL
    }
}

/// `INetCfg_CreateInstance`, which builds a `CNetCfg` with a reference count of 1. `heap_alloc_ok`
/// is whether the `HeapAlloc` succeeds. On failure the C code returns `E_OUTOFMEMORY` and leaves
/// `*ppUnk` untouched, which the `Err` return reflects.
pub fn inet_cfg_create_instance(heap_alloc_ok: bool) -> Result<NetConfiguration, HResult> {
    if !heap_alloc_ok {
        return Err(E_OUTOFMEMORY);
    }
    Ok(NetConfiguration { ref_count: 1 })
}
