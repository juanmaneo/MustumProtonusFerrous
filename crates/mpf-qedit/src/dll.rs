use crate::class_factory::{ClassFactory, ClassId};
use crate::com::{
    is_equal_guid, Guid, Hresult, CLASS_E_CLASSNOTAVAILABLE, E_NOINTERFACE, E_OUTOFMEMORY,
    IID_ICLASSFACTORY, IID_IUNKNOWN, S_FALSE,
};

/// `DllGetClassObject`. The IID is checked before the CLSID, so an unknown CLSID with a bad IID
/// reports `E_NOINTERFACE`. `alloc_ok` stands for `CoTaskMemAlloc` succeeding. On `Err`, the C
/// code leaves `*ppv` untouched.
pub fn dll_get_class_object(
    clsid: &Guid,
    iid: &Guid,
    alloc_ok: bool,
) -> Result<ClassFactory, Hresult> {
    if !is_equal_guid(iid, &IID_ICLASSFACTORY) && !is_equal_guid(iid, &IID_IUNKNOWN) {
        return Err(E_NOINTERFACE);
    }
    let class = ClassId::from_clsid(clsid).ok_or(CLASS_E_CLASSNOTAVAILABLE)?;
    if !alloc_ok {
        return Err(E_OUTOFMEMORY);
    }
    Ok(ClassFactory::new(class))
}

/// `DllCanUnloadNow`. `dlls/qedit` has no body for it, so the winecrt0 default applies, which
/// always returns `S_FALSE`.
pub fn dll_can_unload_now() -> Hresult {
    S_FALSE
}
