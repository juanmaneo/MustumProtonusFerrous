//! Checks the paths that the oracle rows do not reach: the input limits, the bit reader's
//! truncation, the block types and their codes, and the chunk walk with inconsistent sizes.
//! The streams are written with a small encoder that follows RFC 1951.

use crate::header::{parse_header, HeaderError};
use crate::huffman::Code;
use crate::mszip::{decompress, Bits};
use crate::InflateError;

/// The order in which a dynamic block stores the code-length code lengths (RFC 1951 3.2.7).
const ORDER: [usize; 19] = [
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];

#[test]
fn input_limits() {
    assert_eq!(
        decompress(&[0; 38913], 0).err(),
        Some(InflateError::DataFormat)
    );
    assert_eq!(
        decompress(&[0; 38912], 0).err(),
        Some(InflateError::IllegalData)
    );
}

#[test]
fn signature_and_block_header_truncation() {
    assert_eq!(
        decompress(b"C", 0).err(),
        Some(InflateError::InputExhausted)
    );
    assert_eq!(decompress(b"XY", 0).err(), Some(InflateError::IllegalData));
    assert_eq!(
        decompress(b"CK", 0).err(),
        Some(InflateError::InputExhausted)
    );
}

#[test]
fn dynamic_header_truncated_after_an_unaligned_block() {
    let input = stream(|w| {
        w.bits(0, 1);
        w.bits(1, 2);
        w.code(fixed_symbol(256));
        w.bits(0, 1);
        w.bits(2, 2);
    });
    assert_eq!(
        decompress(&input, 0).err(),
        Some(InflateError::InputExhausted)
    );
}

#[test]
fn stored_block_truncation() {
    let inputs: [&[u8]; 3] = [b"CK\x01", b"CK\x01\x05\x00", b"CK\x01\x05\x00\xfa\xffhel"];
    for input in inputs {
        assert_eq!(
            decompress(input, 5).err(),
            Some(InflateError::InputExhausted)
        );
    }
}

#[test]
fn stored_length_past_the_window() {
    assert_eq!(
        decompress(b"CK\x01\x01\x80\xfe\x7f", 0).err(),
        Some(InflateError::IllegalData)
    );
}

#[test]
fn output_past_the_window() {
    let literal = stream(|w| {
        stored_block(w, false, &[0; 32768]);
        fixed_header(w);
        w.code(fixed_symbol(97));
    });
    assert_eq!(
        decompress(&literal, 0).err(),
        Some(InflateError::IllegalData)
    );

    let copy = stream(|w| {
        stored_block(w, false, &[0; 32767]);
        fixed_header(w);
        w.code(fixed_symbol(257));
        w.code(fixed_distance(0));
    });
    assert_eq!(decompress(&copy, 0).err(), Some(InflateError::IllegalData));
}

#[test]
fn output_shorter_than_requested() {
    let input = stream(|w| stored_block(w, true, b"hello"));
    assert_eq!(
        decompress(&input, 11).err(),
        Some(InflateError::OutputShort)
    );
    assert_eq!(decompress(&input, 2).ok(), Some(b"he".to_vec()));
}

#[test]
fn fixed_block_literals_and_end() {
    let input = stream(|w| {
        fixed_header(w);
        for symbol in [0, 143, 144, 255, 256] {
            w.code(fixed_symbol(symbol));
        }
    });
    assert_eq!(decompress(&input, 4).ok(), Some(vec![0, 143, 144, 255]));
}

#[test]
fn fixed_block_rejects_unassigned_codes() {
    let unassigned = stream(|w| {
        fixed_header(w);
        w.code(fixed_symbol(286));
    });
    assert_eq!(
        decompress(&unassigned, 0).err(),
        Some(InflateError::IllegalData)
    );

    let far_distance = stream(|w| {
        fixed_header(w);
        w.code(fixed_symbol(97));
        w.code(fixed_symbol(257));
        w.code(fixed_distance(30));
        w.bits(0, 16);
    });
    assert_eq!(
        decompress(&far_distance, 0).err(),
        Some(InflateError::IllegalData)
    );

    let before_start = stream(|w| {
        fixed_header(w);
        w.code(fixed_symbol(97));
        w.code(fixed_symbol(257));
        w.code(fixed_distance(1));
    });
    assert_eq!(
        decompress(&before_start, 0).err(),
        Some(InflateError::IllegalData)
    );
}

