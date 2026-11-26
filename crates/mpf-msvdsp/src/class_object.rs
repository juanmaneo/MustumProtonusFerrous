//! `DllGetClassObject` from msvdsp.c and `DllCanUnloadNow` from libs/winecrt0/dll_canunload.c.

use crate::guid::Guid;
use crate::hresult::{HResult, CLASS_E_CLASSNOTAVAILABLE, S_FALSE};

/// `DllGetClassObject`: the DLL serves no class. The C body sets `*out` to NULL and returns
/// `CLASS_E_CLASSNOTAVAILABLE` for any CLSID and IID, so `out` becomes `None` here; `T` stands in
/// for the interface pointer type.
pub fn dll_get_class_object<T>(_clsid: &Guid, _riid: &Guid, out: &mut Option<T>) -> HResult {
    *out = None;
    CLASS_E_CLASSNOTAVAILABLE
}

/// `DllCanUnloadNow`: always `S_FALSE`, so the DLL is never unloaded.
pub fn dll_can_unload_now() -> HResult {
    S_FALSE
}
