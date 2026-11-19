//! Replays of the 25 oracle rows in `target/canon/mfplat`. The oracle compiles the verbatim
//! `mediatype.c` and `main.c` ranges with `gcc -fsanitize=address,undefined -g`, with the Win32
//! shims in `wine_mock.h`, and prints one test per row. Each `oracle_oNN` test reproduces row NN,
//! and its expected value is what the C printed for that row. No row trips the sanitizer. The one
//! boundary vector that does, negating `INT_MIN` in `MFGetStrideForBitmapInfoHeader`, is left out.
//! See `edge_checks::bottom_up_stride_of_int_min_wraps`.

use crate::dxva::{convert_color_info_to_dxva, VideoColorInfo};
use crate::frame_rate::{average_time_to_frame_rate, frame_rate_to_average_time};
use crate::guid::{parse_guid, Guid};
use crate::video_format::{format_stride, image_size, plane_size, stride_for_bitmap_info_header};
use crate::{media_subtype_guid, video_format_guid, E_INVALIDARG, MF_E_INVALIDMEDIATYPE};

pub(crate) fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

#[test]
fn oracle_o01() {
    assert_eq!(frame_rate_to_average_time(24000u32, 1001u32), 417188u64);
}

#[test]
fn oracle_o02() {
    assert_eq!(frame_rate_to_average_time(30u32, 1u32), 333333u64);
}

#[test]
fn oracle_o03() {
    assert_eq!(frame_rate_to_average_time(7u32, 3u32), 4285714u64);
}

#[test]
fn oracle_o04() {
    assert_eq!(frame_rate_to_average_time(0u32, 5u32), 0u64);
}

#[test]
fn oracle_o05() {
    assert_eq!(frame_rate_to_average_time(30000u32, 1001u32), 333667u64);
}

#[test]
fn oracle_o06() {
    assert_eq!(average_time_to_frame_rate(417188u64), (24000u32, 1001u32));
}

#[test]
fn oracle_o07() {
    assert_eq!(average_time_to_frame_rate(333640u64), (30000u32, 1001u32));
}

#[test]
fn oracle_o08() {
    assert_eq!(average_time_to_frame_rate(0u64), (0u32, 0u32));
}

#[test]
fn oracle_o09() {
    assert_eq!(average_time_to_frame_rate(200000000u64), (1u32, 10u32));
}

#[test]
fn oracle_o10() {
    assert_eq!(
        average_time_to_frame_rate(3333333u64),
        (10000000u32, 3333333u32)
    );
}

#[test]
fn oracle_o11() {
    assert_eq!(stride_for_bitmap_info_header(0x14u32, 3u32), Ok(-12i32));
}

#[test]
fn oracle_o12() {
    assert_eq!(
        stride_for_bitmap_info_header(0x0u32, 4u32),
        Err(MF_E_INVALIDMEDIATYPE)
    );
}

#[test]
fn oracle_o13() {
    assert_eq!(
        format_stride(&video_format_guid(0x00000016u32), 7u32),
        Some((28u32, false))
    );
}

#[test]
fn oracle_o14() {
    assert_eq!(format_stride(&video_format_guid(0x58585858u32), 4u32), None);
}

#[test]
fn oracle_o15() {
    assert_eq!(
        image_size(&video_format_guid(0x3231564eu32), 5u32, 4u32),
        Ok(36u32)
    );
}

#[test]
fn oracle_o16() {
    assert_eq!(
        image_size(&video_format_guid(0x3131564eu32), 5u32, 2u32),
        Ok(24u32)
    );
}

#[test]
fn oracle_o17() {
    assert_eq!(
        image_size(&video_format_guid(0x00000051u32), 5u32, 3u32),
        Ok(30u32)
    );
}

#[test]
fn oracle_o18() {
    assert_eq!(
        image_size(&media_subtype_guid(0xe436eb7du32), 3u32, 2u32),
        Ok(24u32)
    );
}

#[test]
fn oracle_o19() {
    assert_eq!(
        image_size(&video_format_guid(0x30323449u32), 7u32, 0u32),
        Err(E_INVALIDARG)
    );
}

#[test]
fn oracle_o20() {
    assert_eq!(plane_size(0x3231564eu32, 5u32, 4u32), 30u32);
}

#[test]
fn oracle_o21() {
    assert_eq!(plane_size(0x31434d49u32, 5u32, 4u32), 48u32);
}

#[test]
fn oracle_o22() {
    assert_eq!(
        convert_color_info_to_dxva(&VideoColorInfo {
            interlace_mode: 7,
            source_chroma_subsampling: 1,
            nominal_range: 2,
            transfer_matrix: 3,
            source_lighting: 4,
            color_primaries: 5,
            transfer_function: 6
        }),
        0x3151a103u32
    );
}

#[test]
fn oracle_o23() {
    assert_eq!(
        convert_color_info_to_dxva(&VideoColorInfo {
            interlace_mode: 263,
            source_chroma_subsampling: 1,
            nominal_range: 2,
            transfer_matrix: 3,
            source_lighting: 4,
            color_primaries: 5,
            transfer_function: 6
        }),
        0x3151a107u32
    );
}

#[test]
fn oracle_o24() {
    assert_eq!(
        parse_guid(&wide("12345678-ABCD-ef01-2345-6789abcdef01")),
        Some(Guid {
            data1: 0x12345678,
            data2: 0xabcd,
            data3: 0xef01,
            data4: [0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01]
        })
    );
}

#[test]
fn oracle_o25() {
    assert_eq!(
        parse_guid(&wide("12345678_ABCD-EF01-2345-6789ABCDEF01")),
        None
    );
}
