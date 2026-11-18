//! Oracle rows for `mpf-msg711_acm`. Each row is a command line for Wine's
//! `msg711.c`, built verbatim with `gcc -fsanitize=address,undefined`, and
//! the output that the C harness printed for it. Rows that trip a sanitizer
//! or exit non-zero are not listed.

use crate::convert::{stream_convert, Convert};
use crate::format::{format_details, get_format_index};
use crate::law::{alaw2linear, alaw2ulaw, linear2alaw, linear2ulaw, ulaw2alaw, ulaw2linear};
use crate::open::stream_open;
use crate::size::stream_size;
use crate::suggest::format_suggest;
use crate::tag::format_tag_details;
use crate::WaveFormat;

const VECTORS: &[(&str, &str)] = &[
    ("law alaw2linear 0", "value=-5504"),
    ("law alaw2linear 213", "value=8"),
    ("law ulaw2linear 0", "value=-32124"),
    ("law ulaw2linear 255", "value=0"),
    ("law linear2alaw 1000", "value=250"),
    ("law linear2alaw -1000", "value=122"),
    ("law linear2ulaw 1000", "value=206"),
    ("law linear2ulaw -32768", "value=0"),
    ("law alaw2ulaw 42", "value=0"),
    ("law ulaw2alaw 255", "value=213"),
    ("index 1 2 44100 16", "index=7"),
    ("index 6 1 8000 8", "index=0"),
    ("tag 1 6 0", "ret=0 tag=6 index=1 size=18 std=8 name=A-Law"),
    ("tag 2 0 0", "ret=0 tag=6 index=1 size=18 std=8 name=A-Law"),
    (
        "details 0 6 3 0 0 0 0",
        "ret=0 tag=6 ch=2 rate=11025 bits=8 block=2 avg=22050 cb=0",
    ),
    (
        "details 0 1 5 0 0 0 0",
        "ret=0 tag=1 ch=2 rate=22050 bits=16 block=4 avg=88200 cb=17185",
    ),
    (
        "details 1 0 0 7 1 8000 8",
        "ret=0 tag=7 ch=1 rate=8000 bits=8 block=30583 avg=305419896 cb=17185",
    ),
    (
        "suggest 0 16 16 1 2 44100 16 0 0 0 0",
        "ret=0 tag=6 ch=2 rate=44100 bits=8 block=2 avg=88200 cb=17185",
    ),
    ("size 1 1 6 1000", "ret=0 len=2000"),
    ("size 0 6 1 1000", "ret=0 len=2000"),
    ("open 0 6 1 8000 8 1 1 8000 16", "ret=0 convert=AlawToPcm16"),
    ("open 0 1 1 8000 16 6 1 8000 8", "ret=0 convert=PcmToAlaw"),
    (
        "convert 0 6 1 8000 8 1 1 8000 16 0102ff80 8",
        "ret=0 src=4 dst=8 out=80eb80e850038015",
    ),
    (
        "convert 0 1 1 8000 16 6 1 8000 8 0000008000ff7f 3",
        "ret=0 src=6 dst=3 out=d52a5a",
    ),
    (
        "convert 0 7 1 8000 8 6 1 8000 8 ff00 4",
        "ret=0 src=2 dst=2 out=d52aa5a5",
    ),
];

