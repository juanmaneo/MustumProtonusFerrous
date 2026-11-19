//! Replays of the oracle rows R01 to R24 in `target/canon/dmusic`. The oracle compiles the verbatim
//! `dlls/dmusic` ranges with `gcc -fsanitize=address,undefined -g`, with the Win32 shims in
//! `mock.h`, and `oracle_main.c ok` prints one line per row. Each `oracle_oNN` test replays row RNN,
//! and its expected values are the ones the C printed. The oracle allocates buffers with `calloc`,
//! so the bytes that no event writes are zero, as they are here.
//!
//! Three vectors trip the sanitizers and are left out. Each runs alone in the oracle, because ASan
//! stops at its first report:
//! - X01: `SetUsedBytes(3)` then `PackStructured`. UBSan reports a misaligned `DMUS_EVENTHEADER`.
//!   The Rust port has no alignment rule, so it writes the same bytes at offset 3.
//! - X02: `PackStructured` at 1, then at `INT64_MIN`. UBSan reports signed overflow in `rtDelta`.
//!   See `edge_checks::ref_time_delta_wraps`.
//! - X03: a generator with operator 60 in `parse_soundfont_generators`. ASan reports a
//!   stack-buffer-overflow in the generator table. See `edge_checks::generator_past_table_is_ignored`.

use crate::buffer::EventBuffer;
use crate::patch::{midilocale_to_patch, patch_to_midilocale, MidiLocale};
use crate::soundfont::{
    mod_src_to_conn_src, mod_src_to_conn_transform, parse_generators, parse_modulators, SfBag,
    SfGen, SfGenerators, SfMod, SF_GEN_END_OPER, SF_GEN_INITIAL_ATTENUATION, SF_GEN_INSTRUMENT,
    SF_GEN_KEYNUM, SF_GEN_PAN, SF_GEN_SAMPLE_ID,
};
use crate::{Guid, DMUS_E_BUFFER_EMPTY, DMUS_E_BUFFER_FULL, GUID_NULL, KSDATAFORMAT_SUBTYPE_MIDI};

