use crate::com::{is_equal_guid, Guid, Interface, IID_IUIFRAMEWORK, IID_IUNKNOWN};
use crate::{Hresult, E_NOINTERFACE, S_OK};

/// `UIRibbonFrameworkImpl` from `dlls/uiribbon/uiribbon_private.h`. `Release` frees the object
/// when the count reaches zero. In the port, the caller drops it at that point.
pub struct UIRibbonFramework {
    ref_count: u32,
}

impl UIRibbonFramework {
    /// `UIRibbonFrameworkImpl_QueryInterface`. `IUnknown` and `IUIFramework` take a reference.
    /// For any other IID, `ppvObject` is left untouched.
    pub fn query_interface(&mut self, iid: &Guid, obj: &mut Option<Interface>) -> Hresult {
        if is_equal_guid(iid, &IID_IUNKNOWN) || is_equal_guid(iid, &IID_IUIFRAMEWORK) {
            self.add_ref();
            *obj = Some(Interface::UIRibbonFramework);
            return S_OK;
        }
        E_NOINTERFACE
    }

    /// `UIRibbonFrameworkImpl_AddRef`.
    pub fn add_ref(&mut self) -> u32 {
        self.ref_count += 1;
        self.ref_count
    }

    /// `UIRibbonFrameworkImpl_Release`. The caller drops the object when this returns 0, and must
    /// not release it again after that.
    pub fn release(&mut self) -> u32 {
        self.ref_count -= 1;
        self.ref_count
    }
}

/// `UIRibbonFrameworkImpl_Create`. The object starts with one reference. The `pUnkOuter`
/// argument is never read. The C `HeapAlloc` failure path (`E_OUTOFMEMORY`) is not ported.
pub fn ui_ribbon_framework_create() -> UIRibbonFramework {
    UIRibbonFramework { ref_count: 1 }
}