const EDGE: &[(&str, &str)] = &[
    ("law alaw2linear 128", "value=5504"),
    ("law alaw2linear 85", "value=-8"),
    ("law alaw2linear 256", "value=-5504"),
    ("law ulaw2linear 127", "value=0"),
    ("law ulaw2linear 128", "value=32124"),
    ("law linear2alaw 0", "value=213"),
    ("law linear2alaw -1", "value=85"),
    ("law linear2alaw 31", "value=212"),
    ("law linear2alaw 32", "value=215"),
    ("law linear2alaw 100000", "value=170"),
    ("law linear2alaw -100000", "value=42"),
    ("law linear2alaw 2147483647", "value=170"),
    ("law linear2alaw -2147483648", "value=42"),
    ("law linear2ulaw 32767", "value=128"),
    ("law linear2ulaw -1", "value=126"),
    ("law linear2ulaw 8158", "value=159"),
    ("law linear2ulaw 8159", "value=159"),
    ("law linear2ulaw 65535", "value=126"),
    ("law alaw2ulaw 0", "value=42"),
    ("law alaw2ulaw 213", "value=254"),
    ("law alaw2ulaw 128", "value=170"),
    ("law ulaw2alaw 0", "value=42"),
    ("law ulaw2alaw 127", "value=85"),
    ("law ulaw2alaw 128", "value=170"),
    ("index 0 0 0 0", "index=4294967295"),
    ("index 2 1 8000 16", "index=4294967295"),
    ("index 65537 1 8000 16", "index=0"),
    ("index 7 2 22050 8", "index=5"),
    ("index 6 2 44100 8", "index=7"),
    ("index 1 1 11025 16", "index=2"),
    ("index 1 1 44100 16", "index=6"),
    ("index 1 2 22050 16", "index=5"),
    ("index 1 1 8000 8", "index=4294967295"),
    ("index 6 1 16000 8", "index=4294967295"),
    ("tag 0 0 3", "ret=512"),
    ("tag 0 0 4294967295", "ret=512"),
    ("tag 0 1 0", "ret=0 tag=1 index=0 size=16 std=8 name=PCM"),
    (
        "tag 0 65537 0",
        "ret=0 tag=1 index=0 size=16 std=8 name=PCM",
    ),
    ("tag 0 0 2", "ret=0 tag=7 index=2 size=18 std=8 name=U-Law"),
    ("tag 2 1 0", "ret=0 tag=1 index=0 size=16 std=8 name=PCM"),
    ("tag 2 7 0", "ret=0 tag=7 index=2 size=18 std=8 name=U-Law"),
    ("tag 2 5 0", "ret=512"),
    ("tag 1 7 0", "ret=0 tag=7 index=2 size=18 std=8 name=U-Law"),
    ("tag 1 5 0", "ret=512"),
    ("tag 1 65537 0", "ret=512"),
    ("tag 3 0 0", "ret=8"),
    (
        "details 1 0 0 0 0 0 0",
        "ret=512 tag=0 ch=0 rate=0 bits=0 block=30583 avg=305419896 cb=17185",
    ),
    (
        "details 1 0 0 1 1 8000 16",
        "ret=0 tag=1 ch=1 rate=8000 bits=16 block=30583 avg=305419896 cb=17185",
    ),
    (
        "details 1 0 0 1 1 8000 8",
        "ret=512 tag=1 ch=1 rate=8000 bits=8 block=30583 avg=305419896 cb=17185",
    ),
    (
        "details 2 0 0 1 1 8000 16",
        "ret=8 tag=1 ch=1 rate=8000 bits=16 block=30583 avg=305419896 cb=17185",
    ),
    (
        "details 1 1 0 6 2 8000 8",
        "ret=0 tag=6 ch=2 rate=8000 bits=8 block=30583 avg=305419896 cb=17185",
    ),
    (
        "details 0 65537 0 1 1 8000 16",
        "ret=11 tag=1 ch=1 rate=8000 bits=16 block=30583 avg=305419896 cb=17185",
    ),
    (
        "details 0 5 0 0 0 0 0",
        "ret=11 tag=5 ch=0 rate=0 bits=0 block=30583 avg=305419896 cb=17185",
    ),
    (
        "details 0 6 99 1 1 8000 8",
        "ret=512 tag=6 ch=1 rate=8000 bits=8 block=30583 avg=305419896 cb=17185",
    ),
    (
        "details 0 7 2 0 0 0 0",
        "ret=0 tag=7 ch=1 rate=11025 bits=8 block=1 avg=11025 cb=0",
    ),
    (
        "details 0 6 7 0 0 0 0",
        "ret=0 tag=6 ch=2 rate=44100 bits=8 block=2 avg=88200 cb=0",
    ),
    (
        "details 0 1 7 0 0 0 0",
        "ret=0 tag=1 ch=2 rate=44100 bits=16 block=4 avg=176400 cb=17185",
    ),
    (
        "details 0 1 0 0 0 0 0",
        "ret=0 tag=1 ch=1 rate=8000 bits=16 block=2 avg=16000 cb=17185",
    ),
    (
        "details 0 0 0 0 0 0 0",
        "ret=11 tag=0 ch=0 rate=0 bits=0 block=30583 avg=305419896 cb=17185",
    ),
    (
        "details 0 1 8 0 0 0 0",
        "ret=512 tag=1 ch=0 rate=0 bits=0 block=30583 avg=305419896 cb=17185",
    ),
    (
        "suggest 0 15 16 1 1 8000 16 0 0 0 0",
        "ret=512 tag=0 ch=0 rate=0 bits=0 block=30583 avg=305419896 cb=17185",
    ),
    (
        "suggest 0 16 15 1 1 8000 16 0 0 0 0",
        "ret=512 tag=0 ch=0 rate=0 bits=0 block=30583 avg=305419896 cb=17185",
    ),
    (
        "suggest 0 16 16 2 1 8000 16 0 0 0 0",
        "ret=512 tag=0 ch=0 rate=0 bits=0 block=30583 avg=305419896 cb=17185",
    ),
    (
        "suggest 983040 16 16 1 1 8000 16 6 1 8000 8",
        "ret=0 tag=6 ch=1 rate=8000 bits=8 block=1 avg=8000 cb=17185",
    ),
    (
        "suggest 65536 16 16 1 2 8000 16 0 1 8000 8",
        "ret=512 tag=0 ch=2 rate=8000 bits=8 block=30583 avg=305419896 cb=17185",
    ),
    (
        "suggest 0 16 16 6 1 8000 8 0 0 0 0",
        "ret=0 tag=1 ch=1 rate=8000 bits=16 block=2 avg=16000 cb=17185",
    ),
    (
        "suggest 0 16 16 7 2 44100 8 0 0 0 0",
        "ret=0 tag=1 ch=2 rate=44100 bits=16 block=4 avg=176400 cb=17185",
    ),
    (
        "suggest 0 16 16 1 2 44100 16 0 0 0 0",
        "ret=0 tag=6 ch=2 rate=44100 bits=8 block=2 avg=88200 cb=17185",
    ),
    (
        "suggest 131072 16 16 1 2 44100 16 0 1 8000 8",
        "ret=0 tag=6 ch=1 rate=44100 bits=8 block=1 avg=88200 cb=17185",
    ),
    (
        "suggest 262144 16 16 1 2 44100 16 0 1 8000 8",
        "ret=0 tag=6 ch=2 rate=8000 bits=8 block=2 avg=16000 cb=17185",
    ),
    (
        "suggest 524288 16 16 1 1 8000 16 6 1 8000 16",
        "ret=512 tag=6 ch=1 rate=8000 bits=16 block=30583 avg=305419896 cb=17185",
    ),
    ("size 1 1 6 2147483648", "ret=0 len=0"),
    ("size 1 6 1 4294967295", "ret=0 len=2147483647"),
    ("size 0 6 1 4294967295", "ret=0 len=4294967294"),
    ("size 1 1 1 1000", "ret=8"),
    ("size 0 1 1 1000", "ret=8"),
    ("size 1 6 7 1000", "ret=0 len=1000"),
    ("size 0 7 6 1000", "ret=0 len=1000"),
    ("size 1 7 6 1000", "ret=0 len=1000"),
    ("size 0 6 7 1000", "ret=0 len=1000"),
    ("size 2 1 6 10", "ret=8"),
    ("size 1 6 0 10", "ret=8"),
    ("size 0 1 6 1000", "ret=0 len=500"),
    ("open 0 1 1 8000 16 1 1 8000 16", "ret=8"),
    ("open 0 6 1 8000 8 1 1 8000 8", "ret=512"),
    ("open 0 6 1 8000 8 1 2 8000 16", "ret=8"),
    ("open 0 6 1 8000 8 1 1 11025 16", "ret=8"),
    ("open 0 1 1 8000 8 6 1 8000 8", "ret=512"),
    ("open 0 1 2 8000 16 6 2 8000 8", "ret=0 convert=PcmToAlaw"),
    ("open 0 1 1 8000 16 6 1 16000 8", "ret=512"),
    (
        "open 0 7 1 8000 8 1 1 8000 16",
        "ret=0 convert=MulawToPcm16",
    ),
    ("open 0 1 1 8000 16 7 1 8000 8", "ret=0 convert=PcmToMulaw"),
    ("open 0 7 1 8000 8 6 1 8000 8", "ret=0 convert=MulawToAlaw"),
    ("open 0 6 1 8000 8 7 1 8000 8", "ret=0 convert=AlawToMulaw"),
    ("open 0 7 1 8000 8 6 1 11025 8", "ret=8"),
    ("open 0 7 1 8000 8 6 2 8000 8", "ret=8"),
    ("open 0 6 1 8000 8 7 2 8000 8", "ret=8"),
    ("open 0 6 1 11025 8 7 1 8000 8", "ret=8"),
    ("open 0 7 1 8000 8 7 1 8000 8", "ret=8"),
    ("open 0 6 1 8000 8 8 1 8000 8", "ret=512"),
    ("open 1 6 1 8000 8 1 1 8000 16", "ret=0 convert=AlawToPcm16"),
    ("open 4 6 1 8000 8 1 1 8000 16", "ret=0 convert=AlawToPcm16"),
    (
        "convert 16 6 1 8000 8 1 1 8000 16 0102ff80 8",
        "ret=0 src=4 dst=8 out=80eb80e850038015",
    ),
    (
        "convert 4 6 1 8000 8 1 1 8000 16 0102ff80 8",
        "ret=0 src=4 dst=8 out=80eb80e850038015",
    ),
    (
        "convert 32 6 1 8000 8 1 1 8000 16 0102ff80 8",
        "ret=0 src=4 dst=8 out=80eb80e850038015",
    ),
    (
        "convert 256 6 1 8000 8 1 1 8000 16 0102ff80 8",
        "ret=0 src=4 dst=8 out=80eb80e850038015",
    ),
    (
        "convert 0 6 1 8000 8 1 1 8000 16 0102ff80 6",
        "ret=0 src=3 dst=6 out=80eb80e85003",
    ),
    (
        "convert 0 6 1 8000 8 1 1 8000 16 01 8",
        "ret=0 src=1 dst=2 out=80eba5a5a5a5a5a5",
    ),
    (
        "convert 0 6 1 8000 8 1 1 8000 16 - 8",
        "ret=0 src=0 dst=0 out=a5a5a5a5a5a5a5a5",
    ),
    (
        "convert 0 6 1 8000 8 1 1 8000 16 0102 0",
        "ret=0 src=0 dst=0 out=",
    ),
    (
        "convert 0 1 1 8000 16 7 1 8000 8 0000008000ff7f 3",
        "ret=0 src=6 dst=3 out=ff0067",
    ),
    (
        "convert 0 1 1 8000 16 6 1 8000 8 ffff 1",
        "ret=0 src=2 dst=1 out=55",
    ),
    (
        "convert 0 1 1 8000 16 6 1 8000 8 - 4",
        "ret=0 src=0 dst=0 out=a5a5a5a5",
    ),
    (
        "convert 0 1 2 8000 16 6 2 8000 8 00000001 2",
        "ret=0 src=4 dst=2 out=d5c5",
    ),
    (
        "convert 0 7 1 8000 8 1 1 8000 16 0102ff80 8",
        "ret=0 src=4 dst=8 out=8486848a00007c7d",
    ),
    (
        "convert 0 7 1 8000 8 6 1 8000 8 0102ff80 4",
        "ret=0 src=4 dst=4 out=2b28d5aa",
    ),
    (
        "convert 0 6 1 8000 8 7 1 8000 8 0102ff80 4",
        "ret=0 src=4 dst=4 out=2b28d1aa",
    ),
    ("convert 0 1 1 8000 16 1 1 8000 16 0000 2", "ret=8"),
];

