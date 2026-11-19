//! Oracle rows for the mciwave port.
//!
//! The rows are the output of Wine's verbatim `dlls/mciwave/mciwave.c`, built
//! with ASan and UBSan by `target/canon/mciwave/build.sh`. The harness mocks
//! only the winmm and Win32 externals. [`replay`] runs each command through the
//! port and formats the result the way `harness.c` does, so every row is
//! compared as exact text. Each command runs on fresh state, as in the harness.

use super::*;
use crate::device::{Env, WaveDevice};
use crate::format::WaveFormat;
use crate::playback::Playback;
use crate::settings::SetParams;

/// Rows from Wine's oracle: (commands, replies), in `rows.txt` order.
#[rustfmt::skip]
const VECTORS: &[(&str, &str)] = &[
    ("muldiv 3 4 2|muldiv -7 3 2|muldiv 5 5 0|muldiv 2147483647 2 1", "ret=6|ret=-11|ret=-1|ret=-1"),
    ("b2t 0 1000 1,1,11025,11025,1,8,0|b2t 9 22050 1,2,44100,176400,4,16,0|b2t 3 100 1,1,11025,11025,1,8,0|b2t 8 1234 1,1,11025,11025,1,8,0", "ret=91|ret=5513|ret=0|ret=1234"),
    ("t2b 0 1 1,1,11025,11025,1,8,0 10|t2b 8 1234 1,1,11025,11025,1,8,0 0|t2b 9 22050 1,2,44100,176400,4,16,0 1000", "ret=10|ret=1234|ret=88200"),
    ("align 5 1,2,44100,176400,4,16,0|align 5 1,1,11025,11025,1,8,0", "ret=8|ret=5"),
    ("default", "fmt=1,1,11025,11025,1,8,0"),
    ("riff 1 2,1,8000,4000,512,4,32|riff 0 1,1,11025,1,1,16,0|riff 1 1,1,8000,1,1,16,0", "ret=0 size=50 fmt=2,1,8000,4000,512,4,32|ret=0 size=16 fmt=1,1,11025,22050,2,16,0|ret=0 size=16 fmt=1,1,8000,16000,2,16,0"),
    ("play 0 2 0 0 8 1000 0 1,1,11025,11025,1,8,0 0|play 1 2 0 0 8 1000 0 1,1,11025,11025,1,8,0 0", "ret=275 seek=none|ret=265 seek=0"),
    ("play 1 6 5 0 8 1000 0 1,1,11025,11025,1,16,0 1|play 1 2 0 0 8 1000 0 1,1,11025,11025,1,8,0 1", "ret=0 seek=4|ret=0 seek=0"),
    ("play 1 6 1000 0 8 1000 0 1,1,11025,11025,1,8,0 1", "ret=0 seek=none"),
    ("play 1 6 2000 0 8 1000 0 1,1,11025,11025,1,8,0 1|play 1 10 0 2000 8 1000 0 1,1,11025,11025,1,8,0 1", "ret=282 seek=none|ret=282 seek=none"),
    ("record 10 0 0 8 1,1,11025,11025,1,8,0 0|record 2 0 0 8 1,1,11025,11025,1,8,0 0", "ret=0 seek=none|ret=265 seek=0"),
    ("seek 512 0 8 1000 1,1,11025,11025,1,8,0|seek 8 5 8 1000 1,2,44100,176400,4,16,0", "ret=0 pos=1000|ret=0 pos=4"),
    ("seek 768 0 8 1000 1,1,11025,11025,1,8,0|seek 0 0 8 1000 1,1,11025,11025,1,8,0|seek 8 2000 8 1000 1,1,11025,11025,1,8,0", "ret=284 pos=0|ret=273 pos=0|ret=282 pos=0"),
    ("set 4194304 2 2 0 0 1 0 0 0 0 0 0 0|set 4194304 1 1 0 0 5 0 0 0 0 0 0 0|set 8388608 0 2 0 0 0 1 0 0 0 0 0 0|set 8388608 0 2 0 0 0 5 0 0 0 0 0 0|set 67108864 0 0 0 0 0 0 0 0 0 0 0 0|set 67108864 0 0 0 0 65535 0 0 0 0 0 0 0|set 134217728 0 0 0 0 0 0 0 0 0 0 0 0", "ret=0 tf=0 in=1 out=65535 fmt=0,0,0,0,0,0,0|ret=282 tf=0 in=65535 out=65535 fmt=0,0,0,0,0,0,0|ret=0 tf=0 in=65535 out=1 fmt=0,0,0,0,0,0,0|ret=282 tf=0 in=65535 out=65535 fmt=0,0,0,0,0,0,0|ret=0 tf=0 in=65535 out=65535 fmt=0,0,0,0,0,0,0|ret=0 tf=0 in=65535 out=65535 fmt=0,0,0,0,0,0,0|ret=0 tf=0 in=65535 out=65535 fmt=0,0,0,0,0,0,0"),
    ("set 65536 0 0 0 0 0 0 2 0 0 0 0 0|set 1024 0 0 5 0 0 0 0 0 0 0 0 0|set 4096 0 0 0 0 0 0 0 0 0 0 0 0|set 512 0 0 0 0 0 0 0 0 0 0 0 0|set 2048 0 0 0 0 0 0 0 0 0 0 0 0|set 10240 0 0 0 0 0 0 0 0 0 0 0 0|set 131072 0 0 0 0 0 0 0 2 0 0 0 0|set 65536 0 0 0 0 0 0 1 0 0 0 0 0|set 262144 0 0 0 0 0 0 0 0 44100 0 0 0|set 524288 0 0 0 0 0 0 0 0 0 22050 0 0|set 1048576 0 0 0 0 0 0 0 0 0 0 4 0|set 2097152 0 0 0 0 0 0 0 0 0 0 0 16", "ret=282 tf=0 in=65535 out=65535 fmt=0,0,0,0,0,0,0|ret=293 tf=0 in=65535 out=65535 fmt=0,0,0,0,0,0,0|ret=274 tf=0 in=65535 out=65535 fmt=0,0,0,0,0,0,0|ret=274 tf=0 in=65535 out=65535 fmt=0,0,0,0,0,0,0|ret=270 tf=0 in=65535 out=65535 fmt=0,0,0,0,0,0,0|ret=0 tf=0 in=65535 out=65535 fmt=0,0,0,0,0,0,0|ret=0 tf=0 in=65535 out=65535 fmt=0,2,0,0,0,0,0|ret=0 tf=0 in=65535 out=65535 fmt=0,0,0,0,0,0,0|ret=0 tf=0 in=65535 out=65535 fmt=0,0,44100,0,0,0,0|ret=0 tf=0 in=65535 out=65535 fmt=0,0,0,22050,0,0,0|ret=0 tf=0 in=65535 out=65535 fmt=0,0,0,0,4,0,0|ret=0 tf=0 in=65535 out=65535 fmt=0,0,0,0,0,16,0"),
    ("status 256 8 0 0 0 0 525 65535 65535 1,1,11025,11025,1,8,0|status 256 1 1 8 1000 0 525 65535 65535 1,1,11025,11025,1,8,0|status 256 1 0 0 1000 0 525 65535 65535 1,1,11025,11025,1,8,0|status 0 8 0 0 0 0 525 65535 65535 1,1,11025,11025,1,8,0", "ret=0 value=1|ret=0 value=1000|ret=274 value=0|ret=273 value=0"),
    ("status 256 4 0 0 0 0 526 65535 65535 1,1,11025,11025,1,8,0|status 256 16388 0 0 0 0 525 65535 65535 1,1,11025,11025,1,8,0|status 256 99 0 0 0 0 525 65535 65535 1,1,11025,11025,1,8,0|status 256 4194304 0 0 0 0 525 65535 65535 1,1,11025,11025,1,8,0|status 256 8388608 0 0 0 0 525 3 4 1,1,11025,11025,1,8,0|status 256 7 0 0 0 0 524 65535 65535 1,1,11025,11025,1,8,0|status 256 6 0 0 0 0 525 65535 65535 1,1,11025,11025,1,8,0|status 768 2 1 8 1000 50 525 65535 65535 1,1,11025,11025,1,8,0|status 256 2 1 8 1000 50 525 65535 65535 1,1,11025,11025,1,8,0", "ret=65536 value=34472462|ret=0 value=11025|ret=274 value=0|ret=65536 value=75628543|ret=0 value=4|ret=65536 value=34799616|ret=65536 value=34930688|ret=0 value=0|ret=0 value=50"),
    ("status 256 16385 0 0 0 0 525 65535 65535 1,1,11025,11025,1,8,0|status 256 16385 0 0 0 0 525 65535 65535 2,1,8000,4000,512,4,32|status 256 16386 0 0 0 0 525 65535 65535 1,1,11025,11025,1,8,0|status 256 16387 0 0 0 0 525 65535 65535 1,1,11025,11025,1,8,0|status 256 16389 0 0 0 0 525 65535 65535 1,1,11025,11025,1,8,0|status 256 16390 0 0 0 0 525 65535 65535 1,1,11025,11025,1,8,0|status 256 16391 0 0 0 0 525 65535 65535 1,1,11025,11025,1,8,0|status 256 5 0 0 0 0 525 65535 65535 1,1,11025,11025,1,8,0|status 256 3 0 0 0 0 525 65535 65535 1,1,11025,11025,1,8,0", "ret=65536 value=75497473|ret=0 value=2|ret=0 value=1|ret=0 value=11025|ret=0 value=1|ret=0 value=8|ret=0 value=2863289685|ret=65536 value=34865153|ret=0 value=1"),
    ("caps 256 4 0 0|caps 256 16386 1 2|caps 256 16385 2 0|caps 256 99 0 0|caps 0 4 0 0|caps 256 1 0 0|caps 256 2 0 0|caps 256 3 0 0|caps 256 5 0 0|caps 256 6 0 0|caps 256 7 0 0|caps 256 8 0 0|caps 256 9 0 0", "ret=65536 value=34210314|ret=0 value=2|ret=0 value=2|ret=261 value=0|ret=261 value=0|ret=65536 value=34865153|ret=65536 value=34865153|ret=65536 value=34799616|ret=65536 value=34865153|ret=65536 value=34865153|ret=65536 value=34799616|ret=65536 value=34865153|ret=65536 value=34865153"),
    ("info 256 64 -|info 512 64 C:/x.wav|info 512 64 -|info 8388608 4 -|info 512 0 -|info 1 64 -|info 4194304 64 -", "ret=0 str=[Wine's audio player]|ret=0 str=[C:/x.wav]|ret=0 str=[]|ret=0 str=[Win]|ret=268 str=[]|ret=259 str=[]|ret=0 str=[Wine Wave In]"),
    ("pause 526 0 1 0|pause 526 0 0 5|pause 525 0 1 0|pause 529 0 0 0|pause 527 1 1 0", "ret=0 status=529 notify=1|ret=277 status=526 notify=0|ret=302 status=525 notify=0|ret=0 status=529 notify=0|ret=0 status=529 notify=1"),
    ("resume 529 0 1 0|resume 529 1 0 0|resume 529 1 1 5|resume 526 0 0 0|resume 525 0 1 0", "ret=0 status=526 notify=1|ret=0 status=527 notify=0|ret=277 status=529 notify=0|ret=0 status=526 notify=0|ret=302 status=525 notify=0"),
    ("stop 526 0 1 1 0|stop 527 1 1 0 9|stop 525 0 1 1 0|stop 526 0 1 0 0", "ret=0 status=525 notify=2|ret=9 status=525 notify=0|ret=0 status=525 notify=2|ret=0 status=525 notify=1"),
    ("cue 525 1|cue 526 1|cue 525 0|cue 527 1", "ret=0 notify=1|ret=302 notify=0|ret=0 notify=0|ret=302 notify=0"),
];

