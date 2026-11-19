//! Edge checks for the oracle's `edge` rows E01 to E25 in `target/canon/dmusic`. Each `eNN_` test
//! replays row ENN, and its expected values are the ones the verbatim C printed. The module also
//! covers the two flagged vectors that the sanitizers reject (X02 and X03, see `oracle_checks`),
//! the `HRESULT` values, and the derived trait impls, which the oracle rows do not call.

use crate::buffer::{rounded_buffer_size, EventBuffer};
use crate::oracle_checks::{empty_generators, nonzero_text, used_text};
use crate::patch::{midilocale_to_patch, patch_to_midilocale, MidiLocale};
use crate::soundfont::{
    mod_src_to_conn_src, mod_src_to_conn_transform, parse_generators, parse_modulators, SfBag,
    SfGen, SfMod, SF_GEN_END_OPER, SF_GEN_INITIAL_ATTENUATION, SF_GEN_INSTRUMENT, SF_GEN_SAMPLE_ID,
};
use crate::{
    make_dmhresult_error, DMUS_E_BUFFER_EMPTY, DMUS_E_BUFFER_FULL, DMUS_E_INVALID_EVENT, GUID_NULL,
    KSDATAFORMAT_SUBTYPE_MIDI,
};

const BLOB: [u8; 5] = [0x11, 0x22, 0x33, 0x44, 0x55];

fn one_bag(gen_count: u16) -> [SfBag; 2] {
    [
        SfBag {
            gen_ndx: 0,
            mod_ndx: 0,
        },
        SfBag {
            gen_ndx: gen_count,
            mod_ndx: 0,
        },
    ]
}

#[test]
fn e01_patch_to_midilocale_all_ones() {
    assert_eq!(
        patch_to_midilocale(0xFFFF_FFFF),
        MidiLocale {
            bank: 0x8000_7F7F,
            instrument: 0x7F,
        }
    );
}

#[test]
fn e02_midilocale_to_patch_keeps_the_low_two_bank_bytes_only() {
    let locale = MidiLocale {
        bank: 0x7F7F_7F7F,
        instrument: 0x7F,
    };
    assert_eq!(midilocale_to_patch(Some(&locale)), 0x007F_7F7F);
}

#[test]
fn e03_midilocale_to_patch_ignores_bank_bits_above_the_second_byte() {
    let locale = MidiLocale {
        bank: 0x0001_0000,
        instrument: 0,
    };
    assert_eq!(midilocale_to_patch(Some(&locale)), 0);
}

#[test]
fn e04_structured_status_byte_0x80_is_accepted() {
    let mut buffer = EventBuffer::new(64, GUID_NULL);
    assert_eq!(buffer.pack_structured(1000, 0, 0x80), Ok(()));
    assert_eq!(buffer.used_bytes(), 24);
    assert_eq!(buffer.start_time(), Ok(1000));
    assert_eq!(
        used_text(&buffer),
        "03000000 00000000 00000000 00000000 01000000 80000000"
    );
}

#[test]
fn e05_structured_keeps_the_bytes_above_the_status_byte() {
    let mut buffer = EventBuffer::new(64, GUID_NULL);
    assert_eq!(buffer.pack_structured(1000, 0, 0xFF00_0090), Ok(()));
    assert_eq!(
        used_text(&buffer),
        "03000000 00000000 00000000 00000000 01000000 900000ff"
    );
}

#[test]
fn e06_unstructured_empty_payload_gives_a_zero_header() {
    let mut buffer = EventBuffer::new(64, GUID_NULL);
    assert_eq!(buffer.pack_unstructured(1000, 0, &[]), Ok(()));
    assert_eq!(buffer.used_bytes(), 24);
    assert_eq!(
        used_text(&buffer),
        "00000000 00000000 00000000 00000000 00000000 00000000"
    );
}

#[test]
fn e07_unstructured_four_bytes_fill_a_24_byte_buffer() {
    let mut buffer = EventBuffer::new(24, GUID_NULL);
    assert_eq!(buffer.pack_unstructured(1000, 0, &BLOB[..4]), Ok(()));
    assert_eq!(buffer.used_bytes(), 24);
    assert_eq!(
        used_text(&buffer),
        "04000000 00000000 00000000 00000000 00000000 11223344"
    );
}

