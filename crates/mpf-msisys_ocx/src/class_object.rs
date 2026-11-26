//! `DllGetClassObject` from `dlls/msisys.ocx/msisys.c`.

/// `HRESULT`, as a signed 32-bit value so that a set severity bit makes it negative.
pub type Hresult = i32;

/// `CLASS_E_CLASSNOTAVAILABLE` from `include/winerror.h`.
pub const CLASS_E_CLASSNOTAVAILABLE: Hresult = 0x8004_0111u32 as i32;

/// `DllGetClassObject`. The DLL registers no classes, so every request gets
/// `CLASS_E_CLASSNOTAVAILABLE`.
pub fn dll_get_class_object() -> Hresult {
    CLASS_E_CLASSNOTAVAILABLE
}
