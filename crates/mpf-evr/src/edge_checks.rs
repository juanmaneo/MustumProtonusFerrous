//! Edge cases of the ported functions: the HRESULT bit patterns, the float conversions that
//! saturate, the signed overflow that wraps, each rectangle clause on its own, the 16-stream
//! limit, and the delete quirk. The oracle rows cover the common paths.

use super::oracle_checks::{aspect, edges, hresult, native_of, norm, scale, video_position};
use super::*;

#[test]
fn hresult_constants_have_their_win32_bit_patterns() {
    assert_eq!(S_OK, 0);
    assert_eq!(E_POINTER as u32, 0x8000_4003);
    assert_eq!(E_INVALIDARG as u32, 0x8007_0057);
    assert_eq!(MF_E_BUFFERTOOSMALL as u32, 0xC00D_36B1);
    assert_eq!(MF_E_INVALIDREQUEST as u32, 0xC00D_36B2);
    assert_eq!(MF_E_INVALIDSTREAMNUMBER as u32, 0xC00D_36B3);
}

/// A NaN edge is not the full frame, so it takes the scaling path, and the NaN converts to 0.
#[test]
fn scale_rect_nan_edges_convert_to_zero() {
    assert_eq!(scale(640, 480, [f32::NAN, 0.0, 0.5, 1.0]), [0, 0, 320, 480]);
    assert_eq!(scale(640, 480, [f32::NAN; 4]), [0, 0, 0, 0]);
}

#[test]
fn scale_rect_saturates_edges_beyond_i32() {
    assert_eq!(scale(1, 1, [0.0, 0.0, 1e10, 1.0]), [0, 0, i32::MAX, 1]);
    assert_eq!(scale(1, 1, [-1e10, 0.0, 1.0, 1.0]), [i32::MIN, 0, 1, 1]);
}

#[test]
fn scale_rect_full_frame_keeps_a_width_above_i32_max_as_negative() {
    assert_eq!(
        edges(scale_rect(u32::MAX, 1, &NormalizedRect::FULL)),
        [0, 0, -1, 1]
    );
}

#[test]
fn check_normalized_rect_rejects_each_clause_on_its_own() {
    let rejected = [
        [0.6, 0.0, 0.5, 1.0],
        [0.0, 0.6, 1.0, 0.5],
        [-0.5, 0.0, 1.0, 1.0],
        [0.0, -0.5, 1.0, 1.0],
        [0.0, 0.0, 1.5, 1.0],
        [0.0, 0.0, 1.0, 1.5],
    ];
    for values in rejected {
        assert_eq!(check_normalized_rect(&norm(values)), Err(E_INVALIDARG));
    }

    for values in [[0.0, 0.0, 1.0, 1.0], [0.5; 4], [1.0; 4]] {
        assert_eq!(check_normalized_rect(&norm(values)), Ok(()));
    }
    assert_eq!(check_normalized_rect(&norm([f32::NAN; 4])), Ok(()));
}

#[test]
fn video_position_checks_each_rectangle_on_its_own() {
    assert_eq!(
        video_position(Some([0.0, 0.0, 2.0, 1.0]), Some([0, 0, 10, 10])),
        E_INVALIDARG
    );
    assert_eq!(
        video_position(Some([0.0, 0.0, 1.0, 1.0]), Some([10, 10, 5, 5])),
        E_INVALIDARG
    );
    assert_eq!(video_position(None, Some([10, 0, 5, 5])), E_INVALIDARG);
    assert_eq!(video_position(None, Some([0, 10, 5, 5])), E_INVALIDARG);
    assert_eq!(
        video_position(Some([0.0, 0.0, 1.0, 1.0]), Some([0, 0, 10, 10])),
        S_OK
    );
}

#[test]
fn aspect_ratio_mode_rejects_bits_above_the_mask() {
    assert_eq!(AR_MODE_MASK, 7);
    assert_eq!(hresult(check_aspect_ratio_mode(7)), S_OK);
    assert_eq!(hresult(check_aspect_ratio_mode(8)), E_INVALIDARG);
    assert_eq!(hresult(check_aspect_ratio_mode(0x8000_0000)), E_INVALIDARG);
}

#[test]
fn gcd_with_zero_and_at_the_extremes() {
    assert_eq!(gcd(1, 0), 1);
    assert_eq!(gcd(0, 1), 1);
    assert_eq!(gcd(u32::MAX, u32::MAX), u32::MAX);
    assert_eq!(gcd(u32::MAX, u32::MAX - 1), 1);
}

