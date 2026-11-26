//! Port of `DllGetClassObject` in `dlls/netcfgx/netcfg.c`.

use crate::class_factory::ClassFactory;
use crate::com::{is_equal_guid, Guid, HResult, CLASS_E_CLASSNOTAVAILABLE, CLSID_CNETCFG};
use crate::netcfg::Iface;

/// `DllGetClassObject`, the `@ stdcall -private` export. Only `CLSID_CNetCfg` has a class object;
/// any other CLSID fails with `CLASS_E_CLASSNOTAVAILABLE` and leaves `ppv` unchanged.
pub fn dll_get_class_object(rclsid: &Guid, riid: &Guid, ppv: &mut Option<Iface>) -> HResult {
    if !is_equal_guid(rclsid, &CLSID_CNETCFG) {
        return CLASS_E_CLASSNOTAVAILABLE;
    }
    ClassFactory.query_interface(riid, ppv)
}
