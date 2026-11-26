//! Port of the portable logic in Wine's `dlls/chakra/chakra.c`, plus the winecrt0
//! defaults that `chakra.dll` links against (`libs/winecrt0/dll_canunload.c`,
//! `libs/winecrt0/register.c`, `libs/winecrt0/stub.c`).
//!
//! Ported:
//! - `DllGetClassObject` as [`dll_get_class_object`]: returns `CLASS_E_CLASSNOTAVAILABLE`
//!   and never writes the out-slot. The C body only logs `clsid` and `riid`, so the port does not take them.
//! - `DllCanUnloadNow` as [`dll_can_unload_now`]: `S_FALSE`. chakra.c has no body for it; winecrt0 supplies it.
//! - `DllRegisterServer` and `DllUnregisterServer` as [`dll_register_server`] and
//!   [`dll_unregister_server`]: `S_OK`. winecrt0 registers each `WINE_REGISTRY` resource.
//!   chakra.dll has none (its `Makefile.in` has no `RC_SRCS`), so the enumeration never calls back.
//! - The 156 `@ stub` exports of `chakra.spec`: [`is_stub_export`] lists them, and
//!   [`unimplemented_stub`] models `__wine_spec_unimplemented_stub`, which raises
//!   `EXCEPTION_WINE_STUB` non-continuably with the module and export names as arguments.
//!
//! Skipped:
//! - The `FIXME` and `WINE_DEFAULT_DEBUG_CHANNEL` logging, and `debugstr_guid`.
//! - The `Js*`, `MemProtectHeap*` and `RecyclerNativeHeap*` exports. Their bodies do not exist
//!   in the reference, and a stub never returns, so there is no behaviour beyond the raise.
//! - The raise itself. [`unimplemented_stub`] returns a [`StubRaise`] value and does not raise.
//! - The winecrt0 `register_resource` callback. It only runs for `WINE_REGISTRY` resources, which chakra.dll lacks.

mod exports;
mod stub;

pub use exports::{is_stub_export, STUB_EXPORTS};
pub use stub::{unimplemented_stub, StubRaise, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB};

/// The module name winebuild embeds in each stub (`-F chakra.dll`, see `tools/winebuild/main.c`).
pub const MODULE_NAME: &str = "chakra.dll";

/// `S_OK`, from `winerror.h`.
pub const S_OK: i32 = 0;
/// `S_FALSE`, from `winerror.h`.
pub const S_FALSE: i32 = 1;
/// `CLASS_E_CLASSNOTAVAILABLE`, from `winerror.h`.
pub const CLASS_E_CLASSNOTAVAILABLE: i32 = 0x8004_0111u32 as i32;

/// `DllGetClassObject`: returns `CLASS_E_CLASSNOTAVAILABLE` and leaves `out` unchanged, whether or not it is null.
pub fn dll_get_class_object<T>(_out: Option<&mut T>) -> i32 {
    CLASS_E_CLASSNOTAVAILABLE
}

/// `DllCanUnloadNow` from `libs/winecrt0/dll_canunload.c`: always `S_FALSE`.
pub fn dll_can_unload_now() -> i32 {
    S_FALSE
}

/// `DllRegisterServer` from `libs/winecrt0/register.c`. With no `WINE_REGISTRY` resources the result stays `S_OK`.
pub fn dll_register_server() -> i32 {
    S_OK
}

/// `DllUnregisterServer` from `libs/winecrt0/register.c`. Same as [`dll_register_server`].
pub fn dll_unregister_server() -> i32 {
    S_OK
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
