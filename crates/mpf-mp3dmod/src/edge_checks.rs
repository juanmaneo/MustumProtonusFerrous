//! Edge tests: the extremes of the `u16` and `u32` fields, each error branch of every entry point
//! in its check order, and each operand of the compound checks. The extreme values were also run
//! through the Wine-verbatim harness in `target/canon/mp3dmod`, with the same results as asserted
//! here.

use crate::frame::{framesize, frametime, input_size_info, output_size_info, SizeInfo};
use crate::hresult::{self, convert_dmo_to_mf_error};
use crate::mf::{output_available_type, MfAudioType, MfSubtype};
use crate::types::{
    output_type, set_input_type, set_output_type, Encoding, WaveFormat, GUID_NULL, WAVE_FORMAT_PCM,
    WMFORMAT_WAVEFORMATEX, WMMEDIASUBTYPE_MP3, WMMEDIATYPE_AUDIO,
};

fn input_format(channels: u16, samples_per_sec: u32) -> WaveFormat {
    WaveFormat {
        channels,
        samples_per_sec,
        ..WaveFormat::default()
    }
}

fn output_format(
    channels: u16,
    bits_per_sample: u16,
    samples_per_sec: u32,
    block_align: u16,
    avg_bytes_per_sec: u32,
) -> WaveFormat {
    WaveFormat {
        format_tag: WAVE_FORMAT_PCM,
        channels,
        samples_per_sec,
        avg_bytes_per_sec,
        block_align,
        bits_per_sample,
        cb_size: 0,
    }
}

fn mf_type(
    subtype: MfSubtype,
    bits_per_sample: u32,
    channels: u32,
    samples_per_sec: u32,
    avg_bytes_per_sec: u32,
    block_alignment: u32,
) -> MfAudioType {
    MfAudioType {
        subtype,
        bits_per_sample,
        channels,
        samples_per_sec,
        avg_bytes_per_sec,
        block_alignment,
        all_samples_independent: true,
    }
}

#[test]
fn frame_size_and_time_at_the_extremes() {
    let widest = WaveFormat {
        block_align: u16::MAX,
        samples_per_sec: 44100,
        ..WaveFormat::default()
    };
    assert_eq!(framesize(&widest), 1152 * 65535);
    assert_eq!(frametime(&input_format(2, 8000)), Some(1_440_000));
    assert_eq!(frametime(&input_format(2, u32::MAX)), Some(2));
    assert_eq!(frametime(&input_format(2, 0)), None);
}

#[test]
fn output_type_wraps_at_the_extremes() {
    // 65535 channels give a 131070-byte block, which is stored in a u16 as 65534.
    let wide = output_type(0, Some(&input_format(u16::MAX, 44100)), 0).unwrap();
    assert_eq!(
        (wide.channels, wide.block_align, wide.avg_bytes_per_sec),
        (65535, 65534, 2_890_049_400)
    );
    // The average rate wraps modulo 2^32: 2^30 * 4 = 2^32.
    let fast = output_type(0, Some(&input_format(2, 1 << 30)), 0).unwrap();
    assert_eq!(fast.avg_bytes_per_sec, 0);
    assert_eq!(
        output_type(0, Some(&input_format(1, 8000)), 1),
        Ok(output_format(1, 8, 8000, 1, 8000))
    );
    assert_eq!(
        output_type(0, Some(&input_format(1, 8000)), 2),
        Err(hresult::DMO_E_NO_MORE_ITEMS)
    );
    assert_eq!(
        output_type(0, Some(&input_format(0, 44100)), 0),
        Err(hresult::DMO_E_NO_MORE_ITEMS)
    );
    assert_eq!(
        output_type(1, Some(&input_format(2, 44100)), 0),
        Err(hresult::DMO_E_INVALIDSTREAMINDEX)
    );
    assert_eq!(output_type(0, None, 0), Err(hresult::DMO_E_TYPE_NOT_SET));
}

