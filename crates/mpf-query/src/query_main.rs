/// `HRESULT` from the Windows headers: a signed 32-bit value.
pub type HRESULT = i32;

/// `E_NOTIMPL` from `include/winerror.h`.
pub const E_NOTIMPL: HRESULT = 0x8000_4001_u32 as i32;

/// `CLASS_E_CLASSNOTAVAILABLE` from `include/winerror.h`.
pub const CLASS_E_CLASSNOTAVAILABLE: HRESULT = 0x8004_0111_u32 as i32;

/// `CI_E_NOT_RUNNING` from `include/cierror.h`.
pub const CI_E_NOT_RUNNING: HRESULT = 0x8004_1820_u32 as i32;

/// `DllGetClassObject` from `dlls/query/query_main.c`. Returns `CLASS_E_CLASSNOTAVAILABLE` and
/// writes nothing.
pub fn dll_get_class_object() -> HRESULT {
    CLASS_E_CLASSNOTAVAILABLE
}

/// `CIState` from `dlls/query/query_main.c`. Returns `CI_E_NOT_RUNNING` and writes nothing.
pub fn ci_state() -> HRESULT {
    CI_E_NOT_RUNNING
}

/// `LocateCatalogsA` from `dlls/query/query_main.c`. Returns `CI_E_NOT_RUNNING` and writes nothing.
pub fn locate_catalogs_a() -> HRESULT {
    CI_E_NOT_RUNNING
}

/// `LocateCatalogsW` from `dlls/query/query_main.c`. Returns `CI_E_NOT_RUNNING` and writes nothing.
pub fn locate_catalogs_w() -> HRESULT {
    CI_E_NOT_RUNNING
}

/// `LoadIFilter` from `dlls/query/query_main.c`. Sets `*ppIUnk` to NULL and returns `E_NOTIMPL`.
/// `out` is the `ppIUnk` out-parameter.
pub fn load_ifilter<T>(out: &mut Option<T>) -> HRESULT {
    *out = None;
    E_NOTIMPL
}
