//! Tests against Wine's oracle vectors, then Rust-only edge cases.
//!
//! The vectors are the replies of Wine's verbatim `imaadp32.c`, built with ASan
//! and UBSan by `target/canon/imaadp32.acm/build.sh`. [`replay`] runs each
//! command through the port and prints the result the way the C harness does,
//! so every vector is compared as the exact text.

use super::*;

#[rustfmt::skip]
const VECTORS: &[(&str, &str)] = &[
    ("index 17 1 8000 4055 256 4 2 505", "0"),
    ("index 1 1 12345 24690 2 16 0 0", "16"),
    ("init 17 1 8000 0 0 4 0 0", "0 17 1 8000 4055 256 4 2 505"),
    ("init 1 1 8000 0 0 16 0 0", "512 1 1 8000 0 0 16 0 0"),
    ("tag 0 0 0", "0 0 1 1 16 16 [PCM]"),
    ("tag 2 0 0", "0 1 17 1 20 8 [IMA ADPCM]"),
    ("tag 1 0 17", "0 1 17 1 20 8 [IMA ADPCM]"),
    ("tag 1 0 85", "512 0 85 0 0 0 []"),
    ("details 0 1 0 0 0 0 0 0 0 0 0", "0 1 1 1 8000 8000 1 8 0 0"),
    ("details 1 0 0 1 1 8000 8000 1 8 0 0", "0 1 1 1 8000 8000 1 8 0 0"),
    ("details 0 17 1 0 0 0 0 0 0 0 0", "0 1 17 2 8000 8110 512 4 2 505"),
    ("suggest 0 30 30 1 1 8000 8000 1 8 0 0 0 0 0 0 0 0 0 0", "0 17 1 8000 4055 256 4 2 505"),
    ("suggest 0 30 30 17 1 8000 4055 256 4 2 505 0 0 0 0 0 0 0 0", "0 1 1 8000 16000 2 16 0 0"),
    ("suggest 0 16 30 17 1 8000 4055 256 4 2 505 0 0 0 0 0 0 0 0", "512 1 1 8000 0 0 16 0 0"),
    ("size 1 1 1 8000 8000 1 8 0 0 17 1 8000 4055 256 4 2 505 0 512", "0 1010 512"),
    ("size 0 1 1 8000 8000 1 8 0 0 17 1 8000 4055 256 4 2 505 4000 0", "0 4000 2048"),
    ("size 1 17 1 8000 4055 256 4 2 505 1 1 8000 16000 2 16 0 0 0 4040", "0 1024 4040"),
    ("open 0 1 1 8000 16000 2 16 0 0 17 1 8000 4055 256 4 2 505", "0 cvtMM16imaK 0 0"),
    ("open 0 17 1 8000 4055 256 4 2 505 1 1 8000 16000 2 16 0 0", "0 cvtMMimaK 0 0"),
    ("open 0 1 2 8000 32000 4 16 0 0 17 2 8000 8110 512 4 2 505", "0 cvtSS16imaK 0 0"),
    ("open 0 17 2 8000 8110 512 4 2 505 1 2 8000 32000 4 16 0 0", "0 cvtSSima16K 0 0"),
    ("convert 0 17 1 8000 0 6 4 2 5 1 1 8000 16000 2 16 0 0 6 10 0 0 341205009a3c", "0 6 10 0 0 34122d122a121f122912"),
    ("convert 0 17 1 8000 0 6 4 2 5 1 1 8000 8000 1 8 0 0 6 5 0 0 341205009a3c", "0 6 5 0 0 9292929292"),
    ("convert 0 17 2 8000 0 16 4 2 9 1 2 8000 32000 4 16 0 0 16 36 0 0 00010300ff7f0a005de2178f46b16ac9", "0 16 36 0 0 0001ff7ff300ff7f0501ff7f1101ff7ff300e07f3201cb7f4d01fc7fd200e97fc100b17f"),
    ("convert 0 1 1 8000 16000 2 16 0 0 17 1 8000 0 6 4 2 5 10 6 0 0 0000e803d007d08aff7f", "0 10 6 32 0 00000000777f"),
];