const HASHES: &[(&str, &str)] = &[
    ("hash alaw2linear", "hash=dd8c77dce877aa85"),
    ("hash ulaw2linear", "hash=a9e1809c40131686"),
    ("hash linear2alaw", "hash=268fdcee39de8325"),
    ("hash linear2ulaw", "hash=a47d847e972492c5"),
    ("hash alaw2ulaw", "hash=22f62d7534399265"),
    ("hash ulaw2alaw", "hash=68e669443853b0a5"),
];

/// The harness `init_wfx`: the fields it does not take are fixed values.
fn wave_format(format_tag: i64, channels: i64, rate: i64, bits: i64) -> WaveFormat {
    WaveFormat {
        format_tag: format_tag as u16,
        channels: channels as u16,
        samples_per_sec: rate as u32,
        avg_bytes_per_sec: 0x1234_5678,
        block_align: 0x7777,
        bits_per_sample: bits as u16,
        cb_size: 0x4321,
    }
}

fn num(text: &str) -> i64 {
    text.parse().unwrap()
}

/// The harness `put_wfx`.
fn describe(wfx: &WaveFormat) -> String {
    format!(
        " tag={} ch={} rate={} bits={} block={} avg={} cb={}",
        wfx.format_tag,
        wfx.channels,
        wfx.samples_per_sec,
        wfx.bits_per_sample,
        wfx.block_align,
        wfx.avg_bytes_per_sec,
        wfx.cb_size
    )
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// The harness `unhex`: a trailing odd character is dropped, and "-" is empty.
fn unhex(text: &str) -> Vec<u8> {
    text.as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect()
}

fn convert_name(convert: Convert) -> &'static str {
    match convert {
        Convert::AlawToPcm16 => "AlawToPcm16",
        Convert::PcmToAlaw => "PcmToAlaw",
        Convert::MulawToPcm16 => "MulawToPcm16",
        Convert::PcmToMulaw => "PcmToMulaw",
        Convert::MulawToAlaw => "MulawToAlaw",
        Convert::AlawToMulaw => "AlawToMulaw",
    }
}

