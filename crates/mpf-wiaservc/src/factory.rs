//! `WIASERVC_ClassFactory` from `dlls/wiaservc/factory.c`: the static class factory that
//! `CoRegisterClassObject` hands out for `CLSID_WiaDevMgr`.

use crate::com::{
    Guid, Hresult, CLASS_E_NOAGGREGATION, E_NOINTERFACE, E_NOTIMPL, IID_ICLASSFACTORY, IID_IUNKNOWN,
};
use crate::devmgr::WiaDevMgr;

#[derive(Debug)]
pub struct ClassFactory;

impl ClassFactory {
    /// The static object reports a fixed count, as `WIASERVC_IClassFactory_AddRef` does.
    pub fn add_ref(&self) -> u32 {
        2
    }

    /// The static object reports a fixed count, as `WIASERVC_IClassFactory_Release` does.
    pub fn release(&self) -> u32 {
        1
    }

    pub fn query_interface(&self, riid: &Guid) -> Result<&ClassFactory, Hresult> {
        if *riid == IID_IUNKNOWN || *riid == IID_ICLASSFACTORY {
            Ok(self)
        } else {
            Err(E_NOINTERFACE)
        }
    }

    /// `WIASERVC_IClassFactory_CreateInstance`. `alloc_ok` models whether the `HeapAlloc` inside
    /// `wiadevmgr_Constructor` succeeds.
    pub fn create_instance(
        &self,
        has_outer: bool,
        riid: &Guid,
        alloc_ok: bool,
    ) -> Result<WiaDevMgr, Hresult> {
        if has_outer {
            return Err(CLASS_E_NOAGGREGATION);
        }
        let devmgr = WiaDevMgr::new(alloc_ok)?;
        let result = devmgr.query_interface(riid).map(|_| ());
        devmgr.release();
        result?;
        Ok(devmgr)
    }

    pub fn lock_server(&self, _lock: bool) -> Hresult {
        E_NOTIMPL
    }
}
