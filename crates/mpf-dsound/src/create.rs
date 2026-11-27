//! The control flow of the create entry points in dlls/dsound/dsound.c, capture.c and duplex.c.
//! The COM calls are passed in as closures: create allocates the object, initialize calls its
//! Initialize method, and release calls its Release method. The out parameter is None for a
//! NULL pointer, and Some(value) for a non-NULL pointer that holds value.

use crate::hresult::{
    Hresult, DSERR_ALREADYINITIALIZED, DSERR_INVALIDPARAM, DSERR_NOAGGREGATION, DS_OK,
};

/// DirectSoundCreate. An aggregated call clears the out parameter and returns
/// DSERR_INVALIDPARAM. Initialize may also return DSERR_ALREADYINITIALIZED, which counts as
/// success.
pub fn direct_sound_create<T>(
    out: Option<&mut Option<T>>,
    outer_unknown: bool,
    create: impl FnOnce() -> Result<T, Hresult>,
    initialize: impl FnOnce(&T) -> Hresult,
    release: impl FnOnce(T),
) -> Hresult {
    let Some(out) = out else {
        return DSERR_INVALIDPARAM;
    };
    if outer_unknown {
        *out = None;
        return DSERR_INVALIDPARAM;
    }
    let (obj, hr) = create_and_initialize(create, initialize, release, true);
    *out = obj;
    hr
}

/// DirectSoundCreate8. The control flow is the same as DirectSoundCreate. Only the object
/// type differs.
pub fn direct_sound_create8<T>(
    out: Option<&mut Option<T>>,
    outer_unknown: bool,
    create: impl FnOnce() -> Result<T, Hresult>,
    initialize: impl FnOnce(&T) -> Hresult,
    release: impl FnOnce(T),
) -> Hresult {
    direct_sound_create(out, outer_unknown, create, initialize, release)
}

/// DirectSoundCaptureCreate. An aggregated call returns DSERR_NOAGGREGATION and leaves the out
/// parameter untouched. Initialize must succeed; DSERR_ALREADYINITIALIZED is a failure here.
pub fn direct_sound_capture_create<T>(
    out: Option<&mut Option<T>>,
    outer_unknown: bool,
    create: impl FnOnce() -> Result<T, Hresult>,
    initialize: impl FnOnce(&T) -> Hresult,
    release: impl FnOnce(T),
) -> Hresult {
    let Some(out) = out else {
        return DSERR_INVALIDPARAM;
    };
    if outer_unknown {
        return DSERR_NOAGGREGATION;
    }
    let (obj, hr) = create_and_initialize(create, initialize, release, false);
    *out = obj;
    hr
}

/// DirectSoundCaptureCreate8. An aggregated call clears the out parameter and returns
/// DSERR_NOAGGREGATION. Initialize must succeed, as in DirectSoundCaptureCreate.
pub fn direct_sound_capture_create8<T>(
    out: Option<&mut Option<T>>,
    outer_unknown: bool,
    create: impl FnOnce() -> Result<T, Hresult>,
    initialize: impl FnOnce(&T) -> Hresult,
    release: impl FnOnce(T),
) -> Hresult {
    let Some(out) = out else {
        return DSERR_INVALIDPARAM;
    };
    if outer_unknown {
        *out = None;
        return DSERR_NOAGGREGATION;
    }
    let (obj, hr) = create_and_initialize(create, initialize, release, false);
    *out = obj;
    hr
}

/// DirectSoundFullDuplexCreate. The NULL, aggregation and Initialize rules are those of
/// DirectSoundCaptureCreate8.
pub fn direct_sound_full_duplex_create<T>(
    out: Option<&mut Option<T>>,
    outer_unknown: bool,
    create: impl FnOnce() -> Result<T, Hresult>,
    initialize: impl FnOnce(&T) -> Hresult,
    release: impl FnOnce(T),
) -> Hresult {
    direct_sound_capture_create8(out, outer_unknown, create, initialize, release)
}

/// Creates an object and initializes it. A failed create gives None. A failed Initialize
/// releases the object and gives None. DSERR_ALREADYINITIALIZED counts as success only when
/// tolerate_already is set.
fn create_and_initialize<T>(
    create: impl FnOnce() -> Result<T, Hresult>,
    initialize: impl FnOnce(&T) -> Hresult,
    release: impl FnOnce(T),
    tolerate_already: bool,
) -> (Option<T>, Hresult) {
    let obj = match create() {
        Ok(obj) => obj,
        Err(hr) => return (None, hr),
    };
    match initialize(&obj) {
        DS_OK => (Some(obj), DS_OK),
        DSERR_ALREADYINITIALIZED if tolerate_already => (Some(obj), DS_OK),
        hr => {
            release(obj);
            (None, hr)
        }
    }
}
