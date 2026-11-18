//! Rust-only edge cases: the arms the oracle scripts do not reach, the refusals that replace
//! Wine's out-of-bounds reads and writes, and the stream state around them.

use super::oracle_checks::replay;
use crate::format::{AudioSpec, SampleFormat, WaveFormat};
use crate::media::{
    available_type, check_media_type, get_input_type, get_output_type, FormatKind, Major,
    MediaType, Subtype,
};
use crate::resampler::{
    OutputData, Resampler, Status, OUTPUT_SYNCPOINT, OUTPUT_TIME, OUTPUT_TIMELENGTH,
    SET_TYPE_TEST_ONLY,
};
use crate::stub::{self, Stub};
use crate::time::{duration, from_user, rescale, to_user, NO_PTS, TIME_BASE};
use crate::Error;

/// A `WAVEFORMATEX` header with the given fields and a zero `cbSize`.
fn wfx(tag: u16, channels: u16, rate: u32, block_align: u16, bits: u16) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&tag.to_le_bytes());
    bytes.extend_from_slice(&channels.to_le_bytes());
    bytes.extend_from_slice(&rate.to_le_bytes());
    bytes.extend_from_slice(&(rate * u32::from(block_align)).to_le_bytes());
    bytes.extend_from_slice(&block_align.to_le_bytes());
    bytes.extend_from_slice(&bits.to_le_bytes());
    bytes.extend_from_slice(&0u16.to_le_bytes());
    bytes
}

/// A `WAVEFORMATEXTENSIBLE` of 2 channels at 16 bits with the given mask and `SubFormat` tag.
fn extensible(channel_mask: u32, sub_tag: u32) -> Vec<u8> {
    let mut bytes = wfx(0xfffe, 2, 48000, 4, 16);
    bytes[16..18].copy_from_slice(&22u16.to_le_bytes());
    bytes.extend_from_slice(&16u16.to_le_bytes());
    bytes.extend_from_slice(&channel_mask.to_le_bytes());
    bytes.extend_from_slice(&sub_tag.to_le_bytes());
    bytes.extend_from_slice(&[0, 0, 0x10, 0, 0x80, 0, 0, 0xaa, 0, 0x38, 0x9b, 0x71]);
    bytes
}

/// An audio `WAVEFORMATEX` media type with PCM as its subtype.
fn media(format: Vec<u8>) -> MediaType {
    MediaType {
        major: Major::Audio,
        format_kind: FormatKind::WaveFormatEx,
        subtype: Subtype::Pcm,
        format,
    }
}

/// 2 channels of 16-bit PCM at `rate`, with a 4-byte block align.
fn pcm16(rate: u32) -> MediaType {
    media(wfx(1, 2, rate, 4, 16))
}

/// A resampler with both types set, from `input` to `output`.
fn configured(input: &MediaType, output: &MediaType) -> Resampler<Stub> {
    stub::reset();
    let mut resampler = Resampler::<Stub>::default();
    resampler
        .set_input_type(0, Some(input), 0)
        .expect("input type");
    resampler
        .set_output_type(0, Some(output), 0)
        .expect("output type");
    resampler
}

#[test]
fn sample_format_maps_tag_and_depth() {
    use crate::format::sample_format;
    assert_eq!(sample_format(1, 8), Some(SampleFormat::U8));
    assert_eq!(sample_format(1, 16), Some(SampleFormat::S16));
    assert_eq!(sample_format(1, 32), Some(SampleFormat::S32));
    assert_eq!(sample_format(3, 32), Some(SampleFormat::Flt));
    assert_eq!(sample_format(3, 64), Some(SampleFormat::Dbl));
    assert_eq!(sample_format(3, 16), None);
    assert_eq!(sample_format(2, 16), None);
}

#[test]
fn sample_format_byte_widths() {
    let widths = [
        SampleFormat::U8,
        SampleFormat::S16,
        SampleFormat::S32,
        SampleFormat::Flt,
        SampleFormat::Dbl,
    ]
    .map(SampleFormat::bytes);
    assert_eq!(widths, [1, 2, 4, 4, 8]);
}

#[test]
fn wave_format_reads_the_header_fields() {
    let parsed = WaveFormat::parse(&wfx(1, 2, 48000, 4, 16));
    assert_eq!(
        parsed,
        Some(WaveFormat {
            tag: 1,
            channels: 2,
            samples_per_sec: 48000,
            avg_bytes_per_sec: 192000,
            block_align: 4,
            bits_per_sample: 16,
        })
    );
    assert_eq!(WaveFormat::parse(&[0; 17]), None);
}