#[test]
fn e08_a_31_byte_buffer_rounds_up_and_holds_a_five_byte_event() {
    let mut buffer = EventBuffer::new(31, GUID_NULL);
    assert_eq!(buffer.max_bytes(), 32);
    assert_eq!(buffer.pack_unstructured(1000, 0, &BLOB), Ok(()));
    assert_eq!(buffer.used_bytes(), 32);
    assert_eq!(
        used_text(&buffer),
        "05000000 00000000 00000000 00000000 00000000 11223344 55000000 00000000"
    );
}

#[test]
fn e09_a_five_byte_event_does_not_fit_in_28_bytes() {
    let mut buffer = EventBuffer::new(28, GUID_NULL);
    assert_eq!(
        buffer.pack_unstructured(1000, 0, &BLOB),
        Err(DMUS_E_BUFFER_FULL)
    );
    assert_eq!(buffer.used_bytes(), 0);
    assert_eq!(buffer.start_time(), Err(DMUS_E_BUFFER_EMPTY));
    assert_eq!(used_text(&buffer), "");
}

#[test]
fn e10_third_structured_event_in_a_48_byte_buffer_is_full() {
    let mut buffer = EventBuffer::new(48, GUID_NULL);
    buffer.pack_structured(1000, 0, 0x90).unwrap();
    buffer.pack_structured(1000, 0, 0x90).unwrap();
    assert_eq!(
        buffer.pack_structured(1000, 0, 0x90),
        Err(DMUS_E_BUFFER_FULL)
    );
    assert_eq!(buffer.used_bytes(), 48);
    assert_eq!(
        used_text(&buffer),
        "03000000 00000000 00000000 00000000 01000000 90000000 \
         03000000 00000000 00000000 00000000 01000000 90000000"
    );
}

#[test]
fn e11_set_used_bytes_accepts_zero_and_the_full_size() {
    let mut buffer = EventBuffer::new(64, GUID_NULL);
    assert_eq!(buffer.set_used_bytes(0), Ok(()));
    assert_eq!(buffer.set_used_bytes(64), Ok(()));
    assert_eq!(buffer.used_bytes(), 64);
}

#[test]
fn e12_a_20_byte_buffer_keeps_its_size() {
    let buffer = EventBuffer::new(20, GUID_NULL);
    assert_eq!(buffer.max_bytes(), 20);
    assert_eq!(buffer.used_bytes(), 0);
}

#[test]
fn e13_start_time_reads_zero_once_bytes_are_used_without_an_event() {
    let mut buffer = EventBuffer::new(64, GUID_NULL);
    buffer.set_used_bytes(24).unwrap();
    assert_eq!(buffer.start_time(), Ok(0));
}

#[test]
fn e14_first_unstructured_event_replaces_the_start_time() {
    let mut buffer = EventBuffer::new(64, GUID_NULL);
    buffer.set_start_time(-5);
    assert_eq!(buffer.pack_unstructured(7, 0, &[]), Ok(()));
    assert_eq!(buffer.used_bytes(), 24);
    assert_eq!(buffer.start_time(), Ok(7));
    assert_eq!(
        used_text(&buffer),
        "00000000 00000000 00000000 00000000 00000000 00000000"
    );
}

#[test]
fn e15_buffer_sizes_round_up_to_a_multiple_of_four_and_wrap_at_32_bits() {
    assert_eq!(rounded_buffer_size(1), 4);
    assert_eq!(rounded_buffer_size(4), 4);
    assert_eq!(rounded_buffer_size(5), 8);
    assert_eq!(rounded_buffer_size(0xFFFF_FFFC), 0xFFFF_FFFC);
    assert_eq!(rounded_buffer_size(0xFFFF_FFFD), 0);
}

#[test]
fn e16_midi_controller_127_maps_to_0x00ff() {
    assert_eq!(mod_src_to_conn_src(0x00FF), 0x00FF);
}

#[test]
fn e17_convex_curve_bits_map_to_the_convex_transform() {
    assert_eq!(mod_src_to_conn_transform(0x0800), 0x0002);
}

#[test]
fn e18_linear_source_with_no_flags_has_no_transform() {
    assert_eq!(mod_src_to_conn_transform(0x0000), 0x0000);
}

#[test]
fn e19_instrument_generator_in_an_instrument_stops_the_bag_unset() {
    let bags = one_bag(2);
    let gens = [
        SfGen {
            oper: SF_GEN_INSTRUMENT,
            amount: 5,
        },
        SfGen {
            oper: SF_GEN_INITIAL_ATTENUATION,
            amount: 100,
        },
    ];
    let mut out = empty_generators();
    assert!(!parse_generators(&bags, &gens, 0, false, &mut out));
    assert_eq!(nonzero_text(&out), "none");
}