fn num(token: &str) -> u32 {
    token.parse().expect("unsigned token")
}

fn signed(token: &str) -> i32 {
    token.parse().expect("signed token")
}

fn wfx(token: &str) -> WaveFormat {
    let f: Vec<u32> = token.split(',').map(num).collect();
    WaveFormat {
        format_tag: f[0] as u16,
        channels: f[1] as u16,
        samples_per_sec: f[2],
        avg_bytes_per_sec: f[3],
        block_align: f[4] as u16,
        bits_per_sample: f[5] as u16,
        cb_size: f[6] as u16,
    }
}

fn fmt_text(f: &WaveFormat) -> String {
    format!(
        "{},{},{},{},{},{},{}",
        f.format_tag,
        f.channels,
        f.samples_per_sec,
        f.avg_bytes_per_sec,
        f.block_align,
        f.bits_per_sample,
        f.cb_size
    )
}

fn notify_flag(token: &str) -> u32 {
    if num(token) != 0 {
        MCI_NOTIFY
    } else {
        0
    }
}

fn seek_text(seek: Option<u32>) -> String {
    match seek {
        Some(position) => position.to_string(),
        None => "none".to_string(),
    }
}

/// Play and record end with the device stopped, and a stream that was opened
/// is closed with success, as the C code does once its buffer loop finishes.
fn finish(dev: &mut WaveDevice, flags: u32, outcome: Playback) -> (u32, Option<u32>) {
    match outcome {
        Playback::Done { ret, seek } => (ret, seek),
        Playback::Stream { seek, .. } => (dev.end_playback(flags, 0), Some(seek)),
    }
}

