//! Edge cases of the ported logic that the oracle vectors do not reach. `ORACLE_EDGES` holds
//! further command lines with the output the oracle printed for them. The sanitizer-flagged
//! `mod 0 0 782 0 -2147483648` (signed negation of `INT_MIN`) is left out. The Rust-only tests
//! cover what the C cannot run (a zero sample rate traps there) and the caller-supplied caps.

use std::fmt::Debug;

use super::oracle_checks::replay;
use crate::dls::Connection;
use crate::format::{pcm16, synth_format};
use crate::generator::GenSetting;
use crate::modulator::Modulator;
use crate::port::{
    negotiate, Caps, PortParams, SYNTH_CAPS, SYNTH_DEFAULTS, VALID_CHANNEL_GROUPS, VALID_VOICES,
};
use crate::region::{region_matches, LoopSettings, RgnRange, WaveLoop};
use crate::sink::{sample_to_ref_time, Link};
use crate::Error;

#[rustfmt::skip]
const ORACLE_EDGES: &[(&str, &str)] = &[
    ("negotiate 0 32 0 0 0 0 0 0 0 0", "00000000 36 111 32 2 2 22050 1 0 0"),
    ("negotiate 0 36 64 0 0 0 0 0 1 0", "00000001 36 239 32 2 2 22050 1 0 0"),
    ("negotiate 0 36 64 0 0 0 0 0 0 0", "00000000 36 239 32 2 2 22050 1 0 0"),
    ("negotiate 0 36 80 0 0 0 0 0 0 0", "00000000 36 239 32 2 2 22050 1 0 0"),
    ("negotiate 0 36 136 0 0 0 0 0 0 3", "00000001 36 239 32 2 2 11025 1 0 3"),
    ("negotiate 0 36 136 0 0 0 0 0 0 255", "00000001 36 239 32 2 2 11025 1 0 3"),
    ("negotiate 0 36 1 0 0 0 0 0 0 0", "00000001 36 239 1 2 2 22050 1 0 0"),
    ("negotiate 0 36 2 0 0 0 0 0 0 0", "00000001 36 239 32 1 2 22050 1 0 0"),
    ("negotiate 0 36 4 0 0 0 0 0 0 0", "00000001 36 239 32 2 2 22050 1 0 0"),
    ("negotiate 0 36 8 0 0 0 5000 0 0 0", "00000001 36 239 32 2 2 11025 1 0 0"),
    ("negotiate 0 36 8 0 0 0 22050 0 0 0", "00000000 36 239 32 2 2 22050 1 0 0"),
    ("negotiate 0 36 32 0 0 0 0 255 0 0", "00000001 36 239 32 2 2 22050 7 0 0"),
    ("negotiate 0 36 32 0 0 0 0 0 0 0", "00000000 36 239 32 2 2 22050 0 0 0"),
    ("negotiate 0 36 0 1 0 0 0 0 0 0", "00000000 36 239 32 2 2 22050 1 0 0"),
    ("format 0 2 22050", "88781131"),
    ("format 0 0 0", "88781131"),
    ("sinkfmt none", "2 22050 88200 4 16 0"),
    ("sinkfmt closed", "2 22050 88200 4 16 0"),
    ("sinkfmt open 1 11025", "1 11025 22050 2 16 0"),
    ("desired none", "88781131"),
    ("desired open 2 22050", "00000000 176400"),
    ("s2r 0 5 none", "00000000 2000"),
    ("s2r 0 5 closed", "00000000 2000"),
    ("r2s 0 1000000 none", "00000000 2205"),
    ("gen 0", "-"),
    ("gen 1", "48"),
    ("gen 3", "59"),
    ("gen 2", "-"),
    ("gen 525", "-"),
    ("gen 1280", "8"),
    ("gen 1281", "9"),
    ("gen 128", "15"),
    ("gen 129", "16"),
    ("gen 4", "17"),
    ("gen 261", "21"),
    ("gen 260", "22"),
    ("gen 277", "23"),
    ("gen 276", "24"),
    ("gen 783", "25"),
    ("gen 778", "26"),
    ("gen 784", "27"),
    ("gen 779", "28"),
    ("gen 782", "29"),
    ("gen 781", "30"),
    ("gen 523", "33"),
    ("gen 518", "34"),
    ("gen 524", "35"),
    ("gen 519", "36"),
    ("gen 522", "37"),
    ("gen 521", "38"),
    ("set 1 0 1 0 0", "13 0000000000000000"),
    ("set 1 0 3 0 0", "5 0000000000000000"),
    ("set 1 0 1280 0 0", "10 0000000000000000"),
    ("set 5 0 3 0 0", "7 0000000000000000"),
    ("set 5 0 1280 0 0", "11 0000000000000000"),
    ("set 9 0 3 0 0", "6 0000000000000000"),
    ("set 9 0 1 0 0", "-"),
    ("set 3 0 3 0 0", "-"),
    ("set 2 0 3 0 0", "-"),
    ("set 5 0 525 0 0", "-"),
    ("set 1 0 525 0 0", "-"),
    ("set 0 1 3 0 0", "-"),
    ("set 0 0 3 4096 0", "-"),
    ("set 2 0 3 16384 0", "-"),
    ("set 1 0 0 16384 0", "-"),
    ("set 0 0 0 0 0", "-"),
    ("set 0 0 2 0 0", "-"),
    ("set 0 0 525 0 0", "-"),
    ("set 0 0 1 0 0", "48 0000000000000000"),
    ("set 0 0 1 0 -2147483648", "48 c0e0000000000000"),
    ("set 0 0 3 0 -2147483648", "59 c0e0000000000000"),
    ("set 0 0 3 0 65536", "59 3ff0000000000000"),
    ("set 0 0 782 0 0", "29 408f400000000000"),
    ("set 0 0 782 0 65536", "29 408f380000000000"),
    ("set 0 0 522 0 0", "37 408f400000000000"),
    ("set 0 0 1281 0 0", "9 0000000000000000"),
    ("set 0 0 128 0 0", "15 0000000000000000"),
    ("set 0 0 129 0 0", "16 0000000000000000"),
    ("set 0 0 4 0 0", "17 0000000000000000"),
    ("set 0 0 261 0 0", "21 0000000000000000"),
    ("set 0 0 260 0 0", "22 0000000000000000"),
    ("set 0 0 277 0 0", "23 0000000000000000"),
    ("set 0 0 276 0 0", "24 0000000000000000"),
    ("set 0 0 783 0 0", "25 0000000000000000"),
    ("set 0 0 778 0 0", "26 0000000000000000"),
    ("set 0 0 784 0 0", "27 0000000000000000"),
    ("set 0 0 779 0 0", "28 0000000000000000"),
    ("set 0 0 781 0 0", "30 0000000000000000"),
    ("set 0 0 523 0 0", "33 0000000000000000"),
    ("set 0 0 518 0 0", "34 0000000000000000"),
    ("set 0 0 524 0 0", "35 0000000000000000"),
    ("set 0 0 519 0 0", "36 0000000000000000"),
    ("set 0 0 521 0 0", "38 0000000000000000"),
    ("ctrl 0 0", "0 0"),
    ("ctrl 2 0", "2 0"),
    ("ctrl 2 1", "2 4"),
    ("ctrl 2 2", "2 8"),
    ("ctrl 2 3", "2 12"),
    ("ctrl 3 0", "3 0"),
    ("ctrl 6 0", "14 0"),
    ("ctrl 7 0", "10 0"),
    ("ctrl 8 0", "13 0"),
    ("ctrl 4 0", "-"),
    ("ctrl 1 0", "-"),
    ("ctrl 9 0", "-"),
    ("ctrl 127 0", "-"),
    ("ctrl 128 0", "0 16"),
    ("ctrl 255 0", "127 16"),
    ("ctrl 256 0", "16 0"),
    ("ctrl 257 0", "-"),
    ("mod 0 0 0 0 0", "-"),
    ("mod 4 0 3 0 65536", "-"),
    ("mod 0 4 3 0 65536", "-"),
    ("mod 1 4 1 0 65536", "-"),
    ("mod 0 0 525 0 65536", "-"),
    ("mod 0 0 30 0 65536", "-"),
    ("mod 0 0 5 0 65536", "-"),
    ("mod 0 0 6 0 65536", "-"),
    ("mod 5 0 3 0 65536", "-"),
    ("mod 9 0 3 0 65536", "-"),
    ("mod 1 3 2 0 65536", "-"),
    ("mod 0 0 3 0 -2147483648", "0 0 0 0 59 c0e0000000000000"),
    ("mod 1 3 1 0 65536", "3 0 0 0 13 3ff0000000000000"),
    ("mod 1 3 3 0 0", "3 0 0 0 5 0000000000000000"),
    ("mod 1 3 1280 0 65536", "3 0 0 0 10 3ff0000000000000"),
    ("mod 5 3 1280 0 65536", "3 0 0 0 11 3ff0000000000000"),
    ("mod 128 0 3 32768 65536", "0 17 0 0 59 3ff0000000000000"),
    ("mod 128 0 3 16384 65536", "0 18 0 0 59 3ff0000000000000"),
    ("mod 0 0 3 0 65536", "0 0 0 0 59 3ff0000000000000"),
    ("mod 0 2 3 0 65536", "0 0 2 0 59 3ff0000000000000"),
    ("mod 0 0 4 0 65536", "0 0 0 0 17 3ff0000000000000"),
    ("mod 2 0 3 0 65536", "2 0 0 0 59 3ff0000000000000"),
    ("mod 6 0 3 0 65536", "14 0 0 0 59 3ff0000000000000"),
    ("mod 7 0 3 0 65536", "10 0 0 0 59 3ff0000000000000"),
    ("mod 0 0 1 16384 65536", "0 2 0 0 48 3ff0000000000000"),
    ("mod 8 0 1 0 65536", "13 0 0 0 48 3ff0000000000000"),
    ("mod 2 0 1 0 65536", "2 0 0 0 48 3ff0000000000000"),
    ("mod 3 0 1 0 65536", "3 0 0 0 48 3ff0000000000000"),
    ("mod 0 8 1 0 655360", "0 0 13 0 48 4024000000000000"),
    ("mod 1 8 1 0 655360", "13 0 0 0 13 3ff0000000000000"),
    ("mod 1 3 1 0 0", "3 0 0 0 13 0000000000000000"),
    ("chan 0 0", "8000000000000000"),
    ("region 0", "8000000000000000 403e19999999999a"),
    ("region 131072", "c000000000000000 403e19999999999a"),
    ("patch 0 0 0", "00000000"),
    ("patch 0 0 9", "80000000"),
    ("patch 5 128 1", "00010005"),
    ("patch 2147483648 0 9", "80000000"),
    ("fallback 2147483648 0", "80000000"),
    ("fallback 2147483648 1", "-"),
    ("fallback 0 0", "-"),
    ("fallback 1 0", "-"),
    ("match 5 5 0 4 0 9", "0"),
    ("match 5 10 0 9 0 9", "0"),
    ("match 5 5 0 9 0 9", "1"),
    ("match 5 5 6 9 0 9", "0"),
    ("loop 0 0 0 0", "none"),
    ("loop 1 0 10 20", "1 10 30"),
    ("loop 1 2 10 20", "- 10 30"),
    ("loop 2 0 10 20", "1 10 30"),
];