#[rustfmt::skip]
const EDGE: &[(&str, &str)] = &[
    ("index 17 1 8192 0 256 4 2 0", "8"),
    ("index 2 1 8000 0 0 4 2 0", "4294967295"),
    ("index 1 1 12345 24691 2 16 0 0", "4294967295"),
    ("index 1 1 12345 24690 2 8 0 0", "4294967295"),
    ("index 1 2 12345 49380 4 16 0 0", "16"),
    ("index 1 3 12345 0 6 16 0 0", "4294967295"),
    ("index 1 1 12345 24690 3 16 0 0", "4294967295"),
    ("index 17 1 12345 0 256 4 0 0", "4294967295"),
    ("index 17 1 12345 0 256 8 2 0", "4294967295"),
    ("index 17 0 8000 0 256 4 2 0", "4294967295"),
    ("index 85 1 8000 0 0 0 0 0", "4294967295"),
    ("init 17 1 12345 0 300 4 2 0", "0 17 1 12345 6245 300 4 2 593"),
    ("init 17 2 12345 0 300 4 2 0", "0 17 2 12345 12639 300 4 2 293"),
    ("init 17 1 8000 0 0 4 2 0", "0 17 1 8000 4055 256 4 2 505"),
    ("init 17 2 8000 0 0 4 2 0", "0 17 2 8000 8110 512 4 2 505"),
    ("init 17 1 22050 0 0 4 2 0", "0 17 1 22050 11100 512 4 2 1017"),
    ("init 17 1 44100 0 0 4 2 0", "0 17 1 44100 22125 1024 4 2 2041"),
    ("init 17 1 11025 0 0 4 2 0", "0 17 1 11025 5588 256 4 2 505"),
    ("tag 0 2 0", "512 2 0 0 0 0 []"),
    ("tag 2 0 1", "0 0 1 1 16 16 [PCM]"),
    ("tag 2 0 3", "512 0 3 0 0 0 []"),
    ("tag 1 0 80", "512 0 80 0 0 0 []"),
    ("tag 1 0 2", "512 0 2 0 0 0 []"),
    ("tag 3 0 0", "8 0 0 0 0 0 []"),
    ("tag 0 1 0", "0 1 17 1 20 8 [IMA ADPCM]"),
    ("tag 0 1 17", "0 1 17 1 20 8 [IMA ADPCM]"),
    ("details 0 2 0 0 0 0 0 0 0 0 0", "11 0 2 0 0 0 0 0 0 0"),
    ("details 0 65537 0 0 0 0 0 0 0 0 0", "11 0 1 0 0 0 0 0 0 0"),
    ("details 0 1 16 0 0 0 0 0 0 0 0", "512 0 1 0 0 0 0 0 0 0"),
    ("details 0 17 8 0 0 0 0 0 0 0 0", "512 0 17 0 0 0 0 0 0 0"),
    ("details 0 17 1 0 0 0 0 0 0 0 0", "0 1 17 2 8000 8110 512 4 2 505"),
    ("details 1 0 0 1 1 12345 0 1 16 0 0", "512 0 1 1 12345 0 1 16 0 0"),
    ("details 1 17 0 1 1 8000 4055 256 4 2 505", "512 0 1 1 8000 4055 256 4 2 505"),
    ("details 5 0 0 0 0 0 0 0 0 0 0", "8 0 0 0 0 0 0 0 0 0"),
    ("suggest 0 15 30 1 1 8000 8000 1 8 0 0 0 0 0 0 0 0 0 0", "512 0 0 0 0 0 0 0 0"),
    ("suggest 0 30 15 1 1 8000 8000 1 8 0 0 0 0 0 0 0 0 0 0", "512 0 0 0 0 0 0 0 0"),
    ("suggest 0 30 30 1 1 8000 8000 1 8 0 0 1 1 8000 16000 2 16 0 0", "512 1 1 8000 16000 2 16 0 0"),
    ("suggest 0 30 30 2 1 8000 8000 1 8 0 0 0 0 0 0 0 0 0 0", "512 0 0 0 0 0 0 0 0"),
    ("suggest 0 30 16 1 1 8000 8000 1 8 0 0 0 0 0 0 0 0 0 0", "512 17 1 8000 0 0 4 0 0"),
    ("suggest 65536 30 30 1 1 8000 8000 1 8 0 0 0 2 22050 0 0 0 0 0", "512 0 1 8000 0 0 4 0 0"),
    ("suggest 131072 30 30 1 1 8000 8000 1 8 0 0 0 1 22050 0 0 0 0 0", "0 17 1 8000 4055 256 4 2 505"),
    ("suggest 262144 30 30 1 1 8000 8000 1 8 0 0 0 1 8000 0 0 8 0 0", "0 17 1 8000 4055 256 4 2 505"),
    ("suggest 262144 30 30 17 1 12345 0 300 4 2 0 0 0 0 0 0 8 0 0", "0 1 1 0 0 2 16 0 0"),
    ("suggest 524288 30 30 1 1 8000 8000 1 8 0 0 2 0 0 0 0 0 0 0", "512 17 1 8000 0 0 0 0 0"),
    ("suggest 524288 30 30 17 1 8000 4055 256 4 2 505 17 1 8000 4055 256 4 2 505", "512 17 1 8000 4055 256 4 2 505"),
    ("suggest 786432 30 30 17 1 8000 4055 256 4 2 505 0 1 12345 0 0 8 0 0", "512 1 1 12345 12345 1 8 0 0"),
    ("size 2 1 1 8000 8000 1 8 0 0 17 1 8000 4055 256 4 2 505 0 0", "8 0 0"),
    ("size 1 1 1 8000 8000 1 8 0 0 1 1 8000 16000 2 16 0 0 0 0", "8 0 0"),
    ("size 0 1 1 8000 8000 1 8 0 0 1 1 8000 16000 2 16 0 0 4000 0", "8 4000 0"),
    ("size 1 17 1 8000 4055 256 4 2 505 1 1 8000 16000 2 16 0 0 0 0", "512 0 0"),
    ("size 1 17 1 8000 4055 256 4 2 505 1 1 8000 16000 2 16 0 0 0 1000", "512 0 1000"),
    ("size 1 1 1 8000 8000 1 8 0 0 17 1 8000 4055 256 4 2 505 0 100", "512 0 100"),
    ("size 0 1 1 8000 8000 1 8 0 0 17 1 8000 4055 256 4 2 505 504 0", "512 504 0"),
    ("size 0 1 1 8000 8000 1 8 0 0 17 1 8000 4055 256 4 2 505 505 0", "0 505 256"),
    ("size 0 17 1 8000 4055 256 4 2 505 1 1 8000 16000 2 16 0 0 0 0", "512 0 0"),
    ("size 1 17 1 8000 4055 256 4 2 505 1 1 8000 16000 2 16 0 0 0 1010", "0 256 1010"),
    ("size 0 17 1 8000 4055 256 4 2 505 1 1 8000 16000 2 16 0 0 1000 0", "0 1000 4040"),
    ("size 0 1 2 8000 32000 4 16 0 0 17 2 8000 8110 512 4 2 505 4000 0", "0 4000 1024"),
    ("open 0 17 1 8000 4055 256 4 2 505 1 2 8000 16000 2 8 0 0", "8"),
    ("open 0 1 1 8000 8000 1 8 0 0 17 1 8000 4055 256 4 2 505", "8"),
    ("open 0 1 1 8000 16000 2 16 0 0 1 1 8000 16000 2 16 0 0", "8"),
    ("open 0 17 1 8000 4055 256 4 2 505 17 1 8000 4055 256 4 2 505", "8"),
    ("open 0 17 1 8000 4055 256 4 2 505 1 1 11025 22050 2 16 0 0", "8"),
    ("open 0 1 1 8000 16000 2 16 0 0 17 1 11025 4055 256 4 2 505", "8"),
    ("open 0 17 1 8000 0 256 4 2 0 1 1 8000 16000 2 16 0 0", "8"),
    ("open 0 17 1 8000 0 16 4 2 9 1 1 8000 16000 2 16 0 0", "8"),
    ("open 0 85 1 8000 0 256 4 2 505 1 1 8000 16000 2 16 0 0", "512"),
    ("open 0 17 1 8000 0 256 8 2 505 1 1 8000 16000 2 16 0 0", "512"),
    ("open 0 17 2 8000 0 512 4 2 505 1 1 8000 16000 2 16 0 0", "8"),
    ("open 0 17 1 8000 4055 256 4 2 505 1 2 8000 16000 2 16 0 0", "8"),
    ("open 0 1 1 8000 16000 2 16 0 0 1 1 8000 16000 2 16 0 0", "8"),
    ("open 0 1 1 8000 8000 1 8 0 0 17 1 8000 4055 256 4 2 505", "8"),
    ("open 0 1 1 8000 16000 2 16 0 0 1 1 11025 22050 2 16 0 0", "8"),
    ("open 0 1 1 8000 4055 256 4 2 505 1 1 8000 16000 2 16 0 0", "512"),
    ("open 0 17 1 8000 0 4 4 2 1 1 1 8000 16000 2 16 0 0", "0 cvtMMimaK 0 0"),
    ("open 0 1 1 8000 16000 2 16 0 0 17 1 8000 0 4 4 2 1", "0 cvtMM16imaK 0 0"),
    ("open 0 1 1 8000 16000 2 16 0 0 17 2 8000 0 8 4 2 1", "8"),
    ("open 0 1 2 8000 32000 4 16 0 0 17 2 8000 0 16 4 2 9", "0 cvtSS16imaK 0 0"),
    ("open 0 1 1 8000 16000 2 16 0 0 17 1 8000 0 4 4 2 5", "0 cvtMM16imaK 0 0"),
    ("open 0 17 1 8000 0 6 4 2 5 1 1 8000 16000 2 16 0 0", "0 cvtMMimaK 0 0"),
    ("convert 0 17 1 8000 0 6 4 2 5 1 1 8000 16000 2 16 0 0 6 10 40 40 341205009a3c", "0 6 10 40 40 34122d122a121f122912"),
    ("convert 0 17 1 8000 0 6 4 2 5 1 1 8000 16000 2 16 0 0 12 20 0 0 341205009a3c00800a0077f1", "0 12 20 0 0 34122d122a121f122912008022806e808f800080"),
    ("convert 0 17 1 8000 0 6 4 2 5 1 1 8000 16000 2 16 0 0 8 20 0 0 341205009a3c0080", "0 6 10 0 0 34122d122a121f12291200000000000000000000"),
    ("convert 0 17 1 8000 0 6 4 2 5 1 1 8000 16000 2 16 0 0 6 10 0 0 000000000000", "0 6 10 0 0 00000000000000000000"),
    ("convert 0 17 1 8000 0 6 4 2 5 1 1 8000 16000 2 16 0 0 6 0 0 0 341205009a3c", "0 0 0 0 0 -"),
    ("convert 0 17 1 8000 0 6 4 2 5 1 1 8000 16000 2 16 0 0 0 10 0 0 -", "0 0 0 0 0 00000000000000000000"),
    ("convert 0 17 2 8000 0 16 4 2 9 1 2 8000 32000 4 16 0 0 16 36 0 0 0001ff00ff7fff005de2178f46b16ac9", "0 16 36 0 0 0001ff7f0080ff7ffd2fff7ffb7fff7fdcc22a1aff7f0ad8ff7f5574c8a55744c99569c1"),
    ("convert 0 17 1 8000 0 8 4 2 9 1 1 8000 16000 2 16 0 0 8 18 0 0 0070580077777777", "0 8 18 0 0 0070ff7fff7fff7fff7fff7fff7fff7fff7f"),
    ("convert 0 17 1 8000 0 8 4 2 9 1 1 8000 8000 1 8 0 0 8 9 0 0 00805800ffffffff", "0 8 9 0 0 000000000000000000"),
    ("convert 0 17 1 8000 0 8 4 2 9 1 1 8000 16000 2 16 0 0 8 18 0 0 00805800ffffffff", "0 8 18 0 0 008000800080008000800080008000800080"),
    ("convert 0 17 1 8000 0 4 4 2 5 1 1 8000 16000 2 16 0 0 10 20 0 0 1000020055ab137e8001", "0 8 20 0 0 10001c002d001e00140055ab5aab55ab62ab66ab"),
    ("convert 0 17 1 8000 0 4 4 2 1 1 1 8000 16000 2 16 0 0 4 2 0 0 0c000000", "0 4 2 0 0 0c00"),
    ("convert 0 17 2 8000 0 8 4 2 1 1 2 8000 32000 4 16 0 0 8 4 0 0 0c00feff01000080", "0 8 4 0 0 0c000100"),
    ("convert 0 1 1 8000 16000 2 16 0 0 17 1 8000 0 6 4 2 5 20 12 0 0 0000e803d007d08aff7f0080ffff39300700d8dc", "0 20 12 64 0 00000000777f008020007777"),
    ("convert 0 1 1 8000 16000 2 16 0 0 17 1 8000 0 6 4 2 5 10 6 88 0 0000e803d007d08aff7f", "0 10 6 88 0 00005800807c"),
    ("convert 0 1 1 8000 16000 2 16 0 0 17 1 8000 0 6 4 2 5 10 6 0 0 0000e803d007d08aff7f", "0 10 6 32 0 00000000777f"),
    ("convert 0 1 1 8000 16000 2 16 0 0 17 1 8000 0 6 4 2 5 10 0 5 9 0000e803d007d08aff7f", "0 0 0 5 9 -"),
    ("convert 0 1 1 8000 16000 2 16 0 0 17 1 8000 0 6 4 2 5 0 6 5 9 -", "0 0 0 5 9 000000000000"),
    ("convert 0 1 1 8000 16000 2 16 0 0 17 1 8000 0 6 4 2 5 20 12 50 7 0000e803d007d08aff7f0080ffff39300700d8dc", "0 20 12 81 7 00003200347f0080430077a1"),
    ("convert 16 1 1 8000 16000 2 16 0 0 17 1 8000 0 6 4 2 5 20 12 50 7 0000e803d007d08aff7f0080ffff39300700d8dc", "0 20 12 64 0 00000000777f008020007777"),
    ("convert 16 1 1 8000 16000 2 16 0 0 17 1 8000 0 6 4 2 5 20 12 0 0 0000e803d007d08aff7f0080ffff39300700d8dc", "0 20 12 64 0 00000000777f008020007777"),
    ("convert 0 1 2 8000 32000 4 16 0 0 17 2 8000 0 16 4 2 9 36 16 88 88 00000000e80318fcd007b80bd08a3075ff7f0080393060a42c010700fbff28230100ffff", "0 36 16 84 84 0000580000005800807c9a8008f43091"),
    ("convert 0 1 2 8000 32000 4 16 0 0 17 2 8000 0 16 4 2 9 36 16 0 0 00000000e80318fcd007b80bd08a3075ff7f0080393060a42c010700fbff28230100ffff", "0 36 16 37 49 0000000000000000777f978b7ff74fb7"),
    ("convert 0 1 1 8000 16000 2 16 0 0 17 1 8000 0 8 4 2 9 18 8 0 0 ff7f0080ff7f0080ff7f0080ff7f0080ff7f", "0 18 8 28 0 ff7f00002f3f3f3f"),
    ("convert 0 1 1 8000 16000 2 16 0 0 17 1 8000 0 4 4 2 1 2 4 0 0 ffff", "0 2 4 0 0 ffff0000"),
    ("convert 0 1 2 8000 32000 4 16 0 0 17 2 8000 0 8 4 2 1 4 8 0 0 e80318fc", "0 4 8 0 0 e803000018fc0000"),
    ("convert 0 17 1 8000 0 6 4 2 5 1 1 8000 16000 2 16 0 0 6 10 40 40 000000000000", "0 6 10 40 40 00000000000000000000"),
    ("convert 0 17 1 8000 0 6 4 2 5 1 1 8000 16000 2 16 0 0 0 0 40 40 -", "0 0 0 40 40 -"),
];

