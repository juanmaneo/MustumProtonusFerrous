//! Edge cases around the oracle rows: timestamp ties, the `i64::MAX` fallback, empty descriptors,
//! the `SUCCEEDED` rule for `S_FALSE`, the sign of the advanced attribute and the stale
//! `STREAM_FLAG_PRESENTED` bit of a deselected stream. The expected values follow the C source
//! line by line. The harness did not generate them.

use crate::{
    allow_video_processor, first_stream_index, is_source_stopped, next_selected_stream,
    start_source, stream_read_index, transform_at_index, StreamSlot, StreamState,
    MF_E_INVALIDREQUEST, MF_E_INVALIDSTREAMNUMBER, MF_E_MEDIA_SOURCE_NO_STREAMS_SELECTED,
    MF_MEDIA_TYPE_VIDEO, MF_SOURCE_READER_ANY_STREAM, MF_SOURCE_READER_FIRST_AUDIO_STREAM,
    MF_SOURCE_READER_FIRST_VIDEO_STREAM, MF_SOURCE_READER_INVALID_STREAM_INDEX,
    STREAM_FLAG_PRESENTED, STREAM_FLAG_SELECTED, S_OK,
};

fn st(selected: bool, eos: bool, responses: u32, last_sample_ts: i64) -> StreamState {
    StreamState {
        selected,
        eos,
        responses,
        last_sample_ts,
    }
}

fn slot(has_stream: bool, flags: u32) -> StreamSlot {
    StreamSlot { has_stream, flags }
}

#[test]
fn next_selected_stream_without_streams_has_no_selection() {
    assert_eq!(
        next_selected_stream(&[]),
        Err(MF_E_MEDIA_SOURCE_NO_STREAMS_SELECTED)
    );
}

#[test]
fn next_selected_stream_tie_keeps_the_earliest_stream() {
    // The comparison is strict, so an equal timestamp does not replace the earlier stream.
    assert_eq!(
        next_selected_stream(&[st(true, false, 0, 5), st(true, false, 0, 5)]),
        Ok(0)
    );
}

#[test]
fn next_selected_stream_skips_a_drained_stream_with_the_lowest_timestamp() {
    assert_eq!(
        next_selected_stream(&[st(true, true, 0, 1), st(true, false, 0, 100)]),
        Ok(1)
    );
}

#[test]
fn next_selected_stream_never_picks_a_timestamp_of_max() {
    // A timestamp equal to i64::MAX is not below the initial minimum, so it is never chosen.
    // With nothing below it, the first selected stream is used.
    assert_eq!(
        next_selected_stream(&[st(false, false, 0, 0), st(true, false, 0, i64::MAX)]),
        Ok(1)
    );
    assert_eq!(
        next_selected_stream(&[st(true, false, 0, i64::MAX), st(true, false, 0, 5)]),
        Ok(1)
    );
}

#[test]
fn stream_read_index_direct_index_is_used_when_selected() {
    let streams = [st(false, false, 0, 0), st(true, false, 0, 0)];
    assert_eq!(stream_read_index(1, 0, 0, &streams), Ok(1));
}

#[test]
fn stream_read_index_direct_index_that_is_deselected_is_invalid_request() {
    let streams = [st(false, false, 0, 0), st(true, false, 0, 0)];
    assert_eq!(
        stream_read_index(0, 0, 0, &streams),
        Err(MF_E_INVALIDREQUEST)
    );
}

#[test]
fn stream_read_index_first_video_uses_the_cached_index() {
    // The cached index selects stream 0, which is deselected, even though stream 1 is selected.
    let streams = [st(false, false, 0, 0), st(true, false, 0, 0)];
    assert_eq!(
        stream_read_index(MF_SOURCE_READER_FIRST_VIDEO_STREAM, 0, 0, &streams),
        Err(MF_E_INVALIDREQUEST)
    );
}

#[test]
fn stream_read_index_absent_first_audio_is_out_of_range() {
    // An absent audio stream is cached as MF_SOURCE_READER_INVALID_STREAM_INDEX, which is past the
    // end of any descriptor.
    let streams = [st(true, false, 0, 0)];
    assert_eq!(
        stream_read_index(
            MF_SOURCE_READER_FIRST_AUDIO_STREAM,
            0,
            MF_SOURCE_READER_INVALID_STREAM_INDEX,
            &streams
        ),
        Err(MF_E_INVALIDSTREAMNUMBER)
    );
}

#[test]
fn stream_read_index_plain_index_past_the_end_is_out_of_range() {
    // 0xFFFF_FFFB is not one of the reserved indices, so it is a plain index past the end.
    let streams = [st(true, false, 0, 0), st(true, false, 0, 0)];
    assert_eq!(
        stream_read_index(0xFFFF_FFFB, 0, 0, &streams),
        Err(MF_E_INVALIDSTREAMNUMBER)
    );
}

