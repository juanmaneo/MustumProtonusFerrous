//! Replays the rows of the ASan/UBSan Wine oracle (verbatim wave.c and media_source.c) through the port.
//!
//! Each row is an input line of vectors.txt and its output line from the oracle run.

use crate::rate::{fastest_rate, is_rate_supported, set_rate, slowest_rate, RateDirection};
use crate::thin::{StreamEvent, StreamThinning};
use crate::wave::{WaveFormatEx, WaveSink};
use crate::{HResult, E_FAIL, S_OK};

#[rustfmt::skip]
const VECTORS: &[(&str, &str)] = &[
    ("wave|1,1,11025,8,0|s0102", "s=00000000|5249464600000000574156454A554E4B1C00000000000000000000000000000000000000000000000000000000000000666D74201200000001000100112B0000112B000001000800000064617461000000000102|84|2"),
    ("wave|1,1,11025,8,0|s0102030405;s;f", "s=00000000,s=00000000,f=00000000/00000000|524946464F000000574156454A554E4B1C00000000000000000000000000000000000000000000000000000000000000666D74201200000001000100112B0000112B000001000800000064617461050000000102030405|95|5"),
    ("wave|1,1,11025,8,0|s0102;f;f", "s=00000000,f=00000000/00000000,f=00000000/80070057|524946464C000000574156454A554E4B1C00000000000000000000000000000000000000000000000000000000000000666D74201200000001000100112B0000112B000001000800000064617461020000000102|92|2"),
    ("wave|1,1,11025,8,0|f;s0102", "f=00000000/00000000,s=00000000|5249464600000000574156454A554E4B1C00000000000000000000000000000000000000000000000000000000000000666D74201200000001000100112B0000112B000001000800000064617461000000000102|92|2"),
    ("wave|1,1,11025,8,0|d;d;s01", "d=00000000,d=C00D3E85,s=C00D4A38||0|0"),
    ("wave|1,2,4294967295,16,0|s01;f", "s=00000000,f=00000000/00000000|524946464B000000574156454A554E4B1C00000000000000000000000000000000000000000000000000000000000000666D74201200000001000200FFFFFFFFFCFFFF1F040010000000646174610100000001|91|1"),
    ("wave|3,2,44100,16,22|s0102;d;f", "s=00000000,d=00000000,f=00000000/00000000|524946464C000000574156454A554E4B1C00000000000000000000000000000000000000000000000000000000000000666D7420120000000300020044AC000010B1020004001000160064617461020000000102|92|2"),
    ("wave|1,1,11025,8,0|s01;f;s02", "s=00000000,f=00000000/00000000,s=00000000|524946464B000000574156454A554E4B1C00000000000000000000000000000000000000000000000000000000000000666D74201200000001000100112B0000112B000001000800000064617461010000000102|92|2"),
    ("rate|slow", "rate=00000000"),
    ("rate|fast|0", "rate=49742400"),
    ("rate|fast|1", "rate=C9742400"),
    ("rate|supported|1000001", "ret=C00D36D0 nearest=49742410"),
    ("rate|set|0|-1", "ret=C00D36D2 put=0 params=0"),
    ("rate|set|1|2", "ret=00000000 put=1 params=1"),
    ("rate|set|0|nan", "ret=C00D36D0 put=0 params=0"),
    ("thin|accept|1|0|ok|0", "accept=0"),
    ("thin|accept|0|1|fail|0", "accept=0"),
    ("thin|accept|0|1|ok|256", "accept=1"),
    ("thin|set|0|1", "thin=1 ev=T1"),
    ("thin|send|1|0|none", "thin=0 ev=T0 S"),
    ("thin|send|0|0|fail", "thin=0 ev=-"),
    ("thin|send|1|0|ok", "thin=0 ev=T0 S"),
    ("thin|send|1|1|ok", "thin=1 ev=S"),
];

#[test]
fn rows_match_wine_oracle() {
    for (row, expected) in VECTORS {
        assert_eq!(replay(row), *expected, "row {row}");
    }
}

fn replay(row: &str) -> String {
    let f: Vec<&str> = row.split('|').collect();
    if f[0] == "wave" {
        replay_wave(f[1], f[2])
    } else if f[0] == "rate" {
        replay_rate(&f)
    } else {
        replay_thin(&f)
    }
}

