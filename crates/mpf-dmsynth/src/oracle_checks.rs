//! Replays the commands of the Wine oracle against this crate. The oracle compiles Wine's `synth.c`
//! and `synthsink.c` verbatim under ASan/UBSan, and its driver prints one result line per command.
//! `VECTORS` holds the command and output lines as the driver printed them.

use crate::dls::Connection;
use crate::format::{synth_format, WAVEFORMATEX_SIZE};
use crate::generator::{channel_attenuation, gen_for_destination, gen_setting};
use crate::modulator::{mod_source, modulator};
use crate::port::{negotiate, PortParams, SYNTH_CAPS, SYNTH_DEFAULTS};
use crate::region::{
    drum_fallback, loop_settings, noteon_patch, region_attenuation, region_matches, RgnRange,
    WaveLoop,
};
use crate::sink::{desired_buffer_size, ref_time_to_sample, sample_to_ref_time, sink_format, Link};

fn uarg(text: &str) -> u64 {
    text.parse().expect("unsigned number")
}

fn int(text: &str) -> i64 {
    text.parse().expect("signed number")
}

/// The parameters of an open sink. Only `open` reads its `ch rate` arguments.
fn sink_params(mode: &str, args: &[&str]) -> PortParams {
    match mode {
        "open" => PortParams {
            audio_channels: uarg(args[0]) as u32,
            sample_rate: uarg(args[1]) as u32,
            ..SYNTH_DEFAULTS
        },
        _ => SYNTH_DEFAULTS,
    }
}

fn sink_link<'a>(mode: &str, params: &'a PortParams) -> Link<'a> {
    match mode {
        "none" => Link::Detached,
        "open" => Link::Open(params),
        _ => Link::Closed,
    }
}

fn connection(tok: &[&str]) -> Connection {
    Connection {
        source: uarg(tok[1]) as u16,
        control: uarg(tok[2]) as u16,
        destination: uarg(tok[3]) as u16,
        transform: uarg(tok[4]) as u16,
        scale: int(tok[5]) as i32,
    }
}

fn negotiate_text(tok: &[&str]) -> String {
    let current = ((int(tok[1]) as i32) != 0).then_some(&SYNTH_DEFAULTS);
    let requested = (tok.len() == 11).then(|| PortParams {
        size: uarg(tok[2]) as u32,
        valid_params: uarg(tok[3]) as u32,
        voices: uarg(tok[4]) as u32,
        channel_groups: uarg(tok[5]) as u32,
        audio_channels: uarg(tok[6]) as u32,
        sample_rate: uarg(tok[7]) as u32,
        effect_flags: uarg(tok[8]) as u32,
        share: int(tok[9]) as i32,
        features: uarg(tok[10]) as u32,
    });
    match negotiate(current, requested.as_ref(), SYNTH_CAPS) {
        Ok(opened) => {
            let p = opened.params;
            format!(
                "{:08x} {} {} {} {} {} {} {} {} {}",
                opened.hresult(),
                p.size,
                p.valid_params,
                p.voices,
                p.channel_groups,
                p.audio_channels,
                p.sample_rate,
                p.effect_flags,
                p.share,
                p.features
            )
        }
        Err(error) => format!("{:08x}", error.hresult()),
    }
}

fn format_text(tok: &[&str]) -> String {
    let params = PortParams {
        audio_channels: uarg(tok[2]) as u32,
        sample_rate: uarg(tok[3]) as u32,
        ..SYNTH_DEFAULTS
    };
    let current = ((int(tok[1]) as i32) != 0).then_some(&params);
    match synth_format(current) {
        Ok(f) => format!(
            "00000000 {} {} {} {} {} {} {WAVEFORMATEX_SIZE}",
            f.channels,
            f.samples_per_sec,
            f.avg_bytes_per_sec,
            f.block_align,
            f.bits_per_sample,
            f.cb_size
        ),
        Err(error) => format!("{:08x}", error.hresult()),
    }
}

fn sinkfmt_text(tok: &[&str]) -> String {
    let params = sink_params(tok[1], &tok[2..]);
    let f = sink_format(sink_link(tok[1], &params));
    format!(
        "{} {} {} {} {} {}",
        f.channels,
        f.samples_per_sec,
        f.avg_bytes_per_sec,
        f.block_align,
        f.bits_per_sample,
        f.cb_size
    )
}

fn desired_text(tok: &[&str]) -> String {
    let params = sink_params(tok[1], &tok[2..]);
    match desired_buffer_size(sink_link(tok[1], &params)) {
        Ok(size) => format!("00000000 {size}"),
        Err(error) => format!("{:08x}", error.hresult()),
    }
}

fn s2r_text(tok: &[&str]) -> String {
    let params = sink_params(tok[3], &tok[4..]);
    match sample_to_ref_time(int(tok[1]), int(tok[2]), sink_link(tok[3], &params)) {
        Some(time) => format!("00000000 {time}"),
        None => "-".to_string(),
    }
}

fn r2s_text(tok: &[&str]) -> String {
    let params = sink_params(tok[3], &tok[4..]);
    let sample = ref_time_to_sample(int(tok[1]), int(tok[2]), sink_link(tok[3], &params));
    format!("00000000 {sample}")
}

fn gen_text(tok: &[&str]) -> String {
    match gen_for_destination(uarg(tok[1]) as u16) {
        Some(gen) => gen.to_string(),
        None => "-".to_string(),
    }
}

fn set_text(tok: &[&str]) -> String {
    match gen_setting(&connection(tok)) {
        Some(setting) => format!("{} {:016x}", setting.generator, setting.value.to_bits()),
        None => "-".to_string(),
    }
}

