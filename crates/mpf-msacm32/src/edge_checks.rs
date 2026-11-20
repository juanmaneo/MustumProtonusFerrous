//! Hand-derived checks for the paths the oracle rows do not reach: the refusals that
//! Wine asserts on (the deviations listed in `lib.rs`), the sample helpers at their
//! limits, and the derived trait impls.

use std::fmt::Debug;

use crate::convert::{stream_open, Usage, STREAMOPEN_ASYNC};
use crate::details::{
    driver_details, driver_open_accepts, format_details, FormatDetails, FormatTagDetails,
    OpenDescription, FCC_TYPE_AUDIOCODEC, FORMAT_INDEX,
};
use crate::format::{format_index, WaveFormat, PCM_FORMATS};
use crate::sample::{mix, read, wider, write, Depth};
use crate::size::{stream_size, StreamSize, SIZE_DESTINATION, SIZE_SOURCE};
use crate::suggest::Suggest;
use crate::{Error, WAVE_FORMAT_PCM};

fn pcm(channels: u16, rate: u32, block_align: u16, bits: u16) -> WaveFormat {
    WaveFormat {
        format_tag: WAVE_FORMAT_PCM,
        channels,
        samples_per_sec: rate,
        avg_bytes_per_sec: rate * u32::from(block_align),
        block_align,
        bits_per_sample: bits,
    }
}

fn debug(value: &impl Debug) -> String {
    format!("{value:?}")
}

#[test]
fn error_codes_match_wine() {
    assert_eq!(Error::InvalidParam.code(), 11);
    assert_eq!(Error::NotSupported.code(), 8);
    assert_eq!(Error::NotPossible.code(), 512);
}

#[test]
fn format_table_and_index() {
    assert_eq!(PCM_FORMATS[0], (1, 8, 8000));
    assert_eq!(PCM_FORMATS[5], (2, 24, 8000));
    assert_eq!(PCM_FORMATS[12], (1, 8, 22050));
    assert_eq!(PCM_FORMATS[35], (2, 24, 96000));
    assert_eq!(format_index(&pcm(1, 8000, 1, 8)), Some(0));
    assert_eq!(format_index(&pcm(2, 96000, 6, 24)), Some(35));
    assert_eq!(format_index(&pcm(3, 8000, 3, 8)), None);
    assert_eq!(format_index(&pcm(1, 8000, 1, 12)), None);
    assert_eq!(format_index(&pcm(1, 12345, 1, 8)), None);
}

#[test]
fn format_details_index_bounds() {
    let mut d = FormatDetails {
        format_index: 36,
        ..Default::default()
    };
    let mut fmt = pcm(2, 8000, 2, 8);
    let (d_before, fmt_before) = (d, fmt);
    assert_eq!(
        format_details(FORMAT_INDEX, &mut d, &mut fmt),
        Err(Error::InvalidParam)
    );
    assert_eq!((d, fmt), (d_before, fmt_before));

    d.format_index = 0;
    assert_eq!(format_details(FORMAT_INDEX, &mut d, &mut fmt), Ok(()));
    assert_eq!(
        fmt,
        WaveFormat {
            format_tag: WAVE_FORMAT_PCM,
            channels: 1,
            samples_per_sec: 8000,
            avg_bytes_per_sec: 8000,
            block_align: 1,
            bits_per_sample: 8,
        }
    );
    assert_eq!((d.format_tag, d.support, d.cb_wfx), (1, 2, 16));
}

#[test]
fn driver_open_accepts_only_codec_open() {
    let codec = OpenDescription {
        fcc_type: FCC_TYPE_AUDIOCODEC,
        fcc_comp: 0,
    };
    assert!(driver_open_accepts(None));
    assert!(driver_open_accepts(Some(&codec)));
    assert!(!driver_open_accepts(Some(&OpenDescription {
        fcc_type: 0,
        fcc_comp: 0,
    })));
    assert!(!driver_open_accepts(Some(&OpenDescription {
        fcc_type: FCC_TYPE_AUDIOCODEC,
        fcc_comp: 1,
    })));
}