#[test]
fn audio_spec_follows_wave_format_and_extensible_layouts() {
    let stereo16 = AudioSpec {
        format: SampleFormat::S16,
        channels: 2,
        sample_rate: 48000,
        block_align: 4,
    };
    assert_eq!(
        AudioSpec::from_format(&wfx(1, 2, 48000, 4, 16)),
        Some(stereo16)
    );
    assert_eq!(AudioSpec::from_format(&extensible(0x3, 1)), Some(stereo16));
    assert_eq!(AudioSpec::from_format(&extensible(0x0, 1)), None);
    assert_eq!(AudioSpec::from_format(&extensible(0x3, 1)[..38]), None);
    assert_eq!(AudioSpec::from_format(&wfx(1, 2, 48000, 0, 16)), None);
    assert_eq!(AudioSpec::from_format(&wfx(3, 2, 48000, 4, 16)), None);
    assert_eq!(AudioSpec::from_format(&wfx(1, 2, 48000, 4, 16)[..17]), None);
}

#[test]
fn frame_bytes_and_wrap_count_whole_frames() {
    let spec = AudioSpec {
        format: SampleFormat::S16,
        channels: 2,
        sample_rate: 48000,
        block_align: 4,
    };
    assert_eq!(spec.frame_bytes(3), 12);
    assert_eq!(spec.wrap(8), Some((2, 8)));
    assert_eq!(spec.wrap(10), Some((2, 8)));
    assert_eq!(spec.wrap(0), Some((0, 0)));
    let wide = AudioSpec {
        block_align: 2,
        ..spec
    };
    assert_eq!(wide.wrap(8), None);
    assert_eq!(wide.wrap(16), None);
}

#[test]
fn check_media_type_accepts_pcm_and_float_and_refuses_the_rest() {
    assert_eq!(check_media_type(&pcm16(48000)), Ok(4));
    assert_eq!(check_media_type(&media(extensible(0x3, 1))), Ok(4));
    let float = MediaType {
        subtype: Subtype::Float,
        ..pcm16(48000)
    };
    assert_eq!(check_media_type(&float), Ok(4));

    let refused = |media: MediaType| check_media_type(&media);
    let mut video = pcm16(48000);
    video.major = Major::Other;
    assert_eq!(refused(video), Err(Error::InvalidType));
    let mut other_format = pcm16(48000);
    other_format.format_kind = FormatKind::Other;
    assert_eq!(refused(other_format), Err(Error::InvalidType));
    let mut other_subtype = pcm16(48000);
    other_subtype.subtype = Subtype::Other;
    assert_eq!(refused(other_subtype), Err(Error::InvalidType));
    assert_eq!(
        refused(media(extensible(0x3, 1)[..38].to_vec())),
        Err(Error::InvalidType)
    );
    assert_eq!(refused(media(vec![0; 17])), Err(Error::InvalidType));
    assert_eq!(
        refused(media(wfx(1, 2, 48000, 4, 0))),
        Err(Error::InvalidType)
    );
    let mut no_avg = wfx(1, 2, 48000, 4, 16);
    no_avg[8..12].fill(0);
    assert_eq!(refused(media(no_avg)), Err(Error::InvalidType));
    assert_eq!(
        refused(media(wfx(1, 0, 48000, 4, 16))),
        Err(Error::InvalidType)
    );
    assert_eq!(refused(media(wfx(1, 2, 0, 4, 16))), Err(Error::InvalidType));
    assert_eq!(
        refused(media(wfx(1, 2, 48000, 0, 16))),
        Err(Error::InvalidType)
    );
}

#[test]
fn available_types_alternate_float_and_pcm() {
    let float_tag = available_type(0);
    assert_eq!(float_tag.subtype, Subtype::Float);
    assert_eq!(float_tag.format, wfx(3, 0, 0, 0, 0));
    assert_eq!(available_type(1).subtype, Subtype::Pcm);
    assert_eq!(available_type(1).format, wfx(1, 0, 0, 0, 0));
    assert_eq!(available_type(2).format, wfx(3, 2, 48000, 8, 32));
    assert_eq!(available_type(3).format, wfx(1, 2, 48000, 4, 16));
    assert_eq!(check_media_type(&available_type(2)), Ok(8));
}

