/// `S_FALSE` from `include/winerror.h`.
pub const S_FALSE: u32 = 1;

/// `CLASS_E_CLASSNOTAVAILABLE` from `include/winerror.h`.
pub const CLASS_E_CLASSNOTAVAILABLE: u32 = 0x8004_0111;

/// `DllCanUnloadNow` from `libs/winecrt0/dll_canunload.c`, linked into every Wine DLL.
pub fn dll_can_unload_now() -> u32 {
    S_FALSE
}

/// `DllGetClassObject` from `dlls/pstorec/pstorec.c`. The C body only traces its arguments and
/// never registers a class, so every call returns `CLASS_E_CLASSNOTAVAILABLE`.
pub fn dll_get_class_object() -> u32 {
    CLASS_E_CLASSNOTAVAILABLE
}
