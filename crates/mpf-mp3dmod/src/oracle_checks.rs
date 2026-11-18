//! Tests against Wine's oracle rows. Each row is the argument list for the Wine-verbatim harness in
//! `target/canon/mp3dmod` (see `gen.sh`) and the line the harness printed for it. [`run`] replays a
//! row through the port and prints the result the way the harness does.
//!
//! The row `frame 2 0` is not listed: the sanitizer reports a division by zero in Wine's
//! `get_frametime` there. The harness passes the PCM subtype when a flag is zero. `GUID_NULL` stands
//! in for it, since both are rejected the same way.

use crate::frame::{framesize, frametime, input_size_info, output_size_info, SizeInfo};
use crate::hresult::convert_dmo_to_mf_error;
use crate::mf::{output_available_type, MfSubtype};
use crate::types::{
    output_type, set_input_type, set_output_type, Encoding, Guid, WaveFormat, GUID_NULL,
    WMFORMAT_WAVEFORMATEX, WMMEDIASUBTYPE_MP3, WMMEDIATYPE_AUDIO,
};
use crate::Hresult;

#[rustfmt::skip]
const ROWS: &[(&str, &str)] = &[
    ("frame 2 44100", "framesize=2304 frametime=261224"),
    ("size_in 0 1 1", "hr=0 size=0 align=1"),
    ("size_in 0 1 0", "hr=0x80040203"),
    ("size_out 0 1 1 16", "hr=0 size=4608 align=1"),
    ("in 0 1 1 1 2", "hr=0"),
    ("in 0 0 1 1 2", "hr=0x80040205"),
    ("out_type 0 1 2 44100 0", "hr=0 tag=1 ch=2 rate=44100 bits=16 block=4 avg=176400"),
    ("out_type 0 1 2 44100 3", "hr=0 tag=1 ch=1 rate=44100 bits=8 block=1 avg=44100"),
    ("out_type 0 1 2 44100 4", "hr=0x80040206"),
    ("set_out 0 1 44100 1 16 2 44100 4 176400", "hr=0 enc=Signed16"),
    ("set_out 0 1 44100 1 8 1 22050 1 22050", "hr=0 enc=Unsigned8"),
    ("set_out 0 1 44100 1 32 2 11025 8 88200", "hr=0 enc=Float32"),
    ("set_out 0 1 44100 1 24 2 44100 6 264600", "hr=0x80040205 enc=none"),
    ("set_out 0 1 44100 1 16 2 44100 3 132300", "hr=0x80070057 enc=none"),
    ("set_out 0 1 44100 1 16 2 48000 4 192000", "hr=0x80040205 enc=none"),
    ("mf_avail 0 1 2 44100 0", "hr=0 major=audio subtype=float bits=32 channels=2 rate=44100 avg=352800 block=8 indep=1"),
    ("mf_avail 0 1 2 44100 2", "hr=0 major=audio subtype=pcm bits=8 channels=2 rate=44100 avg=88200 block=2 indep=1"),
    ("mf_avail 0 1 1 8000 3", "hr=0xc00d36b9"),
    ("mf_avail 1 1 2 44100 0", "hr=0xc00d36b3"),
    ("dmo2mf 0x80040201", "hr=0xc00d36b3"),
    ("dmo2mf 0x80040203", "hr=0xc00d6d60"),
];

#[test]
fn oracle_rows_match_wine() {
    for (row, expected) in ROWS {
        assert_eq!(run(row), *expected, "row: {row}");
    }
}