fn replay(commands: &str) -> String {
    commands
        .split('|')
        .map(run_command)
        .collect::<Vec<_>>()
        .join("|")
}

fn run_command(line: &str) -> String {
    let t: Vec<&str> = line.split_whitespace().collect();
    match t[0] {
        "b2t" => {
            let dev = WaveDevice {
                time_format: num(t[1]),
                format: wfx(t[3]),
                ..WaveDevice::default()
            };
            format!("ret={}", time::byte_to_time(&dev, num(t[2])))
        }
        "t2b" => {
            let dev = WaveDevice {
                time_format: num(t[1]),
                format: wfx(t[3]),
                cksize: num(t[4]),
                ..WaveDevice::default()
            };
            format!("ret={}", time::time_to_byte(&dev, num(t[2])))
        }
        "muldiv" => format!(
            "ret={}",
            time::mul_div(signed(t[1]), signed(t[2]), signed(t[3]))
        ),
        "align" => format!(
            "ret={}",
            time::align_up(num(t[1]), wfx(t[2]).block_align)
                .expect("align with block align 0 is excluded")
        ),
        "default" => {
            let mut dev = WaveDevice {
                format: WaveFormat::default(),
                format_is_default: false,
                ..WaveDevice::default()
            };
            dev.set_default_format();
            format!("fmt={}", fmt_text(&dev.format))
        }
        "riff" => {
            let mut dev = WaveDevice {
                format: wfx(t[2]),
                format_is_default: t[1] == "0",
                ..WaveDevice::default()
            };
            dev.riff_skeleton();
            format!(
                "ret=0 size={} fmt={}",
                dev.format.chunk_len(),
                fmt_text(&dev.format)
            )
        }
        "play" => {
            let mut dev = WaveDevice {
                has_file: num(t[1]) != 0,
                time_format: num(t[5]),
                cksize: num(t[6]),
                position: num(t[7]),
                format: wfx(t[8]),
                ..WaveDevice::default()
            };
            let env = Env {
                open_ok: num(t[9]) != 0,
                ..Env::default()
            };
            let flags = num(t[2]);
            let outcome = playback::play(&mut dev, &env, flags, num(t[3]), num(t[4]), 0);
            let (ret, seek) = finish(&mut dev, flags, outcome);
            format!("ret={ret} seek={}", seek_text(seek))
        }
        "record" => {
            let mut dev = WaveDevice {
                time_format: num(t[4]),
                format: wfx(t[5]),
                ..WaveDevice::default()
            };
            let env = Env {
                open_ok: num(t[6]) != 0,
                ..Env::default()
            };
            let flags = num(t[1]);
            let outcome = playback::record(&mut dev, &env, flags, num(t[2]), num(t[3]), 0);
            let (ret, seek) = finish(&mut dev, flags, outcome);
            format!("ret={ret} seek={}", seek_text(seek))
        }
        "seek" => {
            let mut dev = WaveDevice {
                time_format: num(t[3]),
                cksize: num(t[4]),
                format: wfx(t[5]),
                ..WaveDevice::default()
            };
            let ret = control::seek(&mut dev, &Env::default(), num(t[1]), num(t[2]));
            format!("ret={ret} pos={}", dev.position)
        }
        "set" => {
            let mut dev = WaveDevice {
                format: WaveFormat::default(),
                ..WaveDevice::default()
            };
            let env = Env {
                wave_in_devs: num(t[2]),
                wave_out_devs: num(t[3]),
                ..Env::default()
            };
            let params = SetParams {
                time_format: num(t[4]),
                input: num(t[6]) as u16,
                output: num(t[7]) as u16,
                format_tag: num(t[8]) as u16,
                channels: num(t[9]) as u16,
                samples_per_sec: num(t[10]),
                avg_bytes_per_sec: num(t[11]),
                block_align: num(t[12]) as u16,
                bits_per_sample: num(t[13]) as u16,
            };
            let ret = settings::set(&mut dev, &env, num(t[1]), &params);
            format!(
                "ret={ret} tf={} in={} out={} fmt={}",
                dev.time_format,
                dev.input,
                dev.output,
                fmt_text(&dev.format)
            )
        }
        "status" => {
            let mut dev = WaveDevice {
                has_file: num(t[3]) != 0,
                time_format: num(t[4]),
                cksize: num(t[5]),
                position: num(t[6]),
                status: num(t[7]),
                input: num(t[8]) as u16,
                output: num(t[9]) as u16,
                format: wfx(t[10]),
                ..WaveDevice::default()
            };
            let reply = query::status(&mut dev, num(t[1]), num(t[2]));
            format!("ret={} value={}", reply.ret, reply.value)
        }
        "caps" => {
            let mut dev = WaveDevice::default();
            let env = Env {
                wave_in_devs: num(t[3]),
                wave_out_devs: num(t[4]),
                ..Env::default()
            };
            let reply = query::dev_caps(&mut dev, &env, num(t[1]), num(t[2]));
            format!("ret={} value={}", reply.ret, reply.value)
        }
        "info" => {
            let mut dev = WaveDevice {
                file_name: if t[3] == "-" {
                    None
                } else {
                    Some(t[3].to_string())
                },
                ..WaveDevice::default()
            };
            let (ret, text) = query::info(&mut dev, num(t[1]), num(t[2]));
            format!("ret={ret} str=[{text}]")
        }
        "pause" | "resume" => {
            let mut dev = WaveDevice {
                status: num(t[1]),
                f_input: num(t[2]) != 0,
                ..WaveDevice::default()
            };
            let env = Env {
                control_ret: num(t[4]),
                ..Env::default()
            };
            let flags = notify_flag(t[3]);
            let ret = if t[0] == "pause" {
                control::pause(&mut dev, &env, flags, true)
            } else {
                control::resume(&mut dev, &env, flags, true)
            };
            format!("ret={ret} status={} notify={}", dev.status, dev.sent.len())
        }
        "stop" => {
            let mut dev = WaveDevice {
                status: num(t[1]),
                f_input: num(t[2]) != 0,
                has_callback: num(t[4]) != 0,
                ..WaveDevice::default()
            };
            let env = Env {
                control_ret: num(t[5]),
                ..Env::default()
            };
            let ret = control::stop(&mut dev, &env, notify_flag(t[3]), true);
            format!("ret={ret} status={} notify={}", dev.status, dev.sent.len())
        }
        "cue" => {
            let mut dev = WaveDevice {
                status: num(t[1]),
                ..WaveDevice::default()
            };
            let ret = control::cue(&mut dev, notify_flag(t[2]), true);
            format!("ret={ret} notify={}", dev.sent.len())
        }
        other => panic!("unknown harness command: {other}"),
    }
}

#[test]
fn oracle_rows_match_wine() {
    for (commands, expected) in VECTORS {
        assert_eq!(replay(commands), *expected, "row: {commands}");
    }
}

#[test]
#[should_panic(expected = "unknown harness command: bogus")]
fn unknown_command_panics() {
    run_command("bogus 1");
}
