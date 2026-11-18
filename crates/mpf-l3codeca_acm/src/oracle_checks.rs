//! Tests against Wine's oracle vectors, then Rust-only edge cases.
//!
//! The vectors are the replies of Wine's verbatim `mpegl3.c`, built with ASan
//! and UBSan by `target/canon/l3codeca.acm/build.sh`. [`replay`] runs each
//! command through the port and prints the result the way the C harness does,
//! so every vector is compared as the exact text.

use super::*;
use crate::convert::{stream_convert, Decoder, Usage, MPG123_ERR};

#[rustfmt::skip]
const VECTORS: &[(&str, &str)] = &[
    ("index 1 1 8000 8000 1 8 0 0", "0"),
    ("index 85 1 22050 0 1 0 12 1", "8"),
    ("tag 0 2 0", "0 2 80 40 0 MPeg"),
    ("tag 0 3 0", "512"),
    ("tag 2 0 0", "0 2 80 40 0 MPeg"),
    ("details 0 1 5 0 0 0 0 0 0 0 0", "0 1 2 11025 22050 2 8 0 0"),
    ("details 0 1 48 0 0 0 0 0 0 0 0", "512 1 0 0 0 0 0 0 0"),
    ("details 0 65537 0 0 0 0 0 0 0 0 0", "11 1 0 0 0 0 0 0 0"),
    ("suggest 0 30 30 85 1 22050 0 1 0 12 1 0 0 0 0 0 0 0 0", "0 1 1 22050 44100 2 16 0 0"),
    ("suggest 0 30 30 1 1 8000 8000 1 8 0 0 0 0 0 0 0 0 0 0", "512 0 1 8000 0 0 16 0 0"),
    ("suggest 983040 30 30 85 1 22050 0 1 0 12 1 80 1 22050 0 1 0 0 0", "512 80 1 22050 0 1 0 0 0"),
    ("size 1 1 1 22050 44100 2 16 0 0 85 1 22050 16000 1 0 12 1 4000", "0 2304"),
    ("size 0 85 1 22050 16000 1 0 12 1 1 1 22050 44100 2 16 0 0 9216", "0 27648"),
    ("size 1 85 1 22050 16000 1 0 12 1 1 1 22050 44100 2 16 0 0 9216", "0 3340"),
    ("size 0 1 1 22050 44100 2 16 0 0 85 1 22050 16000 1 0 12 1 4000", "0 4670"),
    ("size 1 1 1 22050 44100 2 16 0 0 85 1 22050 16000 1 0 12 1 2000", "0 3261085696"),
    ("size 1 85 1 22050 16000 1 0 12 1 1 2 22050 88200 4 16 0 0 4607", "512"),
    ("open 0 85 1 22050 0 1 0 12 1 1 1 22050 44100 2 16 0 0", "0"),
    ("open 0 85 1 22050 0 1 0 11 1 1 1 22050 44100 2 16 0 0", "512"),
    ("open 0 85 1 22050 0 1 0 12 1 1 1 22050 44100 2 8 0 0", "8"),
    ("convert 0 100 4096 0 0:1000 -10:0", "0 100 1000 F100=0,R4096=0/1000,R3096=-10/0"),
    ("convert 0 100 4096 0 -11:0 0:200 -10:0", "0 100 200 F100=0,R4096=-11/0,R4096=0/200,R3896=-10/0"),
    ("convert 0 100 4096 -1", "0 0 0 F100=-1"),
    ("convert 16 100 4096 0 0:500 -10:0", "0 100 500 seek,close,open,F100=0,R4096=0/500,R3596=-10/0"),
    ("convert 0 100 4096 0 0:300 -1:0", "0 0 0 F100=0,R4096=0/300,R3796=-1/0"),
];

