//! Port of `struct EnumUserIdentity` and its `IEnumUserIdentity` methods in Wine's `dlls/msident/msident.c`.

use crate::guid::{is_equal_guid, Guid, IID_IENUM_USER_IDENTITY, IID_IUNKNOWN};
use crate::hresult::{E_NOINTERFACE, E_NOTIMPL, HRESULT, S_OK};
use crate::refcount::RefCount;

/// The enumerator that `UserIdentityManager_EnumIdentities` allocates, with `ref = 1`. The C
/// `EnumUserIdentity_Release` calls `free` when the count reaches zero; here the caller drops the value.
#[derive(Debug)]
pub struct EnumUserIdentity {
    ref_count: RefCount,
}

impl EnumUserIdentity {
    pub(crate) fn new() -> Self {
        Self {
            ref_count: RefCount::initial(),
        }
    }

    /// `EnumUserIdentity_QueryInterface`. `IUnknown` and `IEnumUserIdentity` add a reference.
    pub fn query_interface(&mut self, riid: &Guid) -> Result<(), HRESULT> {
        if is_equal_guid(&IID_IUNKNOWN, riid) || is_equal_guid(&IID_IENUM_USER_IDENTITY, riid) {
            self.add_ref();
            Ok(())
        } else {
            Err(E_NOINTERFACE)
        }
    }

    /// `EnumUserIdentity_AddRef`: returns the new count.
    pub fn add_ref(&mut self) -> u32 {
        self.ref_count.add_ref()
    }

    /// `EnumUserIdentity_Release`: returns the new count. Zero means the C code calls `free`,
    /// so the caller must drop the enumerator.
    pub fn release(&mut self) -> u32 {
        self.ref_count.release()
    }

    /// The `ref` field. This accessor is not a C function; the tests use it to read the count.
    pub fn ref_count(&self) -> i32 {
        self.ref_count.get()
    }

    /// `EnumUserIdentity_Next`: always `E_NOTIMPL`, and it writes no output.
    pub fn next(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `EnumUserIdentity_Skip`: always `E_NOTIMPL`.
    pub fn skip(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `EnumUserIdentity_Reset`: always `E_NOTIMPL`.
    pub fn reset(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `EnumUserIdentity_Clone`: always `E_NOTIMPL`, and it does not write `ppenum`.
    pub fn clone_enum(&self) -> HRESULT {
        E_NOTIMPL
    }

    /// `EnumUserIdentity_GetCount`: sets `count` to 0 and returns `S_OK`.
    pub fn get_count(&self, count: &mut u32) -> HRESULT {
        *count = 0;
        S_OK
    }
}
