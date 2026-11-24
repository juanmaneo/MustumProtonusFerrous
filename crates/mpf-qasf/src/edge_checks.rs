//! Rust-only edge cases: the arms the oracle commands do not reach, and the wrapping of sums that
//! Wine would overflow.

use crate::allocator::{
    buffer_size, decide_buffer_size, AllocatorProperties, Format, Subtype, DEFAULT_BUFFER_SIZE,
};
use crate::output::{
    plan_output, OutputBuffer, OutputRound, DMO_OUTPUT_DATA_BUFFERF_INCOMPLETE,
    DMO_OUTPUT_DATA_BUFFERF_SYNCPOINT, DMO_OUTPUT_DATA_BUFFERF_TIME,
    DMO_OUTPUT_DATA_BUFFERF_TIMELENGTH,
};
use crate::sample::{
    input_buffer, stream_sample, InputBuffer, InputTime, DMO_INPUT_DATA_BUFFERF_SYNCPOINT,
    DMO_INPUT_DATA_BUFFERF_TIME, DMO_INPUT_DATA_BUFFERF_TIMELENGTH, WM_SF_CLEANPOINT,
    WM_SF_DISCONTINUITY,
};
use crate::stream::{open_stream_count, raw_stream_name, Major, MAX_STREAMS};
use crate::SampleTime;

const TIME_AND_LENGTH: u32 = DMO_INPUT_DATA_BUFFERF_TIME | DMO_INPUT_DATA_BUFFERF_TIMELENGTH;

fn req(buffers: i32, buffer_size: i32, align: i32) -> AllocatorProperties {
    AllocatorProperties {
        buffers,
        buffer_size,
        align,
        prefix: 7,
    }
}

fn output(status: u32, timestamp: i64, timelength: i64, len: u32) -> OutputBuffer {
    OutputBuffer {
        status,
        timestamp,
        timelength,
        len,
    }
}

#[test]
fn unknown_format_and_other_subtypes_use_the_default_size() {
    assert_eq!(buffer_size(Format::Other), DEFAULT_BUFFER_SIZE);
    assert_eq!(
        buffer_size(Format::Wave {
            avg_bytes_per_sec: 9,
            subtype: Subtype::Other,
        }),
        DEFAULT_BUFFER_SIZE
    );
}

#[test]
fn pcm_and_float_waves_and_video_use_their_header_sizes() {
    let wave = |subtype| Format::Wave {
        avg_bytes_per_sec: 9,
        subtype,
    };
    assert_eq!(buffer_size(wave(Subtype::Pcm)), 9);
    assert_eq!(buffer_size(wave(Subtype::Float)), 9);
    assert_eq!(buffer_size(Format::Video { size_image: 9 }), 9);
}

#[test]
fn request_above_the_format_size_is_kept() {
    let props = decide_buffer_size(Format::Video { size_image: 100 }, req(0, 500, 0));
    assert_eq!(
        props,
        AllocatorProperties {
            buffers: 1,
            buffer_size: 500,
            align: 1,
            prefix: 7,
        }
    );
}

#[test]
fn negative_request_is_larger_than_any_size_and_is_kept() {
    let props = decide_buffer_size(Format::Other, req(3, i32::MIN, 2));
    assert_eq!(props.buffer_size, i32::MIN);
}

#[test]
fn size_above_i32_max_is_stored_by_two_complement() {
    let props = decide_buffer_size(
        Format::Video {
            size_image: 0x8000_0000,
        },
        req(1, 0, 1),
    );
    assert_eq!(props.buffer_size, i32::MIN);
}

#[test]
fn count_and_alignment_are_raised_only_below_one() {
    let props = decide_buffer_size(Format::Other, req(i32::MIN, 0, i32::MAX));
    assert_eq!((props.buffers, props.align), (1, i32::MAX));
}

#[test]
fn stream_count_is_capped_at_the_filter_room() {
    assert_eq!(open_stream_count(Some(MAX_STREAMS)), 16);
    assert_eq!(open_stream_count(Some(u32::MAX)), 16);
    assert_eq!(open_stream_count(None), 0);
}

#[test]
fn raw_names_follow_the_major_type() {
    assert_eq!(raw_stream_name(Major::Video, 15), "Raw Video 15");
    assert_eq!(raw_stream_name(Major::Other, 0), "Raw Audio 0");
}