// Rejected by the sanitizers, so not in the tables. Each is a C-side
// undefined behaviour that the port turns into an error:
//   init 17 2 12345 0 7 4 2 0
//     sanitizer runtime error: division by zero
//   open 2 17 1 8000 4055 256 4 2 505 1 1 8000 16000 2 16 0 0
//     exit -6 (assert)
//   size 1 1 1 8000 8000 1 8 0 0 17 1 8000 0 0 4 2 505 0 512
//     sanitizer runtime error: division by zero
//   convert 0 17 1 8000 0 4 4 2 5 1 1 8000 16000 2 16 0 0 8 20 0 0 1000020055ab137e
//     sanitizer AddressSanitizer: heap-buffer-overflow on address 0x7b38407e0018 at pc 0x556ed25a5007 bp 0
//   convert 0 1 1 8000 16000 2 16 0 0 17 1 8000 0 4 4 2 5 20 8 0 0 0000010002000300040005000600070008000900
//     sanitizer AddressSanitizer: heap-buffer-overflow on address 0x7b3d14de0018 at pc 0x55ff808dda8d bp 0

/// One numeric word of a command, as the C harness reads it.
fn number(word: Option<&str>) -> u64 {
    word.expect("missing argument").parse().expect("number")
}

/// Eight words in harness order: `wFormatTag`, `nChannels`, `nSamplesPerSec`,
/// `nAvgBytesPerSec`, `nBlockAlign`, `wBitsPerSample`, `cbSize`,
/// `wSamplesPerBlock`.
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
        samples_per_block: next() as u16,
    }
}