#[test]
fn stream_and_index_limits() {
    assert_eq!(get_input_type(1, 0), Err(Error::InvalidStreamIndex));
    assert_eq!(get_input_type(0, 2), Err(Error::NoMoreItems));
    assert_eq!(
        get_input_type(0, 1).map(|media| media.subtype),
        Ok(Subtype::Pcm)
    );
    assert_eq!(get_output_type(1, 0), Err(Error::InvalidStreamIndex));
    assert_eq!(get_output_type(0, 4), Err(Error::NoMoreItems));
    assert_eq!(
        get_output_type(0, 3).map(|media| media.subtype),
        Ok(Subtype::Pcm)
    );
}

#[test]
fn rescale_passes_sentinels_truncates_and_refuses_bad_divisors() {
    assert_eq!(rescale(NO_PTS, 3, 2), NO_PTS);
    assert_eq!(rescale(i64::MAX, 3, 2), i64::MAX);
    assert_eq!(rescale(5, 1, 0), NO_PTS);
    assert_eq!(rescale(5, -1, 1), NO_PTS);
    assert_eq!(rescale(i64::MAX - 1, 2, 1), NO_PTS);
    assert_eq!(rescale(-7, 1, 2), -3);
    assert_eq!(rescale(7, 1, 2), 3);
}

#[test]
fn time_conversions_round_as_wine() {
    assert_eq!(to_user(96000, 0), 0);
    assert_eq!(from_user(96000, 0), 0);
    assert_eq!(duration(4, 24000), 1667);
    assert_eq!(duration(2, 24000), 833);
    assert_eq!(duration(3, 1), 3 * TIME_BASE);
}

#[test]
fn error_and_status_codes_match_wine() {
    let codes = [
        (Error::InvalidStreamIndex, 0x8004_0201),
        (Error::InvalidType, 0x8004_0202),
        (Error::TypeNotSet, 0x8004_0203),
        (Error::NotAccepting, 0x8004_0204),
        (Error::NoMoreItems, 0x8004_0206),
        (Error::Fail, 0x8000_4005),
        (Error::InvalidArg, 0x8007_0057),
    ];
    for (error, code) in codes {
        assert_eq!(error.hresult(), code);
    }
    assert_eq!(Status::Ready.hresult(), 0);
    assert_eq!(Status::NeedInput.hresult(), 1);
}

#[test]
fn index_zero_is_the_only_stream() {
    let mut resampler = Resampler::<Stub>::default();
    assert_eq!(
        resampler.set_input_type(1, None, 0),
        Err(Error::InvalidStreamIndex)
    );
    assert_eq!(
        resampler.set_output_type(1, None, 0),
        Err(Error::InvalidStreamIndex)
    );
}

#[test]
fn invalid_media_is_refused_and_not_stored() {
    let mut resampler = Resampler::<Stub>::default();
    let mut video = pcm16(48000);
    video.major = Major::Other;
    assert_eq!(
        resampler.set_input_type(0, Some(&video), 0),
        Err(Error::InvalidType)
    );
    resampler
        .set_input_type(0, Some(&pcm16(48000)), 0)
        .expect("input type");
    assert_eq!(
        resampler.set_output_type(0, Some(&video), 0),
        Err(Error::InvalidType)
    );
    let mut output = OutputData::default();
    assert_eq!(
        resampler.process_output(1, &mut [0; 8], &mut output),
        Err(Error::TypeNotSet)
    );
}

#[test]
fn test_only_types_are_checked_but_not_stored() {
    let mut resampler = Resampler::<Stub>::default();
    assert_eq!(
        resampler.set_input_type(0, Some(&pcm16(48000)), SET_TYPE_TEST_ONLY),
        Ok(())
    );
    assert_eq!(
        resampler.set_output_type(0, Some(&pcm16(24000)), 0),
        Err(Error::TypeNotSet)
    );
    resampler
        .set_input_type(0, Some(&pcm16(48000)), 0)
        .expect("input type");
    assert_eq!(
        resampler.set_output_type(0, Some(&pcm16(24000)), SET_TYPE_TEST_ONLY),
        Ok(())
    );
    assert_eq!(resampler.process_input(&[0; 8], 0), Err(Error::TypeNotSet));
}

