use crate::class_factory::class_factory_query_interface;
use crate::guid::{is_equal_guid, Guid, CLSID_WINDOWS_MEDIA_PLAYER};

/// `CLASS_E_CLASSNOTAVAILABLE` from `include/winerror.h`.
pub const CLASS_E_CLASSNOTAVAILABLE: u32 = 0x8004_0111;

/// The result of `DllGetClassObject`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClassObject {
    /// `rclsid` is not the WMP class. The C code returns `CLASS_E_CLASSNOTAVAILABLE` and does
    /// not write `*ppv`.
    NotAvailable,
    /// `rclsid` is the WMP class. Holds the HRESULT of `ClassFactory_QueryInterface`. `*ppv` is
    /// the factory on `S_OK` and NULL otherwise.
    Queried(u32),
}

impl ClassObject {
    /// The HRESULT the C function returns.
    pub fn hresult(self) -> u32 {
        match self {
            ClassObject::NotAvailable => CLASS_E_CLASSNOTAVAILABLE,
            ClassObject::Queried(hr) => hr,
        }
    }
}

/// `DllGetClassObject` from `dlls/wmp/wmp_main.c`.
pub fn dll_get_class_object(rclsid: &Guid, riid: &Guid) -> ClassObject {
    if is_equal_guid(&CLSID_WINDOWS_MEDIA_PLAYER, rclsid) {
        ClassObject::Queried(class_factory_query_interface(riid))
    } else {
        ClassObject::NotAvailable
    }
}
