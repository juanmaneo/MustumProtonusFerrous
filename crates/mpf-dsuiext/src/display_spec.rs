//! `IDsDisplaySpecifier` object: `dispspec_QueryInterface`, `dispspec_AddRef`, `dispspec_Release`,
//! `DsDisplaySpecifier_create`, and the vtable entries that only return a fixed value.

use crate::com::{Hresult, Interface, RefCount, E_INVALIDARG, E_NOINTERFACE, E_NOTIMPL, S_OK};
use crate::guid::{Guid, IID_IDS_DISPLAY_SPECIFIER, IID_IUNKNOWN};

/// `ADSTYPE_INVALID`, the first member of `ADSTYPE` in `iads.idl`.
pub const ADSTYPE_INVALID: i32 = 0;

/// `DisplaySpec`: the reference count behind an `IDsDisplaySpecifier`.
pub struct DisplaySpec {
    refs: RefCount,
}

impl DisplaySpec {
    /// The object `DsDisplaySpecifier_create` allocates, before its `QueryInterface` call.
    pub(crate) fn new() -> Self {
        Self {
            refs: RefCount::with_one_reference(),
        }
    }

    /// `DsDisplaySpecifier_create`: queries `iid` on a new object, then releases the creation
    /// reference. A failed query frees the object and leaves `obj` untouched.
    pub fn create(iid: &Guid, obj: &mut Option<Interface>) -> Hresult {
        let mut spec = Self::new();
        let hr = spec.query_interface(Some(iid), Some(obj));
        spec.release();
        hr
    }

    /// `dispspec_QueryInterface`. Only `IUnknown` and `IDsDisplaySpecifier` succeed. A failed
    /// query leaves `obj` untouched, unlike `factory_QueryInterface`.
    pub fn query_interface(
        &mut self,
        iid: Option<&Guid>,
        obj: Option<&mut Option<Interface>>,
    ) -> Hresult {
        let (Some(iid), Some(obj)) = (iid, obj) else {
            return E_INVALIDARG;
        };
        if iid.is_equal(&IID_IUNKNOWN) || iid.is_equal(&IID_IDS_DISPLAY_SPECIFIER) {
            self.add_ref();
            *obj = Some(Interface::IDsDisplaySpecifier);
            return S_OK;
        }
        E_NOINTERFACE
    }

    /// `dispspec_AddRef`.
    pub fn add_ref(&mut self) -> u32 {
        self.refs.add_ref()
    }

    /// `dispspec_Release`. A return of zero means the C code frees the object, which Rust does
    /// when the value is dropped.
    pub fn release(&mut self) -> u32 {
        self.refs.release()
    }

    // The vtable entries below take no arguments. Each returns a fixed value without reading them.

    /// `dispspec_SetServer`: not implemented.
    pub fn set_server() -> Hresult {
        E_NOTIMPL
    }

    /// `dispspec_SetLanguageID`: not implemented.
    pub fn set_language_id() -> Hresult {
        E_NOTIMPL
    }

    /// `dispspec_GetDisplaySpecifier`: not implemented.
    pub fn get_display_specifier() -> Hresult {
        E_NOTIMPL
    }

    /// `dispspec_GetIconLocation`: not implemented.
    pub fn get_icon_location() -> Hresult {
        E_NOTIMPL
    }

    /// `dispspec_GetIcon`: returns a NULL `HICON`.
    pub fn get_icon() -> usize {
        0
    }

    /// `dispspec_GetFriendlyClassName`: not implemented.
    pub fn get_friendly_class_name() -> Hresult {
        E_NOTIMPL
    }

    /// `dispspec_GetFriendlyAttributeName`: not implemented.
    pub fn get_friendly_attribute_name() -> Hresult {
        E_NOTIMPL
    }

    /// `dispspec_IsClassContainer`: always `FALSE`.
    pub fn is_class_container() -> bool {
        false
    }

    /// `dispspec_GetClassCreationInfo`: not implemented.
    pub fn get_class_creation_info() -> Hresult {
        E_NOTIMPL
    }

    /// `dispspec_EnumClassAttributes`: not implemented.
    pub fn enum_class_attributes() -> Hresult {
        E_NOTIMPL
    }

    /// `dispspec_GetAttributeADsType`: always `ADSTYPE_INVALID`.
    pub fn get_attribute_ads_type() -> i32 {
        ADSTYPE_INVALID
    }
}