#[test]
fn clearing_a_type_drops_the_stream() {
    let mut resampler = configured(&pcm16(48000), &pcm16(24000));
    assert_eq!(resampler.set_input_type(0, None, 0), Ok(()));
    assert_eq!(resampler.process_input(&[0; 8], 0), Err(Error::TypeNotSet));

    let mut resampler = configured(&pcm16(48000), &pcm16(24000));
    assert_eq!(resampler.set_output_type(0, None, 0), Ok(()));
    assert_eq!(resampler.process_input(&[0; 8], 0), Err(Error::TypeNotSet));
}

#[test]
fn input_that_cannot_build_a_stream_is_cleared() {
    let mut resampler = configured(&pcm16(48000), &pcm16(24000));
    let input_24 = media(wfx(1, 2, 48000, 6, 24));
    assert_eq!(
        resampler.set_input_type(0, Some(&input_24), 0),
        Err(Error::Fail)
    );
    assert_eq!(resampler.process_input(&[0; 8], 0), Err(Error::TypeNotSet));
    assert_eq!(resampler.set_input_type(0, Some(&pcm16(48000)), 0), Ok(()));
    assert_eq!(resampler.process_input(&[0; 8], 0), Ok(()));
}

#[test]
fn output_that_cannot_build_a_stream_is_cleared() {
    let mut resampler = Resampler::<Stub>::default();
    resampler
        .set_input_type(0, Some(&pcm16(48000)), 0)
        .expect("input type");
    let output_24 = media(wfx(1, 2, 24000, 6, 24));
    assert_eq!(
        resampler.set_output_type(0, Some(&output_24), 0),
        Err(Error::Fail)
    );
    assert_eq!(resampler.set_output_type(0, Some(&pcm16(24000)), 0), Ok(()));
    assert_eq!(resampler.process_input(&[0; 8], 0), Ok(()));
}

#[test]
fn converter_refusal_leaves_no_stream() {
    stub::reset();
    stub::set_refuse(true);
    let mut resampler = Resampler::<Stub>::default();
    resampler
        .set_input_type(0, Some(&pcm16(48000)), 0)
        .expect("input type");
    assert_eq!(
        resampler.set_output_type(0, Some(&pcm16(24000)), 0),
        Err(Error::Fail)
    );
    stub::reset();
}

#[test]
fn process_input_needs_a_stream_and_whole_frames() {
    let mut resampler = Resampler::<Stub>::default();
    assert_eq!(resampler.process_input(&[0; 4], 0), Err(Error::TypeNotSet));

    let mut resampler = configured(&pcm16(48000), &pcm16(24000));
    assert_eq!(resampler.process_input(&[0; 8], 0), Ok(()));
    assert_eq!(
        resampler.process_input(&[0; 8], 0),
        Err(Error::NotAccepting)
    );

    let mut resampler = configured(&media(wfx(1, 2, 48000, 2, 16)), &pcm16(24000));
    assert_eq!(resampler.process_input(&[0; 8], 0), Err(Error::Fail));

    let mut resampler = configured(&pcm16(48000), &pcm16(24000));
    assert_eq!(resampler.process_input(&[], 0), Ok(()));
    assert_eq!(resampler.process_input(&[0; 8], 0), Ok(()));
    stub::set(0, 0);
    let mut output = OutputData::default();
    assert_eq!(
        resampler
            .process_output(1, &mut [0; 16], &mut output)
            .map(Status::hresult),
        Ok(1)
    );
}

#[test]
fn process_output_checks_count_stream_and_converter_results() {
    let mut resampler = Resampler::<Stub>::default();
    let mut output = OutputData::default();
    assert_eq!(
        resampler.process_output(1, &mut [0; 16], &mut output),
        Err(Error::TypeNotSet)
    );

    let mut resampler = configured(&pcm16(48000), &pcm16(24000));
    assert_eq!(
        resampler.process_output(2, &mut [0; 16], &mut output),
        Err(Error::InvalidArg)
    );
    assert_eq!(
        resampler.process_output(0, &mut [0; 16], &mut output),
        Err(Error::InvalidArg)
    );

    stub::set(-1, 0);
    assert_eq!(
        resampler.process_output(1, &mut [0; 16], &mut output),
        Err(Error::Fail)
    );
    assert_eq!(output, OutputData::default());

    stub::set(5, 0);
    assert_eq!(
        resampler.process_output(1, &mut [0; 16], &mut output),
        Err(Error::Fail)
    );
    assert_eq!(output.length, 0);
}

