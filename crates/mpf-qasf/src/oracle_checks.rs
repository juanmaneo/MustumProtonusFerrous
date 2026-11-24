//! Tests against Wine's oracle vectors.
//!
//! The vectors are the output of Wine's verbatim `DecideBufferSize`, the `WMT_OPENED` stream count
//! and name, `OnSample`, the `Receive` timing and the `process_output` loop, built with ASan and
//! UBSan by `target/canon/qasf/build.sh` from `oracle.c`. [`replay`] runs each command through the
//! port and prints the result the way the C driver does, so every vector is compared as exact text.

use crate::allocator::{decide_buffer_size, AllocatorProperties, Format, Subtype};
use crate::output::{plan_output, OutputBuffer, OutputPlan};
use crate::sample::{input_buffer, stream_sample, InputTime};
use crate::stream::{open_stream_count, raw_stream_name, Major};
use crate::SampleTime;
use std::fmt::Debug;
use std::str::FromStr;

/// Each command with the text the C driver prints for it. Vectors that UBSan reports as signed
/// overflow are left out: a `plan` whose timestamp plus timelength overflows, an `input` with no
/// stop at `i64::MAX`, and an `input` whose stop minus start overflows.
#[rustfmt::skip]
const VECTORS: &[(&str, &str)] = &[
    ("buffer video - 1000 1 0 1", "1 1000 1"),
    ("buffer video - 65536 2 0 4", "2 65536 4"),
    ("buffer video - 20000 0 30000 0", "1 30000 1"),
    ("buffer wave float 88200 4 20000 8", "4 88200 8"),
    ("buffer wave pcm 88200 4 20000 8", "4 88200 8"),
    ("buffer wave other 192000 1 0 1", "1 16384 1"),
    ("buffer video - 0 -1 -5 -7", "1 -5 1"),
    ("buffer video - 4294967295 0 0 0", "1 -1 1"),
    ("buffer other pcm 1234 1 0 1", "1 16384 1"),
    ("open ok 0", "0"),
    ("open ok 17", "16"),
    ("open fail 5", "0"),
    ("name video 0", "Raw Video 0"),
    ("name audio 4294967295", "Raw Audio 4294967295"),
    ("sample 1 1000 2000 0", "1000 3000 0 0"),
    ("sample 0 100 50 3", "discard"),
    ("sample 1 18446744073709551615 1 2", "-1 0 1 0"),
    ("sample 1 100 50 3", "100 150 1 1"),
    ("input ok 100 500 1", "7 100 500 400"),
    ("input nostop 100 0 0", "6 100 101 1"),
    ("input fail 100 500 1", "1 0 0 0"),
    ("plan 0/0/0/0", "more=0 0,-,0,0,0"),
    ("plan 3/100/50/64", "more=0 64,100,1,1,0"),
    ("plan 7/100/50/64 -", "more=0 64,100..150,1,1,0 skip"),
    ("plan 16777216/5/0/9 - 4/1/2/0", "more=1 9,-,0,1,1 skip 0,-,0,0,0"),
];

#[test]
fn oracle_vectors_match_wine() {
    for (command, expected) in VECTORS {
        assert_eq!(replay(command), *expected, "command: {command}");
    }
}

#[test]
#[should_panic(expected = "unknown oracle command")]
fn unknown_oracle_command_is_refused() {
    replay("bogus 1");
}

/// Runs one oracle command and returns the text the C driver prints for it.
pub fn replay(command: &str) -> String {
    let words: Vec<&str> = command.split_whitespace().collect();
    match words[0] {
        "buffer" => buffer(&words[1..]),
        "open" => {
            let count = (words[1] == "ok").then(|| number(words[2]));
            open_stream_count(count).to_string()
        }
        "name" => {
            let major = if words[1] == "video" {
                Major::Video
            } else {
                Major::Other
            };
            raw_stream_name(major, number(words[2]))
        }
        "sample" => {
            let connected = number::<u32>(words[1]) != 0;
            match stream_sample(
                connected,
                number(words[2]),
                number(words[3]),
                number(words[4]),
            ) {
                None => "discard".to_string(),
                Some(sample) => format!(
                    "{} {} {} {}",
                    sample.time.start,
                    sample.time.stop.expect("a stream sample has a stop"),
                    u8::from(sample.discontinuity),
                    u8::from(sample.sync_point)
                ),
            }
        }
        "input" => {
            let time = match words[1] {
                "ok" => InputTime::Range {
                    start: number(words[2]),
                    stop: number(words[3]),
                },
                "nostop" => InputTime::NoStop {
                    start: number(words[2]),
                },
                _ => InputTime::Unset,
            };
            let buffer = input_buffer(number::<u32>(words[4]) != 0, time);
            format!(
                "{} {} {} {}",
                buffer.flags,
                buffer.start,
                buffer.start.wrapping_add(buffer.length),
                buffer.length
            )
        }
        "plan" => {
            let buffers: Vec<Option<OutputBuffer>> = words[1..]
                .iter()
                .map(|token| (*token != "-").then(|| output_buffer(token)))
                .collect();
            let round = plan_output(&buffers);
            let mut fields = vec![format!("more={}", u8::from(round.more_data))];
            fields.extend(round.plans.iter().map(|plan| match plan {
                None => "skip".to_string(),
                Some(plan) => plan_text(*plan),
            }));
            fields.join(" ")
        }
        other => panic!("unknown oracle command {other}"),
    }
}

/// Runs `buffer FORMAT SUBTYPE VALUE COUNT SIZE ALIGN` and prints the properties it decides.
fn buffer(words: &[&str]) -> String {
    let format = match words[0] {
        "video" => Format::Video {
            size_image: number(words[2]),
        },
        "wave" => Format::Wave {
            avg_bytes_per_sec: number(words[2]),
            subtype: subtype(words[1]),
        },
        _ => Format::Other,
    };
    let req = AllocatorProperties {
        buffers: number(words[3]),
        buffer_size: number(words[4]),
        align: number(words[5]),
        prefix: 0,
    };
    let props = decide_buffer_size(format, req);
    format!("{} {} {}", props.buffers, props.buffer_size, props.align)
}

fn subtype(text: &str) -> Subtype {
    match text {
        "pcm" => Subtype::Pcm,
        "float" => Subtype::Float,
        _ => Subtype::Other,
    }
}

/// Reads one output buffer from `STATUS/TIMESTAMP/TIMELENGTH/LEN`.
fn output_buffer(token: &str) -> OutputBuffer {
    let fields: Vec<&str> = token.split('/').collect();
    OutputBuffer {
        status: number(fields[0]),
        timestamp: number(fields[1]),
        timelength: number(fields[2]),
        len: number(fields[3]),
    }
}

/// The text the C driver prints for one planned sample: its length, its time, then its sync point,
/// delivery and incomplete flags.
fn plan_text(plan: OutputPlan) -> String {
    let time = match plan.time {
        None => "-".to_string(),
        Some(SampleTime { start, stop: None }) => start.to_string(),
        Some(SampleTime {
            start,
            stop: Some(stop),
        }) => format!("{start}..{stop}"),
    };
    format!(
        "{},{},{},{},{}",
        plan.len,
        time,
        u8::from(plan.sync_point),
        u8::from(plan.deliver),
        u8::from(plan.incomplete)
    )
}

fn number<T: FromStr>(text: &str) -> T
where
    T::Err: Debug,
{
    text.parse().expect("oracle numbers")
}