#[rustfmt::skip]
const EDGE: &[(&str, &str)] = &[
    ("index 0 1 8000 8000 1 8 0 0", "4294967295"),
    ("index 1 1 8000 8000 1 24 0 0", "4294967295"),
    ("index 1 2 44100 0 0 16 0 0", "31"),
    ("index 85 2 48000 0 0 16 0 0", "17"),
    ("index 80 1 8000 0 0 0 0 0", "0"),
    ("tag 2 0 1", "0 0 1 16 36 PCM"),
    ("tag 2 0 3", "512"),
    ("tag 1 0 80", "0 2 80 40 0 MPeg"),
    ("tag 1 0 85", "0 1 85 30 0 MPeg3"),
    ("tag 1 0 2", "512"),
    ("tag 3 0 0", "8"),
    ("tag 0 1 0", "0 1 85 30 0 MPeg3"),
    ("details 1 0 0 1 1 8000 8000 1 8 0 0", "0 1 1 8000 8000 1 8 0 0"),
    ("details 1 0 0 1 1 8000 8000 1 24 0 0", "512 1 1 8000 8000 1 24 0 0"),
    ("details 2 0 0 0 0 0 0 0 0 0 0", "8 0 0 0 0 0 0 0 0"),
    ("details 0 85 0 0 0 0 0 0 0 0 0", "512 85 0 0 0 0 0 0 0"),
    ("suggest 0 15 30 1 1 8000 8000 1 8 0 0 0 0 0 0 0 0 0 0", "512 0 0 0 0 0 0 0 0"),
    ("suggest 0 30 15 1 1 8000 8000 1 8 0 0 0 0 0 0 0 0 0 0", "512 0 0 0 0 0 0 0 0"),
    ("suggest 0 30 30 0 1 8000 8000 1 8 0 0 0 0 0 0 0 0 0 0", "512 0 0 0 0 0 0 0 0"),
    ("suggest 65536 30 30 85 1 22050 0 1 0 12 1 1 1 22050 0 1 0 0 0", "0 1 1 22050 44100 2 16 0 0"),
    ("suggest 983040 30 30 1 1 8000 8000 1 8 0 0 1 1 8000 0 0 24 0 0", "512 1 1 8000 0 0 24 0 0"),
    ("size 2 1 1 22050 44100 2 16 0 0 85 1 22050 16000 1 0 12 1 4000", "8"),
    ("size 1 1 1 22050 44100 2 16 0 0 1 1 22050 44100 2 16 0 0 4000", "8"),
    ("size 0 85 1 22050 16000 1 0 12 1 85 1 22050 16000 1 0 12 1 4000", "8"),
    ("size 0 80 1 22050 16000 1 0 12 1 1 1 22050 44100 2 16 0 0 9216", "0 27648"),
    ("size 1 1 1 22050 44100 2 16 0 0 85 1 22050 16000 1 0 12 1 3000", "512"),
    ("size 0 1 1 22050 44100 2 16 0 0 85 1 22050 16000 1 0 12 1 0", "512"),
    ("size 0 85 1 22050 16000 1 0 12 1 1 1 22050 44100 2 16 0 0 0", "512"),
    ("size 0 1 1 22050 44100 2 16 0 0 85 1 22050 16000 1 0 12 1 4608", "0 4670"),
    ("size 0 85 1 22050 16000 1 0 12 1 1 1 22050 44100 2 16 0 0 835", "0 2304"),
    ("open 0 80 1 22050 0 1 0 0 0 1 1 22050 44100 2 16 0 0", "0"),
    ("open 0 85 1 22050 0 1 0 12 2 1 1 22050 44100 2 16 0 0", "512"),
    ("open 0 1 1 22050 44100 2 16 0 0 1 1 22050 44100 2 16 0 0", "8"),
    ("open 0 85 1 22050 0 1 0 12 1 80 1 22050 0 1 0 0 0", "8"),
    ("open 0 85 1 22050 0 1 0 12 1 1 1 44100 88200 2 16 0 0", "8"),
    ("open 0 85 1 22050 0 1 0 12 1 1 2 22050 88200 4 16 0 0", "8"),
    ("open 0 85 1 22050 0 1 0 12 1 1 1 22050 44100 2 24 0 0", "512"),
    ("open 0 1 1 8000 0 0 0 0 0 1 1 22050 44100 2 16 0 0", "512"),
    ("convert 0 100 4096 0 0:4096", "0 100 4096 F100=0,R4096=0/4096"),
    ("convert 0 0 4096 0 0:1000 -10:0", "0 0 1000 R4096=0/1000,R3096=-10/0"),
    ("convert 0 100 0 0 0:5 -10:0", "0 100 5 F100=0,R0=0/5"),
    ("convert 0 100 4096 0 -1:0", "0 0 0 F100=0,R4096=-1/0"),
    ("convert 0 100 4096 0 -10:0", "0 100 0 F100=0,R4096=-10/0"),
    ("convert 0 100 4096 0", "0 0 0 F100=0,R4096=-1/0"),
];

