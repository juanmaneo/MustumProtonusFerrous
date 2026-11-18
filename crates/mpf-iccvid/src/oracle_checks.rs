//! Tests against Wine's oracle vectors (`oracle.rs`), then Rust-only edge cases.
//!
//! [`run`] replays each oracle command the way `target/canon/iccvid/harness.c`
//! does, so every vector can be compared as the exact text Wine printed.

use super::*;
use crate::oracle::{DC, EDGE, GA, RC, V1_15, V1_16, V1_24, V1_32, V4_15, V4_16, V4_24, V4_32};

fn int(s: &str) -> i64 {
    s.parse().unwrap()
}

fn unhex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// A 12-byte oracle codebook: four bytes each of r, g and b.
fn codebook(s: &str) -> Codebook {
    let b = unhex(s);
    let quad = |i: usize| [b[i], b[i + 1], b[i + 2], b[i + 3]];
    Codebook {
        r: quad(0),
        g: quad(4),
        b: quad(8),
    }
}

/// Frame geometry shared by a `dc` command and its frames.
struct Geometry {
    bits: u32,
    inverted: bool,
    width: usize,
    height: usize,
    stride: usize,
}

impl Geometry {
    /// Reads the five leading operands of a `dc` command.
    fn parse(args: &[&str]) -> Self {
        Self {
            bits: int(args[0]) as u32,
            inverted: int(args[1]) != 0,
            width: int(args[2]) as usize,
            height: int(args[3]) as usize,
            stride: int(args[4]) as usize,
        }
    }

    fn blank(&self) -> Vec<u8> {
        vec![0; self.stride * self.height]
    }

    fn decode(&self, dec: &mut Decoder, frame: &[u8], out: &mut [u8]) -> Result<(), Error> {
        dec.decode_frame(
            frame,
            Target {
                pixels: out,
                width: self.width,
                height: self.height,
                stride: self.stride,
                depth: Depth::from_bits(self.bits),
                inverted: self.inverted,
            },
        )
    }
}

/// Replays one harness command and returns the reply line (without the newline).
fn run(command: &str) -> String {
    let mut words = command.split_whitespace();
    let cmd = words.next().unwrap_or("");
    let args: Vec<&str> = words.collect();
    match cmd {
        "rc" => {
            let mode = if int(args[0]) != 0 {
                Mode::Mono
            } else {
                Mode::Colour
            };
            let data = unhex(args[1]);
            let mut r = Reader::new(&data);
            let c = read_codebook(&mut r, mode).unwrap();
            format!("{}{}{} {}", hex(&c.r), hex(&c.g), hex(&c.b), r.pos)
        }
        "ga" => {
            let [inv, x, y, stride, bpp, height] = args[..] else {
                panic!("ga takes six operands: {command}");
            };
            get_addr(
                int(inv) != 0,
                int(x) as usize,
                int(y) as usize,
                int(stride) as usize,
                int(bpp) as usize,
                int(height) as usize,
            )
            .to_string()
        }
        "v1" | "v4" => {
            let bits = int(args[0]) as u32;
            let inverted = int(args[1]) != 0;
            let stride = int(args[2]) as usize;
            let addr = int(args[3]);
            let len = int(args[4]) as usize;
            let books: Vec<Codebook> = args[5..].iter().map(|s| codebook(s)).collect();
            let mut buf = vec![0xa5; len];
            let depth = Depth::from_bits(bits);
            let res = if cmd == "v1" {
                write_v1(&mut buf, addr, stride, inverted, depth, &books[0])
            } else {
                let quad = [&books[0], &books[1], &books[2], &books[3]];
                write_v4(&mut buf, addr, stride, inverted, depth, quad)
            };
            assert!(res.is_ok(), "{command}");
            hex(&buf)
        }
        "dc" => {
            let g = Geometry::parse(&args);
            let mut dec = Decoder::new();
            let mut out = g.blank();
            args[5..]
                .iter()
                .map(|f| {
                    g.decode(&mut dec, &unhex(f), &mut out).unwrap();
                    hex(&out)
                })
                .collect::<Vec<_>>()
                .join(" ")
        }
        _ => panic!("unknown oracle command {cmd}"),
    }
}

