//! Edge checks for `mpf-dmstyle`: the `HRESULT` encodings, the derived trait impls, the GUID
//! dispatch of every track and operation, and the boundaries of `style_EnumMotif` that the oracle
//! rows do not reach. The accepted GUIDs in `accepted_guids_match_the_c_per_track` are read from the
//! verbatim `IsParamSupported`, `GetParam` and `SetParam` of each track.

use crate::oracle_checks::{filled, wide};
use crate::style::{enum_motif, Pattern, DMUS_MAX_NAME, DMUS_OBJ_NAME};
use crate::track::{
    get_param, is_param_supported, set_param, Track, CHORD_PARAM, COMMAND_PARAM, COMMAND_PARAM2,
    COMMAND_PARAM_NEXT, DISABLE_TIME_SIG, ENABLE_TIME_SIG, IDIRECT_MUSIC_STYLE, MUTE_PARAM,
    RHYTHM_PARAM, SEED_VARIATIONS, TIME_SIGNATURE, VALID_START_TIME, VARIATIONS,
};
use crate::{
    make_dmhresult_error, Guid, Hresult, DMUS_E_GET_UNSUPPORTED, DMUS_E_NOT_FOUND,
    DMUS_E_SET_UNSUPPORTED, DMUS_E_TYPE_UNSUPPORTED, E_POINTER, S_FALSE, S_OK,
};

fn all_guids() -> [Guid; 13] {
    [
        CHORD_PARAM,
        RHYTHM_PARAM,
        COMMAND_PARAM,
        COMMAND_PARAM2,
        COMMAND_PARAM_NEXT,
        MUTE_PARAM,
        VALID_START_TIME,
        DISABLE_TIME_SIG,
        ENABLE_TIME_SIG,
        SEED_VARIATIONS,
        TIME_SIGNATURE,
        IDIRECT_MUSIC_STYLE,
        VARIATIONS,
    ]
}

/// Clones through the trait, so the derived `Clone` runs without tripping `clone_on_copy`.
fn cloned<T: Clone>(value: &T) -> T {
    value.clone()
}

fn all_tracks() -> [Track; 7] {
    [
        Track::Audition,
        Track::Chord,
        Track::Command,
        Track::Motif,
        Track::Mute,
        Track::Style { has_items: false },
        Track::Style { has_items: true },
    ]
}

/// Checks `op` for `track` over every GUID of `all_guids`. The GUIDs in `accepted` must give `ok`,
/// and every other GUID must give `miss`.
fn check_accepts(
    op: fn(Track, Option<&Guid>) -> Hresult,
    track: Track,
    accepted: &[Guid],
    ok: Hresult,
    miss: Hresult,
) {
    for guid in all_guids() {
        let expected = if accepted.contains(&guid) { ok } else { miss };
        assert_eq!(op(track, Some(&guid)), expected);
    }
}

#[test]
fn hresult_values_match_the_c_headers() {
    assert_eq!(S_OK, 0);
    assert_eq!(S_FALSE, 1);
    assert_eq!(E_POINTER as u32, 0x8000_4003);
    assert_eq!(DMUS_E_SET_UNSUPPORTED as u32, 0x8878_1123);
    assert_eq!(DMUS_E_GET_UNSUPPORTED as u32, 0x8878_1124);
    assert_eq!(DMUS_E_NOT_FOUND as u32, 0x8878_1161);
    assert_eq!(DMUS_E_TYPE_UNSUPPORTED as u32, 0x8878_1164);
}

#[test]
fn make_dmhresult_error_sets_severity_facility_and_code() {
    assert_eq!(make_dmhresult_error(0) as u32, 0x8878_1000);
    assert_eq!(make_dmhresult_error(0x0123), DMUS_E_SET_UNSUPPORTED);
    assert_eq!(make_dmhresult_error(0x0161), DMUS_E_NOT_FOUND);
}

#[test]
fn style_name_constants_match_the_c_headers() {
    assert_eq!(DMUS_MAX_NAME, 0x40);
    assert_eq!(DMUS_OBJ_NAME, 4);
}

#[test]
fn guid_eq_compares_every_field() {
    let base = CHORD_PARAM;
    assert!(cloned(&base) == CHORD_PARAM);
    assert!(Guid { data1: 0, ..base } != CHORD_PARAM);
    assert!(Guid { data2: 0, ..base } != CHORD_PARAM);
    assert!(Guid { data3: 0, ..base } != CHORD_PARAM);
    assert!(
        Guid {
            data4: [0; 8],
            ..base
        } != CHORD_PARAM
    );
}

#[test]
fn guid_debug_lists_every_field() {
    let zero = Guid {
        data1: 0,
        data2: 0,
        data3: 0,
        data4: [0; 8],
    };
    assert_eq!(
        format!("{:?}", cloned(&zero)),
        "Guid { data1: 0, data2: 0, data3: 0, data4: [0, 0, 0, 0, 0, 0, 0, 0] }"
    );
}

