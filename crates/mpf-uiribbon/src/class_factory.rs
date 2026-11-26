use crate::com::{is_equal_guid, Guid, Interface, IID_ICLASSFACTORY, IID_IUNKNOWN};
use crate::framework::{ui_ribbon_framework_create, UIRibbonFramework};
use crate::{Hresult, E_NOINTERFACE, S_OK};

/// `IClassFactoryImpl` from `dlls/uiribbon/main.c`. The C code heap-allocates it with one
/// reference. `XFCF_Release` frees it when the count reaches zero. In the port, the caller drops
/// it at that point.
pub struct ClassFactory {
    ref_count: u32,
}

impl ClassFactory {
    pub(crate) fn new() -> Self {
        Self { ref_count: 1 }
    }

    /// `XFCF_QueryInterface`. `IUnknown` and `IClassFactory` take a reference. For any other IID,
    /// `obj` is left untouched.
    pub fn query_interface(&mut self, iid: &Guid, obj: &mut Option<Interface>) -> Hresult {
        if is_equal_guid(iid, &IID_IUNKNOWN) || is_equal_guid(iid, &IID_ICLASSFACTORY) {
            self.add_ref();
            *obj = Some(Interface::ClassFactory);
            return S_OK;
        }
        E_NOINTERFACE
    }

    /// `XFCF_AddRef`.
    pub fn add_ref(&mut self) -> u32 {
        self.ref_count += 1;
        self.ref_count
    }

    /// `XFCF_Release`. The caller drops the factory when this returns 0, and must not release it
    /// again after that.
    pub fn release(&mut self) -> u32 {
        self.ref_count -= 1;
        self.ref_count
    }

    /// `XFCF_CreateInstance`. The C code creates an object with `UIRibbonFrameworkImpl_Create`,
    /// queries it for `iid` and releases the creation reference. The object survives only if the
    /// query took a reference of its own. The `pUnkOuter` argument is never read.
    pub fn create_instance(&self, iid: &Guid) -> (Hresult, Option<UIRibbonFramework>) {
        let mut punk = ui_ribbon_framework_create();
        let mut obj = None;
        let hr = punk.query_interface(iid, &mut obj);
        if punk.release() == 0 {
            (hr, None)
        } else {
            (hr, Some(punk))
        }
    }

    /// `XFCF_LockServer`. It is a `FIXME` stub that returns `S_OK` and has no effect.
    pub fn lock_server(&self, _lock: bool) -> Hresult {
        S_OK
    }
}
