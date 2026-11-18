//! Tests against Wine's oracle vectors.
//!
//! The vectors are the replies of Wine's verbatim `effect_SetInputType` and `effect_SetOutputType`
//! (with the struct and `impl_from_IMediaObject` they use), built with ASan and UBSan by
//! `target/canon/dsdmo/build.sh`. [`replay`] runs each script through the port and prints the
//! result the way the C driver does, so every vector is compared as the exact text.

use crate::effect::TypeState;
use crate::media::{FormatKind, Major, MediaType, Subtype};
use crate::Error;

/// Each script with the text the C driver prints for it. No vector has a `pbFormat` shorter than
/// the 18-byte header: the C code reads past such a buffer, which ASan reports as a
/// heap-buffer-overflow, so those inputs are left out.
#[rustfmt::skip]
const VECTORS: &[(&str, &str)] = &[
    ("in 0 audio pcm wfx 0100020080bb000000ee0200040010000000", "00000000"),
    ("in 0 audio float wfx 0300020080bb000000dc0500080020000000", "00000000"),
    ("in 0 audio pcm wfx 0300020080bb000000dc0500080020000000", "00000000"),
    ("in 0 audio pcm wfx 0100010080bb000080bb0000010008000000", "00000000"),
    ("in 0 audio pcm wfx 0100010080bb000000770100020010000000", "00000000"),
    ("in 0 audio pcm wfx 0100020080bb000000650400060018000000", "80040205"),
    ("in 0 audio pcm wfx 0100020080bb000000dc0500080020000000", "80040205"),
    ("in 0 audio float wfx 0300020080bb000000b80b00100040000000", "80040205"),
    ("in 0 audio pcm wfx feff020080bb000000ee0200040010000000", "80040205"),
    ("in 0 audio pcm wfx 0100030080bb000000650400060010000000", "80040205"),
    ("in 0 audio pcm wfx 0100000080bb000000000000000010000000", "80040205"),
    ("in 0 other pcm wfx 0100020080bb000000ee0200040010000000", "80040205"),
    ("in 0 audio other wfx 0100020080bb000000ee0200040010000000", "80040205"),
    ("in 0 audio pcm other 0100020080bb000000ee0200040010000000", "80040205"),
    ("out 0 audio pcm wfx 0100020080bb000000ee0200040010000000", "80040205"),
    ("out 0 audio pcm wfx 000000000000000000000000000000000000", "00000000"),
    ("out 0 other pcm wfx 000000000000000000000000000000000000", "80040205"),
    ("in 0 audio pcm wfx 0100020080bb000000ee0200040010000000 ; out 0 audio pcm wfx 0100020080bb000000ee0200040010000000", "00000000 ; 00000000"),
    ("in 0 audio pcm wfx 0100020080bb000000ee0200040010000000 ; out 0 audio float wfx 0100020080bb000000ee0200040010000000", "00000000 ; 00000000"),
    ("in 0 audio pcm wfx 0100020080bb000000ee0200040010000000 ; out 0 audio float wfx 0300020080bb000000dc0500080020000000", "00000000 ; 80040205"),
    ("in 0 audio pcm wfx 0100020080bb000000ee0200040010000000 ; in 2 audio pcm wfx - ; state", "00000000 ; 00000000 ; 000000000000000000000000000000000000"),
    ("in 0 audio pcm wfx 0100020080bb000000ee0200040010000000 ; in 0 audio float wfx 0300020080bb000000b80b00100040000000 ; state", "00000000 ; 80040205 ; 0100020080bb000000ee0200040010000000"),
    ("in 0 audio pcm wfx 0100020080bb000000ee0200040010000000 ; out 2 audio pcm wfx 0300020080bb000000dc0500080020000000 ; out 2 other other other -", "00000000 ; 00000000 ; 00000000"),
    ("in 1 audio pcm wfx 0100020080bb000000ee0200040010000000 ; state", "00000000 ; 0100020080bb000000ee0200040010000000"),
    ("in 0 audio pcm wfx 0100020080bb000000ee0200040010000000 ; in 0 audio pcm wfx 0100010080bb000080bb0000010008000000 ; state", "00000000 ; 00000000 ; 0100010080bb000080bb0000010008000000"),
];

#[test]
fn oracle_vectors_match_wine() {
    for (script, expected) in VECTORS {
        assert_eq!(replay(script), *expected, "script: {script}");
    }
}

/// Runs `script` the way the C driver does: `;`-separated commands on one effect, with the printed
/// results joined by ` ; `.
pub fn replay(script: &str) -> String {
    let mut state = TypeState::default();
    let results: Vec<String> = script
        .split(';')
        .map(|command| run(&mut state, command))
        .collect();
    results.join(" ; ")
}

/// Runs one command: `in|out FLAGS MAJOR SUB KIND HEX`, or `state`.
fn run(state: &mut TypeState, command: &str) -> String {
    let words: Vec<&str> = command.split_whitespace().collect();
    match words[0] {
        "state" => hex(&state.format().to_bytes()),
        "in" | "out" => {
            let flags = num(words[1]);
            let format = unhex(words[5]);
            let media = parse_media(&words[2..5], &format);
            let result = if words[0] == "in" {
                state.set_input_type(flags, &media)
            } else {
                state.set_output_type(flags, &media)
            };
            hresult(result)
        }
        other => panic!("unknown oracle command {other}"),
    }
}

/// The media type of the three words `major subtype kind`, over `format`.
fn parse_media<'a>(words: &[&str], format: &'a [u8]) -> MediaType<'a> {
    MediaType {
        major: if words[0] == "audio" {
            Major::Audio
        } else {
            Major::Other
        },
        subtype: match words[1] {
            "pcm" => Subtype::Pcm,
            "float" => Subtype::Float,
            _ => Subtype::Other,
        },
        format_kind: if words[2] == "wfx" {
            FormatKind::WaveFormatEx
        } else {
            FormatKind::Other
        },
        format,
    }
}

/// The text of a call that returns only an `HRESULT`.
fn hresult(result: Result<(), Error>) -> String {
    format!("{:08x}", result.map_or_else(Error::hresult, |()| 0))
}

/// The bytes of a hex string. The driver prints `-` for an empty buffer and reads it back that way.
fn unhex(text: &str) -> Vec<u8> {
    if text == "-" {
        return Vec::new();
    }
    text.as_bytes()
        .chunks(2)
        .map(|pair| {
            let digits = std::str::from_utf8(pair).expect("hex digits are ASCII");
            u8::from_str_radix(digits, 16).expect("hex digits")
        })
        .collect()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn num(text: &str) -> u32 {
    text.parse().expect("oracle flags are decimal")
}
