//! Replays of the Wine oracle rows. The oracle compiles the verbatim `mcimidi.c` with
//! `-fsanitize=address,undefined` and runs each row through `harness.c`, which prints one line per
//! row. `replay` rebuilds that line from the row's command, so every `VECTORS` entry is checked
//! against the C output. The rows that abort under the sanitizers are not in the table.

use super::*;
use std::fmt::Debug;
use std::str::FromStr;

#[rustfmt::skip]
const VECTORS: &[(&str, &str)] = &[
    ("consts", "MCIERR_INVALID_FILE=296 MCIERR_INTERNAL=277 MCIERR_OUT_OF_MEMORY=264 MCIERR_MISSING_PARAMETER=273 MCIERR_FILE_NOT_FOUND=275 MCIERR_OUTOFRANGE=282 MCIERR_FLAGS_NOT_COMPATIBLE=284 MCIERR_SEQ_PORT_INUSE=337 MCIERR_SEQ_PORT_NONEXISTENT=338 MCIERR_SEQ_PORT_MAPNODEVICE=339 MMSYSERR_BADDEVICEID=2 MMSYSERR_ALLOCATED=4 MMSYSERR_NOMEM=7 MIDIERR_INVALIDSETUP=69 MCI_FORMAT_SMPTE_24=4 MCI_FORMAT_SMPTE_25=5 MCI_FORMAT_SMPTE_30=6 MCI_FORMAT_SMPTE_30DROP=7 MCI_WAIT=2 MCI_FROM=4 MCI_TO=8 MCI_SEEK_TO_START=256 MCI_SEEK_TO_END=512"),
    ("mmr2mci 2", "ret=338"),
    ("varlen ff7f 0", "len=2 val=16383 pos=2"),
    ("varlen 8180 0", "len=0 val=0 pos=2"),
    ("word 01 0", "ret=296 w=48879 pos=1"),
    ("long 01020304 0", "ret=0 dw=16909060 pos=4"),
    ("event 053c40 0 3 0 144", "ret=0 index=0 pulse=5 data=00403c90 len=3 lastcmd=144"),
    ("event 00903c40 0 3 0 0", "ret=277 index=0 pulse=0 data=ffffffff len=65535 lastcmd=144"),
    ("mtrk 4d54726b0000000c00903c4060803c0000ff2f00 0 0", "ret=0 first=8 last=20 index=8 length=96 status=1 pulse=0 lastcmd=128 pos=20"),
    ("mtrk 4d54726b0000000b00ff020341424300ff2f00 0 0", "ret=0 first=8 last=19 index=8 length=0 status=1 pulse=0 lastcmd=0 pos=19"),
    ("mtrk 4d54686b0000000c00903c4060803c0000ff2f00 0 0", "ret=296 first=0 last=0 index=0 length=0 status=0 pulse=0 lastcmd=0 pos=4"),
    ("mtrk 4d54726b000000ff00ff2f00 0 0", "ret=0 first=8 last=263 index=8 length=0 status=1 pulse=0 lastcmd=0 pos=263"),
    ("mthd 4d546864000000060001000200604d54726b0000000400ff2f004d54726b0000000400ff2f00 0", "ret=0 format=1 ntracks=2 division=96 tf=0 tempo=500000 tracks=[22,26,0,1,0;34,38,0,1,0]"),
    ("mthd 4d546864000000060000000300604d54726b0000000c00903c4060803c0000ff2f00 0", "ret=0 format=0 ntracks=1 division=96 tf=0 tempo=500000 tracks=[22,34,96,1,128]"),
    ("mthd 4d5468640000000600010001e2504d54726b0000000400ff2f00 0", "ret=296"),
    ("mthd 4d54686400000008000100010060abcd4d54726b0000000400ff2f00 0", "ret=0 format=1 ntracks=1 division=96 tf=0 tempo=500000 tracks=[24,28,0,1,0]"),
    ("convpulse 480 500000 1000", "ms=1041"),
    ("convpulse 57859 500000 1000", "ms=11111"),
    ("convtf 4 16909060", "ms=14582041"),
    ("convms 4 3723456", "tf=1375928833"),
    ("mthdlen 2 96 500000 96,192", "ms=1500"),
    ("findnext 1:10,1:5,0:1", "track=1 hi=5"),
    ("seek 8 1000 4", "ret=0 pos=835380000"),
    ("play 1 8 0 50 100 0", "ret=282 pos=100 end=287454020"),
    ("play 1 12 50 100 0 0", "ret=264 pos=50 end=100"),
];

fn num<T: FromStr>(text: &str) -> T
where
    T::Err: Debug,
{
    text.parse().unwrap()
}

