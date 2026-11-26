/// `HRESULT` from `include/winnt.h`: a 32-bit signed value.
pub type HRESULT = i32;

/// `S_OK` from `include/winerror.h`.
pub const S_OK: HRESULT = 0;

/// `DllGetClassObject` from `dlls/olepro32/olepro32stubs.c`. The C body only traces
/// `FIXME("stub\n")` and returns `S_OK`.
pub fn dll_get_class_object() -> HRESULT {
    S_OK
}
