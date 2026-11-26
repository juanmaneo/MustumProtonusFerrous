//! Port of `struct thumb_cache` and its `IThumbnailCache` methods in `dlls/thumbcache/thumbcache.c`.

use crate::guid::{Guid, IID_ITHUMBNAILCACHE, IID_IUNKNOWN};
use crate::hresult::{query_interface_hr, Hresult, E_NOTIMPL, E_OUTOFMEMORY, S_OK};

/// One `thumb_cache` object, reduced to its reference count.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct ThumbCache {
    pub(crate) ref_count: i32,
}

impl ThumbCache {
    /// `thumbcache_QueryInterface`. `S_OK` takes a reference; `E_NOINTERFACE` takes none.
    pub fn query_interface(&mut self, iid: &Guid) -> Hresult {
        let hr = query_interface_hr(iid, &[IID_IUNKNOWN, IID_ITHUMBNAILCACHE]);
        if hr == S_OK {
            self.add_ref();
        }
        hr
    }

    /// `thumbcache_AddRef`. The count wraps like `InterlockedIncrement`.
    pub fn add_ref(&mut self) -> u32 {
        self.ref_count = self.ref_count.wrapping_add(1);
        self.ref_count as u32
    }

    /// `thumbcache_Release`. C frees the object when the count reaches 0; the port only returns
    /// the count and leaves the drop to the caller.
    pub fn release(&mut self) -> u32 {
        self.ref_count = self.ref_count.wrapping_sub(1);
        self.ref_count as u32
    }

    /// `thumbcache_GetThumbnail`: not implemented, so `E_NOTIMPL`.
    pub fn get_thumbnail(&self) -> Hresult {
        E_NOTIMPL
    }

    /// `thumbcache_GetThumbnailByID`: not implemented, so `E_NOTIMPL`.
    pub fn get_thumbnail_by_id(&self) -> Hresult {
        E_NOTIMPL
    }
}

/// `thumbcache_create`. `allocate` stands in for `malloc`; `None` is the out-of-memory path.
/// The new object starts with a count of 1, `QueryInterface` runs for `iid`, and the creation
/// reference is released. The object comes back only on `S_OK`.
pub fn thumb_cache_create(
    iid: &Guid,
    allocate: &dyn Fn() -> Option<ThumbCache>,
) -> (Hresult, Option<ThumbCache>) {
    let Some(mut this) = allocate() else {
        return (E_OUTOFMEMORY, None);
    };
    this.ref_count = 1;
    let hr = this.query_interface(iid);
    this.release();
    if hr == S_OK {
        (hr, Some(this))
    } else {
        (hr, None)
    }
}
