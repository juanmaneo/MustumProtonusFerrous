//! Port of the portable logic in Wine's `dlls/xinput9_1_0/main.c` and `dlls/xinput9_1_0/xinput9_1_0.spec`.
//!
//! Ported:
//! - `XInputGetDSoundAudioDeviceGuids`: the index and pointer checks, the `XInputGetState` call
//!   it makes, and the NULL GUID writes: [`Xinput9::get_dsound_audio_device_guids`]
//! - `XInputGetCapabilities`, `XInputGetState` and `XInputSetState`: the forwarding to xinput1_4,
//!   and `ERROR_DEVICE_NOT_CONNECTED` when the forwarded pointer is NULL: [`Xinput9::get_capabilities`],
//!   [`Xinput9::get_state`], [`Xinput9::set_state`]
//! - The xinput1_4 pointers loaded once, as `InitOnceExecuteOnce` does: [`Xinput9::new`]
//! - `xinput9_1_0.spec` export table: [`EXPORTS`], [`is_export`]
//!
//! Modelled:
//! - xinput1_4 is an injected [`Xinput14Funcs`]. The loader closure passed to [`Xinput9::new`]
//!   stands in for `init_xinput1_4_funcs`, and each pointer is `None` where the C code leaves it NULL.
//!
//! Skipped:
//! - `DllMain`: it is not a spec export, returns `TRUE` for every reason, and its only other
//!   effect is the lifetime of the xinput1_4 module handle.
//! - `LoadLibraryExW` and `GetProcAddress`: the OS search for `xinput1_4.dll` and the per-name
//!   lookup. The loader closure replaces them.
//! - The `TRACE` and `ERR` messages and `debugstr_guid`.
//! - The `WINAPI` and `DECLSPEC_HOTPATCH` linkage. The spec has no `@ stub` exports, so there is no
//!   `__wine_spec_unimplemented_stub` model.

mod exports;
mod xinput;

pub use exports::{is_export, EXPORTS};
pub use xinput::{
    Capabilities, GetCapabilitiesFn, GetStateFn, Guid, SetStateFn, State, Vibration, Xinput14Funcs,
    Xinput9, ERROR_BAD_ARGUMENTS, ERROR_DEVICE_NOT_CONNECTED, ERROR_SUCCESS, GUID_NULL,
    XUSER_MAX_COUNT,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
