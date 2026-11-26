//! Port of the portable logic in Wine's `dlls/gameinput/gameinput.c`.
//!
//! Ported:
//! - `GameInputCreate` as [`game_input_create`]: a stub that returns `E_NOTIMPL`
//!   and never writes the out-slot.
//! - `DllGetClassObject` as [`dll_get_class_object`]: a stub that returns
//!   `CLASS_E_CLASSNOTAVAILABLE` and never writes the out-slot. The C body never
//!   reads `clsid` or `riid` except for the `FIXME` log, so the port does not take them.
//! - The two HRESULT values: [`E_NOTIMPL`] and [`CLASS_E_CLASSNOTAVAILABLE`].
//!
//! Skipped:
//! - `DllCanUnloadNow`: listed in `gameinput.spec` as private, but it has no body
//!   in `gameinput.c`, so there is no behaviour to port.
//! - The `FIXME` and `WINE_DEFAULT_DEBUG_CHANNEL` logging, and `debugstr_guid`.
//! - The `v0_IGameInput` COM object: `GameInputCreate` always fails, so none is created.

/// `E_NOTIMPL`, from `winerror.h`.
pub const E_NOTIMPL: i32 = 0x8000_4001u32 as i32;
/// `CLASS_E_CLASSNOTAVAILABLE`, from `winerror.h`.
pub const CLASS_E_CLASSNOTAVAILABLE: i32 = 0x8004_0111u32 as i32;

/// `GameInputCreate`: returns `E_NOTIMPL` and leaves `out` unchanged, whether or not it is null.
pub fn game_input_create<T>(_out: Option<&mut T>) -> i32 {
    E_NOTIMPL
}

/// `DllGetClassObject`: returns `CLASS_E_CLASSNOTAVAILABLE` and leaves `out` unchanged.
pub fn dll_get_class_object<T>(_out: Option<&mut T>) -> i32 {
    CLASS_E_CLASSNOTAVAILABLE
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
