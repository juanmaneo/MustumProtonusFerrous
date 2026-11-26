use crate::com::{failed, Guid, E_FAIL, E_NOTIMPL, HRESULT, S_FALSE, S_OK};
use crate::ole_object::{self, OleInterface, Package, ReleaseOutcome};

/// `PersistStorage_QueryInterface`, which delegates to `OleObject_QueryInterface`.
pub fn query_interface(pkg: &mut Package, riid: &Guid) -> Result<OleInterface, HRESULT> {
    ole_object::query_interface(pkg, riid)
}

/// `PersistStorage_AddRef`, which delegates to `OleObject_AddRef`.
pub fn add_ref(pkg: &mut Package) -> i32 {
    ole_object::add_ref(pkg)
}

/// `PersistStorage_Release`, which delegates to `OleObject_Release`.
pub fn release(pkg: &mut Package) -> ReleaseOutcome {
    ole_object::release(pkg)
}

/// `PersistStorage_GetClassID`, which is not implemented.
pub fn get_class_id() -> HRESULT {
    E_NOTIMPL
}

/// `PersistStorage_IsDirty`, which is not implemented.
pub fn is_dirty() -> HRESULT {
    E_NOTIMPL
}

/// `PersistStorage_InitNew`, which is not implemented.
pub fn init_new() -> HRESULT {
    E_NOTIMPL
}

/// `PersistStorage_Save`, which is not implemented.
pub fn save() -> HRESULT {
    E_NOTIMPL
}

/// `PersistStorage_SaveCompleted`, which is not implemented.
pub fn save_completed() -> HRESULT {
    E_NOTIMPL
}

/// `PersistStorage_HandsOffStorage`, which is not implemented.
pub fn hands_off_storage() -> HRESULT {
    E_NOTIMPL
}

/// `discard_string`. Reads one byte at a time with `read` until a NUL byte is read. `read`
/// returns the `IStream_Read` result and the number of bytes read. A failed read, or a read that
/// returns no bytes, ends the loop: `S_OK` and `S_FALSE` become `E_FAIL`, and any other
/// `HRESULT` is returned unchanged.
pub fn discard_string<R>(mut read: R) -> HRESULT
where
    R: FnMut(&mut u8) -> (HRESULT, u32),
{
    let mut chr = 0u8;
    loop {
        let (hr, nbytes) = read(&mut chr);
        if failed(hr) || nbytes == 0 {
            return if hr == S_OK || hr == S_FALSE {
                E_FAIL
            } else {
                hr
            };
        }
        if chr == 0 {
            return S_OK;
        }
    }
}