// Rejected by the sanitizers, so not in the tables. Each is a C-side
// undefined behaviour that the port turns into an error or a panic:
//   size 1 1 1 22050 44100 2 16 0 0 85 1 22050 0 1 0 12 1 4000
//     UBSan division by zero: DEST encode, destination frame is zero
//   size 0 1 1 22050 44100 0 16 0 0 85 1 22050 16000 1 0 12 1 4000
//     UBSan division by zero: SOURCE encode, source block align is zero
//   size 1 85 1 22050 16000 1 0 12 1 1 2 22050 88200 0 16 0 0 4000
//     UBSan division by zero: DEST decode, destination block align is zero
//   size 0 85 1 22050 0 1 0 12 1 1 1 22050 44100 2 16 0 0 4000
//     UBSan division by zero: SOURCE decode, source frame is zero
//   open 2 85 1 22050 0 1 0 12 1 1 1 22050 44100 2 16 0 0
//     exit -6 (assert): ASYNC open, Wine's assert

/// One numeric word of a command, as the C harness reads it.
fn number(word: Option<&str>) -> u64 {
    word.expect("missing argument").parse().expect("number")
}

/// One signed word: the feed result and the script steps.
fn signed(word: Option<&str>) -> i64 {
    word.expect("missing argument").parse().expect("number")
}

/// Eight words in harness order: `wFormatTag`, `nChannels`, `nSamplesPerSec`,
/// `nAvgBytesPerSec`, `nBlockAlign`, `wBitsPerSample`, `cbSize`, `wID`.
fn wfx<'a>(words: &mut impl Iterator<Item = &'a str>) -> WaveFormat {
    let mut next = || number(words.next());
    WaveFormat {
        format_tag: next() as u16,
        channels: next() as u16,
        samples_per_sec: next() as u32,
        avg_bytes_per_sec: next() as u32,
        block_align: next() as u16,
        bits_per_sample: next() as u16,
        cb_size: next() as u16,
        mpeg_id: next() as u16,
    }
}

fn wfx_text(format: &WaveFormat) -> String {
    format!(
        "{} {} {} {} {} {} {} {}",
        format.format_tag,
        format.channels,
        format.samples_per_sec,
        format.avg_bytes_per_sec,
        format.block_align,
        format.bits_per_sample,
        format.cb_size,
        format.mpeg_id
    )
}

