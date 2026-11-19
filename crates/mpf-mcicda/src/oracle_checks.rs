//! Rows recorded from the verbatim Wine C code (`target/canon/mcicda/oracle.c`, built with ASan and
//! UBSan, no reports). Each row is an oracle command and the line the oracle printed for it.
//! `replay` runs the same command on the port and formats the result the way the oracle does.

use crate::devcaps::{get_dev_caps, DevCapsParms};
use crate::device::{CdRom, QChannel};
use crate::error::device_error;
use crate::frame::Msf;
use crate::info::info;
use crate::mci::*;
use crate::open::{close, open, OpenParms};
use crate::seek::{seek, skip_data_tracks, SeekParms};
use crate::set::{set, SetParms};
use crate::status::{status, StatusParms};
use crate::time::{calc_frame, calc_time};
use crate::toc::{Toc, TrackData};
use crate::Driver;

/// A fake device for the oracle state. `toc` and `q` are `None` where the oracle's `none` makes
/// the ioctl fail, and then `last_error` is the failure.
#[derive(Default)]
pub(crate) struct Mock {
    pub(crate) toc: Option<Toc>,
    pub(crate) q: Option<QChannel>,
    pub(crate) last_error: u32,
    pub(crate) cd_drives: [bool; 26],
    pub(crate) open_handle_fails: bool,
    pub(crate) seek_fails: bool,
    pub(crate) seek: Option<Msf>,
    pub(crate) opened: Option<u8>,
    pub(crate) closed: u32,
    pub(crate) doors: Vec<bool>,
}

impl CdRom for Mock {
    fn read_toc(&mut self) -> Result<Toc, u32> {
        self.toc.clone().ok_or(self.last_error)
    }

    fn read_q_channel(&mut self) -> Result<QChannel, u32> {
        self.q.ok_or(self.last_error)
    }

    fn seek_audio(&mut self, to: Msf) -> bool {
        self.seek = Some(to);
        !self.seek_fails
    }

    fn stop_audio(&mut self) {}

    fn set_door(&mut self, open: bool) {
        self.doors.push(open);
    }

    fn is_cd_rom(&self, letter: u8) -> bool {
        self.cd_drives[usize::from(letter - b'A')]
    }

    fn open_handle(&mut self, letter: u8) -> bool {
        self.opened = Some(letter);
        !self.open_handle_fails
    }

    fn close_handle(&mut self) {
        self.closed += 1;
    }
}

/// The oracle rows: `(command, line)`. `T2` and `T3` in the comments stand for the tables
/// `toc=1-2/4:0:2:0,0:20:0:0,0:40:0:0` and `toc=1-3/4:0:2:0,0:20:0:0,0:40:0:0,0:50:0:0`.
#[rustfmt::skip]
const VECTORS: &[(&str, &str)] = &[
    ("frame fmt=2 time=0x1e0c0a", "frame=45930"),
    ("frame fmt=10 time=0x20101 toc=1-3/4:0:2:0,0:20:0:0,0:40:0:0,0:50:0:0", "frame=4800"),
    ("time fmt=0 frame=75", "time=0x000003e9 ret=0x00000000"),
    ("time fmt=10 frame=225000 toc=1-3/4:0:2:0,0:20:0:0,0:40:0:0,0:50:0:0", "time=0x00000a03 ret=0x00040000"),
    ("serial toc=1-2/4:0:2:0,0:20:0:0,0:40:0:0", "serial=1491082"),
    ("serial toc=none", "serial=none"),
    ("skip frame=150 toc=1-2/4:0:2:0,0:20:0:0,0:40:0:0", "ret=0x00000000 frame=90000"),
    ("skip frame=100000 toc=1-2/4:0:2:0,4:20:0:0,4:40:0:0", "ret=0x00000000 frame=179999"),
    ("length flags=0x110 track=2 fmt=2 toc=1-2/4:0:2:0,0:20:0:0,0:40:0:0", "ret=0x00020000 value=0x004a3b13"),
    ("length flags=0x100 fmt=10 toc=1-2/4:0:2:0,0:20:0:0,0:40:0:0", "ret=0x00020000 value=0x004a3927"),
    ("mode flags=0x100 q=12:1:0:2:0", "ret=0x00010000 value=0x02110211"),
    ("position flags=0x300 toc=1-2/4:0:2:0,0:20:0:0,0:40:0:0", "ret=0x00020000 value=0x00000200"),
    ("position flags=0x100 fmt=10 q=11:1:0:3:20 toc=1-2/4:0:2:0,0:20:0:0,0:40:0:0", "ret=0x00040000 value=0x14010001"),
    ("ttype flags=0x110 track=1 toc=1-2/4:0:2:0,0:20:0:0,0:40:0:0", "ret=0x00000000 value=0x00000441"),
    ("status flags=0x110 item=4001 track=1 toc=1-2/4:0:2:0,0:20:0:0,0:40:0:0", "ret=0x00000000 value=0x00000441"),
    ("status flags=0x100 item=5 q=11:1:0:2:0", "ret=0x00010000 value=0x02140001"),
    ("devcaps flags=0x100 item=1", "ret=0x00010000 value=0x02130000"),
    ("error err=21", "ret=0x00000114"),
    ("open flags=0x100 elem=D cd=D", "ret=0x00000000 use=1 shr=1 drive=D"),
    ("info flags=0x400 buf=4", "ret=0x0000015e text=-"),
    ("close pre=2:1", "ret=0x00000000 use=1"),
    ("seek flags=0x200 toc=1-2/4:0:2:0,0:20:0:0,0:40:0:0", "ret=0x00000000 seek=39:59:74"),
    ("set flags=0x400 tf=7", "ret=0x00000125 fmt=2"),
    ("info flags=0x800 buf=4 toc=1-2/4:0:2:0,0:20:0:0,0:40:0:0", "ret=0x00000000 text=\"149\""),
    ("info flags=0x100 buf=32", "ret=0x00000000 text=\"Wine's audio CD\""),
];