fn hr_of(result: Result<(), HResult>) -> HResult {
    match result {
        Ok(()) => S_OK,
        Err(hr) => hr,
    }
}

fn hex_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02X}")).collect()
}

/// `wave|<tag,ch,rate,bits,cb>|<ops>`, where each op is `s<hex>`, `f` or `d`.
fn replay_wave(fmt: &str, ops: &str) -> String {
    let p: Vec<u32> = fmt.split(',').map(|x| x.parse().unwrap()).collect();
    let fmt = WaveFormatEx {
        format_tag: p[0] as u16,
        channels: p[1] as u16,
        samples_per_sec: p[2],
        avg_bytes_per_sec: 0,
        block_align: 0,
        bits_per_sample: p[3] as u16,
        cb_size: p[4] as u16,
    }
    .with_derived_fields();
    let mut sink = WaveSink::new(fmt);
    let results: Vec<String> = ops
        .split(';')
        .map(|op| {
            if let Some(hex) = op.strip_prefix('s') {
                format!("s={:08X}", hr_of(sink.process_sample(&hex_bytes(hex))))
            } else if op == "f" {
                format!("f={S_OK:08X}/{:08X}", hr_of(sink.finalize()))
            } else {
                format!("d={:08X}", hr_of(sink.shutdown()))
            }
        })
        .collect();
    format!(
        "{}|{}|{}|{}",
        results.join(","),
        to_hex(sink.bytes()),
        sink.full_length(),
        sink.data_length()
    )
}

/// `rate|slow`, `rate|fast|<dir>`, `rate|supported|<rate>` or `rate|set|<thin>|<rate>`.
fn replay_rate(f: &[&str]) -> String {
    if f[1] == "slow" {
        format!("rate={:08X}", slowest_rate().to_bits())
    } else if f[1] == "fast" {
        let direction = if f[2] == "0" {
            RateDirection::Forward
        } else {
            RateDirection::Reverse
        };
        format!("rate={:08X}", fastest_rate(direction).to_bits())
    } else if f[1] == "supported" {
        let (hr, nearest) = is_rate_supported(f[2].parse().unwrap());
        format!("ret={hr:08X} nearest={:08X}", nearest.to_bits())
    } else {
        let hr = hr_of(set_rate(f[3].parse().unwrap()));
        let queued = u32::from(hr == S_OK);
        format!("ret={hr:08X} put={queued} params={queued}")
    }
}

fn events_text(events: &[StreamEvent]) -> String {
    if events.is_empty() {
        return "-".into();
    }
    events
        .iter()
        .map(|e| match e {
            StreamEvent::ThinMode(v) => format!("T{v}"),
            StreamEvent::Sample => "S".into(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// `thin|accept|<thin>|<stream>|<ok|fail>|<keyframe>`, `thin|set|<stream>|<new>`
/// or `thin|send|<stream>|<pending>|<none|ok|fail>`.
fn replay_thin(f: &[&str]) -> String {
    if f[1] == "accept" {
        let stream = StreamThinning {
            thin: f[3] == "1",
            pending_thin: false,
        };
        let clean_point = if f[4] == "ok" {
            Some(f[5].parse().unwrap())
        } else {
            None
        };
        format!(
            "accept={}",
            u8::from(stream.accepts_sample(f[2] == "1", clean_point))
        )
    } else if f[1] == "set" {
        let mut stream = StreamThinning {
            thin: f[2] == "1",
            pending_thin: false,
        };
        let events: Vec<StreamEvent> = stream.set_thin(f[3] == "1").into_iter().collect();
        format!("thin={} ev={}", u8::from(stream.thin), events_text(&events))
    } else {
        let mut stream = StreamThinning {
            thin: f[2] == "1",
            pending_thin: f[3] == "1",
        };
        let token = match f[4] {
            "none" => None,
            "fail" => Some(Err(E_FAIL)),
            _ => Some(Ok(())),
        };
        let events = stream.send_sample(token).unwrap_or_default();
        format!("thin={} ev={}", u8::from(stream.thin), events_text(&events))
    }
}
