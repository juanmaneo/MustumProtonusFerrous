//! Port of the CLSID dispatch at the top of `DllGetClassObject` in Wine's `dlls/msdaps/main.c`.

use crate::com::{
    is_equal_guid, Guid, CLSID_WINE_ROWSET_PROXY, CLSID_WINE_ROWSET_SERVER, CLSID_WINE_ROW_PROXY,
    CLSID_WINE_ROW_SERVER,
};

/// The static class factory that `DllGetClassObject` stores in `*obj` for one of the four
/// `wine_*` coclasses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClassObject {
    RowServer,
    RowProxy,
    RowsetServer,
    RowsetProxy,
}

/// The CLSID checks in `DllGetClassObject`. `Some` means the C code returns `S_OK`. `None` means
/// the C code falls through to `msdaps_DllGetClassObject`, which is not ported.
///
/// The C code ignores `iid` on the `S_OK` path, so the port does not take it.
pub fn class_object_for_clsid(clsid: &Guid) -> Option<ClassObject> {
    if is_equal_guid(clsid, &CLSID_WINE_ROW_SERVER) {
        Some(ClassObject::RowServer)
    } else if is_equal_guid(clsid, &CLSID_WINE_ROW_PROXY) {
        Some(ClassObject::RowProxy)
    } else if is_equal_guid(clsid, &CLSID_WINE_ROWSET_SERVER) {
        Some(ClassObject::RowsetServer)
    } else if is_equal_guid(clsid, &CLSID_WINE_ROWSET_PROXY) {
        Some(ClassObject::RowsetProxy)
    } else {
        None
    }
}