/// The reply for `command`, printed the way the C harness prints it.
fn replay(command: &str) -> String {
    let mut words = command.split_whitespace();
    match words.next().expect("oracle command") {
        "index" => {
            let format = wfx(&mut words);
            format::get_format_index(&format)
                .unwrap_or(u32::MAX)
                .to_string()
        }
        "tag" => {
            let query = number(words.next()) as u32;
            let mut details = tag::FormatTagDetails {
                format_tag_index: number(words.next()) as u32,
                format_tag: number(words.next()) as u32,
                support: 0,
                format_size: 0,
                standard_formats: 0,
                name: "",
            };
            match tag::format_tag_details(&mut details, query) {
                Ok(()) => format!(
                    "0 {} {} {} {} {}",
                    details.format_tag_index,
                    details.format_tag,
                    details.format_size,
                    details.standard_formats,
                    details.name
                ),
                Err(error) => error.code().to_string(),
            }
        }
        "details" => {
            let query = number(words.next()) as u32;
            let format_tag = number(words.next()) as u32;
            let format_index = number(words.next()) as u32;
            let mut details = format::FormatDetails {
                format_tag,
                format_index,
                support: 0,
                format: wfx(&mut words),
            };
            let rc = match format::format_details(&mut details, query) {
                Ok(()) => 0,
                Err(error) => error.code(),
            };
            format!("{rc} {}", wfx_text(&details.format))
        }
        "suggest" => {
            let fdw = number(words.next()) as u32;
            let cb_src = number(words.next()) as u32;
            let cb_dst = number(words.next()) as u32;
            let src = wfx(&mut words);
            let mut dst = wfx(&mut words);
            let rc = match format::format_suggest(fdw, cb_src, cb_dst, &src, &mut dst) {
                Ok(()) => 0,
                Err(error) => error.code(),
            };
            format!("{rc} {}", wfx_text(&dst))
        }
        "size" => {
            let query = number(words.next()) as u32;
            let src = wfx(&mut words);
            let dst = wfx(&mut words);
            let given = number(words.next()) as u32;
            let mut sized = size::StreamSize {
                fdw_size: query,
                cb_src_length: 0,
                cb_dst_length: 0,
            };
            if query == size::ACM_STREAMSIZEF_DESTINATION {
                sized.cb_dst_length = given;
            } else {
                sized.cb_src_length = given;
            }
            match size::stream_size(&src, &dst, &mut sized) {
                Ok(()) => {
                    let out = if query == size::ACM_STREAMSIZEF_DESTINATION {
                        sized.cb_src_length
                    } else {
                        sized.cb_dst_length
                    };
                    format!("0 {out}")
                }
                Err(error) => error.code().to_string(),
            }
        }
        "open" => {
            let fdw = number(words.next()) as u32;
            let src = wfx(&mut words);
            let dst = wfx(&mut words);
            match open::stream_open(fdw, &src, &dst) {
                Ok(()) => "0".to_string(),
                Err(error) => error.code().to_string(),
            }
        }
        "convert" => {
            let fdw = number(words.next()) as u32;
            let nsrc = number(words.next()) as usize;
            let ndst = number(words.next()) as usize;
            let feed_ret = signed(words.next()) as i32;
            let steps: Vec<(i32, usize)> = words
                .map(|step| {
                    let (ret, count) = step.split_once(':').expect("ret:size");
                    (ret.parse().expect("ret"), count.parse().expect("size"))
                })
                .collect();
            let mut decoder = Script::new(feed_ret, steps);
            let src = vec![0u8; nsrc];
            let mut dst = vec![0u8; ndst];
            let usage = stream_convert(&mut decoder, fdw, &src, &mut dst);
            format!(
                "0 {} {} {}",
                usage.src_used,
                usage.dst_used,
                decoder.trace.join(",")
            )
        }
        other => panic!("unknown oracle command {other}"),
    }
}

/// A decoder that answers `read` from a script and logs each call the way the
/// C stubs do.
struct Script {
    feed_ret: i32,
    steps: Vec<(i32, usize)>,
    next: usize,
    trace: Vec<String>,
}

impl Script {
    fn new(feed_ret: i32, steps: Vec<(i32, usize)>) -> Self {
        Script {
            feed_ret,
            steps,
            next: 0,
            trace: Vec::new(),
        }
    }
}

impl Decoder for Script {
    fn seek_to_start(&mut self) {
        self.trace.push("seek".to_string());
    }

    fn close(&mut self) {
        self.trace.push("close".to_string());
    }

    fn open_feed(&mut self) {
        self.trace.push("open".to_string());
    }

    fn feed(&mut self, data: &[u8]) -> i32 {
        self.trace
            .push(format!("F{}={}", data.len(), self.feed_ret));
        self.feed_ret
    }

    fn read(&mut self, out: &mut [u8]) -> (i32, usize) {
        let (ret, count) = match self.steps.get(self.next) {
            Some(&step) => {
                self.next += 1;
                step
            }
            None => (MPG123_ERR, 0),
        };
        self.trace.push(format!("R{}={}/{}", out.len(), ret, count));
        (ret, count)
    }
}

fn pcm(
    channels: u16,
    samples_per_sec: u32,
    avg_bytes_per_sec: u32,
    block_align: u16,
    bits_per_sample: u16,
) -> WaveFormat {
    WaveFormat {
        format_tag: WAVE_FORMAT_PCM,
        channels,
        samples_per_sec,
        avg_bytes_per_sec,
        block_align,
        bits_per_sample,
        cb_size: 0,
        mpeg_id: 0,
    }
}