#[test]
fn unconnected_output_takes_no_sample() {
    assert_eq!(
        stream_sample(false, 1, 2, WM_SF_CLEANPOINT | WM_SF_DISCONTINUITY),
        None
    );
}

#[test]
fn data_loss_flag_does_not_set_discontinuity_or_sync() {
    let sample = stream_sample(true, 0, 0, 0x4).expect("connected");
    assert!(!sample.discontinuity && !sample.sync_point);
}

#[test]
fn stop_wraps_when_time_and_duration_overflow_i64() {
    let sample = stream_sample(true, i64::MAX as u64, 2, 0).expect("connected");
    assert_eq!(
        sample.time,
        SampleTime {
            start: i64::MAX,
            stop: Some(i64::MIN + 1),
        }
    );
}

#[test]
fn unset_time_passes_a_zero_start_and_length() {
    assert_eq!(
        input_buffer(false, InputTime::Unset),
        InputBuffer {
            flags: 0,
            start: 0,
            length: 0,
        }
    );
}

#[test]
fn no_stop_has_length_one_even_at_the_top_of_the_range() {
    let buffer = input_buffer(false, InputTime::NoStop { start: i64::MAX });
    assert_eq!(
        buffer,
        InputBuffer {
            flags: TIME_AND_LENGTH,
            start: i64::MAX,
            length: 1,
        }
    );
}

#[test]
fn range_with_stop_before_start_has_a_negative_length() {
    let buffer = input_buffer(true, InputTime::Range { start: 10, stop: 4 });
    assert_eq!(
        buffer,
        InputBuffer {
            flags: DMO_INPUT_DATA_BUFFERF_SYNCPOINT | TIME_AND_LENGTH,
            start: 10,
            length: -6,
        }
    );
}

#[test]
fn empty_round_needs_no_more_data() {
    assert_eq!(
        plan_output(&[]),
        OutputRound {
            more_data: false,
            plans: Vec::new(),
        }
    );
}

#[test]
fn skipped_outputs_do_not_ask_for_more_data() {
    let round = plan_output(&[
        None,
        Some(output(DMO_OUTPUT_DATA_BUFFERF_INCOMPLETE, 0, 0, 0)),
    ]);
    assert!(round.more_data);
    assert!(!plan_output(&[None]).more_data);
}

#[test]
fn empty_incomplete_buffer_asks_for_more_data_but_is_not_delivered() {
    let plan = plan_output(&[Some(output(DMO_OUTPUT_DATA_BUFFERF_INCOMPLETE, 0, 0, 0))]).plans[0]
        .expect("a buffer is planned");
    assert!(plan.incomplete && !plan.deliver);
}

#[test]
fn timelength_without_time_sets_no_time() {
    let plan = plan_output(&[Some(output(DMO_OUTPUT_DATA_BUFFERF_TIMELENGTH, 5, 6, 1))]).plans[0]
        .expect("a buffer is planned");
    assert_eq!(plan.time, None);
}

#[test]
fn time_without_timelength_has_no_stop() {
    let plan = plan_output(&[Some(output(DMO_OUTPUT_DATA_BUFFERF_TIME, 5, 6, 1))]).plans[0]
        .expect("a buffer is planned");
    assert_eq!(plan.time, Some(SampleTime::at(5)));
}

#[test]
fn stop_wraps_when_timestamp_and_timelength_overflow_i64() {
    let status = DMO_OUTPUT_DATA_BUFFERF_TIME | DMO_OUTPUT_DATA_BUFFERF_TIMELENGTH;
    let plan =
        plan_output(&[Some(output(status, i64::MAX, 1, 1))]).plans[0].expect("a buffer is planned");
    assert_eq!(
        plan.time,
        Some(SampleTime {
            start: i64::MAX,
            stop: Some(i64::MIN),
        })
    );
}

#[test]
fn syncpoint_bit_sets_the_sync_point_alone() {
    let plan = plan_output(&[Some(output(DMO_OUTPUT_DATA_BUFFERF_SYNCPOINT, 0, 0, 0))]).plans[0]
        .expect("a buffer is planned");
    assert!(plan.sync_point && !plan.incomplete && plan.time.is_none());
}
