//! `enumwiadevinfo` from `dlls/wiaservc/wiadevmgr.c`: the `IEnumWIA_DEV_INFO` object that
//! `wiadevmgr_EnumDeviceInfo` returns. Its iteration methods are `E_NOTIMPL` stubs.

use crate::com::{
    Guid, Hresult, RefCount, E_NOINTERFACE, E_NOTIMPL, E_OUTOFMEMORY, IID_IENUMWIA_DEV_INFO,
    IID_IUNKNOWN,
};

#[derive(Debug)]
pub struct EnumWiaDevInfo {
    ref_count: RefCount,
}

impl EnumWiaDevInfo {
    /// `wiadevmgr_EnumDeviceInfo`'s allocation, starting at one reference. `alloc_ok` models
    /// whether `HeapAlloc` succeeds.
    pub fn new(alloc_ok: bool) -> Result<EnumWiaDevInfo, Hresult> {
        if alloc_ok {
            Ok(EnumWiaDevInfo {
                ref_count: RefCount::new(1),
            })
        } else {
            Err(E_OUTOFMEMORY)
        }
    }

    pub fn ref_count(&self) -> u32 {
        self.ref_count.get()
    }

    /// `enumwiadevinfo_QueryInterface`. A successful query adds a reference.
    pub fn query_interface(&self, riid: &Guid) -> Result<&EnumWiaDevInfo, Hresult> {
        if *riid == IID_IUNKNOWN || *riid == IID_IENUMWIA_DEV_INFO {
            self.ref_count.add_ref();
            Ok(self)
        } else {
            Err(E_NOINTERFACE)
        }
    }

    pub fn add_ref(&self) -> u32 {
        self.ref_count.add_ref()
    }

    pub fn release(&self) -> u32 {
        self.ref_count.release()
    }

    /// `enumwiadevinfo_Next`: returns `E_NOTIMPL` and writes `*fetched = 0`.
    pub fn next(&self) -> (Hresult, u32) {
        (E_NOTIMPL, 0)
    }

    pub fn skip(&self) -> Hresult {
        E_NOTIMPL
    }

    pub fn reset(&self) -> Hresult {
        E_NOTIMPL
    }

    /// `enumwiadevinfo_Clone`: returns `E_NOTIMPL` and leaves `*ret` untouched.
    pub fn clone_enumerator(&self) -> Hresult {
        E_NOTIMPL
    }

    /// `enumwiadevinfo_GetCount`: returns `E_NOTIMPL` and writes `*count = 0`.
    pub fn get_count(&self) -> (Hresult, u32) {
        (E_NOTIMPL, 0)
    }
}