fn index_row(a: &[&str]) -> String {
    let wfx = wave_format(num(a[0]), num(a[1]), num(a[2]), num(a[3]));
    let index = get_format_index(&wfx).map_or(u32::MAX, |index| index as u32);
    format!("index={index}")
}

fn tag_row(a: &[&str]) -> String {
    match format_tag_details(num(a[0]) as u32, num(a[1]) as u32, num(a[2]) as u32) {
        Ok(d) => format!(
            "ret=0 tag={} index={} size={} std={} name={}",
            d.format_tag, d.format_tag_index, d.format_size, d.standard_formats, d.name
        ),
        Err(e) => format!("ret={}", e.code()),
    }
}

fn details_row(a: &[&str]) -> String {
    let mut wfx = wave_format(num(a[3]), num(a[4]), num(a[5]), num(a[6]));
    let query = num(a[0]) as u32;
    let ret = match format_details(query, num(a[1]) as u32, num(a[2]) as u32, &mut wfx) {
        Ok(()) => 0,
        Err(e) => e.code(),
    };
    format!("ret={ret}{}", describe(&wfx))
}

fn suggest_row(a: &[&str]) -> String {
    let src = wave_format(num(a[3]), num(a[4]), num(a[5]), num(a[6]));
    let mut dst = wave_format(num(a[7]), num(a[8]), num(a[9]), num(a[10]));
    let fdw = num(a[0]) as u32;
    let ret = match format_suggest(fdw, num(a[1]) as u32, num(a[2]) as u32, &src, &mut dst) {
        Ok(()) => 0,
        Err(e) => e.code(),
    };
    format!("ret={ret}{}", describe(&dst))
}

