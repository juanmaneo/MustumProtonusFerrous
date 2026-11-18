//! Oracle rows and edge tests for the msgsm32.acm port.
//!
//! The rows are the output of Wine's verbatim `dlls/msgsm32.acm/msgsm32.c`,
//! built with ASan and UBSan by `target/canon/msgsm32.acm/build.sh`. The
//! harness mocks only libgsm and the Win32 externals. [`replay`] runs each
//! command through the port and prints the result the way `harness.c` does, so
//! every row is compared as exact text. The edge tests reach the branches the
//! rows do not, with expectations read from the C source.

use super::*;
use std::fmt::Debug;
use std::str::FromStr;

const PCM8K: &str = "1,1,8000,16000,2,16,0,0";
const GSM8K: &str = "49,1,8000,1625,65,0,2,320";

/// Rows from Wine's oracle: (command, expected output), in `vectors.txt` order.
#[rustfmt::skip]
const VECTORS: &[(&str, &str)] = &[
    ("details", "ret=0 fcc=audc comp=0 mid=1 pid=36 acm=0x3320000 driver=0x4000000 support=1 tags=2 filters=0 short=[Microsoft GSM 6.10] long=[Wine GSM 6.10 libgsm codec] copyright=[Brought to you by the Wine team...] licensing=[Refer to LICENSE file] features=[]"),
    ("fmtdetails 1 1 0 1,1,8000,16000,2,16,0,0", "ret=0 tag=1 ch=1 sps=8000 avg=16000 align=2 bits=16 cb=0 spb=0 support=1"),
    ("fmtdetails 1 49 0 49,1,8000,1625,65,0,0,0", "ret=512 tag=49 ch=1 sps=8000 avg=1625 align=65 bits=0 cb=0 spb=0 support=0"),
    ("fmtdetails 0 49 5 0,0,0,0,0,0,0,0", "ret=0 tag=49 ch=1 sps=96000 avg=19500 align=65 bits=0 cb=2 spb=320 support=1"),
    ("fmtdetails 0 1 6 0,0,0,0,0,0,0,0", "ret=512 tag=1 ch=0 sps=0 avg=0 align=0 bits=0 cb=0 spb=0 support=0"),
    ("fmtdetails 0 65537 0 0,0,0,0,0,0,0,0", "ret=11 tag=1 ch=0 sps=0 avg=0 align=0 bits=0 cb=0 spb=0 support=0"),
    ("tagdetails 2 0 0", "ret=0 index=1 tag=49 size=20 support=1 std=6 name=[GSM 6.10]"),
    ("tagdetails 1 0 5", "ret=512 index=0 tag=5 size=0 support=0 std=0 name=[]"),
    ("tagdetails 0 2 0", "ret=512 index=2 tag=0 size=0 support=0 std=0 name=[]"),
    ("suggest 0 16 20 1,1,8000,16000,2,16,0,0 0,0,0,0,0,0,2,0", "ret=0 tag=49 ch=1 sps=8000 avg=1625 align=65 bits=0 cb=2 spb=320"),
    ("suggest 262144 16 16 49,1,8000,1625,65,0,2,320 0,0,11025,0,0,0,0,0", "ret=0 tag=1 ch=1 sps=11025 avg=22050 align=2 bits=16 cb=0 spb=0"),
    ("suggest 65536 16 20 1,1,8000,16000,2,16,0,0 0,0,0,0,0,0,0,0", "ret=512 tag=0 ch=1 sps=8000 avg=0 align=0 bits=0 cb=0 spb=0"),
    ("suggest 0 16 20 1,1,8000,16000,2,16,0,0 0,0,0,0,0,0,1,0", "ret=512 tag=49 ch=1 sps=8000 avg=0 align=0 bits=0 cb=1 spb=0"),
    ("streamopen 1,1,8000,16000,2,16,0,0 49,1,8000,1625,65,0,2,320", "ret=0"),
    ("streamopen 1,1,8000,16000,2,16,0,0 49,1,11025,2239,65,0,2,320", "ret=8"),
    ("streamsize 1 1,1,8000,16000,2,16,0,0 49,1,8000,1625,65,0,2,320 0 65", "ret=0 src=640 dst=65"),
    ("streamsize 0 1,1,8000,16000,2,16,0,0 49,1,8000,1625,65,0,2,320 4294967295 0", "ret=0 src=4294967295 dst=0"),
    ("streamsize 1 49,1,8000,1625,65,0,2,320 1,1,8000,16000,2,16,0,0 0 4294967295", "ret=0 src=436207590 dst=4294967295"),
    ("convert 0 1,1,8000,16000,2,16,0,0 49,1,8000,1625,65,0,2,320 640 130 1", "ret=0 used_src=640 used_dst=65 hash=c30114eb"),
    ("convert 0 1,1,8000,16000,2,16,0,0 49,1,8000,1625,65,0,2,320 1000 130 1", "ret=0 used_src=1000 used_dst=130 hash=9dbc24fb"),
    ("convert 0 1,1,8000,16000,2,16,0,0 49,1,8000,1625,65,0,2,320 940 130 2", "ret=0 used_src=940 used_dst=130 hash=540ee306"),
    ("convert 4 1,1,8000,16000,2,16,0,0 49,1,8000,1625,65,0,2,320 1000 130 1", "ret=0 used_src=640 used_dst=65 hash=c30114eb"),
    ("convert 0 49,1,8000,1625,65,0,2,320 1,1,8000,16000,2,16,0,0 100 640 3", "ret=0 used_src=65 used_dst=640 hash=a49b2ca5"),
    ("proc 7", "ret=1"),
    ("proc 4660", "ret=4660"),
];

