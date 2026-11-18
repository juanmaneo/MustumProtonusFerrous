//! Edge cases of the converter checks that the Wine oracle vectors do not reach.

use super::oracle_checks::{header_bytes, media, mf_format_bytes, mf_image_size, AUDIO};
use super::*;

fn frame(subtype: Guid, width: u32, height: u32, flags: u64) -> VideoFormat {
    VideoFormat {
        width,
        height,
        video_flags: flags,
        subtype,
    }
}

fn init_error(format_type: Guid, bytes: &[u8]) -> Option<HResult> {
    init_video_format(&media(MEDIASUBTYPE_RGB32, format_type, bytes)).err()
}

fn check_format(format_type: Guid, bytes: &[u8]) -> Result<u32, HResult> {
    check_dmo_media_type(
        ConverterKind::Converter.output_types(),
        &media(MEDIASUBTYPE_YUY2, format_type, bytes),
        mf_image_size,
    )
}

#[test]
fn fill_rejects_layouts_it_cannot_wrap() {
    assert!(fill(&frame(MEDIASUBTYPE_V216, 3, 2, 0), 0, 4096).is_none());
    assert!(fill(&frame(MEDIASUBTYPE_Y41P, 3, 2, 0), 0, 4096).is_none());
}

#[test]
fn fill_rejects_empty_oversized_and_short_frames() {
    assert!(fill(&frame(MEDIASUBTYPE_RGB8, 0, 2, 0), 0, usize::MAX).is_none());
    assert!(fill(&frame(MEDIASUBTYPE_RGB8, 2, 0, 0), 0, usize::MAX).is_none());
    assert!(fill(&frame(MEDIASUBTYPE_RGB8, u32::MAX, 1, 0), 0, usize::MAX).is_none());
    assert!(fill(
        &frame(MEDIASUBTYPE_RGB8, 1, i32::MAX as u32, 0),
        0,
        usize::MAX
    )
    .is_none());
    assert!(fill(&frame(MEDIASUBTYPE_RGB32, 40000, 40000, 0), 0, usize::MAX).is_none());
    assert!(fill(&frame(MEDIASUBTYPE_I420, 5, 3, 0), 0, 26).is_none());
}

#[test]
fn fill_pitch_equal_to_line_length_is_not_rescaled() {
    let plain = fill(&frame(MEDIASUBTYPE_RGB32, 3, 2, 0), 12, 24).expect("fits");
    assert_eq!(plain.size, 24);
    assert_eq!(plain.planes, [Some(0), None, None, None]);
    assert_eq!(plain.strides, [12, 0, 0, 0]);

    let bottom_up = fill(
        &frame(MEDIASUBTYPE_RGB32, 3, 2, BOTTOM_UP_LINEAR_REP),
        -12,
        24,
    )
    .expect("fits");
    assert_eq!(bottom_up.planes, [Some(12), None, None, None]);
    assert_eq!(bottom_up.strides, [-12, 0, 0, 0]);
}

#[test]
fn fill_bottom_up_planar_moves_only_the_first_plane() {
    let filled = fill(&frame(MEDIASUBTYPE_NV12, 5, 4, BOTTOM_UP_LINEAR_REP), 0, 32).expect("fits");
    assert_eq!(filled.size, 32);
    assert_eq!(filled.planes, [Some(15), Some(20), None, None]);
    assert_eq!(filled.strides, [-5, 6, 0, 0]);
}

#[test]
fn fill_pitched_planar_offsets_follow_the_scaled_strides() {
    let filled = fill(&frame(MEDIASUBTYPE_I420, 5, 3, 0), 8, 27).expect("fits");
    assert_eq!(filled.size, 27);
    assert_eq!(filled.planes, [Some(0), Some(24), Some(32), None]);
    assert_eq!(filled.strides, [8, 4, 4, 0]);
}

#[test]
fn fill_rejects_pitch_whose_scaled_stride_does_not_fit() {
    assert!(fill(&frame(MEDIASUBTYPE_RGB32, 3, 2, 0), i32::MIN, 24).is_none());
}

#[test]
fn alignment_check_rejects_each_misaligned_part() {
    let planes = [Some(0), Some(16), None, None];
    let strides = [-8, 4, 0, 0];
    assert!(check_arrays_alignment(&planes, &strides, 32, 4));
    assert!(!check_arrays_alignment(&planes, &strides, 32, 0));
    assert!(!check_arrays_alignment(&planes, &strides, 30, 4));
    assert!(!check_arrays_alignment(
        &[Some(2), None, None, None],
        &strides,
        32,
        4
    ));
    assert!(!check_arrays_alignment(&planes, &[-8, 3, 0, 0], 32, 4));
}