fn size_row(a: &[&str]) -> String {
    let src = wave_format(num(a[1]), 0, 0, 0);
    let dst = wave_format(num(a[2]), 0, 0, 0);
    match stream_size(num(a[0]) as u32, &src, &dst, num(a[3]) as u32) {
        Ok(len) => format!("ret=0 len={len}"),
        Err(e) => format!("ret={}", e.code()),
    }
}

fn open_row(a: &[&str]) -> String {
    let src = wave_format(num(a[1]), num(a[2]), num(a[3]), num(a[4]));
    let dst = wave_format(num(a[5]), num(a[6]), num(a[7]), num(a[8]));
    match stream_open(num(a[0]) as u32, &src, &dst) {
        Ok(convert) => format!("ret=0 convert={}", convert_name(convert)),
        Err(e) => format!("ret={}", e.code()),
    }
}

/// The harness opens the stream with `fdwOpen` 0 and ignores its `fdwConvert`.
fn convert_row(a: &[&str]) -> String {
    let src = wave_format(num(a[1]), num(a[2]), num(a[3]), num(a[4]));
    let dst = wave_format(num(a[5]), num(a[6]), num(a[7]), num(a[8]));
    let convert = match stream_open(0, &src, &dst) {
        Ok(convert) => convert,
        Err(e) => return format!("ret={}", e.code()),
    };
    let input = unhex(a[9]);
    let mut out = vec![0xA5; num(a[10]) as usize];
    let (used_src, used_dst) = stream_convert(convert, &input, &mut out);
    format!("ret=0 src={used_src} dst={used_dst} out={}", hex(&out))
}