fn words(text: &str) -> WaveFormat {
    wfx(&mut text.split_whitespace())
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
        format.samples_per_block
    )
}

/// The bytes of a hex string, two digits per byte.
fn from_hex(text: &str) -> Vec<u8> {
    (0..text.len())
        .step_by(2)
        .map(|at| u8::from_str_radix(&text[at..at + 2], 16).expect("hex"))
        .collect()
}

/// The hex of `bytes`, or `-` when there are none, as the C harness prints.
fn hex_of(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return "-".to_string();
    }
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// The name of the convert function that `ADPCM_StreamOpen` picks.
fn converter_name(stream: &open::Stream) -> &'static str {
    match (stream.direction, stream.src.channels == 2) {
        (open::Direction::Decode, true) => "cvtSSima16K",
        (open::Direction::Decode, false) => "cvtMMimaK",
        (open::Direction::Encode, true) => "cvtSS16imaK",
        (open::Direction::Encode, false) => "cvtMM16imaK",
    }
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
        "init" => {
            let mut format = wfx(&mut words);
            let rc = match format::init_ima_adpcm(&mut format) {
                Ok(()) => 0,
                Err(error) => error.code(),
            };
            format!("{rc} {}", wfx_text(&format))
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
            let rc = match tag::format_tag_details(&mut details, query) {
                Ok(()) => 0,
                Err(error) => error.code(),
            };
            format!(
                "{rc} {} {} {} {} {} [{}]",
                details.format_tag_index,
                details.format_tag,
                details.support,
                details.format_size,
                details.standard_formats,
                details.name
            )
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
            format!("{rc} {} {}", details.support, wfx_text(&details.format))
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
            let mut sized = size::StreamSize {
                fdw_size: query,
                cb_src_length: number(words.next()) as u32,
                cb_dst_length: number(words.next()) as u32,
            };
            let rc = match size::stream_size(&src, &dst, &mut sized) {
                Ok(()) => 0,
                Err(error) => error.code(),
            };
            format!("{rc} {} {}", sized.cb_src_length, sized.cb_dst_length)
        }
        "open" => {
            let fdw = number(words.next()) as u32;
            let src = wfx(&mut words);
            let dst = wfx(&mut words);
            match open::stream_open(fdw, &src, &dst) {
                Ok(stream) => format!(
                    "0 {} {} {}",
                    converter_name(&stream),
                    stream.step_index[0],
                    stream.step_index[1]
                ),
                Err(error) => error.code().to_string(),
            }
        }
        "convert" => {
            let fdw = number(words.next()) as u32;
            let src_format = wfx(&mut words);
            let dst_format = wfx(&mut words);
            let cb_src = number(words.next()) as usize;
            let cb_dst = number(words.next()) as usize;
            let step_l = number(words.next()) as u8;
            let step_r = number(words.next()) as u8;
            let text = words.next().expect("source bytes");
            let src = if text == "-" {
                vec![0; cb_src]
            } else {
                from_hex(text)
            };
            let mut stream = open::stream_open(0, &src_format, &dst_format).expect("open");
            stream.step_index = [step_l, step_r];
            let mut dst = vec![0; cb_dst];
            let usage = convert::stream_convert(&mut stream, fdw, &src, &mut dst).expect("convert");
            format!(
                "0 {} {} {} {} {}",
                usage.src_used,
                usage.dst_used,
                stream.step_index[0],
                stream.step_index[1],
                hex_of(&dst)
            )
        }
        other => panic!("unknown oracle command {other}"),
    }
}