fn num<T: FromStr>(token: &str) -> T
where
    T::Err: Debug,
{
    token.parse().expect("numeric token")
}

fn wfx(token: &str) -> WaveFormat {
    let f: Vec<u64> = token.split(',').map(num::<u64>).collect();
    WaveFormat {
        format_tag: f[0] as u16,
        channels: f[1] as u16,
        samples_per_sec: f[2] as DWORD,
        avg_bytes_per_sec: f[3] as DWORD,
        block_align: f[4] as u16,
        bits_per_sample: f[5] as u16,
        cb_size: f[6] as u16,
        samples_per_block: f[7] as u16,
    }
}

fn show(w: &WaveFormat) -> String {
    format!(
        "tag={} ch={} sps={} avg={} align={} bits={} cb={} spb={}",
        w.format_tag,
        w.channels,
        w.samples_per_sec,
        w.avg_bytes_per_sec,
        w.block_align,
        w.bits_per_sample,
        w.cb_size,
        w.samples_per_block
    )
}

/// The source bytes of the harness: a linear congruential generator from a seed.
fn pattern(len: usize, seed: u32) -> Vec<u8> {
    let mut x = seed;
    (0..len)
        .map(|_| {
            x = x.wrapping_mul(1_103_515_245).wrapping_add(12_345);
            (x >> 16) as u8
        })
        .collect()
}

/// FNV-1a, which the harness uses to hash the destination buffer.
fn fnv1a(bytes: &[u8]) -> u32 {
    bytes.iter().fold(2_166_136_261u32, |h, &b| {
        (h ^ u32::from(b)).wrapping_mul(16_777_619)
    })
}

/// libgsm stand-in with the harness mock's arithmetic. Each call is logged as
/// (operation, frame length), so tests can check the frame layout.
#[derive(Default)]
struct Mock {
    calls: Vec<(char, usize)>,
}

impl GsmCodec for Mock {
    fn reset_frame_index(&mut self) {
        self.calls.push(('r', 0));
    }

    fn encode(&mut self, pcm: &[u8], frame: &mut [u8]) {
        self.calls.push(('e', frame.len()));
        let h = fnv1a(pcm);
        for (j, byte) in frame.iter_mut().enumerate() {
            *byte = ((h >> (8 * (j & 3))) as u8).wrapping_add(j as u8);
        }
    }