fn check(table: &[(&str, &str)]) {
    for &(command, expected) in table {
        assert_eq!(run(command), expected, "{command}");
    }
}

#[test]
fn oracle_read_codebook() {
    check(RC);
}

#[test]
fn oracle_get_addr() {
    check(GA);
}

#[test]
fn oracle_write_v1_15() {
    check(V1_15);
}

#[test]
fn oracle_write_v1_16() {
    check(V1_16);
}

#[test]
fn oracle_write_v1_24() {
    check(V1_24);
}

#[test]
fn oracle_write_v1_32() {
    check(V1_32);
}

#[test]
fn oracle_write_v4_15() {
    check(V4_15);
}

#[test]
fn oracle_write_v4_16() {
    check(V4_16);
}

#[test]
fn oracle_write_v4_24() {
    check(V4_24);
}

#[test]
fn oracle_write_v4_32() {
    check(V4_32);
}

#[test]
fn oracle_decode_cinepak() {
    check(DC);
}

#[test]
fn oracle_decode_cinepak_edge_frames() {
    check(EDGE);
}

#[test]
#[should_panic(expected = "unknown oracle command")]
fn unknown_oracle_command_panics() {
    run("bogus");
}

#[test]
#[should_panic(expected = "ga takes six operands")]
fn get_addr_with_missing_operands_panics() {
    run("ga 1 2 3");
}

#[test]
fn every_frame_prefix_is_ok_or_truncated() {
    for &(command, _) in DC.iter().chain(EDGE) {
        let args: Vec<&str> = command.split_whitespace().skip(1).collect();
        let g = Geometry::parse(&args);
        let frames: Vec<Vec<u8>> = args[5..].iter().map(|f| unhex(f)).collect();
        for (i, frame) in frames.iter().enumerate() {
            for end in 0..frame.len() {
                let mut dec = Decoder::new();
                let mut out = g.blank();
                for earlier in &frames[..i] {
                    g.decode(&mut dec, earlier, &mut out).unwrap();
                }
                let res = g.decode(&mut dec, &frame[..end], &mut out);
                assert!(
                    matches!(res, Ok(()) | Err(Error::Truncated)),
                    "{command}: frame {i} cut to {end} bytes"
                );
            }
        }
    }
}

#[test]
fn depth_from_bits_maps_unlisted_depths_to_24() {
    assert!(matches!(Depth::from_bits(15), Depth::Bits15));
    assert!(matches!(Depth::from_bits(16), Depth::Bits16));
    assert!(matches!(Depth::from_bits(24), Depth::Bits24));
    assert!(matches!(Depth::from_bits(32), Depth::Bits32));
    assert!(matches!(Depth::from_bits(8), Depth::Bits24));
}

/// Builds a frame with the given flags and strips, after a 10-byte header.
fn frame(flags: u8, strips: &[Vec<u8>]) -> Vec<u8> {
    let mut out = vec![flags, 0, 0, 0, 0, 0, 0, 0];
    out.extend_from_slice(&(strips.len() as u16).to_be_bytes());
    out.extend(strips.concat());
    out
}

/// A strip header (kind 0x1000) for a strip that ends at row `bottom`, then its chunks.
fn strip(bottom: u16, chunks: &[u8]) -> Vec<u8> {
    let size = 12 + chunks.len() as u16;
    let mut out = vec![0x10, 0x00];
    out.extend_from_slice(&size.to_be_bytes());
    out.extend_from_slice(&[0, 0, 0, 0]);
    out.extend_from_slice(&bottom.to_be_bytes());
    out.extend_from_slice(&[0, 0]);
    out.extend_from_slice(chunks);
    out
}

/// A chunk header that declares `size` bytes, counting the header itself.
fn chunk_header(id: u16, size: u16) -> Vec<u8> {
    let mut out = id.to_be_bytes().to_vec();
    out.extend_from_slice(&size.to_be_bytes());
    out
}

fn chunk(id: u16, body: &[u8]) -> Vec<u8> {
    let mut out = chunk_header(id, 4 + body.len() as u16);
    out.extend_from_slice(body);
    out
}