fn ctrl_text(tok: &[&str]) -> String {
    match mod_source(uarg(tok[1]) as u16, uarg(tok[2]) as u16) {
        Some((source, flags)) => format!("{source} {flags}"),
        None => "-".to_string(),
    }
}

fn mod_text(tok: &[&str]) -> String {
    match modulator(&connection(tok)) {
        Some(m) => format!(
            "{} {} {} {} {} {:016x}",
            m.src1,
            m.flags1,
            m.src2,
            m.flags2,
            m.dest,
            m.amount.to_bits()
        ),
        None => "-".to_string(),
    }
}

fn chan_text(tok: &[&str]) -> String {
    let attenuation = channel_attenuation(int(tok[1]) as i32, int(tok[2]) as i32);
    format!("{:016x}", attenuation.to_bits())
}

fn region_text(tok: &[&str]) -> String {
    let [first, second] = region_attenuation(int(tok[1]) as i32).map(f64::to_bits);
    format!("{first:016x} {second:016x}")
}

fn patch_text(tok: &[&str]) -> String {
    let patch = noteon_patch(int(tok[1]) as i32, int(tok[2]) as i32, int(tok[3]) as i32);
    format!("{patch:08x}")
}

fn fallback_text(tok: &[&str]) -> String {
    let found = (int(tok[2]) as i32) != 0;
    match drum_fallback(uarg(tok[1]) as u32, found) {
        Some(patch) => format!("{patch:08x}"),
        None => "-".to_string(),
    }
}

fn match_text(tok: &[&str]) -> String {
    let key_range = RgnRange {
        low: uarg(tok[3]) as u16,
        high: uarg(tok[4]) as u16,
    };
    let vel_range = RgnRange {
        low: uarg(tok[5]) as u16,
        high: uarg(tok[6]) as u16,
    };
    let key = int(tok[1]) as i32;
    let vel = int(tok[2]) as i32;
    i32::from(region_matches(key_range, vel_range, key, vel)).to_string()
}

fn loop_text(tok: &[&str]) -> String {
    let first = WaveLoop {
        loop_type: uarg(tok[2]) as u32,
        start: uarg(tok[3]) as u32,
        length: uarg(tok[4]) as u32,
    };
    match loop_settings(uarg(tok[1]) as u32, first) {
        Some(settings) => {
            let mode = match settings.sample_mode {
                Some(mode) => mode.to_string(),
                None => "-".to_string(),
            };
            format!("{mode} {} {}", settings.start, settings.end)
        }
        None => "none".to_string(),
    }
}

/// Replays one oracle command line and returns the line the driver prints for it.
pub(super) fn replay(line: &str) -> String {
    let tok: Vec<&str> = line.split_whitespace().collect();
    let op = tok.first().copied().unwrap_or("");
    match (op, tok.len()) {
        ("negotiate", 3 | 11) => negotiate_text(&tok),
        ("format", 4) => format_text(&tok),
        ("sinkfmt", 2 | 4) => sinkfmt_text(&tok),
        ("desired", 2 | 4) => desired_text(&tok),
        ("s2r", 4 | 6) => s2r_text(&tok),
        ("r2s", 4 | 6) => r2s_text(&tok),
        ("gen", 2) => gen_text(&tok),
        ("set", 6) => set_text(&tok),
        ("ctrl", 3) => ctrl_text(&tok),
        ("mod", 6) => mod_text(&tok),
        ("chan", 3) => chan_text(&tok),
        ("region", 2) => region_text(&tok),
        ("patch", 4) => patch_text(&tok),
        ("fallback", 3) => fallback_text(&tok),
        ("match", 7) => match_text(&tok),
        ("loop", 5) => loop_text(&tok),
        _ => "bad-op".to_string(),
    }
}

#[rustfmt::skip]
const VECTORS: &[(&str, &str)] = &[
    ("negotiate 0 none", "00000000 36 239 32 2 2 22050 1 0 0"),
    ("negotiate 1 none", "8878112f"),
    ("negotiate 0 31 239 32 2 2 22050 1 0 0", "80070057"),
    ("negotiate 0 36 239 0 0 4 4000 3 1 5", "00000001 36 239 1 1 2 11025 3 0 1"),
    ("negotiate 0 35 239 40 3 2 96001 7 0 7", "00000001 36 111 40 3 2 96000 7 0 0"),
    ("format 1 65536 22050", "00000000 0 22050 0 0 16 0 18"),
    ("format 1 2 4294967295", "00000000 2 4294967295 4294967292 4 16 0 18"),
    ("desired closed 2 22050", "8000ffff"),
    ("desired open 3 16000", "00000000 192000"),
    ("s2r 0 -3 open 2 11025", "00000000 -2000"),
    ("r2s 0 -1000000 open 2 11025", "00000000 -1102"),
    ("gen 128", "15"),
    ("set 9 0 3 16384 65536", "6 3ff0000000000000"),
    ("set 0 0 782 0 -2147483648", "29 40e07d0000000000"),
    ("ctrl 138 48", "10 19"),
    ("ctrl 256 0", "16 0"),
    ("mod 1 8 1 0 655360", "13 0 0 0 13 3ff0000000000000"),
    ("mod 0 0 782 0 65536", "0 0 0 0 29 bff0000000000000"),
    ("mod 129 0 1 16384 65536", "1 18 0 0 48 3ff0000000000000"),
    ("chan 0 600", "c04e000000000000"),
    ("patch -1 0 3", "ffffffff"),
    ("fallback 2147483653 0", "80000000"),
    ("match 0 0 0 0 0 0", "1"),
    ("loop 2 1 4294967295 1", "3 4294967295 0"),
    ("bogus 1", "bad-op"),
];

#[test]
fn oracle_vectors_match_wine() {
    for (command, expected) in VECTORS {
        assert_eq!(replay(command), *expected, "{command}");
    }
}