    fn decode(&mut self, frame: &[u8], pcm: &mut [u8]) {
        self.calls.push(('d', frame.len()));
        let h = fnv1a(frame);
        for (k, sample) in pcm.iter_mut().enumerate() {
            *sample = ((h >> (8 * (k & 3))) as u8).wrapping_add(k as u8);
        }
    }
}

/// Runs one harness command through the port and prints it as `harness.c` does.
fn replay(line: &str) -> String {
    let t: Vec<&str> = line.split_whitespace().collect();
    match t[0] {
        "details" => {
            let d = driver_details();
            format!(
                "ret={MMSYSERR_NOERROR} fcc={} comp={} mid={} pid={} acm=0x{:x} driver=0x{:x} support={} tags={} filters={} short=[{}] long=[{}] copyright=[{}] licensing=[{}] features=[{}]",
                String::from_utf8_lossy(&d.fcc_type.to_le_bytes()),
                d.fcc_comp,
                d.mid,
                d.pid,
                d.vdw_acm,
                d.vdw_driver,
                d.fdw_support,
                d.format_tags,
                d.filter_tags,
                d.short_name,
                d.long_name,
                d.copyright,
                d.licensing,
                d.features
            )
        }
        "fmtdetails" => {
            let mut afd = FormatDetails {
                format_index: num(t[3]),
                format_tag: num(t[2]),
                format: wfx(t[4]),
                support: 0,
            };
            let ret = format_details(num(t[1]), &mut afd);
            format!("ret={ret} {} support={}", show(&afd.format), afd.support)
        }
        "tagdetails" => {
            let mut aftd = FormatTagDetails {
                index: num(t[2]),
                format_tag: num(t[3]),
                format_size: 0,
                support: 0,
                standard_formats: 0,
                name: "",
            };
            let ret = format_tag_details(num(t[1]), &mut aftd);
            format!(
                "ret={ret} index={} tag={} size={} support={} std={} name=[{}]",
                aftd.index,
                aftd.format_tag,
                aftd.format_size,
                aftd.support,
                aftd.standard_formats,
                aftd.name
            )
        }
        "suggest" => {
            let mut s = FormatSuggest {
                fdw_suggest: num(t[1]),
                src: wfx(t[4]),
                src_size: num(t[2]),
                dst: wfx(t[5]),
                dst_size: num(t[3]),
            };
            let ret = format_suggest(&mut s);
            format!("ret={ret} {}", show(&s.dst))
        }
        "streamopen" => format!("ret={}", stream_open_check(&wfx(t[1]), &wfx(t[2]))),
        "streamsize" => {
            let mut ss = StreamSize {
                fdw_size: num(t[1]),
                src_length: num(t[4]),
                dst_length: num(t[5]),
            };
            let ret = stream_size(&wfx(t[2]), &wfx(t[3]), &mut ss);
            format!("ret={ret} src={} dst={}", ss.src_length, ss.dst_length)
        }
        "convert" => {
            let (src_fmt, dst_fmt) = (wfx(t[2]), wfx(t[3]));
            let src = pattern(num(t[4]), num(t[6]));
            let mut dst = vec![0u8; num(t[5])];
            let mut header = StreamHeader {
                fdw_convert: num(t[1]),
                src: &src,
                dst: &mut dst,
                src_used: 0,
                dst_used: 0,
            };
            let ret = match stream_open_check(&src_fmt, &dst_fmt) {
                MMSYSERR_NOERROR => stream_convert(&mut Mock::default(), &src_fmt, &mut header),
                open => open,
            };
            let (used_src, used_dst) = (header.src_used, header.dst_used);
            format!(
                "ret={ret} used_src={used_src} used_dst={used_dst} hash={:08x}",
                fnv1a(&dst)
            )
        }
        "proc" => match dispatch(num(t[1])) {
            Dispatch::Reply(value) => format!("ret={value}"),
            Dispatch::Configure => "ret=1".to_string(),
            Dispatch::DefDriverProc => "ret=4660".to_string(),
            route => format!("route={route:?}"),
        },
        other => panic!("unknown harness command: {other}"),
    }
}

