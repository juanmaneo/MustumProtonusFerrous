//! `DllMain` and `DllGetClassObject` from `main.c`.

use crate::classfactory::class_factory_query_interface;
use crate::hresult::{HResult, CLASS_E_CLASSNOTAVAILABLE};
use crate::qi::Iid;

pub const DLL_PROCESS_DETACH: u32 = 0;
pub const DLL_PROCESS_ATTACH: u32 = 1;

/// The CLSIDs `DllGetClassObject` compares against. `Other` is any other CLSID.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Clsid {
    DHTMLEdit,
    Other,
}

/// The side effect `DllMain` performs. Its return value is always TRUE.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DllMainAction {
    DisableThreadLibraryCalls,
    ReleaseTypelib,
    Nothing,
}

/// `DllMain`.
pub fn dll_main(reason: u32, reserved: bool) -> DllMainAction {
    match reason {
        DLL_PROCESS_ATTACH => DllMainAction::DisableThreadLibraryCalls,
        DLL_PROCESS_DETACH if !reserved => DllMainAction::ReleaseTypelib,
        _ => DllMainAction::Nothing,
    }
}

/// `DllGetClassObject`. Only `CLSID_DHTMLEdit` has a class; its result is the factory's QueryInterface.
pub fn dll_get_class_object(clsid: Clsid, iid: Iid) -> HResult {
    match clsid {
        Clsid::DHTMLEdit => class_factory_query_interface(iid),
        Clsid::Other => CLASS_E_CLASSNOTAVAILABLE,
    }
}