/// A hex byte string, or `-` for an empty image.
fn bytes(text: &str) -> Vec<u8> {
    if text == "-" {
        return Vec::new();
    }
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).unwrap())
        .collect()
}

fn at(data: &[u8], pos: usize) -> MmioFile<'_> {
    let mut file = MmioFile::new(data);
    file.seek(pos as i32, Whence::Set);
    file
}

/// A comma-separated list, or `-` for an empty list.
fn list(text: &str) -> Vec<u32> {
    if text == "-" {
        return Vec::new();
    }
    text.split(',').map(num::<u32>).collect()
}

fn code(result: Result<(), u32>) -> u32 {
    result.err().unwrap_or(0)
}

/// Runs one oracle row and returns the line that `harness.c` prints for it.
fn replay(line: &str) -> String {
    let t: Vec<&str> = line.split_whitespace().collect();
    match t[0] {
        "consts" => format!(
            "MCIERR_INVALID_FILE={MCIERR_INVALID_FILE} MCIERR_INTERNAL={MCIERR_INTERNAL} \
             MCIERR_OUT_OF_MEMORY={MCIERR_OUT_OF_MEMORY} MCIERR_MISSING_PARAMETER={MCIERR_MISSING_PARAMETER} \
             MCIERR_FILE_NOT_FOUND={MCIERR_FILE_NOT_FOUND} MCIERR_OUTOFRANGE={MCIERR_OUTOFRANGE} \
             MCIERR_FLAGS_NOT_COMPATIBLE={MCIERR_FLAGS_NOT_COMPATIBLE} \
             MCIERR_SEQ_PORT_INUSE={MCIERR_SEQ_PORT_INUSE} \
             MCIERR_SEQ_PORT_NONEXISTENT={MCIERR_SEQ_PORT_NONEXISTENT} \
             MCIERR_SEQ_PORT_MAPNODEVICE={MCIERR_SEQ_PORT_MAPNODEVICE} \
             MMSYSERR_BADDEVICEID={MMSYSERR_BADDEVICEID} MMSYSERR_ALLOCATED={MMSYSERR_ALLOCATED} \
             MMSYSERR_NOMEM={MMSYSERR_NOMEM} MIDIERR_INVALIDSETUP={MIDIERR_INVALIDSETUP} \
             MCI_FORMAT_SMPTE_24={MCI_FORMAT_SMPTE_24} MCI_FORMAT_SMPTE_25={MCI_FORMAT_SMPTE_25} \
             MCI_FORMAT_SMPTE_30={MCI_FORMAT_SMPTE_30} MCI_FORMAT_SMPTE_30DROP={MCI_FORMAT_SMPTE_30DROP} \
             MCI_WAIT={MCI_WAIT} MCI_FROM={MCI_FROM} MCI_TO={MCI_TO} \
             MCI_SEEK_TO_START={MCI_SEEK_TO_START} MCI_SEEK_TO_END={MCI_SEEK_TO_END}"
        ),
        "mmr2mci" => format!("ret={}", mmr2mci(num(t[1]))),
        "varlen" => {
            let data = bytes(t[1]);
            let mut file = at(&data, num(t[2]));
            let (len, val) = file.read_var_len();
            format!("len={len} val={val} pos={}", file.position())
        }
        // The harness starts `w` at 0xBEEF and `dw` at 0xDEADBEEF, and the C keeps them on failure.
        "word" => {
            let data = bytes(t[1]);
            let mut file = at(&data, num(t[2]));
            match file.read_word() {
                Some(w) => format!("ret=0 w={w} pos={}", file.position()),
                None => format!("ret=296 w=48879 pos={}", file.position()),
            }
        }
        "long" => {
            let data = bytes(t[1]);
            let mut file = at(&data, num(t[2]));
            match file.read_long() {
                Some(dw) => format!("ret=0 dw={dw} pos={}", file.position()),
                None => format!("ret=296 dw=3735928559 pos={}", file.position()),
            }
        }
        "event" => {
            let data = bytes(t[1]);
            let mut track = Track {
                index: num(t[2]),
                last: num(t[3]),
                event_pulse: num(t[4]),
                last_command: num(t[5]),
                event_data: 0xFFFF_FFFF,
                event_length: 0xFFFF,
                ..Default::default()
            };
            let ret = code(read_next_event(&mut MmioFile::new(&data), &mut track));
            format!(
                "ret={ret} index={} pulse={} data={:08x} len={} lastcmd={}",
                track.index,
                track.event_pulse,
                track.event_data,
                track.event_length,
                track.last_command
            )
        }
        "mtrk" => {
            let data = bytes(t[1]);
            let mut file = at(&data, num(t[2]));
            let mut track = Track::default();
            let ret = code(read_mtrk(&mut file, &mut track));
            format!(
                "ret={ret} first={} last={} index={} length={} status={} pulse={} lastcmd={} pos={}",
                track.first,
                track.last,
                track.index,
                track.length,
                u8::from(track.status),
                track.event_pulse,
                track.last_command,
                file.position()
            )
        }
        "mthd" => {
            let data = bytes(t[1]);
            match read_mthd(&mut MmioFile::new(&data), num(t[2])) {
                Err(c) => format!("ret={c}"),
                Ok(h) => {
                    let tracks: Vec<String> = h
                        .tracks
                        .iter()
                        .map(|tr| {
                            format!(
                                "{},{},{},{},{}",
                                tr.first,
                                tr.last,
                                tr.length,
                                u8::from(tr.status),
                                tr.last_command
                            )
                        })
                        .collect();
                    format!(
                        "ret=0 format={} ntracks={} division={} tf={} tempo={} tracks=[{}]",
                        h.format,
                        h.tracks.len(),
                        h.division,
                        h.time_format,
                        h.tempo,
                        tracks.join(";")
                    )
                }
            }
        }
        "convpulse" => format!(
            "ms={}",
            pulse_to_ms(num::<u32>(t[1]) as u16, num(t[2]), num(t[3]))
        ),
        "convtf" => format!("ms={}", time_format_to_ms(num(t[1]), num(t[2]))),
        "convms" => format!("tf={}", ms_to_time_format(num(t[1]), num(t[2]))),
        "mthdlen" => {
            let header = Header {
                format: num(t[1]),
                division: num(t[2]),
                time_format: MCI_FORMAT_MILLISECONDS,
                tempo: num(t[3]),
                tracks: list(t[4])
                    .into_iter()
                    .map(|length| Track {
                        length,
                        ..Default::default()
                    })
                    .collect(),
            };
            format!("ms={}", mthd_length_ms(&header))
        }
        "findnext" => {
            let tracks: Vec<Track> = if t[1] == "-" {
                Vec::new()
            } else {
                t[1].split(',')
                    .map(|item| {
                        let (status, pulse) = item.split_once(':').unwrap();
                        Track {
                            status: (num::<u32>(status) & 1) != 0,
                            event_pulse: num(pulse),
                            ..Default::default()
                        }
                    })
                    .collect()
            };
            let (index, hi) = find_next_event(&tracks);
            format!("track={} hi={hi}", index.map_or(-1, |i| i as i64))
        }
        // The harness starts the position at 7, and the C leaves it unchanged on error.
        "seek" => match seek_target(num(t[1]), num(t[2]), num(t[3])) {
            Ok(pos) => format!("ret=0 pos={pos}"),
            Err(c) => format!("ret={c} pos=7"),
        },
        // The harness starts the position at `pos` and the end at 0x11223344 (287454020). The
        // player thread cannot be created, so the Ok path reports MCIERR_OUT_OF_MEMORY (264).
        "play" => {
            let pos: u32 = num(t[5]);
            let parms = PlayParms {
                from: num(t[3]),
                to: num(t[4]),
            };
            match play_range(num::<u32>(t[1]) != 0, num(t[2]), num(t[6]), Some(parms), pos) {
                Err(c) => format!("ret={c} pos={pos} end=287454020"),
                Ok((start, end)) => format!("ret=264 pos={start} end={end}"),
            }
        }
        other => panic!("unknown harness command: {other}"),
    }
}

#[test]
fn oracle_rows_match_wine() {
    for (cmd, expected) in VECTORS {
        assert_eq!(replay(cmd), *expected, "{cmd}");
    }
}

#[test]
fn replay_covers_remaining_arms() {
    assert_eq!(replay("word 0102 0"), "ret=0 w=258 pos=2");
    assert_eq!(replay("long 01 0"), "ret=296 dw=3735928559 pos=1");
    assert_eq!(replay("varlen - 0"), "len=0 val=0 pos=0");
    assert_eq!(replay("mthdlen 1 96 500000 -"), "ms=0");
    assert_eq!(replay("findnext -"), "track=-1 hi=4294967295");
    assert_eq!(replay("seek 264 0 0"), "ret=284 pos=7");
    assert_eq!(replay("seek 0 0 0"), "ret=273 pos=7");
}

#[test]
#[should_panic(expected = "unknown harness command")]
fn replay_rejects_unknown_commands() {
    replay("bogus");
}
