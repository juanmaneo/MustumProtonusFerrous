/// `HRESULT` as a signed 32-bit value, matching the Wine reference.
pub type Hresult = i32;

/// `CLASS_E_CLASSNOTAVAILABLE` from `winerror.h`.
pub const CLASS_E_CLASSNOTAVAILABLE: Hresult = 0x8004_0111u32 as i32;

/// `DllGetClassObject` from `main.c`. The Wine body is a FIXME stub that
/// ignores its inputs and always returns `CLASS_E_CLASSNOTAVAILABLE`.
/// `clsid` and `riid` are 16-byte GUIDs (`None` for NULL); `out_present`
/// records whether the caller passed a non-NULL `obj`. The reference never
/// writes through `obj`.
pub fn dll_get_class_object(
    _clsid: Option<&[u8; 16]>,
    _riid: Option<&[u8; 16]>,
    _out_present: bool,
) -> Hresult {
    CLASS_E_CLASSNOTAVAILABLE
}
