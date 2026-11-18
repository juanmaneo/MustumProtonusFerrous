//! Tests against Wine's oracle vectors.
//!
//! The vectors are the replies of Wine's verbatim `resampler.c`, built with ASan and UBSan by
//! `target/canon/resampledmo/build.sh`. [`replay`] runs each script through the port and prints
//! the result the way the C harness does, so every vector is compared as the exact text.

use crate::format::{sample_format, AudioSpec, SampleFormat};
use crate::media::{
    check_media_type, get_input_type, get_output_type, FormatKind, Major, MediaType, Subtype,
};
use crate::resampler::{OutputData, Resampler, Status};
use crate::stub::{self, Stub};
use crate::time::{from_user, to_user};
use crate::Error;

/// Each script with the text the C harness prints for it.
#[rustfmt::skip]
const VECTORS: &[(&str, &str)] = &[
    ("tag 3 32", "3"),
    ("tag 1 24", "-1"),
    ("check audio wfx pcm 0100020080bb000000ee0200040010000000", "00000000 4"),
    ("check audio wfx other 0100020080bb000000ee0200040010000000", "80040202 -"),
    ("check audio wfx pcm 0100020080bb000000ee0200000010000000", "80040202 -"),
    ("enum 0 0 0", "00000000 18 030000000000000000000000000000000000"),
    ("enum 0 3 1", "00000000 18 0100020080bb000000ee0200040010000000"),
    ("enum 0 4 1", "80040206"),
    ("enum 1 0 0", "80040201"),
    ("wrap 1 16 2 4 8", "2 8"),
    ("wrap 1 16 2 2 8", "4 -1"),
    ("time_to 48000 96000", "20000000"),
    ("time_from 48000 10000000", "48000"),
    ("time_to 1 1000000000000", "-9223372036854775808"),
    ("out 0 0 audio wfx pcm 01000200c05d000000770100040010000000", "80040203"),
    ("in 0 0 null", "00000000"),
    ("in 1 0 audio wfx pcm 0100020080bb000000ee0200040010000000", "80040201"),
    ("in 0 1 audio wfx pcm 0100020080bb000000ee0200040010000000;out 0 1 audio wfx pcm 01000200c05d000000770100040010000000;pin 1000 0000000000000000", "00000000 ; 80040203 ; 80040203"),
    ("in 0 0 audio wfx pcm feff020080bb000000ee02000400100016001000030000000100000000001000800000aa00389b71;out 0 0 audio wfx pcm 01000200c05d000000770100040010000000", "00000000 ; 00000000"),
    ("in 0 0 audio wfx pcm feff020080bb000000ee02000400100016001000000000000100000000001000800000aa00389b71;out 0 0 audio wfx pcm 01000200c05d000000770100040010000000", "00000000 ; 80004005"),
    ("in 0 0 audio wfx pcm 0100020080bb000000ee0200040010000000;out 0 0 audio wfx pcm 01000200c05d000000770100040010000000;pin 1000 0000000000000000;stub 4 0;pout 16", "00000000 ; 00000000 ; 00000000 ; 00000000 16 7 1000 1667"),
    ("in 0 0 audio wfx pcm 0100020080bb000000ee0200040010000000;out 0 0 audio wfx pcm 01000200c05d000000770100040010000000;pin 1000 0000000000000000;stub 4 1;pout 16", "00000000 ; 00000000 ; 00000000 ; 00000000 16 16777223 1000 1667"),
    ("in 0 0 audio wfx pcm 0100020080bb000000ee0200040010000000;out 0 0 audio wfx pcm 01000200c05d000000770100040010000000;pin 1000 0000000000000000;stub 0 0;pout 16", "00000000 ; 00000000 ; 00000000 ; 00000001 0 0 0 0"),
    ("in 0 0 audio wfx pcm 0100020080bb000000ee0200040010000000;out 0 0 audio wfx pcm 01000200c05d000000770100040010000000;pin 1000 0000000000000000;stub -1 0;pout 16", "00000000 ; 00000000 ; 00000000 ; 80004005 0 0 0 0"),
    ("in 0 0 audio wfx pcm 0100020080bb000000ee0200040010000000;out 0 0 audio wfx pcm 01000200c05d000000770100040010000000;pin 1000 0000000000000000;stub 2 0;pout 16", "00000000 ; 00000000 ; 00000000 ; 00000000 8 7 1000 833"),
];

#[test]
fn oracle_vectors_match_wine() {
    for (script, expected) in VECTORS {
        assert_eq!(replay(script), *expected, "script: {script}");
    }
}