/// Runs `row` through the port and returns the line the C harness prints for it.
fn run(row: &str) -> String {
    let words: Vec<&str> = row.split_whitespace().collect();
    let a: Vec<u32> = words[1..].iter().copied().map(number).collect();
    match words[0] {
        "frame" => {
            let format = WaveFormat {
                block_align: a[0] as u16,
                samples_per_sec: a[1],
                ..WaveFormat::default()
            };
            format!(
                "framesize={} frametime={}",
                framesize(&format),
                frametime(&format).unwrap()
            )
        }
        "size_in" => {
            let format = WaveFormat::default();
            size_text(input_size_info(
                a[0],
                set_if(a[1], &format),
                set_if(a[2], &format),
            ))
        }
        "size_out" => {
            let input = WaveFormat::default();
            let output = WaveFormat {
                bits_per_sample: a[3] as u16,
                ..WaveFormat::default()
            };
            size_text(output_size_info(
                a[0],
                set_if(a[1], &input),
                set_if(a[2], &output),
            ))
        }
        "in" => {
            let format = WaveFormat {
                channels: a[4] as u16,
                ..WaveFormat::default()
            };
            let result = set_input_type(
                a[0],
                guid_if(a[1], WMMEDIATYPE_AUDIO),
                guid_if(a[2], WMMEDIASUBTYPE_MP3),
                guid_if(a[3], WMFORMAT_WAVEFORMATEX),
                &format,
            );
            match result {
                Ok(()) => hr(0),
                Err(code) => hr(code),
            }
        }
        "out_type" => {
            let input = WaveFormat {
                channels: a[2] as u16,
                samples_per_sec: a[3],
                ..WaveFormat::default()
            };
            match output_type(a[0], set_if(a[1], &input), a[4]) {
                Ok(f) => format!(
                    "hr=0 tag={} ch={} rate={} bits={} block={} avg={}",
                    f.format_tag,
                    f.channels,
                    f.samples_per_sec,
                    f.bits_per_sample,
                    f.block_align,
                    f.avg_bytes_per_sec
                ),
                Err(code) => hr(code),
            }
        }
        "set_out" => {
            let input = WaveFormat {
                samples_per_sec: a[2],
                ..WaveFormat::default()
            };
            let output = WaveFormat {
                channels: a[5] as u16,
                bits_per_sample: a[4] as u16,
                samples_per_sec: a[6],
                block_align: a[7] as u16,
                avg_bytes_per_sec: a[8],
                ..WaveFormat::default()
            };
            let result = set_output_type(
                a[0],
                set_if(a[1], &input),
                guid_if(a[3], WMFORMAT_WAVEFORMATEX),
                &output,
            );
            match result {
                Ok(encoding) => format!("hr=0 enc={}", encoding_name(encoding)),
                Err(code) => format!("{} enc=none", hr(code)),
            }
        }
        "mf_avail" => {
            let input = WaveFormat {
                channels: a[2] as u16,
                samples_per_sec: a[3],
                ..WaveFormat::default()
            };
            match output_available_type(a[0], set_if(a[1], &input), a[4]) {
                Ok(t) => format!(
                    "hr=0 major=audio subtype={} bits={} channels={} rate={} avg={} block={} indep={}",
                    subtype_name(t.subtype),
                    t.bits_per_sample,
                    t.channels,
                    t.samples_per_sec,
                    t.avg_bytes_per_sec,
                    t.block_alignment,
                    u8::from(t.all_samples_independent)
                ),
                Err(code) => hr(code),
            }
        }
        _ => hr(convert_dmo_to_mf_error(a[0])),
    }
}

fn number(word: &str) -> u32 {
    match word.strip_prefix("0x") {
        Some(hex) => u32::from_str_radix(hex, 16).unwrap(),
        None => word.parse().unwrap(),
    }
}

/// The GUID the harness passes for a flag: `value` when the flag is set, else `GUID_NULL`.
fn guid_if(flag: u32, value: Guid) -> Guid {
    if flag != 0 {
        value
    } else {
        GUID_NULL
    }
}

/// The type the harness passes for a flag: `format` when the flag is set, else no type.
fn set_if(flag: u32, format: &WaveFormat) -> Option<&WaveFormat> {
    (flag != 0).then_some(format)
}

/// The `hr=` text the harness prints: `0` for success, otherwise the code in hex.
fn hr(code: Hresult) -> String {
    if code == 0 {
        "hr=0".to_owned()
    } else {
        format!("hr={code:#x}")
    }
}

fn size_text(result: Result<SizeInfo, Hresult>) -> String {
    match result {
        Ok(info) => format!("hr=0 size={} align={}", info.size, info.alignment),
        Err(code) => hr(code),
    }
}

fn encoding_name(encoding: Encoding) -> &'static str {
    match encoding {
        Encoding::Unsigned8 => "Unsigned8",
        Encoding::Signed16 => "Signed16",
        Encoding::Float32 => "Float32",
    }
}

fn subtype_name(subtype: MfSubtype) -> &'static str {
    match subtype {
        MfSubtype::Pcm => "pcm",
        MfSubtype::Float => "float",
    }
}