#[test]
fn stream_size_refuses_zero_divisors() {
    let mono8 = pcm(1, 8000, 1, 8);
    let silent = WaveFormat {
        avg_bytes_per_sec: 0,
        ..mono8
    };
    let mut s = StreamSize {
        fdw_size: SIZE_DESTINATION,
        src_length: 5,
        dst_length: 7,
    };
    let before = s;
    assert_eq!(
        stream_size(&mono8, &silent, &mut s),
        Err(Error::NotPossible)
    );
    assert_eq!(s, before);

    s.fdw_size = SIZE_SOURCE;
    let before = s;
    assert_eq!(
        stream_size(&silent, &mono8, &mut s),
        Err(Error::NotPossible)
    );
    assert_eq!(s, before);
}

#[test]
fn stream_size_saturates() {
    let fast = WaveFormat {
        avg_bytes_per_sec: u32::MAX,
        ..pcm(1, 8000, 1, 8)
    };
    let slow = pcm(1, 1, 1, 8);
    let mut s = StreamSize {
        fdw_size: SIZE_DESTINATION,
        src_length: 0,
        dst_length: u32::MAX,
    };
    assert_eq!(stream_size(&fast, &slow, &mut s), Ok(()));
    assert_eq!(s.src_length, u32::MAX);
}

#[test]
fn stream_open_async_is_not_supported() {
    let fmt = pcm(1, 8000, 1, 8);
    assert_eq!(
        stream_open(STREAMOPEN_ASYNC, &fmt, &fmt).err(),
        Some(Error::NotSupported)
    );
}

#[test]
fn stream_open_refuses_blocks_smaller_than_a_frame() {
    let stereo8 = pcm(2, 8000, 1, 8);
    let mono8 = pcm(1, 8000, 1, 8);
    let mono16 = pcm(1, 8000, 1, 16);
    assert_eq!(
        stream_open(0, &stereo8, &mono8).err(),
        Some(Error::NotSupported)
    );
    assert_eq!(
        stream_open(0, &mono8, &stereo8).err(),
        Some(Error::NotSupported)
    );
    assert_eq!(
        stream_open(0, &mono16, &mono8).err(),
        Some(Error::NotSupported)
    );
}

#[test]
fn sample_readers_and_writers() {
    assert_eq!(read(Depth::Bits8, &[0x00]), i32::MIN);
    assert_eq!(read(Depth::Bits8, &[0xff]), 0x7f00_0000);
    assert_eq!(read(Depth::Bits8, &[0x80]), 0);
    assert_eq!(read(Depth::Bits16, &[0x00, 0x80]), i32::MIN);
    assert_eq!(read(Depth::Bits16, &[0xff, 0x7f]), 0x7fff_0000);
    assert_eq!(read(Depth::Bits24, &[0xff, 0xff, 0xff]), -256);
    assert_eq!(read(Depth::Bits24, &[0x00, 0x00, 0x80]), i32::MIN);

    let mut out = [0xaa; 3];
    write(Depth::Bits8, 0x7f00_0000, &mut out);
    assert_eq!(out[0], 0xff);
    write(Depth::Bits8, i32::MIN, &mut out);
    assert_eq!(out[0], 0x00);
    write(Depth::Bits8, 0, &mut out);
    assert_eq!(out[0], 0x80);
    write(Depth::Bits16, i32::MIN, &mut out);
    assert_eq!(out[..2], [0x00, 0x80]);
    write(Depth::Bits24, -256, &mut out);
    assert_eq!(out, [0xff, 0xff, 0xff]);
    write(Depth::Bits24, i32::MIN, &mut out);
    assert_eq!(out, [0x00, 0x00, 0x80]);
}