#[test]
fn init_reads_video_info2_and_bottom_up_flag() {
    let flat = header_bytes(72, 4, -2, 0, 0);
    let top_down =
        init_video_format(&media(MEDIASUBTYPE_RGB32, FORMAT_VIDEO_INFO2, &flat)).expect("header");
    assert_eq!(
        (top_down.width, top_down.height, top_down.video_flags),
        (4, 2, 0)
    );

    let bottom_up = header_bytes(72, 4, 2, 0, 0);
    let format = init_video_format(&media(MEDIASUBTYPE_RGB32, FORMAT_VIDEO_INFO2, &bottom_up))
        .expect("header");
    assert_eq!(format.video_flags, BOTTOM_UP_LINEAR_REP);
}

#[test]
fn init_uses_compression_to_choose_stride_direction() {
    let fourcc = header_bytes(48, 4, 2, 0x3259_5559, 0);
    let format =
        init_video_format(&media(MEDIASUBTYPE_YUY2, FORMAT_VIDEO_INFO, &fourcc)).expect("header");
    assert_eq!(format.video_flags, 0);

    let bitfields = header_bytes(48, 4, 2, 3, 0);
    let format = init_video_format(&media(MEDIASUBTYPE_RGB32, FORMAT_VIDEO_INFO, &bitfields))
        .expect("header");
    assert_eq!(format.video_flags, BOTTOM_UP_LINEAR_REP);
}

#[test]
fn init_rejects_short_formats_and_unknown_format_types() {
    let video_info = header_bytes(48, 4, 2, 0, 0);
    let mf = mf_format_bytes(4, 2, 0, &MEDIASUBTYPE_RGB32);
    assert_eq!(
        init_error(FORMAT_VIDEO_INFO, &video_info[..87]),
        Some(E_INVALIDARG)
    );
    assert_eq!(
        init_error(FORMAT_VIDEO_INFO2, &video_info[..111]),
        Some(E_INVALIDARG)
    );
    assert_eq!(
        init_error(FORMAT_MF_VIDEO_FORMAT, &mf[..167]),
        Some(E_INVALIDARG)
    );
    assert_eq!(init_error(MEDIATYPE_VIDEO, &video_info), Some(E_NOTIMPL));
}

#[test]
fn media_type_check_rejects_wrong_type_and_subtype() {
    let bytes = header_bytes(48, 4, 2, 0, 16);
    let audio = DmoMediaType {
        major_type: AUDIO,
        subtype: MEDIASUBTYPE_YUY2,
        format_type: FORMAT_VIDEO_INFO,
        format: &bytes,
    };
    let converter = ConverterKind::Converter.output_types();
    assert_eq!(
        check_dmo_media_type(converter, &audio, mf_image_size),
        Err(DMO_E_INVALIDTYPE)
    );
    let not_listed = media(MEDIASUBTYPE_Y41P, FORMAT_VIDEO_INFO, &bytes);
    assert_eq!(
        check_dmo_media_type(converter, &not_listed, mf_image_size),
        Err(DMO_E_INVALIDTYPE)
    );
}

#[test]
fn media_type_check_rejects_bad_video_info_dimensions() {
    assert_eq!(
        check_format(FORMAT_VIDEO_INFO, &header_bytes(48, 0, 2, 0, 16)),
        Err(DMO_E_INVALIDTYPE)
    );
    assert_eq!(
        check_format(FORMAT_VIDEO_INFO, &header_bytes(48, 4, 0, 0, 16)),
        Err(DMO_E_INVALIDTYPE)
    );
    assert_eq!(
        check_format(FORMAT_VIDEO_INFO2, &header_bytes(72, 4, 0, 0, 24)),
        Err(DMO_E_INVALIDTYPE)
    );
    assert_eq!(
        check_format(FORMAT_VIDEO_INFO2, &header_bytes(72, 4, 2, 0, 24)),
        Ok(24)
    );
}

#[test]
fn media_type_check_rejects_short_and_unknown_formats() {
    let video_info = header_bytes(48, 4, 2, 0, 16);
    let video_info2 = header_bytes(72, 4, 2, 0, 16);
    let mf = mf_format_bytes(4, 2, 0, &MEDIASUBTYPE_YUY2);
    assert_eq!(
        check_format(FORMAT_VIDEO_INFO, &video_info[..87]),
        Err(DMO_E_INVALIDTYPE)
    );
    assert_eq!(
        check_format(FORMAT_VIDEO_INFO2, &video_info2[..111]),
        Err(DMO_E_INVALIDTYPE)
    );
    assert_eq!(
        check_format(FORMAT_MF_VIDEO_FORMAT, &mf[..167]),
        Err(DMO_E_INVALIDTYPE)
    );
    assert_eq!(
        check_format(MEDIATYPE_VIDEO, &video_info),
        Err(DMO_E_INVALIDTYPE)
    );
}