fn exercise<T: Clone + Debug + PartialEq>(value: &T) {
    assert_eq!(value.clone(), *value);
    assert!(!format!("{value:?}").is_empty());
}

#[test]
fn oracle_edges_match_wine() {
    for (command, expected) in ORACLE_EDGES {
        assert_eq!(replay(command), *expected, "{command}");
    }
}

#[test]
fn zero_sample_rate_has_no_reference_time() {
    let params = PortParams {
        sample_rate: 0,
        ..SYNTH_DEFAULTS
    };
    assert_eq!(sample_to_ref_time(0, 5, Link::Open(&params)), None);
    assert_eq!(replay("s2r 0 5 open 2 0"), "-");
}

#[test]
fn negotiate_clamps_to_the_caps() {
    let requested = PortParams {
        valid_params: VALID_VOICES | VALID_CHANNEL_GROUPS,
        voices: 40,
        channel_groups: 9,
        ..SYNTH_DEFAULTS
    };
    let caps = Caps {
        max_voices: 16,
        max_channel_groups: 4,
    };
    let opened = negotiate(None, Some(&requested), caps).expect("negotiated");
    assert_eq!(
        (opened.params.voices, opened.params.channel_groups),
        (16, 4)
    );
    assert_eq!(opened.hresult(), 1);
}

