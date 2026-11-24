//! Replays of the oracle rows R01 to R25 in `target/canon/dmstyle`. The oracle compiles the verbatim
//! `dlls/dmstyle` functions with `gcc -fsanitize=address,undefined -g`, with the Win32 shims in
//! `mock.h`, and `oracle_main.c` prints one line per row. Each test replays the rows named in its
//! doc comment, and its expected values are the ones the C printed. No row trips the sanitizers.
//! The enumeration rows print the first 16 name units, with NUL shown as '.'.

use crate::style::{enum_motif, Pattern, DMUS_MAX_NAME, DMUS_OBJ_NAME};
use crate::track::{
    get_param, is_param_supported, set_param, Track, COMMAND_PARAM2, DISABLE_TIME_SIG,
    IDIRECT_MUSIC_STYLE, MUTE_PARAM, RHYTHM_PARAM, SEED_VARIATIONS, TIME_SIGNATURE,
    VALID_START_TIME, VARIATIONS,
};

/// A name buffer of `DMUS_MAX_NAME` units that holds the ASCII `text`, then NUL units.
pub(crate) fn wide(text: &str) -> [u16; DMUS_MAX_NAME] {
    let mut units = [0; DMUS_MAX_NAME];
    for (unit, byte) in units.iter_mut().zip(text.bytes()) {
        *unit = u16::from(byte);
    }
    units
}

/// The name buffer that the oracle fills with 'Z' before each enumeration.
pub(crate) fn filled() -> [u16; DMUS_MAX_NAME] {
    [u16::from(b'Z'); DMUS_MAX_NAME]
}

/// The first 16 units of `buf`, with NUL shown as '.', as the oracle prints them.
fn render(buf: &[u16]) -> String {
    buf.iter()
        .take(16)
        .map(|&unit| {
            if unit == 0 {
                '.'
            } else {
                char::from(unit as u8)
            }
        })
        .collect()
}

/// Rows R01 to R04: the command track.
#[test]
fn command_track_rows() {
    assert_eq!(
        get_param(Track::Command, Some(&COMMAND_PARAM2)) as u32,
        0x0000_0000
    );
    assert_eq!(
        set_param(Track::Command, Some(&COMMAND_PARAM2)) as u32,
        0x8878_1123
    );
    assert_eq!(is_param_supported(Track::Command, None) as u32, 0x8000_4003);
    assert_eq!(
        is_param_supported(Track::Command, Some(&VALID_START_TIME)) as u32,
        0x8878_1164
    );
}

/// Rows R05 to R07: the chord track.
#[test]
fn chord_track_rows() {
    assert_eq!(
        get_param(Track::Chord, Some(&RHYTHM_PARAM)) as u32,
        0x0000_0000
    );
    assert_eq!(
        set_param(Track::Chord, Some(&RHYTHM_PARAM)) as u32,
        0x8878_1123
    );
    assert_eq!(
        is_param_supported(Track::Chord, Some(&TIME_SIGNATURE)) as u32,
        0x8878_1164
    );
}

/// Rows R08 and R09: the mute track.
#[test]
fn mute_track_rows() {
    assert_eq!(
        get_param(Track::Mute, Some(&MUTE_PARAM)) as u32,
        0x0000_0000
    );
    assert_eq!(
        set_param(Track::Mute, Some(&VALID_START_TIME)) as u32,
        0x8878_1164
    );
}

/// Rows R10 to R13: the motif track.
#[test]
fn motif_track_rows() {
    assert_eq!(
        get_param(Track::Motif, Some(&VALID_START_TIME)) as u32,
        0x0000_0000
    );
    assert_eq!(
        get_param(Track::Motif, Some(&DISABLE_TIME_SIG)) as u32,
        0x8878_1124
    );
    assert_eq!(
        set_param(Track::Motif, Some(&SEED_VARIATIONS)) as u32,
        0x0000_0000
    );
    assert_eq!(
        is_param_supported(Track::Motif, Some(&VALID_START_TIME)) as u32,
        0x0000_0000
    );
}

/// Rows R14 to R17: the style track, with and without a style item.
#[test]
fn style_track_rows() {
    let with_item = Track::Style { has_items: true };
    let without_item = Track::Style { has_items: false };
    assert_eq!(
        get_param(with_item, Some(&IDIRECT_MUSIC_STYLE)) as u32,
        0x0000_0000
    );
    assert_eq!(
        get_param(without_item, Some(&IDIRECT_MUSIC_STYLE)) as u32,
        0x8878_1161
    );
    assert_eq!(
        set_param(without_item, Some(&IDIRECT_MUSIC_STYLE)) as u32,
        0x0000_0000
    );
    assert_eq!(
        is_param_supported(without_item, Some(&VARIATIONS)) as u32,
        0x8878_1164
    );
}

/// Rows R18 to R20: the audition track.
#[test]
fn audition_track_rows() {
    assert_eq!(
        get_param(Track::Audition, Some(&VARIATIONS)) as u32,
        0x0000_0000
    );
    assert_eq!(
        set_param(Track::Audition, Some(&VARIATIONS)) as u32,
        0x8878_1123
    );
    assert_eq!(
        is_param_supported(Track::Audition, Some(&VALID_START_TIME)) as u32,
        0x0000_0000
    );
}

/// Rows R21 to R25: `style_EnumMotif`.
#[test]
fn enum_motif_rows() {
    let motif_a = wide("Motif A");
    let named = [Pattern {
        valid_data: DMUS_OBJ_NAME,
        name: &motif_a,
    }];

    let mut buf = filled();
    assert_eq!(enum_motif(&named, 0, Some(&mut buf)) as u32, 0x0000_0000);
    assert_eq!(buf.iter().position(|&unit| unit == 0), Some(7));
    assert_eq!(render(&buf), "Motif A.ZZZZZZZZ");

    let empty = wide("");
    let unnamed = [
        Pattern {
            valid_data: 0,
            name: &motif_a,
        },
        Pattern {
            valid_data: 0,
            name: &empty,
        },
    ];
    let mut buf = filled();
    assert_eq!(enum_motif(&unnamed, 1, Some(&mut buf)) as u32, 0x0000_0000);
    assert_eq!(buf.iter().position(|&unit| unit == 0), Some(0));
    assert_eq!(render(&buf), ".ZZZZZZZZZZZZZZZ");

    let mut buf = filled();
    assert_eq!(enum_motif(&unnamed, 2, Some(&mut buf)) as u32, 0x0000_0001);
    assert_eq!(buf.iter().position(|&unit| unit == 0), None);
    assert_eq!(render(&buf), "ZZZZZZZZZZZZZZZZ");

    let long = [u16::from(b'A'); DMUS_MAX_NAME];
    let long_named = [Pattern {
        valid_data: DMUS_OBJ_NAME,
        name: &long,
    }];
    let mut buf = filled();
    assert_eq!(
        enum_motif(&long_named, 0, Some(&mut buf)) as u32,
        0x0000_0000
    );
    assert_eq!(buf.iter().position(|&unit| unit == 0), Some(63));
    assert_eq!(render(&buf), "AAAAAAAAAAAAAAAA");

    assert_eq!(enum_motif(&named, 0, None) as u32, 0x8000_4003);
}