#[test]
fn input_type_checks_stream_then_guids_then_channels() {
    let stereo = input_format(2, 44100);
    let rejected: Result<(), u32> = Err(hresult::DMO_E_TYPE_NOT_ACCEPTED);
    assert_eq!(
        set_input_type(
            1,
            WMMEDIATYPE_AUDIO,
            WMMEDIASUBTYPE_MP3,
            WMFORMAT_WAVEFORMATEX,
            &stereo
        ),
        Err(hresult::DMO_E_INVALIDSTREAMINDEX)
    );
    assert_eq!(
        set_input_type(
            0,
            GUID_NULL,
            WMMEDIASUBTYPE_MP3,
            WMFORMAT_WAVEFORMATEX,
            &stereo
        ),
        rejected
    );
    assert_eq!(
        set_input_type(
            0,
            WMMEDIATYPE_AUDIO,
            GUID_NULL,
            WMFORMAT_WAVEFORMATEX,
            &stereo
        ),
        rejected
    );
    assert_eq!(
        set_input_type(0, WMMEDIATYPE_AUDIO, WMMEDIASUBTYPE_MP3, GUID_NULL, &stereo),
        rejected
    );
    assert_eq!(
        set_input_type(
            0,
            WMMEDIATYPE_AUDIO,
            WMMEDIASUBTYPE_MP3,
            WMFORMAT_WAVEFORMATEX,
            &input_format(0, 44100)
        ),
        rejected
    );
    assert_eq!(
        set_input_type(
            0,
            WMMEDIATYPE_AUDIO,
            WMMEDIASUBTYPE_MP3,
            WMFORMAT_WAVEFORMATEX,
            &stereo
        ),
        Ok(())
    );
}

#[test]
fn output_type_checks_stream_then_input_then_format_then_block_then_rate() {
    let stereo = input_format(2, 44100);
    let pcm16 = output_format(2, 16, 44100, 4, 176400);
    assert_eq!(
        set_output_type(1, Some(&stereo), WMFORMAT_WAVEFORMATEX, &pcm16),
        Err(hresult::DMO_E_INVALIDSTREAMINDEX)
    );
    assert_eq!(
        set_output_type(0, None, WMFORMAT_WAVEFORMATEX, &pcm16),
        Err(hresult::DMO_E_TYPE_NOT_SET)
    );
    assert_eq!(
        set_output_type(0, Some(&stereo), GUID_NULL, &pcm16),
        Err(hresult::DMO_E_TYPE_NOT_ACCEPTED)
    );
    assert_eq!(
        set_output_type(
            0,
            Some(&stereo),
            WMFORMAT_WAVEFORMATEX,
            &output_format(2, 12, 44100, 3, 132300)
        ),
        Err(hresult::DMO_E_TYPE_NOT_ACCEPTED)
    );
    assert_eq!(
        set_output_type(
            0,
            Some(&stereo),
            WMFORMAT_WAVEFORMATEX,
            &output_format(2, 16, 44100, 3, 132300)
        ),
        Err(hresult::E_INVALIDARG)
    );
    assert_eq!(
        set_output_type(
            0,
            Some(&stereo),
            WMFORMAT_WAVEFORMATEX,
            &output_format(2, 16, 44100, 4, 176401)
        ),
        Err(hresult::E_INVALIDARG)
    );
    assert_eq!(
        set_output_type(
            0,
            Some(&stereo),
            WMFORMAT_WAVEFORMATEX,
            &output_format(2, 16, 48000, 4, 192000)
        ),
        Err(hresult::DMO_E_TYPE_NOT_ACCEPTED)
    );
    assert_eq!(
        set_output_type(0, Some(&stereo), WMFORMAT_WAVEFORMATEX, &pcm16),
        Ok(Encoding::Signed16)
    );
}

#[test]
fn output_rate_is_the_input_rate_or_a_half_or_quarter_of_it() {
    assert_eq!(
        set_output_type(
            0,
            Some(&input_format(2, 44100)),
            WMFORMAT_WAVEFORMATEX,
            &output_format(2, 16, 22050, 4, 88200)
        ),
        Ok(Encoding::Signed16)
    );
    assert_eq!(
        set_output_type(
            0,
            Some(&input_format(2, 44100)),
            WMFORMAT_WAVEFORMATEX,
            &output_format(2, 16, 11025, 4, 44100)
        ),
        Ok(Encoding::Signed16)
    );
    // 2^30 + 1 times 4 wraps to 4, which is the input rate, so the quarter-rate test accepts it.
    assert_eq!(
        set_output_type(
            0,
            Some(&input_format(2, 4)),
            WMFORMAT_WAVEFORMATEX,
            &output_format(2, 16, 1_073_741_825, 4, 4)
        ),
        Ok(Encoding::Signed16)
    );
}