#[test]
fn negotiate_refuses_an_open_synth_and_a_short_structure() {
    assert_eq!(
        negotiate(Some(&SYNTH_DEFAULTS), None, SYNTH_CAPS),
        Err(Error::AlreadyOpen)
    );
    let short = PortParams {
        size: 31,
        ..SYNTH_DEFAULTS
    };
    assert_eq!(
        negotiate(None, Some(&short), SYNTH_CAPS),
        Err(Error::InvalidArg)
    );
}

#[test]
fn closed_synth_has_no_format() {
    assert_eq!(synth_format(None), Err(Error::NotConfigured));
    assert_eq!(synth_format(Some(&SYNTH_DEFAULTS)), Ok(pcm16(2, 22050)));
}

#[test]
fn region_ranges_are_inclusive() {
    let range = RgnRange { low: 2, high: 5 };
    let all = RgnRange { low: 0, high: 127 };
    assert_eq!(
        [1, 2, 5, 6].map(|key| range.contains(key)),
        [false, true, true, false]
    );
    assert!(region_matches(range, all, 5, 127));
    assert!(!region_matches(range, all, 5, 128));
    assert!(!region_matches(range, all, 1, 0));
}

#[test]
fn error_hresults_match_the_dmusic_codes() {
    assert_eq!(Error::InvalidArg.hresult(), 0x8007_0057);
    assert_eq!(Error::AlreadyOpen.hresult(), 0x8878_112F);
    assert_eq!(Error::NotConfigured.hresult(), 0x8878_1131);
    assert_eq!(Error::Unexpected.hresult(), 0x8000_FFFF);
}