#[test]
fn oracle_rows_match_wine() {
    for (command, expected) in VECTORS {
        assert_eq!(replay(command), *expected, "{command}");
    }
}

#[test]
fn replay_covers_the_remaining_harness_paths() {
    assert_eq!(replay("proc 1"), "ret=1");
    assert_eq!(replay("proc 24586"), "route=DriverDetails");
    assert_eq!(
        replay("convert 0 1,1,8000,16000,2,16,0,0 1,1,8000,16000,2,16,0,0 640 130 1"),
        format!("ret=8 used_src=0 used_dst=0 hash={:08x}", fnv1a(&[0; 130]))
    );
}

#[test]
#[should_panic(expected = "unknown harness command")]
fn replay_rejects_unknown_commands() {
    replay("bogus");
}

#[test]
fn validate_rejects_each_malformed_field() {
    let (pcm, gsm) = (wfx(PCM8K), wfx(GSM8K));
    assert!(format_validate(&pcm));
    assert!(format_validate(&gsm));
    assert!(!format_validate(&WaveFormat { channels: 2, ..pcm }));
    assert!(!format_validate(&WaveFormat {
        bits_per_sample: 8,
        ..pcm
    }));
    assert!(!format_validate(&WaveFormat {
        block_align: 1,
        ..pcm
    }));
    assert!(!format_validate(&WaveFormat {
        avg_bytes_per_sec: 1,
        ..pcm
    }));
    assert!(!format_validate(&WaveFormat {
        format_tag: 2,
        ..pcm
    }));
    assert!(!format_validate(&WaveFormat { channels: 2, ..gsm }));
    assert!(!format_validate(&WaveFormat { cb_size: 1, ..gsm }));
    assert!(!format_validate(&WaveFormat {
        bits_per_sample: 16,
        ..gsm
    }));
    assert!(!format_validate(&WaveFormat {
        block_align: 2,
        ..gsm
    }));
    assert!(!format_validate(&WaveFormat {
        samples_per_block: 160,
        ..gsm
    }));
    assert!(!format_validate(&WaveFormat {
        avg_bytes_per_sec: 1626,
        ..gsm
    }));
}

fn tag_query(query: DWORD, index: DWORD, format_tag: DWORD) -> (LResult, FormatTagDetails) {
    let mut aftd = FormatTagDetails {
        index,
        format_tag,
        format_size: 0,
        support: 0,
        standard_formats: 0,
        name: "",
    };
    (format_tag_details(query, &mut aftd), aftd)
}