#[test]
fn oracle_vectors_match_wine() {
    for &(command, reply) in VECTORS {
        assert_eq!(replay(command), reply, "{command}");
    }
}

#[test]
fn oracle_edge_matches_wine() {
    for &(command, reply) in EDGE {
        assert_eq!(replay(command), reply, "{command}");
    }
}

#[test]
#[should_panic(expected = "unknown oracle command bogus")]
fn unknown_oracle_command_panics() {
    replay("bogus");
}

#[test]
#[should_panic(expected = "asynchronous streams are not supported")]
fn async_open_asserts_as_wine_does() {
    replay("open 2 17 1 8000 4055 256 4 2 505 1 1 8000 16000 2 16 0 0");
}

/// `size` with the given query, returning the filled sizes or the error.
fn sized(
    query: u32,
    src: &WaveFormat,
    dst: &WaveFormat,
    cb_src: u32,
    cb_dst: u32,
) -> Result<size::StreamSize, Error> {
    let mut sized = size::StreamSize {
        fdw_size: query,
        cb_src_length: cb_src,
        cb_dst_length: cb_dst,
    };
    size::stream_size(src, dst, &mut sized).map(|()| sized)
}

#[test]
fn zero_block_align_in_a_size_query_is_not_possible() {
    let pcm = words("1 1 8000 8000 1 8 0 0");
    let pcm_no_block = words("1 1 8000 8000 0 8 0 0");
    let adpcm = words("17 1 8000 4055 256 4 2 505");
    let adpcm_no_block = words("17 1 8000 0 0 4 2 505");
    let not_possible = Err(Error::NotPossible);
    assert_eq!(sized(1, &pcm, &adpcm_no_block, 0, 512), not_possible);
    assert_eq!(sized(1, &adpcm, &pcm_no_block, 0, 512), not_possible);
    assert_eq!(sized(0, &pcm_no_block, &adpcm, 4000, 0), not_possible);
    assert_eq!(sized(0, &adpcm_no_block, &pcm, 4000, 0), not_possible);
}

