use crate::guid::{is_equal_guid, Guid, IID_ICLASSFACTORY, IID_IUNKNOWN};

/// `S_OK` from `include/winerror.h`.
pub const S_OK: u32 = 0;

/// `E_NOINTERFACE` from `include/winerror.h`.
pub const E_NOINTERFACE: u32 = 0x8000_4002;

/// `ClassFactory_QueryInterface` from `dlls/wmp/wmp_main.c`, returning the HRESULT. The C code
/// sets `*ppv` to the factory on `S_OK` and to NULL on `E_NOINTERFACE`.
pub fn class_factory_query_interface(riid: &Guid) -> u32 {
    if is_equal_guid(&IID_IUNKNOWN, riid) || is_equal_guid(&IID_ICLASSFACTORY, riid) {
        S_OK
    } else {
        E_NOINTERFACE
    }
}

/// `ClassFactory_AddRef` from `dlls/wmp/wmp_main.c`. The factory is static, so the count is a
/// fixed 2.
pub fn class_factory_add_ref() -> u32 {
    2
}

/// `ClassFactory_Release` from `dlls/wmp/wmp_main.c`. The factory is static, so the count is a
/// fixed 1.
pub fn class_factory_release() -> u32 {
    1
}

/// `ClassFactory_LockServer` from `dlls/wmp/wmp_main.c`. It ignores the lock flag and returns
/// `S_OK`.
pub fn class_factory_lock_server(_lock: bool) -> u32 {
    S_OK
}