#[test]
fn format_tag_details_resolves_each_query() {
    type Case = (
        (DWORD, DWORD, DWORD),
        (LResult, DWORD, DWORD, DWORD, &'static str),
    );
    let cases: [Case; 11] = [
        ((ACM_FORMATTAGDETAILSF_INDEX, 0, 0), (0, 0, 1, 16, "PCM")),
        (
            (ACM_FORMATTAGDETAILSF_INDEX, 1, 0),
            (0, 1, 49, 20, "GSM 6.10"),
        ),
        (
            (ACM_FORMATTAGDETAILSF_INDEX, 2, 1),
            (ACMERR_NOTPOSSIBLE, 2, 1, 0, ""),
        ),
        (
            (ACM_FORMATTAGDETAILSF_FORMATTAG, 0, 1),
            (0, 0, 1, 16, "PCM"),
        ),
        (
            (ACM_FORMATTAGDETAILSF_FORMATTAG, 0, 49),
            (0, 1, 49, 20, "GSM 6.10"),
        ),
        (
            (ACM_FORMATTAGDETAILSF_FORMATTAG, 0, 5),
            (ACMERR_NOTPOSSIBLE, 0, 5, 0, ""),
        ),
        (
            (ACM_FORMATTAGDETAILSF_FORMATTAG, 0, 65537),
            (ACMERR_NOTPOSSIBLE, 0, 65537, 0, ""),
        ),
        (
            (ACM_FORMATTAGDETAILSF_LARGESTSIZE, 0, 0),
            (0, 1, 49, 20, "GSM 6.10"),
        ),
        (
            (ACM_FORMATTAGDETAILSF_LARGESTSIZE, 0, 1),
            (0, 0, 1, 16, "PCM"),
        ),
        (
            (ACM_FORMATTAGDETAILSF_LARGESTSIZE, 0, 5),
            (ACMERR_NOTPOSSIBLE, 0, 5, 0, ""),
        ),
        ((3, 0, 0), (MMSYSERR_NOTSUPPORTED, 0, 0, 0, "")),
    ];
    for (input, expected) in cases {
        let (ret, a) = tag_query(input.0, input.1, input.2);
        assert_eq!(
            (ret, a.index, a.format_tag, a.format_size, a.name),
            expected,
            "{input:?}"
        );
    }
}

fn details_of(
    query: DWORD,
    format_index: DWORD,
    format_tag: DWORD,
    format: WaveFormat,
) -> (LResult, WaveFormat, DWORD) {
    let mut afd = FormatDetails {
        format_index,
        format_tag,
        format,
        support: 0,
    };
    let ret = format_details(query, &mut afd);
    (ret, afd.format, afd.support)
}

#[test]
fn format_details_writes_index_formats_and_reports_errors() {
    let blank = wfx("0,0,0,0,0,0,0,0");
    let (ret, f, s) = details_of(ACM_FORMATDETAILSF_INDEX, 2, 1, blank);
    assert_eq!(
        (ret, s),
        (MMSYSERR_NOERROR, ACMDRIVERDETAILS_SUPPORTF_CODEC)
    );
    assert_eq!(
        show(&f),
        "tag=1 ch=1 sps=22050 avg=44100 align=2 bits=16 cb=0 spb=0"
    );
    let (ret, f, s) = details_of(ACM_FORMATDETAILSF_INDEX, 1, 49, blank);
    assert_eq!(
        (ret, s),
        (MMSYSERR_NOERROR, ACMDRIVERDETAILS_SUPPORTF_CODEC)
    );
    assert_eq!(
        show(&f),
        "tag=49 ch=1 sps=11025 avg=2239 align=65 bits=0 cb=2 spb=320"
    );
    let (ret, f, s) = details_of(ACM_FORMATDETAILSF_INDEX, 6, 1, blank);
    assert_eq!((ret, s, f.format_tag), (ACMERR_NOTPOSSIBLE, 0, 1));
    let (ret, f, s) = details_of(ACM_FORMATDETAILSF_INDEX, 0, 5, blank);
    assert_eq!((ret, s, f.format_tag), (MMSYSERR_INVALPARAM, 0, 5));
    let (ret, _, s) = details_of(2, 0, 1, blank);
    assert_eq!((ret, s), (MMSYSERR_NOTSUPPORTED, 0));
    let (ret, _, s) = details_of(
        ACM_FORMATDETAILSF_FORMAT,
        0,
        1,
        wfx("1,2,8000,16000,2,16,0,0"),
    );
    assert_eq!((ret, s), (ACMERR_NOTPOSSIBLE, 0));
}

fn suggest_of(
    fdw: DWORD,
    src_size: DWORD,
    dst_size: DWORD,
    src: &str,
    dst: &str,
) -> (LResult, WaveFormat) {
    let mut s = FormatSuggest {
        fdw_suggest: fdw,
        src: wfx(src),
        src_size,
        dst: wfx(dst),
        dst_size,
    };
    (format_suggest(&mut s), s.dst)
}

#[test]
fn format_suggest_rejects_bad_sizes_and_formats() {
    assert_eq!(
        suggest_of(0, 15, 20, PCM8K, "0,0,0,0,0,0,2,0").0,
        ACMERR_NOTPOSSIBLE
    );
    assert_eq!(
        suggest_of(0, 16, 15, PCM8K, "0,0,0,0,0,0,2,0").0,
        ACMERR_NOTPOSSIBLE
    );
    assert_eq!(
        suggest_of(0, 16, 20, "49,1,8000,1625,65,0,0,0", "0,0,0,0,0,0,2,0").0,
        ACMERR_NOTPOSSIBLE
    );
}

#[test]
fn format_suggest_keeps_fields_named_by_flags() {
    let (ret, dst) = suggest_of(
        ACM_FORMATSUGGESTF_NCHANNELS,
        16,
        20,
        GSM8K,
        "0,2,0,0,0,0,2,0",
    );
    assert_eq!((ret, dst.channels), (ACMERR_NOTPOSSIBLE, 2));
    let (ret, dst) = suggest_of(
        ACM_FORMATSUGGESTF_WBITSPERSAMPLE,
        16,
        20,
        GSM8K,
        "0,0,0,0,0,8,2,0",
    );
    assert_eq!((ret, dst.bits_per_sample), (ACMERR_NOTPOSSIBLE, 8));
    let (ret, dst) = suggest_of(
        ACM_FORMATSUGGESTF_WFORMATTAG,
        16,
        20,
        PCM8K,
        "49,0,0,0,0,0,2,0",
    );
    assert_eq!(ret, MMSYSERR_NOERROR);
    assert_eq!(
        show(&dst),
        "tag=49 ch=1 sps=8000 avg=1625 align=65 bits=0 cb=2 spb=320"
    );
}

#[test]
fn stream_open_requires_one_valid_gsm_side_and_equal_rates() {
    let (pcm, gsm) = (wfx(PCM8K), wfx(GSM8K));
    assert_eq!(stream_open_check(&pcm, &pcm), MMSYSERR_NOTSUPPORTED);
    let bad = WaveFormat {
        block_align: 1,
        ..gsm
    };
    assert_eq!(stream_open_check(&bad, &pcm), MMSYSERR_NOTSUPPORTED);
    assert_eq!(stream_open_check(&pcm, &bad), MMSYSERR_NOTSUPPORTED);
    assert_eq!(stream_open_check(&pcm, &gsm), MMSYSERR_NOERROR);
}

fn size_of(
    fdw: DWORD,
    src: &str,
    dst: &str,
    src_length: DWORD,
    dst_length: DWORD,
) -> (LResult, DWORD, DWORD) {
    let mut ss = StreamSize {
        fdw_size: fdw,
        src_length,
        dst_length,
    };
    let ret = stream_size(&wfx(src), &wfx(dst), &mut ss);
    (ret, ss.src_length, ss.dst_length)
}

#[test]
fn stream_size_converts_in_both_directions_and_rejects_others() {
    assert_eq!(size_of(0, PCM8K, GSM8K, 1000, 0), (0, 1000, 130));
    assert_eq!(size_of(0, GSM8K, PCM8K, 100, 0), (0, 100, 640));
    assert_eq!(size_of(1, GSM8K, PCM8K, 0, 1280), (0, 130, 1280));
    assert_eq!(size_of(1, PCM8K, GSM8K, 0, 130), (0, 1280, 130));
    assert_eq!(
        size_of(1, PCM8K, PCM8K, 5, 5),
        (MMSYSERR_NOTSUPPORTED, 5, 5)
    );
    assert_eq!(
        size_of(0, GSM8K, GSM8K, 5, 5),
        (MMSYSERR_NOTSUPPORTED, 5, 5)
    );
    assert_eq!(
        size_of(2, PCM8K, GSM8K, 5, 5),
        (MMSYSERR_NOTSUPPORTED, 5, 5)
    );
}

/// Converts `src` into a destination of `dst_len` zero bytes. Returns the
/// result, the used lengths and the libgsm calls made.
fn convert_trace(
    fdw: DWORD,
    gsm_source: bool,
    src: &[u8],
    dst_len: usize,
) -> (LResult, DWORD, DWORD, Vec<(char, usize)>) {
    let format = if gsm_source { wfx(GSM8K) } else { wfx(PCM8K) };
    let mut dst = vec![0u8; dst_len];
    let mut header = StreamHeader {
        fdw_convert: fdw,
        src,
        dst: &mut dst,
        src_used: 0,
        dst_used: 0,
    };
    let mut codec = Mock::default();
    let ret = stream_convert(&mut codec, &format, &mut header);
    (ret, header.src_used, header.dst_used, codec.calls)
}

#[test]
fn stream_convert_lays_out_frames_for_each_length() {
    let p = |n| pattern(n, 7);
    assert_eq!(convert_trace(0, false, &p(0), 0), (0, 0, 0, vec![('r', 0)]));
    assert_eq!(
        convert_trace(0, false, &p(100), 65),
        (0, 100, 65, vec![('r', 0), ('e', 32), ('e', 33)])
    );
    assert_eq!(
        convert_trace(0, false, &p(640), 65),
        (0, 640, 65, vec![('r', 0), ('e', 32), ('e', 33)])
    );
    assert_eq!(
        convert_trace(0, false, &p(1000), 130),
        (
            0,
            1000,
            130,
            vec![('r', 0), ('e', 32), ('e', 33), ('e', 32), ('e', 33)]
        )
    );
    assert_eq!(
        convert_trace(ACM_STREAMCONVERTF_BLOCKALIGN, false, &p(1000), 130),
        (0, 640, 65, vec![('r', 0), ('e', 32), ('e', 33)])
    );
    assert_eq!(
        convert_trace(0, false, &p(1000), 129),
        (ACMERR_NOTPOSSIBLE, 0, 0, vec![('r', 0)])
    );
    assert_eq!(
        convert_trace(0, true, &p(65), 640),
        (0, 65, 640, vec![('r', 0), ('d', 33), ('d', 32)])
    );
    assert_eq!(convert_trace(0, true, &p(64), 0), (0, 0, 0, vec![('r', 0)]));
    assert_eq!(
        convert_trace(0, true, &p(130), 1279),
        (ACMERR_NOTPOSSIBLE, 0, 0, vec![('r', 0)])
    );
}

#[test]
fn dispatch_routes_every_message_class() {
    for msg in [
        DRV_LOAD,
        DRV_ENABLE,
        DRV_OPEN,
        DRV_CLOSE,
        DRV_DISABLE,
        DRV_FREE,
        DRV_QUERYCONFIGURE,
    ] {
        assert_eq!(dispatch(msg), Dispatch::Reply(1));
    }
    assert_eq!(dispatch(DRV_CONFIGURE), Dispatch::Configure);
    for msg in [DRV_INSTALL, DRV_REMOVE] {
        assert_eq!(dispatch(msg), Dispatch::Reply(DRVCNF_RESTART));
    }
    for msg in [
        ACMDM_DRIVER_NOTIFY,
        ACMDM_STREAM_PREPARE,
        ACMDM_STREAM_UNPREPARE,
    ] {
        assert_eq!(dispatch(msg), Dispatch::Reply(MMSYSERR_NOERROR));
    }
    let handlers = [
        (ACMDM_DRIVER_DETAILS, Dispatch::DriverDetails),
        (ACMDM_FORMATTAG_DETAILS, Dispatch::FormatTagDetails),
        (ACMDM_FORMAT_DETAILS, Dispatch::FormatDetails),
        (ACMDM_FORMAT_SUGGEST, Dispatch::FormatSuggest),
        (ACMDM_STREAM_OPEN, Dispatch::StreamOpen),
        (ACMDM_STREAM_CLOSE, Dispatch::StreamClose),
        (ACMDM_STREAM_SIZE, Dispatch::StreamSize),
        (ACMDM_STREAM_CONVERT, Dispatch::StreamConvert),
    ];
    for (msg, route) in handlers {
        assert_eq!(dispatch(msg), route);
    }
    for msg in [
        ACMDM_HARDWARE_WAVE_CAPS_INPUT,
        ACMDM_HARDWARE_WAVE_CAPS_OUTPUT,
        ACMDM_FILTERTAG_DETAILS,
        ACMDM_FILTER_DETAILS,
        ACMDM_STREAM_RESET,
    ] {
        assert_eq!(dispatch(msg), Dispatch::Reply(MMSYSERR_NOTSUPPORTED));
    }
    assert_eq!(dispatch(0x1234), Dispatch::DefDriverProc);
}