/// Runs `script` the way the C harness does: `;`-separated commands on one resampler, with the
/// printed results joined by ` ; `.
pub fn replay(script: &str) -> String {
    stub::reset();
    let mut resampler = Resampler::<Stub>::default();
    let results: Vec<String> = script
        .split(';')
        .filter_map(|command| run(&mut resampler, command))
        .collect();
    results.join(" ; ")
}

/// Runs one command. `stub` scripts the converter and prints nothing.
fn run(resampler: &mut Resampler<Stub>, command: &str) -> Option<String> {
    let words: Vec<&str> = command.split_whitespace().collect();
    let args = &words[1..];
    let text = match words[0] {
        "stub" => {
            stub::set(num(args[0]) as i32, num(args[1]) as i32);
            return None;
        }
        "tag" => {
            let format = sample_format(num(args[0]) as u32, num(args[1]) as u32);
            av_index(format).to_string()
        }
        "check" => match check_media_type(&parse_media(&args[..4])) {
            Ok(block_align) => format!("00000000 {block_align}"),
            Err(error) => format!("{:08x} -", error.hresult()),
        },
        "enum" => {
            let (stream, index) = (num(args[0]) as u32, num(args[1]) as u32);
            let media = if num(args[2]) == 0 {
                get_input_type(stream, index)
            } else {
                get_output_type(stream, index)
            };
            match media {
                Ok(media) => format!("00000000 {} {}", media.format.len(), hex(&media.format)),
                Err(error) => format!("{:08x}", error.hresult()),
            }
        }
        "wrap" => {
            let format = sample_format(num(args[0]) as u32, num(args[1]) as u32)
                .expect("wrap oracle rows use valid formats");
            let spec = AudioSpec {
                format,
                channels: num(args[2]) as u32,
                sample_rate: 48000,
                block_align: num(args[3]) as u32,
            };
            let len = num(args[4]) as usize;
            let (samples, size) = match spec.wrap(len) {
                Some((samples, size)) => (samples, size as i64),
                None => (len / spec.block_align as usize, -1),
            };
            format!("{samples} {size}")
        }
        "time_to" => to_user(num(args[0]), num(args[1])).to_string(),
        "time_from" => from_user(num(args[0]), num(args[1])).to_string(),
        "in" | "out" => {
            let stream = num(args[0]) as u32;
            let flags = num(args[1]) as u32;
            let media = (args[2] != "null").then(|| parse_media(&args[2..6]));
            let result = if words[0] == "in" {
                resampler.set_input_type(stream, media.as_ref(), flags)
            } else {
                resampler.set_output_type(stream, media.as_ref(), flags)
            };
            hresult(result)
        }
        "pin" => hresult(resampler.process_input(&unhex(args[1]), num(args[0]))),
        "pout" => {
            let mut buffer = vec![0; num(args[0]) as usize];
            let mut output = OutputData::default();
            let result = resampler.process_output(1, &mut buffer, &mut output);
            format!(
                "{:08x} {} {} {} {}",
                result.map_or_else(Error::hresult, Status::hresult),
                output.length,
                output.status,
                output.timestamp,
                output.timelength
            )
        }
        other => panic!("unknown oracle command {other}"),
    };
    Some(text)
}

/// The `AVSampleFormat` number of a sample format, or -1 for `AV_SAMPLE_FMT_NONE`.
fn av_index(format: Option<SampleFormat>) -> i64 {
    match format {
        None => -1,
        Some(SampleFormat::U8) => 0,
        Some(SampleFormat::S16) => 1,
        Some(SampleFormat::S32) => 2,
        Some(SampleFormat::Flt) => 3,
        Some(SampleFormat::Dbl) => 4,
    }
}

/// The media type of the four words `major format subtype hex`.
fn parse_media(words: &[&str]) -> MediaType {
    MediaType {
        major: if words[0] == "audio" {
            Major::Audio
        } else {
            Major::Other
        },
        format_kind: if words[1] == "wfx" {
            FormatKind::WaveFormatEx
        } else {
            FormatKind::Other
        },
        subtype: match words[2] {
            "pcm" => Subtype::Pcm,
            "float" => Subtype::Float,
            _ => Subtype::Other,
        },
        format: unhex(words[3]),
    }
}

/// The text of a call that returns only an `HRESULT`.
fn hresult(result: Result<(), Error>) -> String {
    format!("{:08x}", result.map_or_else(Error::hresult, |()| 0))
}

/// The bytes of a hex string. The harness prints `-` for an empty buffer and reads it back that way.
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

fn num(text: &str) -> i64 {
    text.parse().expect("oracle numbers are decimal")
}