#[test]
fn track_derived_impls_cover_every_variant() {
    for track in all_tracks() {
        assert_eq!(format!("{:?}", cloned(&track)), format!("{track:?}"));
    }
    assert_eq!(
        format!("{:?}", Track::Style { has_items: true }),
        "Style { has_items: true }"
    );
    assert_eq!(format!("{:?}", Track::Mute), "Mute");
}

#[test]
fn null_guid_is_e_pointer_for_every_track_and_operation() {
    for track in all_tracks() {
        assert_eq!(is_param_supported(track, None), E_POINTER);
        assert_eq!(get_param(track, None), E_POINTER);
        assert_eq!(set_param(track, None), E_POINTER);
    }
}

#[test]
fn accepted_guids_match_the_c_per_track() {
    check_accepts(
        is_param_supported,
        Track::Audition,
        &[
            DISABLE_TIME_SIG,
            ENABLE_TIME_SIG,
            SEED_VARIATIONS,
            VALID_START_TIME,
            VARIATIONS,
        ],
        S_OK,
        DMUS_E_TYPE_UNSUPPORTED,
    );
    check_accepts(
        is_param_supported,
        Track::Chord,
        &[CHORD_PARAM, RHYTHM_PARAM],
        S_OK,
        DMUS_E_TYPE_UNSUPPORTED,
    );
    check_accepts(
        is_param_supported,
        Track::Command,
        &[COMMAND_PARAM, COMMAND_PARAM2, COMMAND_PARAM_NEXT],
        S_OK,
        DMUS_E_TYPE_UNSUPPORTED,
    );
    check_accepts(
        is_param_supported,
        Track::Motif,
        &[
            DISABLE_TIME_SIG,
            ENABLE_TIME_SIG,
            SEED_VARIATIONS,
            VALID_START_TIME,
        ],
        S_OK,
        DMUS_E_TYPE_UNSUPPORTED,
    );
    check_accepts(
        is_param_supported,
        Track::Mute,
        &[MUTE_PARAM],
        S_OK,
        DMUS_E_TYPE_UNSUPPORTED,
    );
    check_accepts(
        is_param_supported,
        Track::Style { has_items: true },
        &[
            DISABLE_TIME_SIG,
            ENABLE_TIME_SIG,
            IDIRECT_MUSIC_STYLE,
            SEED_VARIATIONS,
            TIME_SIGNATURE,
        ],
        S_OK,
        DMUS_E_TYPE_UNSUPPORTED,
    );

    check_accepts(
        get_param,
        Track::Audition,
        &[VALID_START_TIME, VARIATIONS],
        S_OK,
        DMUS_E_GET_UNSUPPORTED,
    );
    check_accepts(
        get_param,
        Track::Chord,
        &[CHORD_PARAM, RHYTHM_PARAM],
        S_OK,
        DMUS_E_GET_UNSUPPORTED,
    );
    check_accepts(
        get_param,
        Track::Command,
        &[COMMAND_PARAM, COMMAND_PARAM2, COMMAND_PARAM_NEXT],
        S_OK,
        DMUS_E_GET_UNSUPPORTED,
    );
    check_accepts(
        get_param,
        Track::Motif,
        &[VALID_START_TIME],
        S_OK,
        DMUS_E_GET_UNSUPPORTED,
    );
    check_accepts(
        get_param,
        Track::Mute,
        &[MUTE_PARAM],
        S_OK,
        DMUS_E_TYPE_UNSUPPORTED,
    );
    check_accepts(
        get_param,
        Track::Style { has_items: true },
        &[IDIRECT_MUSIC_STYLE, TIME_SIGNATURE],
        S_OK,
        DMUS_E_GET_UNSUPPORTED,
    );

    check_accepts(
        set_param,
        Track::Audition,
        &[DISABLE_TIME_SIG, ENABLE_TIME_SIG, SEED_VARIATIONS],
        S_OK,
        DMUS_E_SET_UNSUPPORTED,
    );
    check_accepts(
        set_param,
        Track::Chord,
        &[CHORD_PARAM],
        S_OK,
        DMUS_E_SET_UNSUPPORTED,
    );
    check_accepts(
        set_param,
        Track::Command,
        &[COMMAND_PARAM, COMMAND_PARAM_NEXT],
        S_OK,
        DMUS_E_SET_UNSUPPORTED,
    );
    check_accepts(
        set_param,
        Track::Motif,
        &[DISABLE_TIME_SIG, ENABLE_TIME_SIG, SEED_VARIATIONS],
        S_OK,
        DMUS_E_SET_UNSUPPORTED,
    );
    check_accepts(
        set_param,
        Track::Mute,
        &[MUTE_PARAM],
        S_OK,
        DMUS_E_TYPE_UNSUPPORTED,
    );
    check_accepts(
        set_param,
        Track::Style { has_items: true },
        &[
            DISABLE_TIME_SIG,
            ENABLE_TIME_SIG,
            IDIRECT_MUSIC_STYLE,
            SEED_VARIATIONS,
        ],
        S_OK,
        DMUS_E_SET_UNSUPPORTED,
    );
}

