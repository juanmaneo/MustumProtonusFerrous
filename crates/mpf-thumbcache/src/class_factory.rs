//! Port of `struct class_factory` and the `class_factory_*` methods in `dlls/thumbcache/thumbcache.c`.

use crate::guid::{Guid, IID_ICLASSFACTORY, IID_IUNKNOWN};
use crate::hresult::{query_interface_hr, Hresult, S_OK};
use crate::thumb_cache::{thumb_cache_create, ThumbCache};

/// The `thumb_cache_cf` class factory. It keeps no state; its `create_instance` is always
/// [`thumb_cache_create`].
#[derive(Debug, PartialEq, Eq)]
pub struct ClassFactory;

impl ClassFactory {
    /// `class_factory_QueryInterface`.
    pub fn query_interface(&self, iid: &Guid) -> Hresult {
        query_interface_hr(iid, &[IID_IUNKNOWN, IID_ICLASSFACTORY])
    }

    /// `class_factory_AddRef`, which always returns 2.
    pub fn add_ref(&self) -> u32 {
        2
    }

    /// `class_factory_Release`, which always returns 1.
    pub fn release(&self) -> u32 {
        1
    }

    /// `class_factory_CreateInstance`, with `outer` left out.
    pub fn create_instance(
        &self,
        iid: &Guid,
        allocate: &dyn Fn() -> Option<ThumbCache>,
    ) -> (Hresult, Option<ThumbCache>) {
        thumb_cache_create(iid, allocate)
    }

    /// `class_factory_LockServer`. A stub that reports success and counts nothing.
    pub fn lock_server(&self, _lock: bool) -> Hresult {
        S_OK
    }
}
