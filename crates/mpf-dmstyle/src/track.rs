//! The parameter dispatch of the DirectMusic track objects in `dlls/dmstyle`: the
//! `IsParamSupported`, `GetParam` and `SetParam` methods of the audition, chord, command, motif,
//! mute and style tracks. Each one maps a parameter GUID to a status.

use crate::{
    Guid, Hresult, DMUS_E_GET_UNSUPPORTED, DMUS_E_NOT_FOUND, DMUS_E_SET_UNSUPPORTED,
    DMUS_E_TYPE_UNSUPPORTED, E_POINTER, S_OK,
};

/// `GUID_ChordParam`.
pub const CHORD_PARAM: Guid = Guid {
    data1: 0xd2ac_289e,
    data2: 0xb39b,
    data3: 0x11d1,
    data4: [0x87, 0x04, 0x00, 0x60, 0x08, 0x93, 0xb1, 0xbd],
};

/// `GUID_RhythmParam`.
pub const RHYTHM_PARAM: Guid = Guid {
    data1: 0xd2ac_289f,
    data2: 0xb39b,
    data3: 0x11d1,
    data4: [0x87, 0x04, 0x00, 0x60, 0x08, 0x93, 0xb1, 0xbd],
};

/// `GUID_CommandParam`.
pub const COMMAND_PARAM: Guid = Guid {
    data1: 0xd2ac_289d,
    data2: 0xb39b,
    data3: 0x11d1,
    data4: [0x87, 0x04, 0x00, 0x60, 0x08, 0x93, 0xb1, 0xbd],
};

/// `GUID_CommandParam2`.
pub const COMMAND_PARAM2: Guid = Guid {
    data1: 0x28f9_7ef7,
    data2: 0x9538,
    data3: 0x11d2,
    data4: [0x97, 0xa9, 0x00, 0xc0, 0x4f, 0xa3, 0x6e, 0x58],
};

/// `GUID_CommandParamNext`.
pub const COMMAND_PARAM_NEXT: Guid = Guid {
    data1: 0x472a_fe7a,
    data2: 0x281b,
    data3: 0x11d3,
    data4: [0x81, 0x7d, 0x00, 0xc0, 0x4f, 0xa3, 0x6e, 0x58],
};

/// `GUID_MuteParam`.
pub const MUTE_PARAM: Guid = Guid {
    data1: 0xd2ac_28af,
    data2: 0xb39b,
    data3: 0x11d1,
    data4: [0x87, 0x04, 0x00, 0x60, 0x08, 0x93, 0xb1, 0xbd],
};

/// `GUID_Valid_Start_Time`.
pub const VALID_START_TIME: Guid = Guid {
    data1: 0x7f6b_1760,
    data2: 0x1fdb,
    data3: 0x11d3,
    data4: [0x82, 0x26, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00],
};

/// `GUID_DisableTimeSig`.
pub const DISABLE_TIME_SIG: Guid = Guid {
    data1: 0x45fc_707b,
    data2: 0x1db4,
    data3: 0x11d2,
    data4: [0xbc, 0xac, 0x00, 0xa0, 0xc9, 0x22, 0xe6, 0xeb],
};

/// `GUID_EnableTimeSig`.
pub const ENABLE_TIME_SIG: Guid = Guid {
    data1: 0x45fc_707c,
    data2: 0x1db4,
    data3: 0x11d2,
    data4: [0xbc, 0xac, 0x00, 0xa0, 0xc9, 0x22, 0xe6, 0xeb],
};

/// `GUID_SeedVariations`.
pub const SEED_VARIATIONS: Guid = Guid {
    data1: 0x65b7_6fa5,
    data2: 0xff37,
    data3: 0x11d2,
    data4: [0x81, 0x4e, 0x00, 0xc0, 0x4f, 0xa3, 0x6e, 0x58],
};

/// `GUID_TimeSignature`.
pub const TIME_SIGNATURE: Guid = Guid {
    data1: 0xd2ac_28a4,
    data2: 0xb39b,
    data3: 0x11d1,
    data4: [0x87, 0x04, 0x00, 0x60, 0x08, 0x93, 0xb1, 0xbd],
};

/// `GUID_IDirectMusicStyle`.
pub const IDIRECT_MUSIC_STYLE: Guid = Guid {
    data1: 0xd2ac_28a1,
    data2: 0xb39b,
    data3: 0x11d1,
    data4: [0x87, 0x04, 0x00, 0x60, 0x08, 0x93, 0xb1, 0xbd],
};

/// `GUID_Variations`.
pub const VARIATIONS: Guid = Guid {
    data1: 0x11f7_2cce,
    data2: 0x26e6,
    data3: 0x4ecd,
    data4: [0xaf, 0x2e, 0xd6, 0x68, 0xe6, 0x67, 0x07, 0xd8],
};

/// The track objects whose parameter dispatch is ported. `Style` carries whether the track holds a
/// style item, which decides the status of `GetParam` for `GUID_IDirectMusicStyle`.
#[derive(Clone, Copy, Debug)]
pub enum Track {
    Audition,
    Chord,
    Command,
    Motif,
    Mute,
    Style { has_items: bool },
}