#[test]
fn stream_read_index_any_without_selected_streams_has_no_selection() {
    let streams = [st(false, false, 0, 0)];
    assert_eq!(
        stream_read_index(MF_SOURCE_READER_ANY_STREAM, 0, 0, &streams),
        Err(MF_E_MEDIA_SOURCE_NO_STREAMS_SELECTED)
    );
}

#[test]
fn transform_at_index_without_entries_is_none() {
    assert_eq!(transform_at_index(&[], 0), None);
}

#[test]
fn transform_at_index_with_only_hidden_entries_is_none() {
    assert_eq!(transform_at_index(&[true, true], 0), None);
}

#[test]
fn transform_at_index_skips_leading_hidden_entries() {
    assert_eq!(transform_at_index(&[true, false, false], 0), Some(1));
    assert_eq!(transform_at_index(&[true, false, false], 1), Some(2));
}

#[test]
fn allow_video_processor_with_only_the_plain_attribute_set() {
    assert_eq!(allow_video_processor(true, None, Some(1)), (false, true));
}

#[test]
fn allow_video_processor_with_no_attributes_at_all() {
    assert_eq!(allow_video_processor(true, None, None), (false, false));
}

#[test]
fn allow_video_processor_treats_any_nonzero_advanced_value_as_enabled() {
    // The C converts the UINT32 to BOOL, so 0x8000_0000 is enabled and still allows the processor.
    assert_eq!(
        allow_video_processor(true, Some(0x8000_0000), Some(0)),
        (true, true)
    );
}

#[test]
fn is_source_stopped_with_no_streams_follows_the_source_state() {
    assert!(is_source_stopped(true, &[]));
    assert!(!is_source_stopped(false, &[]));
}

#[test]
fn is_source_stopped_ignores_flag_bits_other_than_stopped() {
    // Only STREAM_FLAG_STOPPED decides, so STREAM_FLAG_PRESENTED alone leaves the stream running.
    assert!(!is_source_stopped(
        true,
        &[slot(true, STREAM_FLAG_PRESENTED)]
    ));
}

#[test]
fn first_stream_index_without_streams_is_invalid() {
    assert_eq!(
        first_stream_index(&[], &MF_MEDIA_TYPE_VIDEO),
        MF_SOURCE_READER_INVALID_STREAM_INDEX
    );
}

#[test]
fn first_stream_index_returns_the_first_of_repeated_matches() {
    assert_eq!(
        first_stream_index(
            &[Some(MF_MEDIA_TYPE_VIDEO), Some(MF_MEDIA_TYPE_VIDEO)],
            &MF_MEDIA_TYPE_VIDEO
        ),
        0
    );
}

#[test]
fn first_stream_index_guid_differing_only_in_data4_does_not_match() {
    // IsEqualGUID compares the whole structure, so a near-miss major type is not a match.
    let mut near = MF_MEDIA_TYPE_VIDEO;
    near.data4[7] = 0x72;
    assert_eq!(
        first_stream_index(&[Some(near)], &MF_MEDIA_TYPE_VIDEO),
        MF_SOURCE_READER_INVALID_STREAM_INDEX
    );
}

#[test]
fn start_source_success_with_s_false_still_counts_as_success() {
    // SUCCEEDED accepts any non-negative HRESULT, so S_FALSE (1) marks the selected streams presented.
    let mut flags = [0, 0];
    let hr = start_source(false, &[true, false], &mut flags, &mut |_| 1);
    assert_eq!(hr, 1);
    assert_eq!(flags, [STREAM_FLAG_SELECTED | STREAM_FLAG_PRESENTED, 0]);
}

#[test]
fn start_source_failure_leaves_the_presented_flags_untouched() {
    let mut flags = [0, 0];
    let hr = start_source(false, &[true, true], &mut flags, &mut |_| {
        MF_E_INVALIDREQUEST
    });
    assert_eq!(hr, MF_E_INVALIDREQUEST);
    assert_eq!(flags, [STREAM_FLAG_SELECTED, STREAM_FLAG_SELECTED]);
}

#[test]
fn start_source_stopped_source_starts_even_with_no_selection() {
    let mut calls = Vec::new();
    let mut flags = [0];
    let hr = start_source(false, &[false], &mut flags, &mut |position_empty| {
        calls.push(position_empty);
        S_OK
    });
    assert_eq!(hr, S_OK);
    assert_eq!(calls, vec![false]);
    assert_eq!(flags, [0]);
}

#[test]
fn start_source_deselected_presented_stream_forces_a_restart() {
    // A deselected stream that is still presented counts as a selection change, so Start runs
    // again. Nothing clears its STREAM_FLAG_PRESENTED bit, which matches the C.
    let mut calls = Vec::new();
    let mut flags = [STREAM_FLAG_PRESENTED, 0];
    let hr = start_source(true, &[false, false], &mut flags, &mut |position_empty| {
        calls.push(position_empty);
        S_OK
    });
    assert_eq!(hr, S_OK);
    assert_eq!(calls, vec![true]);
    assert_eq!(flags, [STREAM_FLAG_PRESENTED, 0]);
}
