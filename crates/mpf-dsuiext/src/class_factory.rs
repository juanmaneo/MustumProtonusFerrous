//! `IClassFactory` object: `factory_QueryInterface`, `factory_AddRef`, `factory_Release`,
//! `factory_CreateInstance`, `factory_LockServer`, and `factory_constructor`.

use crate::com::{
    Hresult, Interface, RefCount, CLASS_E_NOAGGREGATION, E_INVALIDARG, E_NOINTERFACE, S_OK,
};
use crate::dll::ClassInfo;
use crate::guid::{Guid, IID_ICLASSFACTORY, IID_IUNKNOWN};

/// `class_factory`: an `IClassFactory` for one `class_info` row.
pub struct ClassFactory {
    refs: RefCount,
    info: &'static ClassInfo,
}

impl ClassFactory {
    pub(crate) fn new(info: &'static ClassInfo) -> Self {
        Self {
            refs: RefCount::with_one_reference(),
            info,
        }
    }

    /// `factory_QueryInterface`. Only `IUnknown` and `IClassFactory` succeed. A failed query
    /// writes `None` to `obj`, unlike `dispspec_QueryInterface`.
    pub fn query_interface(
        &mut self,
        iid: Option<&Guid>,
        obj: Option<&mut Option<Interface>>,
    ) -> Hresult {
        let (Some(iid), Some(obj)) = (iid, obj) else {
            return E_INVALIDARG;
        };
        if iid.is_equal(&IID_IUNKNOWN) || iid.is_equal(&IID_ICLASSFACTORY) {
            self.add_ref();
            *obj = Some(Interface::IClassFactory);
            return S_OK;
        }
        *obj = None;
        E_NOINTERFACE
    }

    /// `factory_AddRef`.
    pub fn add_ref(&mut self) -> u32 {
        self.refs.add_ref()
    }

    /// `factory_Release`. A return of zero means the C code frees the factory.
    pub fn release(&mut self) -> u32 {
        self.refs.release()
    }

    /// `factory_CreateInstance`. `has_outer` is whether the C caller passed a non-NULL `outer`;
    /// aggregation is refused. `obj` is cleared before that check.
    pub fn create_instance(
        &self,
        has_outer: bool,
        iid: Option<&Guid>,
        obj: Option<&mut Option<Interface>>,
    ) -> Hresult {
        let (Some(iid), Some(obj)) = (iid, obj) else {
            return E_INVALIDARG;
        };
        *obj = None;
        if has_outer {
            return CLASS_E_NOAGGREGATION;
        }
        (self.info.constructor)(iid, obj)
    }

    /// `factory_LockServer`: not implemented, returns `S_OK` without effect.
    pub fn lock_server(_lock: bool) -> Hresult {
        S_OK
    }
}

/// `factory_constructor`: builds a factory with one reference, queries `riid` on it, and
/// releases the creation reference.
pub fn factory_constructor(
    info: &'static ClassInfo,
    riid: &Guid,
    obj: &mut Option<Interface>,
) -> Hresult {
    let mut factory = ClassFactory::new(info);
    let hr = factory.query_interface(Some(riid), Some(obj));
    factory.release();
    hr
}
