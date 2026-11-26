//! The class factory that `DllGetClassObject` returns for `CLSID_Sti` (`dlls/sti/sti_main.c`).

use crate::com::{
    Guid, Hresult, CLASS_E_NOAGGREGATION, E_NOINTERFACE, IID_ICLASSFACTORY, IID_ISTILLIMAGEA,
    IID_ISTILLIMAGEW, IID_IUNKNOWN, S_OK,
};
use crate::still_image::{
    sti_create_instance_a, sti_create_instance_w, Face, OuterUnknown, StillImage,
};

/// `sti_cf_QueryInterface`: `IUnknown` and `IClassFactory` are supported.
pub fn class_factory_query_interface(riid: &Guid) -> Hresult {
    if *riid == IID_IUNKNOWN || *riid == IID_ICLASSFACTORY {
        S_OK
    } else {
        E_NOINTERFACE
    }
}

/// `sti_cf_AddRef`: always 2. The factory is static, so the count is not tracked.
pub fn class_factory_add_ref() -> u32 {
    2
}

/// `sti_cf_Release`: always 1.
pub fn class_factory_release() -> u32 {
    1
}

/// `sti_cf_LockServer`: a no-op that returns `S_OK`.
pub fn class_factory_lock_server(_dolock: bool) -> Hresult {
    S_OK
}

/// `sti_cf_CreateInstance` with `sti_create`. `Err` leaves `*ppobj` NULL.
///
/// The C code QueryInterfaces the pointer that `sti_create` returned and then Releases it.
/// For `IUnknown` that query returns the inner face, and for `IStillImageW` the face itself.
/// The AddRef and Release on the object cancel out, so the count is not changed and the
/// result is the returned face.
pub fn class_factory_create_instance(
    riid: &Guid,
    outer: Option<Box<dyn OuterUnknown>>,
    alloc_ok: bool,
) -> Result<(StillImage, Face), Hresult> {
    if outer.is_some() && *riid != IID_IUNKNOWN {
        return Err(CLASS_E_NOAGGREGATION);
    }
    if *riid == IID_IUNKNOWN {
        let (object, _) = sti_create_instance_w(outer, alloc_ok)?;
        Ok((object, Face::Inner))
    } else if *riid == IID_ISTILLIMAGEW {
        let (object, _) = sti_create_instance_w(None, alloc_ok)?;
        Ok((object, Face::StillImageW))
    } else if *riid == IID_ISTILLIMAGEA {
        Err(sti_create_instance_a())
    } else {
        Err(E_NOINTERFACE)
    }
}
