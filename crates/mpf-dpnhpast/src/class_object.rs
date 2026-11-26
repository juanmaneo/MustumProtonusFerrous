//! `DllGetClassObject` from `dlls/dpnhpast/main.c` and `DllCanUnloadNow` from `libs/winecrt0/dll_canunload.c`.

use crate::{Hresult, E_FAIL, S_FALSE};
use std::ffi::c_void;

/// `DllGetClassObject(rclsid, riid, ppv)`. The module has no class factories,
/// so it returns `E_FAIL` and leaves `*ppv` untouched.
pub fn dll_get_class_object(
    _clsid: &[u8; 16],
    _iid: &[u8; 16],
    _ppv: Option<&mut *mut c_void>,
) -> Hresult {
    E_FAIL
}

/// `DllCanUnloadNow()` from the winecrt0 default: `S_FALSE` means the module stays loaded.
pub fn dll_can_unload_now() -> Hresult {
    S_FALSE
}
