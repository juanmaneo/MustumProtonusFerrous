//! `wiadevmgr` from `dlls/wiaservc/wiadevmgr.c`: the `IWiaDevMgr` object that
//! `wiadevmgr_Constructor` builds, and its methods.

use crate::com::{
    Guid, Hresult, RefCount, E_NOINTERFACE, E_NOTIMPL, E_OUTOFMEMORY, IID_IUNKNOWN, IID_IWIADEVMGR,
};
use crate::enum_dev_info::EnumWiaDevInfo;

#[derive(Debug)]
pub struct WiaDevMgr {
    ref_count: RefCount,
}

impl WiaDevMgr {
    /// `wiadevmgr_Constructor`, starting at one reference. `alloc_ok` models whether `HeapAlloc`
    /// succeeds.
    pub fn new(alloc_ok: bool) -> Result<WiaDevMgr, Hresult> {
        if alloc_ok {
            Ok(WiaDevMgr {
                ref_count: RefCount::new(1),
            })
        } else {
            Err(E_OUTOFMEMORY)
        }
    }

    pub fn ref_count(&self) -> u32 {
        self.ref_count.get()
    }

    /// `wiadevmgr_QueryInterface`. A successful query adds a reference.
    pub fn query_interface(&self, riid: &Guid) -> Result<&WiaDevMgr, Hresult> {
        if *riid == IID_IUNKNOWN || *riid == IID_IWIADEVMGR {
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

    /// `wiadevmgr_EnumDeviceInfo`. The C body never reads its `flag` argument.
    pub fn enum_device_info(&self, alloc_ok: bool) -> Result<EnumWiaDevInfo, Hresult> {
        EnumWiaDevInfo::new(alloc_ok)
    }

    pub fn create_device(&self) -> Hresult {
        E_NOTIMPL
    }

    pub fn select_device_dlg(&self) -> Hresult {
        E_NOTIMPL
    }

    pub fn select_device_dlg_id(&self) -> Hresult {
        E_NOTIMPL
    }

    pub fn get_image_dlg(&self) -> Hresult {
        E_NOTIMPL
    }

    pub fn register_event_callback_program(&self) -> Hresult {
        E_NOTIMPL
    }

    pub fn register_event_callback_interface(&self) -> Hresult {
        E_NOTIMPL
    }

    pub fn register_event_callback_clsid(&self) -> Hresult {
        E_NOTIMPL
    }

    pub fn add_device_dlg(&self) -> Hresult {
        E_NOTIMPL
    }
}