#[test]
fn media_type_check_rejects_zero_mf_dimensions_and_propagates_size_errors() {
    assert_eq!(
        check_format(
            FORMAT_MF_VIDEO_FORMAT,
            &mf_format_bytes(0, 2, 0, &MEDIASUBTYPE_YUY2)
        ),
        Err(DMO_E_INVALIDTYPE)
    );
    assert_eq!(
        check_format(
            FORMAT_MF_VIDEO_FORMAT,
            &mf_format_bytes(4, 0, 0, &MEDIASUBTYPE_YUY2)
        ),
        Err(DMO_E_INVALIDTYPE)
    );
    let mf = mf_format_bytes(4, 2, 0, &MEDIASUBTYPE_YUY2);
    let failing = check_dmo_media_type(
        ConverterKind::Converter.output_types(),
        &media(MEDIASUBTYPE_YUY2, FORMAT_MF_VIDEO_FORMAT, &mf),
        |_: &Guid, _: u32, _: u32| Err(E_NOTIMPL),
    );
    assert_eq!(failing, Err(E_NOTIMPL));
}

#[test]
fn output_type_checks_index_and_resizer_subtype() {
    let bytes = header_bytes(48, 4, 2, 0, 16);
    let yuy2 = media(MEDIASUBTYPE_YUY2, FORMAT_VIDEO_INFO, &bytes);
    assert_eq!(
        check_output_type(
            ConverterKind::Converter,
            1,
            &MEDIASUBTYPE_YUY2,
            &yuy2,
            mf_image_size
        ),
        Err(DMO_E_INVALIDSTREAMINDEX)
    );
    assert_eq!(
        check_output_type(
            ConverterKind::Resizer,
            0,
            &MEDIASUBTYPE_RGB32,
            &yuy2,
            mf_image_size
        ),
        Err(DMO_E_INVALIDTYPE)
    );
    assert_eq!(
        check_output_type(
            ConverterKind::Converter,
            0,
            &MEDIASUBTYPE_RGB32,
            &yuy2,
            mf_image_size
        ),
        Ok(16)
    );
}

#[test]
fn available_subtypes_follow_the_list_bounds() {
    let converter = ConverterKind::Converter.input_types();
    assert!(available_subtype(converter, 0, 0).ok() == Some(MEDIASUBTYPE_YV12));
    assert_eq!(
        available_subtype(converter, 1, 0).err(),
        Some(DMO_E_INVALIDSTREAMINDEX)
    );
    assert_eq!(
        available_subtype(converter, 0, converter.len() as u32).err(),
        Some(DMO_E_NO_MORE_ITEMS)
    );
}

#[test]
fn converter_lists_have_the_wine_lengths() {
    assert_eq!(ConverterKind::Converter.input_types().len(), 20);
    assert_eq!(ConverterKind::Converter.output_types().len(), 16);
    assert_eq!(ConverterKind::Resizer.input_types().len(), 12);
    assert_eq!(ConverterKind::Resizer.output_types().len(), 24);
}

#[test]
fn dmo_results_map_to_mf_results() {
    let cases = [
        (DMO_E_INVALIDSTREAMINDEX, MF_E_INVALIDSTREAMNUMBER),
        (DMO_E_INVALIDTYPE, MF_E_INVALIDMEDIATYPE),
        (DMO_E_TYPE_NOT_SET, MF_E_TRANSFORM_TYPE_NOT_SET),
        (DMO_E_NOTACCEPTING, MF_E_NOTACCEPTING),
        (DMO_E_TYPE_NOT_ACCEPTED, MF_E_INVALIDMEDIATYPE),
        (DMO_E_NO_MORE_ITEMS, MF_E_NO_MORE_TYPES),
        (S_FALSE, MF_E_TRANSFORM_NEED_MORE_INPUT),
        (E_NOTIMPL, E_NOTIMPL),
    ];
    for (dmo, mf) in cases {
        assert_eq!(mf_result_from_dmo(dmo), mf);
    }
}

#[test]
fn mf_rgb_subtypes_map_to_dmo_subtypes() {
    let cases = [
        (41, MEDIASUBTYPE_RGB8),
        (24, MEDIASUBTYPE_RGB555),
        (23, MEDIASUBTYPE_RGB565),
        (20, MEDIASUBTYPE_RGB24),
        (22, MEDIASUBTYPE_RGB32),
    ];
    for (mf, dmo) in cases {
        assert!(get_dmo_subtype(&mf_subtype(mf)) == dmo);
    }
}