#[test]
fn size_counts_whole_blocks_and_rejects_no_block() {
    let pcm = words("1 1 8000 8000 1 8 0 0");
    let adpcm = words("17 1 8000 4055 256 4 2 505");
    assert_eq!(sized(1, &pcm, &adpcm, 0, 100), Err(Error::NotPossible));
    let filled = sized(0, &pcm, &adpcm, 4000, 0).expect("size");
    assert_eq!((filled.cb_src_length, filled.cb_dst_length), (4000, 2048));
}

#[test]
fn adpcm_without_block_align_is_rejected_at_open() {
    let adpcm = words("17 1 8000 0 0 4 2 505");
    let pcm = words("1 1 8000 16000 2 16 0 0");
    assert_eq!(
        open::stream_open(0, &adpcm, &pcm).unwrap_err(),
        Error::NotSupported
    );
    assert_eq!(
        open::stream_open(0, &pcm, &adpcm).unwrap_err(),
        Error::NotSupported
    );
}

#[test]
fn stereo_block_must_hold_whole_eight_nibble_groups() {
    let adpcm = words("17 2 8000 0 8 4 2 5");
    let pcm = words("1 2 8000 32000 4 16 0 0");
    assert_eq!(
        open::stream_open(0, &adpcm, &pcm).unwrap_err(),
        Error::NotSupported
    );
    assert_eq!(
        open::stream_open(0, &pcm, &adpcm).unwrap_err(),
        Error::NotSupported
    );
}