#[test]
fn value_types_support_their_derived_traits() {
    let params = SYNTH_DEFAULTS;
    exercise(&Error::InvalidArg);
    exercise(&Error::AlreadyOpen);
    exercise(&Error::NotConfigured);
    exercise(&Error::Unexpected);
    exercise(&Connection {
        source: 0x80,
        control: 0,
        destination: 3,
        transform: 0x4000,
        scale: 65536,
    });
    exercise(&GenSetting {
        generator: 5,
        value: 1.0,
    });
    exercise(&Modulator {
        src1: 0,
        flags1: 0,
        src2: 0,
        flags2: 0,
        dest: 3,
        amount: 1.0,
    });
    exercise(&params);
    exercise(&SYNTH_CAPS);
    exercise(&negotiate(None, None, SYNTH_CAPS).expect("negotiated"));
    exercise(&pcm16(2, 22050));
    exercise(&Link::Detached);
    exercise(&Link::Closed);
    exercise(&Link::Open(&params));
    exercise(&RgnRange { low: 0, high: 127 });
    exercise(&WaveLoop {
        loop_type: 0,
        start: 0,
        length: 0,
    });
    exercise(&LoopSettings {
        sample_mode: None,
        start: 0,
        end: 0,
    });
    assert_ne!(Link::Detached, Link::Closed);
    assert_ne!(Link::Open(&params), Link::Detached);
}