#[test]
fn native_size_keeps_the_low_word_unsigned() {
    assert_eq!(native_of(Some(16_u64 << 32)), [16, 0, 1, 0]);
    assert_eq!(native_of(Some(u64::from(u32::MAX))), [0, -1, 0, 1]);
}

#[test]
fn aspect_ratio_centres_on_the_axis_that_shrinks() {
    assert_eq!(aspect([0, 0, 3, 4], [0, 0, 100, 100]), [13, 0, 88, 100]);
    assert_eq!(aspect([0, 0, 4, 1], [0, 0, 11, 11]), [0, 4, 11, 6]);
    assert_eq!(aspect([0, 0, 1, 2], [-11, -11, 0, 0]), [-7, -11, -2, 0]);
    assert_eq!(aspect([0, 0, 1, 2], [-10, -10, 0, 0]), [-7, -10, -2, 0]);
}

/// The C body computes these widths with signed overflow, which the sanitizer flags. The port
/// wraps them, and the results follow from the wrapped widths.
#[test]
fn aspect_ratio_wraps_signed_overflow() {
    assert_eq!(
        aspect([i32::MAX, 0, -i32::MAX, 0], [0, 0, 100, 100]),
        [0, 50, 100, 50]
    );
    assert_eq!(
        aspect([0, 0, 100, 100], [-i32::MAX, 0, i32::MAX, 100]),
        [-50, 0, 50, 100]
    );
}

#[test]
fn add_respects_the_sixteen_stream_limit() {
    assert_eq!(MAX_INPUT_STREAMS, 16);

    let mut streams = InputStreams::default();
    let fifteen: Vec<u32> = (1..=15).collect();
    assert_eq!(streams.add(&fifteen), Ok(()));
    assert_eq!(streams.add(&[16]), Err(E_INVALIDARG));

    let mut fresh = InputStreams::default();
    let sixteen: Vec<u32> = (1..=16).collect();
    assert_eq!(fresh.add(&sixteen), Err(E_INVALIDARG));
}

#[test]
fn rejected_batch_leaves_the_table_unchanged() {
    let mut streams = InputStreams::default();
    assert_eq!(streams.add(&[5]), Ok(()));
    let before = streams.clone();

    assert_eq!(streams.add(&[9, 9]), Err(E_INVALIDARG));
    assert_eq!(streams.add(&[6, 5]), Err(E_INVALIDARG));
    assert_eq!(streams, before);
}

/// The sorted index of 3 is 1, and the insertion-ordered list drops its entry at index 1, which
/// is 5.
#[test]
fn delete_drops_the_entry_at_the_sorted_index() {
    let mut streams = InputStreams::default();
    assert_eq!(streams.add(&[5, 3]), Ok(()));
    assert_eq!(streams.delete(3), Ok(()));
    assert_eq!(streams.stream_ids(4, 1), Ok(vec![0, 3]));
    assert_eq!(streams.get_zorder(5), Ok(1));
    assert_eq!(streams.get_zorder(3), Err(MF_E_INVALIDSTREAMNUMBER));
}

#[test]
fn delete_of_the_last_sorted_stream_drops_it() {
    let mut streams = InputStreams::default();
    assert_eq!(streams.add(&[3, 5]), Ok(()));
    assert_eq!(streams.delete(5), Ok(()));
    assert_eq!(streams.stream_ids(4, 1), Ok(vec![0, 3]));
}

#[test]
fn set_output_rect_checks_the_rectangle_before_the_stream() {
    let mut streams = InputStreams::default();
    assert_eq!(
        streams.set_output_rect(7, &norm([0.5, 0.0, 0.25, 1.0])),
        Err(E_INVALIDARG)
    );
}

#[test]
fn get_output_rect_of_a_missing_stream_fails() {
    assert_eq!(
        InputStreams::default().get_output_rect(42),
        Err(MF_E_INVALIDSTREAMNUMBER)
    );
}

#[test]
fn stream_ids_needs_a_nonzero_output_size() {
    assert_eq!(
        InputStreams::default().stream_ids(1, 0),
        Err(MF_E_BUFFERTOOSMALL)
    );
}

#[test]
fn set_zorder_checks_the_range_before_the_stream() {
    let mut streams = InputStreams::default();
    assert_eq!(streams.set_zorder(42, 5), Err(E_INVALIDARG));
    assert_eq!(streams.set_zorder(42, 0), Err(MF_E_INVALIDSTREAMNUMBER));
}