#[test]
fn style_get_without_an_item_reports_not_found_for_the_style() {
    let track = Track::Style { has_items: false };
    assert_eq!(
        get_param(track, Some(&IDIRECT_MUSIC_STYLE)),
        DMUS_E_NOT_FOUND
    );
    assert_eq!(get_param(track, Some(&TIME_SIGNATURE)), S_OK);
    assert_eq!(get_param(track, Some(&VARIATIONS)), DMUS_E_GET_UNSUPPORTED);
}

#[test]
fn enum_motif_checks_the_buffer_before_the_index() {
    let name = wide("A");
    let patterns = [Pattern {
        valid_data: DMUS_OBJ_NAME,
        name: &name,
    }];
    assert_eq!(enum_motif(&patterns, 0, None), E_POINTER);
    assert_eq!(enum_motif(&patterns, 9, None), E_POINTER);
}

#[test]
fn enum_motif_past_the_end_leaves_the_buffer_alone() {
    let name = wide("A");
    let patterns = [Pattern {
        valid_data: DMUS_OBJ_NAME,
        name: &name,
    }];
    let mut buf = filled();
    assert_eq!(enum_motif(&[], 0, Some(&mut buf)), S_FALSE);
    assert_eq!(enum_motif(&patterns, 1, Some(&mut buf)), S_FALSE);
    assert_eq!(enum_motif(&patterns, u32::MAX, Some(&mut buf)), S_FALSE);
    assert_eq!(buf, filled());
}

#[test]
fn enum_motif_copies_the_selected_pattern() {
    let first = wide("First");
    let second = wide("Second");
    let patterns = [
        Pattern {
            valid_data: DMUS_OBJ_NAME,
            name: &first,
        },
        Pattern {
            valid_data: DMUS_OBJ_NAME,
            name: &second,
        },
    ];
    let mut buf = filled();
    assert_eq!(enum_motif(&patterns, 1, Some(&mut buf)), S_OK);
    assert_eq!(buf.iter().position(|&unit| unit == 0), Some(6));
    assert_eq!(&buf[..6], &wide("Second")[..6]);
}

#[test]
fn enum_motif_without_the_name_flag_ignores_the_name() {
    let name = wide("XYZ");
    let patterns = [Pattern {
        valid_data: 0x0001,
        name: &name,
    }];
    let mut buf = filled();
    assert_eq!(enum_motif(&patterns, 0, Some(&mut buf)), S_OK);
    assert_eq!(buf[0], 0);
    assert_eq!(&buf[1..], &filled()[1..]);
}

#[test]
fn enum_motif_stops_the_copy_at_an_embedded_nul() {
    let name = [u16::from(b'A'), u16::from(b'B'), 0, u16::from(b'C')];
    let patterns = [Pattern {
        valid_data: DMUS_OBJ_NAME,
        name: &name,
    }];
    let mut buf = filled();
    assert_eq!(enum_motif(&patterns, 0, Some(&mut buf)), S_OK);
    assert_eq!(
        &buf[..4],
        &[u16::from(b'A'), u16::from(b'B'), 0, u16::from(b'Z')]
    );
}

#[test]
fn enum_motif_short_name_without_nul_ends_at_the_slice() {
    let name = [u16::from(b'A'), u16::from(b'B')];
    let patterns = [Pattern {
        valid_data: DMUS_OBJ_NAME,
        name: &name,
    }];
    let mut buf = filled();
    assert_eq!(enum_motif(&patterns, 0, Some(&mut buf)), S_OK);
    assert_eq!(
        &buf[..4],
        &[u16::from(b'A'), u16::from(b'B'), 0, u16::from(b'Z')]
    );
}

#[test]
fn enum_motif_truncates_a_full_name_to_63_units() {
    let mut name = [u16::from(b'A'); DMUS_MAX_NAME];
    name[63] = u16::from(b'B');
    let patterns = [Pattern {
        valid_data: DMUS_OBJ_NAME,
        name: &name,
    }];
    let mut buf = filled();
    assert_eq!(enum_motif(&patterns, 0, Some(&mut buf)), S_OK);
    assert_eq!(buf.iter().position(|&unit| unit == 0), Some(63));
    assert_eq!(&buf[..63], &[u16::from(b'A'); 63]);
}