/// `IDirectMusicTrack8::IsParamSupported` of `track`. A missing GUID is `E_POINTER`, and a GUID
/// the track does not accept is `DMUS_E_TYPE_UNSUPPORTED`.
pub fn is_param_supported(track: Track, guid: Option<&Guid>) -> Hresult {
    let Some(guid) = guid else {
        return E_POINTER;
    };
    match track {
        Track::Audition => listed(
            guid,
            &[
                &DISABLE_TIME_SIG,
                &ENABLE_TIME_SIG,
                &SEED_VARIATIONS,
                &VALID_START_TIME,
                &VARIATIONS,
            ],
            DMUS_E_TYPE_UNSUPPORTED,
        ),
        Track::Chord => listed(
            guid,
            &[&CHORD_PARAM, &RHYTHM_PARAM],
            DMUS_E_TYPE_UNSUPPORTED,
        ),
        Track::Command => listed(
            guid,
            &[&COMMAND_PARAM, &COMMAND_PARAM2, &COMMAND_PARAM_NEXT],
            DMUS_E_TYPE_UNSUPPORTED,
        ),
        Track::Motif => listed(
            guid,
            &[
                &DISABLE_TIME_SIG,
                &ENABLE_TIME_SIG,
                &SEED_VARIATIONS,
                &VALID_START_TIME,
            ],
            DMUS_E_TYPE_UNSUPPORTED,
        ),
        Track::Mute => listed(guid, &[&MUTE_PARAM], DMUS_E_TYPE_UNSUPPORTED),
        Track::Style { .. } => listed(
            guid,
            &[
                &DISABLE_TIME_SIG,
                &ENABLE_TIME_SIG,
                &IDIRECT_MUSIC_STYLE,
                &SEED_VARIATIONS,
                &TIME_SIGNATURE,
            ],
            DMUS_E_TYPE_UNSUPPORTED,
        ),
    }
}

/// `IDirectMusicTrack8::GetParam` of `track`, reduced to its status. A missing GUID is
/// `E_POINTER`. For a style track, `GUID_IDirectMusicStyle` is `S_OK` when the track holds an item
/// and `DMUS_E_NOT_FOUND` when it does not.
pub fn get_param(track: Track, guid: Option<&Guid>) -> Hresult {
    let Some(guid) = guid else {
        return E_POINTER;
    };
    match track {
        Track::Audition => listed(
            guid,
            &[&VALID_START_TIME, &VARIATIONS],
            DMUS_E_GET_UNSUPPORTED,
        ),
        Track::Chord => listed(guid, &[&CHORD_PARAM, &RHYTHM_PARAM], DMUS_E_GET_UNSUPPORTED),
        Track::Command => listed(
            guid,
            &[&COMMAND_PARAM, &COMMAND_PARAM2, &COMMAND_PARAM_NEXT],
            DMUS_E_GET_UNSUPPORTED,
        ),
        Track::Motif => listed(guid, &[&VALID_START_TIME], DMUS_E_GET_UNSUPPORTED),
        Track::Mute => listed(guid, &[&MUTE_PARAM], DMUS_E_TYPE_UNSUPPORTED),
        Track::Style { has_items } => {
            if *guid == IDIRECT_MUSIC_STYLE {
                if has_items {
                    S_OK
                } else {
                    DMUS_E_NOT_FOUND
                }
            } else {
                listed(guid, &[&TIME_SIGNATURE], DMUS_E_GET_UNSUPPORTED)
            }
        }
    }
}

/// `IDirectMusicTrack8::SetParam` of `track`, reduced to its status. A missing GUID is
/// `E_POINTER`.
pub fn set_param(track: Track, guid: Option<&Guid>) -> Hresult {
    let Some(guid) = guid else {
        return E_POINTER;
    };
    match track {
        Track::Audition => listed(
            guid,
            &[&DISABLE_TIME_SIG, &ENABLE_TIME_SIG, &SEED_VARIATIONS],
            DMUS_E_SET_UNSUPPORTED,
        ),
        Track::Chord => listed(guid, &[&CHORD_PARAM], DMUS_E_SET_UNSUPPORTED),
        Track::Command => listed(
            guid,
            &[&COMMAND_PARAM, &COMMAND_PARAM_NEXT],
            DMUS_E_SET_UNSUPPORTED,
        ),
        Track::Motif => listed(
            guid,
            &[&DISABLE_TIME_SIG, &ENABLE_TIME_SIG, &SEED_VARIATIONS],
            DMUS_E_SET_UNSUPPORTED,
        ),
        Track::Mute => listed(guid, &[&MUTE_PARAM], DMUS_E_TYPE_UNSUPPORTED),
        Track::Style { .. } => listed(
            guid,
            &[
                &DISABLE_TIME_SIG,
                &ENABLE_TIME_SIG,
                &IDIRECT_MUSIC_STYLE,
                &SEED_VARIATIONS,
            ],
            DMUS_E_SET_UNSUPPORTED,
        ),
    }
}

/// `S_OK` when `guid` is one of `params`, and `miss` otherwise.
fn listed(guid: &Guid, params: &[&Guid], miss: Hresult) -> Hresult {
    if params.contains(&guid) {
        S_OK
    } else {
        miss
    }
}
