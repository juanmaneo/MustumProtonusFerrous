//! Port of `DllGetClassObject` in `dlls/thumbcache/thumbcache.c`.

use crate::class_factory::ClassFactory;
use crate::guid::{Guid, CLSID_LOCAL_THUMBNAIL_CACHE};
use crate::hresult::{Hresult, CLASS_E_CLASSNOTAVAILABLE, S_OK};

/// `DllGetClassObject`. Only `CLSID_LocalThumbnailCache` is served, through the class factory.
/// Any other CLSID fails with `CLASS_E_CLASSNOTAVAILABLE`.
pub fn dll_get_class_object(clsid: &Guid, iid: &Guid) -> (Hresult, Option<ClassFactory>) {
    if *clsid != CLSID_LOCAL_THUMBNAIL_CACHE {
        return (CLASS_E_CLASSNOTAVAILABLE, None);
    }
    let factory = ClassFactory;
    let hr = factory.query_interface(iid);
    if hr == S_OK {
        (hr, Some(factory))
    } else {
        (hr, None)
    }
}