/// Decodes one frame into a fresh 4x4 24-bit top-down buffer.
fn decode_once(frame: &[u8]) -> Result<(), Error> {
    let g = Geometry {
        bits: 24,
        inverted: false,
        width: 4,
        height: 4,
        stride: 12,
    };
    g.decode(&mut Decoder::new(), frame, &mut g.blank())
}

#[test]
fn strip_count_limit() {
    let mut full = frame(0, &[]);
    full[8..10].copy_from_slice(&32u16.to_be_bytes());
    assert!(matches!(decode_once(&full), Err(Error::TooManyStrips)));

    let mut short = frame(0, &[]);
    short[8..10].copy_from_slice(&31u16.to_be_bytes());
    assert!(matches!(decode_once(&short), Err(Error::Truncated)));
}

#[test]
fn plain_codebook_chunk_past_table_is_rejected() {
    // 261 colour entries (6 bytes each), then 261 mono entries (4 bytes each).
    let colour = frame(0, &[strip(4, &chunk_header(0x2000, 4 + 261 * 6))]);
    let mono = frame(0, &[strip(4, &chunk_header(0x2400, 4 + 261 * 4))]);
    assert!(matches!(decode_once(&colour), Err(Error::CodebookOverflow)));
    assert!(matches!(decode_once(&mono), Err(Error::CodebookOverflow)));
}

#[test]
fn masked_codebook_entry_past_table_is_rejected() {
    // Eight empty mask words cover entries 0..256. The ninth sets entry 260.
    let mut body = vec![0; 32];
    body.extend_from_slice(&0x0800_0000u32.to_be_bytes());
    let colour = frame(0, &[strip(4, &chunk(0x2100, &body))]);
    let mono = frame(0, &[strip(4, &chunk(0x2500, &body))]);
    assert!(matches!(decode_once(&colour), Err(Error::CodebookOverflow)));
    assert!(matches!(decode_once(&mono), Err(Error::CodebookOverflow)));
}

#[test]
fn block_past_end_of_output_is_rejected() {
    // Bottom-up, the second block row starts at byte 8 and needs bytes up to 17.
    let cb = Codebook {
        r: [0x20; 4],
        g: [0x20; 4],
        b: [0x20; 4],
    };
    let mut buf = [0xa5u8; 15];
    let res = write_v1(&mut buf, 0, 8, true, Depth::Bits24, &cb);
    assert!(matches!(res, Err(Error::OutOfBounds)));
}

#[test]
fn decoded_block_past_output_is_rejected() {
    // The first block of a 4x4 bottom-up frame starts at byte 36, past an 8-byte output.
    let cases: [(&str, Vec<u8>); 5] = [
        ("0x3000 V4", chunk(0x3000, &[0x80, 0, 0, 0, 0, 0, 0, 0])),
        ("0x3000 V1", chunk(0x3000, &[0, 0, 0, 0, 0])),
        ("0x3100 V4", chunk(0x3100, &[0xc0, 0, 0, 0, 0, 0, 0, 0])),
        ("0x3100 V1", chunk(0x3100, &[0x80, 0, 0, 0, 0])),
        ("0x3200 V1", chunk(0x3200, &[0])),
    ];
    for (name, body) in cases {
        let frame = frame(0, &[strip(4, &body)]);
        let g = Geometry {
            bits: 24,
            inverted: false,
            width: 4,
            height: 4,
            stride: 12,
        };
        let mut out = [0u8; 8];
        let res = g.decode(&mut Decoder::new(), &frame, &mut out);
        assert!(matches!(res, Err(Error::OutOfBounds)), "{name}");
    }
}

#[test]
fn rows_before_the_start_are_skipped_but_earlier_rows_kept() {
    // Top-down from byte 0, the second block row would start at byte -8.
    let cb = Codebook {
        r: [0x20; 4],
        g: [0x20; 4],
        b: [0x20; 4],
    };
    let mut buf = [0xa5u8; 32];
    let res = write_v1(&mut buf, 0, 8, false, Depth::Bits24, &cb);
    assert!(res.is_ok());
    assert_eq!(buf[..12], [0x20; 12]);
    assert_eq!(buf[12..], [0xa5; 20]);
}