#[test]
fn oracle_rows_match() {
    for (command, line) in VECTORS {
        assert_eq!(replay(command), *line, "{command}");
    }
}

#[test]
#[should_panic(expected = "unknown op")]
fn unknown_op_panics() {
    replay("bogus");
}

#[test]
#[should_panic(expected = "bad token")]
fn bad_token_panics() {
    replay("frame time");
}

/// Runs one oracle command on the port and returns the line the oracle would print.
fn replay(line: &str) -> String {
    let mut words = line.split_whitespace();
    let op = words.next().unwrap_or("");
    let cmd = Command {
        op,
        pairs: words.map(split_pair).collect(),
    };
    let flags = cmd.num("flags", 0);
    let elem: Vec<u16> = cmd.get("elem").unwrap_or("").encode_utf16().collect();
    let mut mock = mock_for(&cmd);
    let mut drv = driver_for(&cmd);
    match op {
        "frame" => format!(
            "frame={}",
            calc_frame(&mut mock, drv.time_format, cmd.num("time", 0))
        ),
        "time" => {
            let (value, ret) = calc_time(
                &mut mock,
                cmd.num("fmt", MCI_FORMAT_MSF),
                cmd.num("frame", 0),
            );
            format!("time=0x{value:08x} ret=0x{ret:08x}")
        }
        "serial" => match &mock.toc {
            Some(toc) => format!("serial={}", toc.serial()),
            None => "serial=none".to_string(),
        },
        "skip" => {
            let mut frame = cmd.num("frame", 0);
            let ret = skip_data_tracks(&mut mock, &mut frame);
            format!("ret=0x{ret:08x} frame={frame}")
        }
        "length" | "mode" | "position" | "ttype" | "status" => {
            let item = match op {
                "length" => MCI_STATUS_LENGTH,
                "mode" => MCI_STATUS_MODE,
                "position" => MCI_STATUS_POSITION,
                "ttype" => MCI_CDA_STATUS_TYPE_TRACK,
                _ => cmd.num("item", 0),
            };
            let mut parms = StatusParms {
                item,
                track: cmd.num("track", 0),
                value: 0,
            };
            let ret = status(&drv, &mut mock, flags, Some(&mut parms));
            format!("ret=0x{ret:08x} value=0x{:08x}", parms.value)
        }
        "error" => format!("ret=0x{:08x}", device_error(mock.last_error)),
        "devcaps" => {
            let mut parms = DevCapsParms {
                item: cmd.num("item", 0),
                value: 0,
            };
            let ret = get_dev_caps(flags, Some(&mut parms));
            format!("ret=0x{ret:08x} value=0x{:08x}", parms.value)
        }
        "open" => {
            let ret = open(
                &mut drv,
                &mut mock,
                flags,
                Some(&OpenParms { element: &elem }),
            );
            let drive = mock
                .opened
                .map_or("-".to_string(), |letter| char::from(letter).to_string());
            format!(
                "ret=0x{ret:08x} use={} shr={} drive={drive}",
                drv.use_count,
                u8::from(drv.shareable)
            )
        }
        "close" => {
            let ret = close(&mut drv, &mut mock);
            format!("ret=0x{ret:08x} use={}", drv.use_count)
        }
        "seek" => {
            let parms = SeekParms {
                to: cmd.num("to", 0),
            };
            let ret = seek(&drv, &mut mock, flags, Some(&parms));
            let at = mock.seek.map_or("-".to_string(), |at| {
                format!("{}:{}:{}", at.minute, at.second, at.frame)
            });
            format!("ret=0x{ret:08x} seek={at}")
        }
        "set" => {
            let parms = SetParms {
                time_format: cmd.num("tf", 0),
            };
            let ret = set(&mut drv, &mut mock, flags, Some(&parms));
            format!("ret=0x{ret:08x} fmt={}", drv.time_format)
        }
        "info" => {
            let mut buffer = vec![0u16; cmd.num("buf", 0) as usize];
            let ret = info(&drv, &mut mock, flags, Some(&mut buffer[..]));
            if ret != 0 {
                format!("ret=0x{ret:08x} text=-")
            } else {
                format!("ret=0x{ret:08x} text=\"{}\"", quoted(&buffer))
            }
        }
        other => panic!("unknown op: {other}"),
    }
}

