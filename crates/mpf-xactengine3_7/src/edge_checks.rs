//! Rust-only edge cases over every notification byte.

use super::*;

const ALL_NOTIFY: u32 = NOTIFY_SOUND_BANK
    | NOTIFY_WAVE_BANK
    | NOTIFY_CUE
    | NOTIFY_WAVE
    | NOTIFY_CUE_INDEX
    | NOTIFY_WAVE_INDEX;

#[test]
fn notify_bits_are_distinct_and_cover_six_bits() {
    assert_eq!(ALL_NOTIFY, 0x3f);
    assert_eq!(NOTIFY_SOUND_BANK.count_ones(), 1);
    assert_eq!(NOTIFY_WAVE_BANK.count_ones(), 1);
    assert_eq!(NOTIFY_CUE.count_ones(), 1);
    assert_eq!(NOTIFY_WAVE.count_ones(), 1);
    assert_eq!(NOTIFY_CUE_INDEX.count_ones(), 1);
    assert_eq!(NOTIFY_WAVE_INDEX.count_ones(), 1);
}

#[test]
fn notification_maps_are_identity_on_one_to_eighteen() {
    for ty in 0..=255u8 {
        let expected = if (1..=18).contains(&ty) { ty } else { 0 };
        assert_eq!(xact_notification_type_from_fact(ty), expected, "xf {ty}");
        assert_eq!(fact_notification_type_from_xact(ty), expected, "fx {ty}");
    }
}

#[test]
fn notify_flags_only_use_known_bits() {
    for ty in 0..=255u8 {
        assert_eq!(notify_flags(ty) & !ALL_NOTIFY, 0, "nf {ty}");
    }
}

#[test]
fn types_without_a_chain_entry_select_no_fields() {
    for ty in [0u8, 9, 10, 11].into_iter().chain(19..=255) {
        assert_eq!(notify_flags(ty), 0, "nf {ty}");
    }
}

#[test]
fn wave_index_only_for_wave_prepared_and_destroyed() {
    for ty in 0..=255u8 {
        let has_index = notify_flags(ty) & NOTIFY_WAVE_INDEX != 0;
        assert_eq!(has_index, ty == 12 || ty == 16, "nf {ty}");
    }
}

#[test]
fn cue_types_never_carry_wave_fields() {
    for ty in [1u8, 2, 3, 4, 5, 8] {
        assert_eq!(
            notify_flags(ty) & (NOTIFY_WAVE_BANK | NOTIFY_WAVE),
            0,
            "nf {ty}"
        );
    }
}

#[test]
fn wave_play_stop_and_loop_carry_sound_and_wave_fields() {
    for ty in [13u8, 14, 15] {
        assert_eq!(notify_flags(ty), ALL_NOTIFY & !NOTIFY_WAVE_INDEX, "nf {ty}");
    }
}