#[test]
fn dynamic_distance_codes_are_checked_when_used() {
    let mut literal = [0u8; 258];
    literal[97] = 1;
    literal[257] = 1;
    let mut distance = [0u8; 31];
    distance[0] = 1;
    distance[30] = 1;
    let input = dynamic_stream(&literal, &distance, |w, lit, dist| {
        w.code(lit[257]);
        w.code(dist[30]);
    });
    assert_eq!(decompress(&input, 0).err(), Some(InflateError::IllegalData));
}

#[test]
fn over_subscribed_distance_code_fails_when_used() {
    let mut literal = [0u8; 258];
    literal[97] = 1;
    literal[257] = 1;
    let input = dynamic_stream(&literal, &[1, 1, 1], |w, lit, _| {
        w.code(lit[257]);
        w.bits(0, 16);
    });
    assert_eq!(decompress(&input, 0).err(), Some(InflateError::IllegalData));
}

#[test]
fn empty_literal_code_is_rejected() {
    let input = dynamic_stream(&[0; 257], &[0], |_, _, _| {});
    assert_eq!(decompress(&input, 0).err(), Some(InflateError::IllegalData));
}

#[test]
fn empty_code_length_code_is_rejected() {
    let input = stream(|w| dynamic_block(w, 257, 1, &[0; 19], &[]));
    assert_eq!(decompress(&input, 0).err(), Some(InflateError::IllegalData));
}

#[test]
fn repeat_past_the_code_lengths_is_rejected() {
    let mut cl = [0u8; 19];
    cl[0] = 1;
    cl[18] = 1;
    let input = stream(|w| dynamic_block(w, 257, 1, &cl, &[(18, 127, 7), (18, 127, 7)]));
    assert_eq!(decompress(&input, 0).err(), Some(InflateError::IllegalData));
}

#[test]
fn lone_code_of_length_one() {
    let code = Code::new(&[1]);
    assert!(code.is_well_formed());
    assert_eq!(code.decode(&mut Bits::new(&[0])).ok(), Some(0));
    assert_eq!(
        code.decode(&mut Bits::new(&[0xff, 0xff])).err(),
        Some(InflateError::IllegalData)
    );
}

#[test]
fn compressed_chunk_walk_is_checked() {
    assert_eq!(
        parse_header(&compressed(15, &[])).err(),
        Some(HeaderError::BadFileType)
    );
    assert_eq!(
        parse_header(&compressed(21, &[5, 0, 12])).err(),
        Some(HeaderError::BadFileType)
    );
    let mut truncated = chunk(5, &stored(b"hello"));
    truncated.truncate(14);
    assert_eq!(
        parse_header(&compressed(21, &truncated)).err(),
        Some(HeaderError::BadFileType)
    );
    assert_eq!(
        parse_header(&compressed(21, &chunk(6, &stored(b"hello!")))).err(),
        Some(HeaderError::BadFileType)
    );
    assert_eq!(
        parse_header(&compressed(27, &chunk(5, &stored(b"hello")))).err(),
        Some(HeaderError::BadFileType)
    );
    assert_eq!(
        parse_header(&compressed(21, &chunk(0x8001, &stored(b"hello")))).err(),
        Some(HeaderError::BadAlloc)
    );
}

/// Builds a file with the `xof 0302` header, the format fourcc `kind` and the float size `0032`.
fn file(kind: &[u8], body: &[u8]) -> Vec<u8> {
    let mut file = b"xof 0302".to_vec();
    file.extend_from_slice(kind);
    file.extend_from_slice(b"0032");
    file.extend_from_slice(body);
    file
}

fn compressed(declared: u32, chunks: &[u8]) -> Vec<u8> {
    let mut body = declared.to_le_bytes().to_vec();
    body.extend_from_slice(chunks);
    file(b"bzip", &body)
}

/// One chunk of the compressed body: its decompressed size, its compressed size, the data.
fn chunk(decomp: u16, packed: &[u8]) -> Vec<u8> {
    let mut out = decomp.to_le_bytes().to_vec();
    out.extend_from_slice(&(packed.len() as u16).to_le_bytes());
    out.extend_from_slice(packed);
    out
}

/// Packs bits as a deflate stream: fields and extra bits least significant bit first, Huffman
/// codes most significant bit first.
#[derive(Default)]
struct Writer {
    bits: Vec<bool>,
}

impl Writer {
    fn bits(&mut self, value: u32, count: u8) {
        for i in 0..count {
            self.bits.push((value >> i) & 1 == 1);
        }
    }

    fn code(&mut self, (code, length): (u32, u8)) {
        for i in (0..length).rev() {
            self.bits.push((code >> i) & 1 == 1);
        }
    }

