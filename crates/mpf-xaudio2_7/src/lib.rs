//! Port of the portable logic in Wine's `dlls/xaudio2_7/xaudio_dll.c`, `dlls/xaudio2_7/xapo.c`
//! and `dlls/xaudio2_7/xaudio2_7.spec`, built for XAUDIO2_VER 7.
//!
//! Ported:
//! - `DllGetClassObject` class selection: [`class_object`], [`ClassObject`], [`XapoClass`]
//! - The `pOuter` gate in `XAudio2CF_CreateInstance` and `xapocf_CreateInstance`:
//!   [`aggregation_gate`], [`xaudio2_create_instance_gate`]
//! - The class dispatch in `xapocf_CreateInstance`: [`xapo_create_instance_class`]
//! - `XAudio2CF_LockServer` and `xapocf_LockServer`: [`xaudio2_cf_lock_server`], [`xapo_cf_lock_server`]
//! - `IXAudio2Impl_QueryInterface`: [`xaudio2_query_interface`]; `XAPOFX_QueryInterface`:
//!   [`xapo_query_interface`], [`XapoInterface`]
//! - `IXAudio2Impl_RegisterForCallbacks`, `IXAudio2Impl_UnregisterForCallbacks` and the in-use
//!   check of `IXAudio2Impl_CreateMasteringVoice`: [`Engine`]
//! - `XAPOFX_CalcInputFrames` and `XAPOFX_CalcOutputFrames`: [`calc_input_frames`],
//!   [`calc_output_frames`]
//! - The return value of `XAPO_GetRegistrationProperties`: [`get_registration_properties_status`]
//! - The HRESULT, CLSID and IID values these functions use: [`hresult`] constants, [`Guid`] and
//!   the `CLSID_*` and `IID_*` constants
//!
//! Skipped:
//! - `DllCanUnloadNow`, `DllRegisterServer` and `DllUnregisterServer`: the tree has no C body for
//!   them; widl generates them from `xaudio_register.idl` (`#pragma makedep register`).
//! - A `@ stub` model: `xaudio2_7.spec` has no `@ stub` exports, only the four `-private` exports.
//! - `x3daudio.c`: compiles to nothing for XAUDIO2_VER 7.
//! - `xapofx.c` `CreateFX`: guarded by `XAUDIO2_VER >= 8` or `XAPOFX1_VER`, so it is not built here.
//! - `xaudio_allocator.c`: a CoTaskMem wrapper with no deterministic logic.
//! - Object construction (`volume_meter_create`, `reverb_create`), the `ppobj` out-parameter,
//!   the FAudio forwarders, `wrap_effect_chain` and `wrap_voice_sends`, COM vtables, critical
//!   sections, reference counts and `TRACE`/`FIXME` output.

mod class_object;
mod engine;
mod guid;
mod hresult;
mod interfaces;
mod xapo;

pub use class_object::{
    aggregation_gate, class_object, xapo_cf_lock_server, xapo_create_instance_class,
    xaudio2_cf_lock_server, xaudio2_create_instance_gate, ClassObject, XapoClass,
    CLSID_AUDIO_REVERB, CLSID_AUDIO_VOLUME_METER, CLSID_FX_REVERB, CLSID_XAUDIO2,
};
pub use engine::Engine;
pub use guid::{is_equal_guid, Guid};
pub use hresult::{
    failed, CLASS_E_CLASSNOTAVAILABLE, CLASS_E_NOAGGREGATION, E_INVALIDARG, E_NOINTERFACE,
    E_NOTIMPL, S_FALSE, S_OK, XAUDIO2_E_INVALID_CALL,
};
pub use interfaces::{
    xapo_query_interface, xaudio2_query_interface, XapoInterface, IID_IUNKNOWN, IID_IXAPO,
    IID_IXAPO27, IID_IXAPO27_PARAMETERS, IID_IXAPO_PARAMETERS, IID_IXAUDIO2,
};
pub use xapo::{calc_input_frames, calc_output_frames, get_registration_properties_status};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