#[test]
fn e20_sample_id_generator_in_a_preset_stops_the_bag_unset() {
    let bags = one_bag(2);
    let gens = [
        SfGen {
            oper: SF_GEN_SAMPLE_ID,
            amount: 2,
        },
        SfGen {
            oper: SF_GEN_INITIAL_ATTENUATION,
            amount: 100,
        },
    ];
    let mut out = empty_generators();
    assert!(!parse_generators(&bags, &gens, 0, true, &mut out));
    assert_eq!(nonzero_text(&out), "none");
}

#[test]
fn e21_sample_id_generator_in_an_instrument_is_set_then_stops_the_bag() {
    let bags = one_bag(2);
    let gens = [
        SfGen {
            oper: SF_GEN_SAMPLE_ID,
            amount: 2,
        },
        SfGen {
            oper: SF_GEN_INITIAL_ATTENUATION,
            amount: 100,
        },
    ];
    let mut out = empty_generators();
    assert!(!parse_generators(&bags, &gens, 0, false, &mut out));
    assert_eq!(nonzero_text(&out), "53:2");
}

#[test]
fn e22_an_empty_bag_is_valid_and_sets_nothing() {
    let bags = one_bag(0);
    let mut out = empty_generators();
    assert!(parse_generators(&bags, &[], 0, false, &mut out));
    assert_eq!(nonzero_text(&out), "none");
}

#[test]
fn e23_a_repeated_modulator_replaces_the_first_one_in_place() {
    let bags = [
        SfBag {
            gen_ndx: 0,
            mod_ndx: 0,
        },
        SfBag {
            gen_ndx: 0,
            mod_ndx: 5,
        },
    ];
    let mods = [
        SfMod {
            src_mod: 2,
            dest_gen: 17,
            amount: 100,
            amount_src_mod: 0,
            transform: 0,
        },
        SfMod {
            src_mod: 3,
            dest_gen: 17,
            amount: 10,
            amount_src_mod: 0,
            transform: 0,
        },
        SfMod {
            src_mod: 2,
            dest_gen: 17,
            amount: 5,
            amount_src_mod: 1,
            transform: 0,
        },
        SfMod {
            src_mod: 2,
            dest_gen: 17,
            amount: 7,
            amount_src_mod: 1,
            transform: 0x100,
        },
        SfMod {
            src_mod: 2,
            dest_gen: 17,
            amount: 9,
            amount_src_mod: 0,
            transform: 0,
        },
    ];
    let mut out = Vec::new();
    parse_modulators(&bags, &mods, 0, &mut out);
    assert_eq!(
        out,
        vec![
            SfMod {
                src_mod: 2,
                dest_gen: 17,
                amount: 9,
                amount_src_mod: 0,
                transform: 0,
            },
            mods[1],
            mods[2],
            mods[3],
        ]
    );
}

#[test]
fn e24_an_empty_modulator_range_adds_nothing() {
    let bags = [
        SfBag {
            gen_ndx: 0,
            mod_ndx: 0,
        },
        SfBag {
            gen_ndx: 0,
            mod_ndx: 0,
        },
    ];
    let mut out = Vec::new();
    parse_modulators(&bags, &[], 0, &mut out);
    assert!(out.is_empty());
}

#[test]
fn e25_a_status_byte_without_the_top_bit_is_invalid() {
    let mut buffer = EventBuffer::new(64, GUID_NULL);
    assert_eq!(
        buffer.pack_structured(1000, 0, 0x70),
        Err(DMUS_E_INVALID_EVENT)
    );
    assert_eq!(buffer.used_bytes(), 0);
    assert_eq!(buffer.start_time(), Err(DMUS_E_BUFFER_EMPTY));
}

#[test]
fn hresult_values_match_the_dmusic_error_codes() {
    assert_eq!(make_dmhresult_error(0x0135), DMUS_E_BUFFER_EMPTY);
    assert_eq!(make_dmhresult_error(0x0136), DMUS_E_BUFFER_FULL);
    assert_eq!(make_dmhresult_error(0x013F), DMUS_E_INVALID_EVENT);
    assert_eq!(DMUS_E_BUFFER_EMPTY as u32, 0x8878_1135);
    assert_eq!(DMUS_E_BUFFER_FULL as u32, 0x8878_1136);
    assert_eq!(DMUS_E_INVALID_EVENT as u32, 0x8878_113F);
}