/// One oracle line: the op, then the `key=value` tokens.
struct Command<'a> {
    op: &'a str,
    pairs: Vec<(&'a str, &'a str)>,
}

impl Command<'_> {
    fn get(&self, key: &str) -> Option<&str> {
        self.pairs
            .iter()
            .find(|(name, _)| *name == key)
            .map(|&(_, value)| value)
    }

    fn num(&self, key: &str, default: u32) -> u32 {
        self.get(key).map_or(default, parse_num)
    }
}

fn split_pair(token: &str) -> (&str, &str) {
    token.split_once('=').expect("bad token")
}

/// The fake device for a command. `toc=none` and an absent `q=` fail their ioctl.
fn mock_for(cmd: &Command) -> Mock {
    let mut cd_drives = [false; 26];
    for letter in cmd.get("cd").unwrap_or("").bytes() {
        cd_drives[usize::from(letter.to_ascii_uppercase() - b'A')] = true;
    }
    Mock {
        toc: cmd.get("toc").filter(|text| *text != "none").map(parse_toc),
        q: cmd.get("q").filter(|text| *text != "none").map(parse_q),
        last_error: cmd.num("err", 0),
        cd_drives,
        ..Default::default()
    }
}

/// The driver state the oracle sets before a command. An `open` starts closed, and every other
/// command starts open, unless `pre=USE:SHAREABLE` says otherwise.
fn driver_for(cmd: &Command) -> Driver {
    let (use_count, shareable) = match cmd.get("pre") {
        Some(pre) => {
            let (use_count, shareable) = pre.split_once(':').expect("bad pre");
            (parse_num(use_count) as i32, parse_num(shareable) != 0)
        }
        None => (if cmd.op == "open" { 0 } else { 1 }, false),
    };
    Driver {
        use_count,
        shareable,
        time_format: cmd.num("fmt", MCI_FORMAT_MSF),
    }
}

/// `toc=F-L/c:m:s:f,...`: tracks F to L, then the entries that follow.
fn parse_toc(text: &str) -> Toc {
    let (range, entries) = text.split_once('/').expect("bad toc");
    let (first, last) = range.split_once('-').expect("bad toc");
    let entries: Vec<TrackData> = entries.split(',').map(parse_track).collect();
    Toc::new(parse_num(first) as u8, parse_num(last) as u8, &entries)
}

/// One `c:m:s:f` entry: the control nibble, then the address.
fn parse_track(entry: &str) -> TrackData {
    let mut fields = entry.split(':').map(|field| parse_num(field) as u8);
    let mut next = || fields.next().expect("bad toc entry");
    let control = next();
    TrackData {
        control,
        address: Msf {
            minute: next(),
            second: next(),
            frame: next(),
        },
    }
}

/// `q=S:T:M:S:F`: the audio status in hex, then the track and the absolute address.
fn parse_q(text: &str) -> QChannel {
    let mut fields = text.split(':');
    let mut next = || fields.next().expect("bad q");
    let audio_status = u8::from_str_radix(next(), 16).expect("bad q");
    let track = parse_num(next()) as u8;
    let absolute = Msf {
        minute: parse_num(next()) as u8,
        second: parse_num(next()) as u8,
        frame: parse_num(next()) as u8,
    };
    QChannel {
        audio_status,
        track,
        absolute,
    }
}

/// A decimal number, or a hexadecimal one with a `0x` prefix, as `strtoul` with base 0 reads.
fn parse_num(text: &str) -> u32 {
    match text.strip_prefix("0x") {
        Some(hex) => u32::from_str_radix(hex, 16).expect("bad number"),
        None => text.parse().expect("bad number"),
    }
}

/// The text the oracle prints for a wide buffer: the characters before the first NUL.
pub(crate) fn quoted(units: &[u16]) -> String {
    units
        .iter()
        .take_while(|&&unit| unit != 0)
        .map(|&unit| char::from(unit as u8))
        .collect()
}