#[test]
fn overlapping_blocks_need_the_last_footprint_in_the_adpcm_buffer() {
    let adpcm = words("17 1 8000 0 4 4 2 5");
    let pcm = words("1 1 8000 16000 2 16 0 0");
    let mut decode = open::stream_open(0, &adpcm, &pcm).expect("open");
    let mut out = [0u8; 20];
    assert_eq!(
        convert::stream_convert(&mut decode, 0, &[0u8; 8], &mut out),
        Err(Error::InvalidParam)
    );
    let mut encode = open::stream_open(0, &pcm, &adpcm).expect("open");
    let mut dst = [0u8; 8];
    assert_eq!(
        convert::stream_convert(&mut encode, 0, &[0u8; 20], &mut dst),
        Err(Error::InvalidParam)
    );
}

#[test]
fn start_flag_resets_the_step_before_an_overlap_error() {
    let adpcm = words("17 1 8000 0 4 4 2 5");
    let pcm = words("1 1 8000 16000 2 16 0 0");
    let mut stream = open::stream_open(0, &pcm, &adpcm).expect("open");
    stream.step_index = [5, 9];
    let start = convert::ACM_STREAMCONVERTF_START;
    assert_eq!(
        convert::stream_convert(&mut stream, start, &[0u8; 20], &mut [0u8; 8]),
        Err(Error::InvalidParam)
    );
    assert_eq!(stream.step_index, [0, 0]);
}

#[test]
fn init_without_samples_per_block_sets_fields_and_is_not_possible() {
    let mut format = words("17 2 12345 0 7 4 2 9");
    assert_eq!(format::init_ima_adpcm(&mut format), Err(Error::NotPossible));
    assert_eq!(wfx_text(&format), "17 2 12345 0 7 4 2 0");
}

#[test]
fn unsupported_format_tag_query_leaves_the_details_alone() {
    let mut details = tag::FormatTagDetails {
        format_tag_index: 0,
        format_tag: 1,
        support: 0,
        format_size: 0,
        standard_formats: 0,
        name: "",
    };
    assert_eq!(
        tag::format_tag_details(&mut details, 9),
        Err(Error::NotSupported)
    );
    assert_eq!(details.name, "");
}