/// Bytes in groups of four, each byte as two lowercase hex digits, joined by spaces. This is the
/// `bytes=` field of the oracle output.
pub(crate) fn hex_groups(bytes: &[u8]) -> String {
    bytes
        .chunks(4)
        .map(|group| {
            group
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// The used bytes of `buffer`, grouped as the oracle prints them.
pub(crate) fn used_text(buffer: &EventBuffer) -> String {
    hex_groups(&buffer.data()[..buffer.used_bytes() as usize])
}

/// The generators with a nonzero value, as `ret=... nonzero=` prints them, or `none`.
pub(crate) fn nonzero_text(gens: &SfGenerators) -> String {
    let parts: Vec<String> = gens
        .iter()
        .enumerate()
        .filter(|(_, v)| **v != 0)
        .map(|(i, v)| format!("{i}:{v}"))
        .collect();
    if parts.is_empty() {
        "none".to_string()
    } else {
        parts.join(",")
    }
}

pub(crate) fn empty_generators() -> SfGenerators {
    [0; SF_GEN_END_OPER as usize]
}

#[test]
fn oracle_o01() {
    assert_eq!(midilocale_to_patch(None), 0);
}

#[test]
fn oracle_o02() {
    let locale = MidiLocale {
        bank: 0x8000_0101,
        instrument: 0x81,
    };
    assert_eq!(midilocale_to_patch(Some(&locale)), 0x8001_0101);
}

#[test]
fn oracle_o03() {
    let locale = MidiLocale {
        bank: 0xFFFF_FFFF,
        instrument: 0xFFFF_FFFF,
    };
    assert_eq!(midilocale_to_patch(Some(&locale)), 0x807F_7F7F);
}

#[test]
fn oracle_o04() {
    assert_eq!(
        patch_to_midilocale(0x80FF_7F7F),
        MidiLocale {
            bank: 0x8000_7F7F,
            instrument: 0x7F,
        }
    );
}

#[test]
fn oracle_o05() {
    let buffer = EventBuffer::new(5, GUID_NULL);
    assert_eq!(buffer.max_bytes(), 8);
    assert_eq!(buffer.format(), KSDATAFORMAT_SUBTYPE_MIDI);
}

#[test]
fn oracle_o06() {
    let custom = Guid {
        data1: 0x1a82_f8bc,
        data2: 0x3f8b,
        data3: 0x11d2,
        data4: [0xb7, 0x74, 0x00, 0x60, 0x08, 0x33, 0x16, 0xc1],
    };
    let buffer = EventBuffer::new(0xFFFF_FFFD, custom);
    assert_eq!(buffer.max_bytes(), 0);
    assert_eq!(buffer.format(), custom);
}

#[test]
fn oracle_o07() {
    let mut buffer = EventBuffer::new(64, GUID_NULL);
    assert_eq!(buffer.pack_structured(1000, 2, 0x0064_3C90), Ok(()));
    assert_eq!(buffer.used_bytes(), 24);
    assert_eq!(buffer.start_time(), Ok(1000));
    assert_eq!(
        used_text(&buffer),
        "03000000 02000000 00000000 00000000 01000000 903c6400"
    );
}

#[test]
fn oracle_o08() {
    let mut buffer = EventBuffer::new(64, GUID_NULL);
    buffer.pack_structured(1000, 2, 0x0064_3C90).unwrap();
    assert_eq!(buffer.pack_structured(1500, 3, 0x0040_3C90), Ok(()));
    assert_eq!(buffer.used_bytes(), 48);
    assert_eq!(buffer.start_time(), Ok(1000));
    assert_eq!(
        used_text(&buffer),
        "03000000 02000000 00000000 00000000 01000000 903c6400 \
         03000000 03000000 f4010000 00000000 01000000 903c4000"
    );
}

#[test]
fn oracle_o09() {
    let mut buffer = EventBuffer::new(64, GUID_NULL);
    buffer.pack_structured(1000, 2, 0x0064_3C90).unwrap();
    buffer.pack_structured(1500, 3, 0x0040_3C90).unwrap();
    assert_eq!(
        buffer.pack_structured(2000, 0, 0x70),
        Err(DMUS_E_BUFFER_FULL)
    );
    assert_eq!(buffer.used_bytes(), 48);
}

#[test]
fn oracle_o10() {
    let mut buffer = EventBuffer::new(64, GUID_NULL);
    assert_eq!(
        buffer.pack_unstructured(2000, 1, &[0xF0, 0x7E, 0xF7]),
        Ok(())
    );
    assert_eq!(buffer.used_bytes(), 24);
    assert_eq!(buffer.start_time(), Ok(2000));
    assert_eq!(
        used_text(&buffer),
        "03000000 01000000 00000000 00000000 00000000 f07ef700"
    );
}

#[test]
fn oracle_o11() {
    let mut buffer = EventBuffer::new(24, GUID_NULL);
    buffer.pack_structured(1000, 0, 0x0064_3C90).unwrap();
    assert_eq!(
        buffer.pack_structured(1100, 0, 0x0064_3C90),
        Err(DMUS_E_BUFFER_FULL)
    );
    assert_eq!(buffer.used_bytes(), 24);
    assert_eq!(buffer.start_time(), Ok(1000));
    assert_eq!(
        used_text(&buffer),
        "03000000 00000000 00000000 00000000 01000000 903c6400"
    );
}

#[test]
fn oracle_o12() {
    let mut buffer = EventBuffer::new(24, GUID_NULL);
    buffer.pack_structured(1000, 2, 0x0064_3C90).unwrap();
    assert_eq!(
        buffer.pack_structured(1100, 0, 0x70),
        Err(DMUS_E_BUFFER_FULL)
    );
    assert_eq!(buffer.used_bytes(), 24);
    assert_eq!(
        used_text(&buffer),
        "03000000 02000000 00000000 00000000 01000000 903c6400"
    );
}

#[test]
fn oracle_o13() {
    let mut buffer = EventBuffer::new(64, GUID_NULL);
    assert_eq!(buffer.set_used_bytes(65), Err(DMUS_E_BUFFER_FULL));
    assert_eq!(buffer.set_used_bytes(40), Ok(()));
    assert_eq!(buffer.used_bytes(), 40);
}

#[test]
fn oracle_o14() {
    let mut buffer = EventBuffer::new(64, GUID_NULL);
    assert_eq!(buffer.start_time(), Err(DMUS_E_BUFFER_EMPTY));
    buffer.set_start_time(77);
    assert_eq!(buffer.start_time(), Err(DMUS_E_BUFFER_EMPTY));
    assert_eq!(buffer.pack_structured(5, 0, 0x90), Ok(()));
    assert_eq!(buffer.start_time(), Ok(5));
}

#[test]
fn oracle_o15() {
    let mut buffer = EventBuffer::new(64, GUID_NULL);
    buffer.pack_structured(1000, 0, 0x90).unwrap();
    buffer.flush();
    assert_eq!(buffer.start_time(), Err(DMUS_E_BUFFER_EMPTY));
    assert_eq!(buffer.pack_structured(9000, 0, 0x90), Ok(()));
    assert_eq!(buffer.used_bytes(), 24);
    assert_eq!(buffer.start_time(), Ok(9000));
    assert_eq!(
        used_text(&buffer),
        "03000000 00000000 00000000 00000000 01000000 90000000"
    );
}

#[test]
fn oracle_o16() {
    let mut buffer = EventBuffer::new(64, GUID_NULL);
    buffer.pack_structured(5000, 0, 0x90).unwrap();
    assert_eq!(buffer.pack_structured(4000, 0, 0x90), Ok(()));
    assert_eq!(buffer.used_bytes(), 48);
    assert_eq!(buffer.start_time(), Ok(5000));
    assert_eq!(
        used_text(&buffer),
        "03000000 00000000 00000000 00000000 01000000 90000000 \
         03000000 00000000 18fcffff ffffffff 01000000 90000000"
    );
}

#[test]
fn oracle_o17() {
    assert_eq!(mod_src_to_conn_src(0x0081), 0x0081);
}

#[test]
fn oracle_o18() {
    assert_eq!(mod_src_to_conn_src(0x0011), 0);
}

#[test]
fn oracle_o19() {
    assert_eq!(mod_src_to_conn_src(0x0010), 0x0100);
}

#[test]
fn oracle_o20() {
    assert_eq!(mod_src_to_conn_transform(0x0700), 0x0031);
}

#[test]
fn oracle_o21() {
    assert_eq!(mod_src_to_conn_transform(0x0C00), 0x0003);
}

#[test]
fn oracle_o22() {
    let bags = [
        SfBag {
            gen_ndx: 0,
            mod_ndx: 0,
        },
        SfBag {
            gen_ndx: 2,
            mod_ndx: 0,
        },
    ];
    let gens = [
        SfGen {
            oper: SF_GEN_KEYNUM,
            amount: 0x3C,
        },
        SfGen {
            oper: SF_GEN_INITIAL_ATTENUATION,
            amount: 100,
        },
    ];
    let mut out = empty_generators();
    assert!(parse_generators(&bags, &gens, 0, false, &mut out));
    assert_eq!(nonzero_text(&out), "46:60,48:100");
}

#[test]
fn oracle_o23() {
    let bags = [
        SfBag {
            gen_ndx: 0,
            mod_ndx: 0,
        },
        SfBag {
            gen_ndx: 4,
            mod_ndx: 0,
        },
    ];
    let gens = [
        SfGen {
            oper: SF_GEN_KEYNUM,
            amount: 0x3C,
        },
        SfGen {
            oper: SF_GEN_INITIAL_ATTENUATION,
            amount: 100,
        },
        SfGen {
            oper: SF_GEN_INSTRUMENT,
            amount: 5,
        },
        SfGen {
            oper: SF_GEN_SAMPLE_ID,
            amount: 2,
        },
    ];
    let mut out = empty_generators();
    assert!(!parse_generators(&bags, &gens, 0, true, &mut out));
    assert_eq!(nonzero_text(&out), "41:5,48:100");
}

#[test]
fn oracle_o24() {
    let bags = [
        SfBag {
            gen_ndx: 0,
            mod_ndx: 0,
        },
        SfBag {
            gen_ndx: 0,
            mod_ndx: 2,
        },
        SfBag {
            gen_ndx: 0,
            mod_ndx: 3,
        },
    ];
    let mods = [
        SfMod {
            src_mod: 2,
            dest_gen: SF_GEN_PAN,
            amount: 100,
            amount_src_mod: 0,
            transform: 0,
        },
        SfMod {
            src_mod: 0x81,
            dest_gen: SF_GEN_INITIAL_ATTENUATION,
            amount: -200,
            amount_src_mod: 2,
            transform: 0x100,
        },
        SfMod {
            src_mod: 2,
            dest_gen: SF_GEN_PAN,
            amount: 50,
            amount_src_mod: 0,
            transform: 0,
        },
    ];
    let mut out = Vec::new();
    parse_modulators(&bags, &mods, 0, &mut out);
    parse_modulators(&bags, &mods, 1, &mut out);
    assert_eq!(
        out,
        vec![
            SfMod {
                src_mod: 2,
                dest_gen: 17,
                amount: 50,
                amount_src_mod: 0,
                transform: 0,
            },
            SfMod {
                src_mod: 0x81,
                dest_gen: 48,
                amount: -200,
                amount_src_mod: 2,
                transform: 0x100,
            },
        ]
    );
}
