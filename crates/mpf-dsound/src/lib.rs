//! Port of the portable logic in Wine's dlls/dsound/dsound.spec and in dlls/dsound/dsound.c,
//! dsound_main.c, capture.c, duplex.c, mixer.c, primary.c and sound3d.c.
//!
//! Ported:
//! - dsound.spec export table: [`EXPORTS`], [`ExportKind`], [`export`], [`export_by_ordinal`]
//! - The create control flow of DirectSoundCreate and DirectSoundCreate8: [`direct_sound_create`],
//!   [`direct_sound_create8`]
//! - The control flow of DirectSoundCaptureCreate, DirectSoundCaptureCreate8 and
//!   DirectSoundFullDuplexCreate: [`direct_sound_capture_create`],
//!   [`direct_sound_capture_create8`], [`direct_sound_full_duplex_create`]
//! - DllGetClassObject and the DSOUND_CF table: [`dll_get_class_object`], [`CLASS_FACTORIES`]
//! - DSOUND_RecalcVolPan, DSOUND_AmpFactorToVolPan and DSOUND_BufPtrDiff: [`recalc_vol_pan`],
//!   [`amp_factor_to_vol_pan`], [`buf_ptr_diff`], [`VolumePan`]
//! - The sample readers get8, get16, get24, get32 and getieee32 from mixer.c
//! - The 3D vector helpers from sound3d.c: [`Vector`] and the functions beside it
//! - speaker_config_to_channel_mask from primary.c: [`speaker_config_to_channel_mask`]
//! - The HRESULT values from winerror.h and dsound.h, and MAKE_DSHRESULT
//!
//! Skipped:
//! - DirectSoundEnumerateA and W, DirectSoundCaptureEnumerateA and W, and GetDeviceID: they
//!   read the audio device list.
//! - The COM objects and their vtables, and the DSOUND_Create, DSOUND_Create8,
//!   DSOUND_CaptureCreate, DSOUND_CaptureCreate8 and DSOUND_FullDuplexCreate helpers. They
//!   allocate objects and call the driver. The create functions here take those steps as
//!   closures.
//! - DllCanUnloadNow, DllRegisterServer and DllUnregisterServer. dlls/dsound has no C body
//!   for them, so they are listed in [`EXPORTS`] only.
//! - The AddRef call in DllGetClassObject, and the CreateInstance pointer of each class factory.
//! - Mixing, resampling, get_raw and DSOUND_Calc3DBuffer: they need buffer state and the driver.
//! - IKsPropertySet (propset.c), and the E_NOTIMPL return in DirectSoundDevice_CreateSoundBuffer
//!   (dsound.c), which is one branch of a larger function.
//! - The compare-and-swap retry loop in capped_refcount_dec (buffer.c), which needs concurrent
//!   writers.
//! - The clamp of the volume to -10000 in DSOUND_AmpFactorToVolPan. It cannot fire, see
//!   [`amp_factor_to_vol_pan`].

mod class_object;
mod create;
mod exports;
mod hresult;
mod mixer;
mod speaker;
mod vector;

pub use class_object::{
    dll_get_class_object, ClassFactory, Guid, CLASS_FACTORIES, IID_CLASS_FACTORY, IID_UNKNOWN,
};
pub use create::{
    direct_sound_capture_create, direct_sound_capture_create8, direct_sound_create,
    direct_sound_create8, direct_sound_full_duplex_create,
};
pub use exports::{export, export_by_ordinal, Export, ExportKind, EXPORTS};
pub use hresult::{
    make_dshresult, make_hresult, Hresult, CLASS_E_CLASSNOTAVAILABLE, CLASS_E_NOAGGREGATION,
    DSERR_ALREADYINITIALIZED, DSERR_INVALIDPARAM, DSERR_NOAGGREGATION, DSERR_OUTOFMEMORY, DS_OK,
    E_FAIL, E_INVALIDARG, E_NOINTERFACE, E_OUTOFMEMORY, S_OK,
};
pub use mixer::{
    amp_factor_to_vol_pan, buf_ptr_diff, get16, get24, get32, get8, getieee32, recalc_vol_pan,
    VolumePan, DSBVOLUME_MIN, DS_MAX_CHANNELS,
};
pub use speaker::{
    speaker_config_to_channel_mask, DSSPEAKER_5POINT1_BACK, DSSPEAKER_7POINT1_SURROUND,
    DSSPEAKER_7POINT1_WIDE, DSSPEAKER_HEADPHONE, DSSPEAKER_MONO, DSSPEAKER_QUAD, DSSPEAKER_STEREO,
    DSSPEAKER_SURROUND, KSAUDIO_SPEAKER_5POINT1,
};
pub use vector::{
    angle_between_vectors_deg, angle_between_vectors_rad, project_vector, rad_to_deg,
    scalar_product, vector_between_two_points, vector_magnitude, vector_product, Vector,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