fn law_value(name: &str, value: i64) -> i64 {
    match name {
        "alaw2linear" => i64::from(alaw2linear(value as u8)),
        "ulaw2linear" => i64::from(ulaw2linear(value as u8)),
        "linear2alaw" => i64::from(linear2alaw(value as i32)),
        "linear2ulaw" => i64::from(linear2ulaw(value as i16)),
        "alaw2ulaw" => i64::from(alaw2ulaw(value as u8)),
        "ulaw2alaw" => i64::from(ulaw2alaw(value as u8)),
        other => panic!("unknown law function {other}"),
    }
}

fn law_row(a: &[&str]) -> String {
    format!("value={}", law_value(a[0], num(a[1])))
}

/// The harness `fnv_value`: FNV-1a over the four little-endian bytes.
fn fnv(mut hash: u64, value: i32) -> u64 {
    let bits = value as u32;
    for shift in [0, 8, 16, 24] {
        hash ^= u64::from((bits >> shift) & 0xFF);
        hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
    }
    hash
}

fn hash_row(a: &[&str]) -> String {
    let (name, domain): (&str, std::ops::Range<i64>) = match a[0] {
        "alaw2linear" | "ulaw2linear" | "alaw2ulaw" | "ulaw2alaw" => (a[0], 0..256),
        "linear2alaw" | "linear2ulaw" => (a[0], -32768..32768),
        other => panic!("unknown hash {other}"),
    };
    let mut hash = 0xCBF2_9CE4_8422_2325;
    for value in domain {
        hash = fnv(hash, law_value(name, value) as i32);
    }
    format!("hash={hash:016x}")
}

fn replay(line: &str) -> String {
    let mut words = line.split_whitespace();
    let command = words.next().unwrap_or("");
    let a: Vec<&str> = words.collect();
    match command {
        "index" => index_row(&a),
        "tag" => tag_row(&a),
        "details" => details_row(&a),
        "suggest" => suggest_row(&a),
        "size" => size_row(&a),
        "open" => open_row(&a),
        "convert" => convert_row(&a),
        "law" => law_row(&a),
        "hash" => hash_row(&a),
        _ => panic!("unknown oracle command {command:?}"),
    }
}

fn assert_rows(rows: &[(&str, &str)]) {
    for (line, expected) in rows {
        assert_eq!(replay(line), *expected, "{line}");
    }
}

#[test]
fn oracle_vectors_match_wine() {
    assert_rows(VECTORS);
}

#[test]
fn oracle_edge_matches_wine() {
    assert_rows(EDGE);
}

#[test]
fn oracle_hashes_match_wine() {
    assert_rows(HASHES);
}

#[test]
#[should_panic(expected = "asynchronous streams")]
fn async_open_is_rejected() {
    replay("open 2 1 1 8000 16 1 1 8000 8");
}

#[test]
#[should_panic(expected = "unknown oracle command")]
fn unknown_command_is_rejected() {
    replay("nope 1");
}

#[test]
#[should_panic(expected = "unknown law function")]
fn unknown_law_is_rejected() {
    replay("law nope 1");
}

#[test]
#[should_panic(expected = "unknown hash")]
fn unknown_hash_is_rejected() {
    replay("hash nope");
}
