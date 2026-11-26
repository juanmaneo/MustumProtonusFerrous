//! `DllGetClassObject`, the `class_info` table it searches, and `DllCanUnloadNow`.

use crate::class_factory::factory_constructor;
use crate::com::{Hresult, Interface, CLASS_E_CLASSNOTAVAILABLE, E_INVALIDARG, S_FALSE};
use crate::display_spec::DisplaySpec;
use crate::guid::{Guid, CLSID_DS_DISPLAY_SPECIFIER};

/// One row of `class_info[]`: a CLSID and the constructor for its objects.
pub struct ClassInfo {
    pub clsid: Guid,
    pub constructor: fn(&Guid, &mut Option<Interface>) -> Hresult,
}

pub(crate) static CLASS_INFO: [ClassInfo; 1] = [ClassInfo {
    clsid: CLSID_DS_DISPLAY_SPECIFIER,
    constructor: DisplaySpec::create,
}];

/// `DllGetClassObject`. Clears `obj` once the arguments are known to be valid, then hands out
/// a factory for the first `class_info` row whose CLSID matches.
pub fn dll_get_class_object(
    clsid: Option<&Guid>,
    iid: Option<&Guid>,
    obj: Option<&mut Option<Interface>>,
) -> Hresult {
    let (Some(clsid), Some(iid), Some(obj)) = (clsid, iid, obj) else {
        return E_INVALIDARG;
    };
    *obj = None;
    for info in &CLASS_INFO {
        if info.clsid.is_equal(clsid) {
            return factory_constructor(info, iid, obj);
        }
    }
    CLASS_E_CLASSNOTAVAILABLE
}

/// `DllCanUnloadNow`, the `winecrt0` default. The DLL reports that it cannot be unloaded.
pub fn dll_can_unload_now() -> Hresult {
    S_FALSE
}
