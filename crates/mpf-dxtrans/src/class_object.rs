/// `CLASS_E_CLASSNOTAVAILABLE` from `winerror.h`.
pub const CLASS_E_CLASSNOTAVAILABLE: u32 = 0x8004_0111;

/// `DllGetClassObject` from `main.c`. The Wine body is a FIXME stub that
/// ignores its inputs and always returns `CLASS_E_CLASSNOTAVAILABLE`.
/// `clsid` and `riid` are 16-byte GUIDs (`None` for NULL); `out_present`
/// records whether the caller passed a non-NULL `out` pointer. The reference
/// never writes through `out`.
pub fn dll_get_class_object(
    _clsid: Option<&[u8; 16]>,
    _riid: Option<&[u8; 16]>,
    _out_present: bool,
) -> u32 {
    CLASS_E_CLASSNOTAVAILABLE
}