#[test]
fn mf_output_types_for_each_index_and_channel_count() {
    let stereo = input_format(2, 48000);
    assert_eq!(
        output_available_type(0, Some(&stereo), 3),
        Ok(mf_type(MfSubtype::Float, 32, 1, 48000, 192000, 4))
    );
    assert_eq!(
        output_available_type(0, Some(&stereo), 4),
        Ok(mf_type(MfSubtype::Pcm, 16, 1, 48000, 96000, 2))
    );
    assert_eq!(
        output_available_type(0, Some(&stereo), 5),
        Ok(mf_type(MfSubtype::Pcm, 8, 1, 48000, 48000, 1))
    );
    assert_eq!(
        output_available_type(0, Some(&input_format(1, 8000)), 2),
        Ok(mf_type(MfSubtype::Pcm, 8, 1, 8000, 8000, 1))
    );
    // 2^31 times the 8-byte block wraps to 0.
    assert_eq!(
        output_available_type(0, Some(&input_format(2, 1 << 31)), 0),
        Ok(mf_type(MfSubtype::Float, 32, 2, 1 << 31, 0, 8))
    );
    assert_eq!(
        output_available_type(1, Some(&stereo), 0),
        Err(hresult::MF_E_INVALIDSTREAMNUMBER)
    );
    assert_eq!(
        output_available_type(0, None, 0),
        Err(hresult::MF_E_TRANSFORM_TYPE_NOT_SET)
    );
    assert_eq!(
        output_available_type(0, Some(&stereo), 6),
        Err(hresult::MF_E_NO_MORE_TYPES)
    );
    assert_eq!(
        output_available_type(0, Some(&input_format(1, 8000)), 3),
        Err(hresult::MF_E_NO_MORE_TYPES)
    );
}

#[test]
fn size_queries_need_both_types_and_the_first_stream() {
    let stereo = input_format(2, 44100);
    let pcm16 = output_format(2, 16, 44100, 4, 176400);
    assert_eq!(
        input_size_info(1, Some(&stereo), Some(&pcm16)),
        Err(hresult::DMO_E_INVALIDSTREAMINDEX)
    );
    assert_eq!(
        input_size_info(0, None, Some(&pcm16)),
        Err(hresult::DMO_E_TYPE_NOT_SET)
    );
    assert_eq!(
        input_size_info(0, Some(&stereo), None),
        Err(hresult::DMO_E_TYPE_NOT_SET)
    );
    assert_eq!(
        input_size_info(0, Some(&stereo), Some(&pcm16)),
        Ok(SizeInfo {
            size: 0,
            alignment: 1
        })
    );
    assert_eq!(
        output_size_info(1, Some(&stereo), Some(&pcm16)),
        Err(hresult::DMO_E_INVALIDSTREAMINDEX)
    );
    assert_eq!(
        output_size_info(0, None, Some(&pcm16)),
        Err(hresult::DMO_E_TYPE_NOT_SET)
    );
    assert_eq!(
        output_size_info(0, Some(&stereo), None),
        Err(hresult::DMO_E_TYPE_NOT_SET)
    );
    // 2 * 1152 * 65535 / 8
    let widest = output_format(2, u16::MAX, 44100, 0, 0);
    assert_eq!(
        output_size_info(0, Some(&stereo), Some(&widest)),
        Ok(SizeInfo {
            size: 18_874_080,
            alignment: 1
        })
    );
}

#[test]
fn dmo_errors_map_to_mf_errors_and_other_codes_pass_through() {
    assert_eq!(
        convert_dmo_to_mf_error(hresult::DMO_E_INVALIDTYPE),
        hresult::MF_E_INVALIDMEDIATYPE
    );
    assert_eq!(
        convert_dmo_to_mf_error(hresult::DMO_E_TYPE_NOT_ACCEPTED),
        hresult::MF_E_INVALIDMEDIATYPE
    );
    assert_eq!(
        convert_dmo_to_mf_error(hresult::DMO_E_NOTACCEPTING),
        hresult::MF_E_NOTACCEPTING
    );
    assert_eq!(
        convert_dmo_to_mf_error(hresult::DMO_E_NO_MORE_ITEMS),
        hresult::MF_E_NO_MORE_TYPES
    );
    assert_eq!(convert_dmo_to_mf_error(hresult::E_FAIL), hresult::E_FAIL);
    assert_eq!(convert_dmo_to_mf_error(0), 0);
}

#[test]
fn derived_traits_print_every_variant() {
    assert_eq!(format!("{:?}", Encoding::Unsigned8), "Unsigned8");
    assert_eq!(format!("{:?}", Encoding::Signed16), "Signed16");
    assert_eq!(format!("{:?}", Encoding::Float32), "Float32");
    assert_eq!(format!("{:?}", MfSubtype::Pcm), "Pcm");
    assert_eq!(format!("{:?}", MfSubtype::Float), "Float");
    assert_eq!(
        format!(
            "{:?}",
            SizeInfo {
                size: 0,
                alignment: 1
            }
        ),
        "SizeInfo { size: 0, alignment: 1 }"
    );
    assert!(format!("{:?}", WMMEDIATYPE_AUDIO).starts_with("Guid {"));
    assert!(format!("{:?}", input_format(2, 44100)).starts_with("WaveFormat {"));
    assert!(
        format!("{:?}", mf_type(MfSubtype::Pcm, 8, 1, 8000, 8000, 1)).starts_with("MfAudioType {")
    );
}