#[test]
fn process_output_consumes_the_pending_input() {
    let mut resampler = configured(&pcm16(48000), &pcm16(24000));
    let mut output = OutputData::default();
    stub::set(2, 0);
    assert_eq!(resampler.process_input(&[0; 8], 0), Ok(()));
    assert_eq!(
        resampler.process_output(1, &mut [0; 16], &mut output),
        Ok(Status::Ready)
    );
    assert_eq!(resampler.process_input(&[0; 8], 0), Ok(()));
}

#[test]
fn copy_path_fills_a_buffer_too_small_for_whole_frames() {
    let mut resampler = configured(&pcm16(48000), &media(wfx(1, 2, 24000, 2, 16)));
    stub::set(2, 0);
    let mut output = OutputData::default();
    resampler.process_input(&[0; 8], 0).expect("input");
    assert_eq!(
        resampler.process_output(1, &mut [0; 16], &mut output),
        Ok(Status::Ready)
    );
    assert_eq!(output.length, 8);
    assert_eq!(
        output.status,
        OUTPUT_SYNCPOINT | OUTPUT_TIME | OUTPUT_TIMELENGTH
    );
    assert_eq!(stub::last_call(), (8, 2));

    let mut resampler = configured(&pcm16(48000), &media(wfx(1, 2, 24000, 2, 16)));
    stub::set(4, 0);
    resampler.process_input(&[0; 8], 0).expect("input");
    assert_eq!(
        resampler.process_output(1, &mut [0; 8], &mut output),
        Err(Error::Fail)
    );

    stub::set(-1, 0);
    resampler.process_input(&[0; 8], 0).expect("input");
    assert_eq!(
        resampler.process_output(1, &mut [0; 16], &mut output),
        Err(Error::Fail)
    );
}

#[test]
fn copy_path_clamps_the_request_to_one_rate_of_frames() {
    let mut resampler = configured(&pcm16(48000), &media(wfx(1, 2, 2, 2, 16)));
    stub::set(2, 0);
    let mut output = OutputData::default();
    resampler.process_input(&[0; 8], 0).expect("input");
    assert_eq!(
        resampler.process_output(1, &mut [0; 16], &mut output),
        Ok(Status::Ready)
    );
    assert_eq!(stub::last_call(), (2, 2));
    assert_eq!(output.length, 8);
    assert_eq!(output.timelength, 10_000_000);
}

#[test]
fn drain_without_input_converts_no_frames_in() {
    let mut resampler = configured(&pcm16(48000), &pcm16(24000));
    stub::set(4, 0);
    let mut output = OutputData::default();
    assert_eq!(
        resampler.process_output(1, &mut [0; 16], &mut output),
        Ok(Status::Ready)
    );
    assert_eq!(stub::last_call(), (4, 0));
    assert_eq!(output.length, 16);
    assert_eq!(output.status, OUTPUT_SYNCPOINT | OUTPUT_TIMELENGTH);
    assert_eq!(output.timestamp, NO_PTS);
    assert_eq!(output.timelength, 1667);
}

#[test]
fn replay_matches_the_harness_for_untimed_and_unusual_commands() {
    assert_eq!(
        replay("in 0 0 audio wfx pcm 0100020080bb000000ee0200040010000000;out 0 0 audio wfx pcm 01000200c05d000000770100040010000000;pin -9223372036854775808 0000000000000000;stub 4 0;pout 16"),
        "00000000 ; 00000000 ; 00000000 ; 00000000 16 5 -9223372036854775808 1667"
    );
    assert_eq!(
        replay("tag 1 8;tag 1 16;tag 1 32;tag 3 64;tag 2 8"),
        "0 ; 1 ; 2 ; 4 ; -1"
    );
    assert_eq!(
        replay("check video wfx pcm 0100020080bb000000ee0200040010000000;check audio pcm pcm 0100020080bb000000ee0200040010000000;check audio wfx float 0100020080bb000000ee0200040010000000"),
        "80040202 - ; 80040202 - ; 00000000 4"
    );
    assert_eq!(
        replay("check audio wfx pcm -;pin 0 -"),
        "80040202 - ; 80040203"
    );
}

#[test]
#[should_panic(expected = "unknown oracle command bogus")]
fn replay_rejects_unknown_commands() {
    replay("bogus 1 2");
}