#[test]
fn sample_mixers_saturate() {
    assert_eq!(mix(Depth::Bits8, 0x7f00_0000, 0x7f00_0000), 0x7f00_0000);
    assert_eq!(mix(Depth::Bits8, i32::MIN, i32::MIN), i32::MIN);
    assert_eq!(mix(Depth::Bits8, 0x4000_0000, 0x4000_0000), 0x7f00_0000);
    assert_eq!(mix(Depth::Bits8, 0x4000_0000, 0), 0x4000_0000);
    assert_eq!(mix(Depth::Bits16, 0x7fff_0000, 0x7fff_0000), 0x7fff_0000);
    assert_eq!(mix(Depth::Bits16, i32::MIN, i32::MIN), i32::MIN);
    assert_eq!(mix(Depth::Bits16, 0x1000_0000, 0x1000_0000), 0x2000_0000);
    assert_eq!(mix(Depth::Bits24, 0x7fff_ff00, 0x7fff_ff00), -512);
    assert_eq!(mix(Depth::Bits24, 0x7fff_ff00, 1), 0x7fff_ff00);
    assert_eq!(mix(Depth::Bits24, -0x7fff_ff00, -1), -0x7fff_ff00);
    assert_eq!(mix(Depth::Bits24, 256, 256), 512);
}

#[test]
fn wider_depth_and_bytes() {
    assert_eq!(wider(Depth::Bits16, Depth::Bits8), Depth::Bits16);
    assert_eq!(wider(Depth::Bits8, Depth::Bits16), Depth::Bits16);
    assert_eq!(wider(Depth::Bits24, Depth::Bits8), Depth::Bits24);
    assert_eq!(wider(Depth::Bits8, Depth::Bits8), Depth::Bits8);
    assert_eq!(
        [
            Depth::Bits8.bytes(),
            Depth::Bits16.bytes(),
            Depth::Bits24.bytes()
        ],
        [1, 2, 3]
    );
}

#[test]
fn derived_impls_are_exercised() {
    let fmt = pcm(1, 8000, 1, 8);
    assert_eq!(Clone::clone(&fmt), fmt);
    assert!(debug(&fmt).contains("8000"));
    assert_eq!(WaveFormat::default().channels, 0);

    assert_eq!(Clone::clone(&Error::NotPossible), Error::NotPossible);
    assert_eq!(debug(&Error::NotPossible), "NotPossible");

    assert_eq!(Clone::clone(&Depth::Bits16), Depth::Bits16);
    assert_eq!(debug(&Depth::Bits24), "Bits24");

    let conv = stream_open(0, &fmt, &fmt).expect("mono 8-bit stream");
    assert_eq!(Clone::clone(&conv), conv);
    assert!(debug(&conv).starts_with("Converter"));

    let usage = conv.convert(&[1], &mut [0]);
    assert_eq!(Clone::clone(&usage), usage);
    assert_eq!(
        usage,
        Usage {
            src_used: 1,
            dst_used: 1
        }
    );
    assert_eq!(debug(&usage), "Usage { src_used: 1, dst_used: 1 }");

    let details = driver_details();
    assert_eq!(Clone::clone(&details), details);
    assert!(debug(&details).contains("MS-PCM"));

    let tag = FormatTagDetails::default();
    assert_eq!(Clone::clone(&tag), tag);
    assert!(debug(&tag).contains("standard_formats"));

    let details = FormatDetails::default();
    assert_eq!(Clone::clone(&details), details);
    assert!(debug(&details).contains("cb_wfx"));

    let open = OpenDescription {
        fcc_type: FCC_TYPE_AUDIOCODEC,
        fcc_comp: 0,
    };
    assert_eq!(Clone::clone(&open), open);
    assert!(debug(&open).contains("fcc_type"));

    let size = StreamSize::default();
    assert_eq!(Clone::clone(&size), size);
    assert!(debug(&size).contains("dst_length"));

    let suggest = Suggest::default();
    assert_eq!(Clone::clone(&suggest), suggest);
    assert!(debug(&suggest).contains("cb_dst"));
}
