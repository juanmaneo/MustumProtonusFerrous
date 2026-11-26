/// `HRESULT`, as a signed 32-bit value.
pub type Hresult = i32;

/// `CLASS_E_CLASSNOTAVAILABLE` from `include/winerror.h`.
pub const CLASS_E_CLASSNOTAVAILABLE: Hresult = 0x8004_0111u32 as i32;

/// A 16-byte GUID in memory order, which `REFCLSID` and `REFIID` point at.
pub type Guid = [u8; 16];

/// `DllGetClassObject` from `dlls/inkobj/inkobj.c`. The C body only traces `rclsid` and never
/// reads `riid`, so a NULL ID gets the same result as any other.
pub fn dll_get_class_object(_rclsid: Option<&Guid>, _riid: Option<&Guid>) -> Hresult {
    CLASS_E_CLASSNOTAVAILABLE
}