    fn align(&mut self) {
        while !self.bits.len().is_multiple_of(8) {
            self.bits.push(false);
        }
    }

    fn bytes(&mut self, data: &[u8]) {
        for &byte in data {
            self.bits(u32::from(byte), 8);
        }
    }

    fn finish(self) -> Vec<u8> {
        self.bits
            .chunks(8)
            .map(|bits| {
                bits.iter()
                    .enumerate()
                    .fold(0, |byte, (i, &bit)| byte | (u8::from(bit) << i))
            })
            .collect()
    }
}

/// A complete MSZIP chunk: the `CK` signature, then the blocks that `build` writes.
fn stream(build: impl FnOnce(&mut Writer)) -> Vec<u8> {
    let mut writer = Writer::default();
    writer.bytes(b"CK");
    build(&mut writer);
    writer.finish()
}

fn stored(data: &[u8]) -> Vec<u8> {
    stream(|w| stored_block(w, true, data))
}

fn stored_block(writer: &mut Writer, last: bool, data: &[u8]) {
    writer.bits(u32::from(last), 1);
    writer.bits(0, 2);
    writer.align();
    let len = data.len() as u32;
    writer.bits(len, 16);
    writer.bits(!len & 0xffff, 16);
    writer.bytes(data);
}

fn fixed_header(writer: &mut Writer) {
    writer.bits(1, 1);
    writer.bits(1, 2);
}

/// The fixed literal and length code of RFC 1951 3.2.6.
fn fixed_symbol(symbol: usize) -> (u32, u8) {
    match symbol {
        0..=143 => (0x30 + symbol as u32, 8),
        144..=255 => (0x190 + (symbol - 144) as u32, 9),
        256..=279 => ((symbol - 256) as u32, 7),
        _ => (0xc0 + (symbol - 280) as u32, 8),
    }
}

fn fixed_distance(symbol: usize) -> (u32, u8) {
    (symbol as u32, 5)
}

/// Writes a dynamic block header for the code lengths `nl` and `nd`, with the code-length code
/// `cl`, then `symbols` as (code-length symbol, extra value, extra bits).
fn dynamic_block(
    writer: &mut Writer,
    nl: usize,
    nd: usize,
    cl: &[u8; 19],
    symbols: &[(usize, u32, u8)],
) {
    writer.bits(1, 1);
    writer.bits(2, 2);
    writer.bits((nl - 257) as u32, 5);
    writer.bits((nd - 1) as u32, 5);
    writer.bits(15, 4);
    for &symbol in &ORDER {
        writer.bits(u32::from(cl[symbol]), 3);
    }
    let codes = canonical(cl);
    for &(symbol, value, count) in symbols {
        writer.code(codes[symbol]);
        writer.bits(value, count);
    }
}

/// A stream with one dynamic block whose literal and distance code lengths are `literal` and
/// `distance`, each length written as a code-length symbol. `data` then writes codes made from
/// those lengths.
fn dynamic_stream(
    literal: &[u8],
    distance: &[u8],
    data: impl FnOnce(&mut Writer, &[(u32, u8)], &[(u32, u8)]),
) -> Vec<u8> {
    let lengths: Vec<u8> = literal.iter().chain(distance).copied().collect();
    let symbols: Vec<(usize, u32, u8)> = lengths
        .iter()
        .map(|&length| (usize::from(length), 0, 0))
        .collect();
    let mut cl = [0u8; 19];
    cl[0] = 1;
    cl[1] = 1;
    stream(|writer| {
        dynamic_block(writer, literal.len(), distance.len(), &cl, &symbols);
        data(writer, &canonical(literal), &canonical(distance));
    })
}

/// The canonical Huffman codes of RFC 1951 3.2.2, as (code, length) per symbol. A symbol with
/// length zero gets the placeholder (0, 0).
fn canonical(lengths: &[u8]) -> Vec<(u32, u8)> {
    let mut count = [0u32; 16];
    for &length in lengths {
        count[usize::from(length)] += 1;
    }
    count[0] = 0;
    let mut next = [0u32; 16];
    let mut code = 0;
    for (len, slot) in next.iter_mut().enumerate().skip(1) {
        code = (code + count[len - 1]) << 1;
        *slot = code;
    }
    lengths
        .iter()
        .map(|&length| {
            if length == 0 {
                (0, 0)
            } else {
                let slot = &mut next[usize::from(length)];
                *slot += 1;
                (*slot - 1, length)
            }
        })
        .collect()
}