fn mpeg3(samples_per_sec: u32, avg_bytes_per_sec: u32, block_align: u16) -> WaveFormat {
    WaveFormat {
        format_tag: WAVE_FORMAT_MPEGLAYER3,
        channels: 1,
        samples_per_sec,
        avg_bytes_per_sec,
        block_align,
        bits_per_sample: 0,
        cb_size: 12,
        mpeg_id: 1,
    }
}

#[test]
fn oracle_vectors_match_wine() {
    for (command, reply) in VECTORS {
        assert_eq!(replay(command), *reply, "{command}");
    }
}

#[test]
fn oracle_edge_matches_wine() {
    for (command, reply) in EDGE {
        assert_eq!(replay(command), *reply, "{command}");
    }
}

#[test]
#[should_panic(expected = "unknown oracle command bogus")]
fn unknown_oracle_command_panics() {
    replay("bogus 1");
}

#[test]
#[should_panic(expected = "asynchronous streams are not supported")]
fn async_open_asserts_as_wine_does() {
    let src = mpeg3(22050, 0, 1);
    let dst = pcm(1, 22050, 44100, 2, 16);
    let _ = open::stream_open(open::ACM_STREAMOPENF_ASYNC, &src, &dst);
}

#[test]
fn zero_divisors_and_rates_are_not_possible_instead_of_undefined() {
    let cases = [
        (
            size::ACM_STREAMSIZEF_DESTINATION,
            pcm(1, 22050, 44100, 2, 16),
            mpeg3(22050, 0, 1),
        ),
        (
            size::ACM_STREAMSIZEF_SOURCE,
            pcm(1, 22050, 44100, 0, 16),
            mpeg3(22050, 16000, 1),
        ),
        (
            size::ACM_STREAMSIZEF_DESTINATION,
            mpeg3(22050, 16000, 1),
            pcm(2, 22050, 88200, 0, 16),
        ),
        (
            size::ACM_STREAMSIZEF_SOURCE,
            mpeg3(22050, 0, 1),
            pcm(1, 22050, 44100, 2, 16),
        ),
        (
            size::ACM_STREAMSIZEF_DESTINATION,
            pcm(1, 22050, 44100, 2, 16),
            mpeg3(0, 16000, 1),
        ),
        (
            size::ACM_STREAMSIZEF_DESTINATION,
            mpeg3(0, 16000, 1),
            pcm(1, 22050, 44100, 2, 16),
        ),
        (
            size::ACM_STREAMSIZEF_SOURCE,
            pcm(1, 22050, 44100, 2, 16),
            mpeg3(0, 16000, 1),
        ),
        (
            size::ACM_STREAMSIZEF_SOURCE,
            mpeg3(0, 16000, 1),
            pcm(1, 22050, 44100, 2, 16),
        ),
    ];
    for (query, src, dst) in cases {
        let mut sized = size::StreamSize {
            fdw_size: query,
            cb_src_length: 4000,
            cb_dst_length: 4000,
        };
        assert_eq!(
            size::stream_size(&src, &dst, &mut sized),
            Err(Error::NotPossible)
        );
        assert_eq!((sized.cb_src_length, sized.cb_dst_length), (4000, 4000));
    }
}

#[test]
fn formats_usage_and_errors_compare_and_print() {
    let fmt_pcm = pcm(1, 8000, 8000, 1, 8);
    assert_eq!(fmt_pcm.clone(), fmt_pcm);
    assert!(format!("{fmt_pcm:?}").contains("samples_per_sec: 8000"));
    assert_ne!(Error::NotPossible, Error::NotSupported);
    assert_eq!(format!("{:?}", Error::InvalidParam), "InvalidParam");
    let usage = Usage {
        src_used: 1,
        dst_used: 2,
    };
    assert_ne!(
        usage,
        Usage {
            src_used: 0,
            dst_used: 0
        }
    );
    assert!(format!("{usage:?}").contains("dst_used: 2"));
}