/// X02 in `oracle_checks`: the C computes `ref_time - start_time` in signed arithmetic, and
/// UBSan reports the overflow. The port wraps, so the delta is the two's-complement difference.
#[test]
fn ref_time_delta_wraps() {
    let mut buffer = EventBuffer::new(64, GUID_NULL);
    buffer.pack_structured(1, 0, 0x90).unwrap();
    buffer.pack_structured(i64::MIN, 0, 0x90).unwrap();
    assert_eq!(
        used_text(&buffer),
        "03000000 00000000 00000000 00000000 01000000 90000000 \
         03000000 00000000 ffffffff ffffff7f 01000000 90000000"
    );
}

/// X03 in `oracle_checks`: the C stores a generator whose operator is past the end of the table,
/// and ASan reports a stack buffer overflow. The port ignores that generator and carries on.
#[test]
fn generator_past_table_is_ignored() {
    let bags = one_bag(2);
    let gens = [
        SfGen {
            oper: SF_GEN_END_OPER,
            amount: 1,
        },
        SfGen {
            oper: 0xFFFF,
            amount: 2,
        },
    ];
    let mut out = empty_generators();
    assert!(parse_generators(&bags, &gens, 0, false, &mut out));
    assert_eq!(nonzero_text(&out), "none");
}

#[test]
fn modulator_keys_are_distinct() {
    let base = SfMod {
        src_mod: 2,
        dest_gen: 17,
        amount: 1,
        amount_src_mod: 0,
        transform: 0,
    };
    let other_dest = SfMod {
        dest_gen: 18,
        ..base
    };
    let other_src = SfMod { src_mod: 3, ..base };
    let other_amount_src = SfMod {
        amount_src_mod: 1,
        ..base
    };
    let other_transform = SfMod {
        transform: 0x100,
        ..base
    };
    let replacement = SfMod { amount: 9, ..base };
    let bags = [
        SfBag {
            gen_ndx: 0,
            mod_ndx: 0,
        },
        SfBag {
            gen_ndx: 0,
            mod_ndx: 6,
        },
    ];
    let mods = [
        base,
        other_dest,
        other_src,
        other_amount_src,
        other_transform,
        replacement,
    ];
    let mut out = Vec::new();
    parse_modulators(&bags, &mods, 0, &mut out);
    assert_eq!(out.len(), 5);
    assert_eq!(out[0].amount, 9);
}

#[test]
fn derived_trait_impls_clone_compare_and_debug() {
    assert_eq!(
        Clone::clone(&KSDATAFORMAT_SUBTYPE_MIDI),
        KSDATAFORMAT_SUBTYPE_MIDI
    );
    assert_eq!(
        format!("{GUID_NULL:?}"),
        "Guid { data1: 0, data2: 0, data3: 0, data4: [0, 0, 0, 0, 0, 0, 0, 0] }"
    );

    let locale = MidiLocale {
        bank: 1,
        instrument: 2,
    };
    assert_eq!(Clone::clone(&locale), locale);
    assert_eq!(
        format!("{locale:?}"),
        "MidiLocale { bank: 1, instrument: 2 }"
    );

    let bag = SfBag {
        gen_ndx: 1,
        mod_ndx: 2,
    };
    assert_eq!(Clone::clone(&bag), bag);
    assert_eq!(format!("{bag:?}"), "SfBag { gen_ndx: 1, mod_ndx: 2 }");

    let gen = SfGen {
        oper: 46,
        amount: 60,
    };
    assert_eq!(Clone::clone(&gen), gen);
    assert_eq!(format!("{gen:?}"), "SfGen { oper: 46, amount: 60 }");

    let sf_mod = SfMod {
        src_mod: 2,
        dest_gen: 17,
        amount: -1,
        amount_src_mod: 0,
        transform: 0,
    };
    assert_eq!(Clone::clone(&sf_mod), sf_mod);
    assert_eq!(
        format!("{sf_mod:?}"),
        "SfMod { src_mod: 2, dest_gen: 17, amount: -1, amount_src_mod: 0, transform: 0 }"
    );

    let buffer = EventBuffer::new(8, GUID_NULL);
    assert_eq!(buffer.clone(), buffer);
    assert!(format!("{buffer:?}").contains("write_pos: 0"));
}
