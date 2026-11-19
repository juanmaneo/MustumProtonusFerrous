//! Edge tests for the mciwave port: the branches the oracle rows do not reach.
//!
//! The expectations come from `dlls/mciwave/mciwave.c`. Where the C divides by a
//! zero block align (undefined behaviour), the test pins the MCIERR_INTERNAL
//! that the port returns instead.

use super::*;
use crate::device::{Env, WaveDevice};
use crate::format::{WaveFormat, PCM_DEFAULT};
use crate::playback::Playback;
use crate::settings::SetParams;

/// 11025 bytes a second at one byte a block, so a third of a second is 3675 bytes.
const BUFFER_3675: u32 = 3675;

fn env(open_ok: bool) -> Env {
    Env {
        open_ok,
        ..Env::default()
    }
}

fn device(time_format: u32, cksize: u32) -> WaveDevice {
    WaveDevice {
        time_format,
        cksize,
        ..WaveDevice::default()
    }
}

/// A file with `cksize` bytes of data, timed in bytes.
fn play_device(cksize: u32) -> WaveDevice {
    WaveDevice {
        has_file: true,
        ..device(MCI_FORMAT_BYTES, cksize)
    }
}

/// PCM with a zero block align. normalize_pcm keeps it zero, and the RIFF
/// skeleton does too.
fn no_align() -> WaveFormat {
    WaveFormat {
        block_align: 0,
        bits_per_sample: 0,
        ..PCM_DEFAULT
    }
}

#[test]
fn mul_div_with_a_negative_divisor() {
    assert_eq!(time::mul_div(3, 4, -2), -6);
}

#[test]
fn mul_div_sign_combinations_round_half_away_from_zero() {
    assert_eq!(time::mul_div(-7, -3, 2), 11);
    assert_eq!(time::mul_div(7, -3, 2), -11);
}

#[test]
fn mul_div_limits_are_symmetric() {
    assert_eq!(time::mul_div(-2_147_483_647, 1, 1), -2_147_483_647);
    assert_eq!(time::mul_div(2_147_483_647, 1, 1), 2_147_483_647);
    assert_eq!(time::mul_div(-2_147_483_647, 2, 1), -1);
}

#[test]
fn time_conversions_of_an_unknown_format_are_zero() {
    let dev = device(3, 1000);
    assert_eq!(time::byte_to_time(&dev, 100), 0);
    assert_eq!(time::time_to_byte(&dev, 100), 0);
}

#[test]
fn time_to_byte_clamps_a_millisecond_value_at_the_end_of_the_data() {
    let dev = device(MCI_FORMAT_MILLISECONDS, 10);
    assert_eq!(time::time_to_byte(&dev, 1), 10);
}

#[test]
fn time_to_byte_keeps_a_millisecond_value_past_the_end_of_the_data() {
    let dev = device(MCI_FORMAT_MILLISECONDS, 10);
    assert_eq!(time::time_to_byte(&dev, 2), 22);
}

#[test]
fn time_to_byte_within_the_data_is_not_clamped() {
    let dev = device(MCI_FORMAT_MILLISECONDS, 1000);
    assert_eq!(time::time_to_byte(&dev, 1), 11);
}

#[test]
fn block_alignment_needs_a_block_align() {
    assert_eq!(time::align_up(5, 0), None);
    assert_eq!(time::round_down(5, 0), None);
    assert_eq!(time::align_up(5, 2), Some(6));
    assert_eq!(time::round_down(5, 2), Some(4));
}

#[test]
fn normalize_pcm_leaves_other_formats_alone() {
    let mut format = WaveFormat {
        format_tag: 2,
        channels: 2,
        samples_per_sec: 8000,
        avg_bytes_per_sec: 1,
        block_align: 7,
        bits_per_sample: 16,
        cb_size: 0,
    };
    format.normalize_pcm();
    assert_eq!(format.block_align, 7);
    assert_eq!(format.avg_bytes_per_sec, 1);
}

#[test]
fn chunk_len_of_a_non_pcm_format_includes_its_extra_bytes() {
    assert_eq!(PCM_DEFAULT.chunk_len(), 16);
    let format = WaveFormat {
        format_tag: 2,
        cb_size: 32,
        ..PCM_DEFAULT
    };
    assert_eq!(format.chunk_len(), 50);
}

#[test]
fn notify_supersedes_a_pending_callback() {
    let mut dev = WaveDevice {
        has_callback: true,
        ..WaveDevice::default()
    };
    dev.notify_success(MCI_NOTIFY, true);
    assert_eq!(dev.sent, [MCI_NOTIFY_SUPERSEDED, MCI_NOTIFY_SUCCESSFUL]);
    assert!(!dev.has_callback);
}

#[test]
fn notify_success_needs_success_and_notify() {
    let mut dev = WaveDevice::default();
    dev.notify_success(MCI_NOTIFY, false);
    dev.notify_success(0, true);
    assert!(dev.sent.is_empty());
}

#[test]
fn swap_callback_keeps_a_callback_only_with_notify_and_a_window() {
    let mut dev = WaveDevice {
        has_callback: true,
        ..WaveDevice::default()
    };
    dev.swap_callback(0, 7);
    assert_eq!(dev.sent, [MCI_NOTIFY_ABORTED]);
    assert!(!dev.has_callback);

    dev.swap_callback(MCI_NOTIFY, 0x0001_0000);
    assert!(!dev.has_callback);

    dev.swap_callback(MCI_NOTIFY, 7);
    assert!(dev.has_callback);
    assert_eq!(dev.sent, [MCI_NOTIFY_ABORTED]);
}

#[test]
fn abort_callback_reports_only_a_pending_callback() {
    let mut dev = WaveDevice::default();
    dev.abort_callback();
    assert!(dev.sent.is_empty());

    dev.has_callback = true;
    dev.abort_callback();
    assert_eq!(dev.sent, [MCI_NOTIFY_ABORTED]);
    assert!(!dev.has_callback);
}

#[test]
fn end_playback_keeps_a_callback_without_notify() {
    let mut dev = WaveDevice {
        has_callback: true,
        status: MCI_MODE_PLAY,
        ..WaveDevice::default()
    };
    assert_eq!(dev.end_playback(0, 0), 0);
    assert!(dev.has_callback);
    assert!(dev.sent.is_empty());
    assert_eq!(dev.status, MCI_MODE_STOP);
}

#[test]
fn end_playback_reports_success_or_failure_to_a_callback() {
    let mut dev = WaveDevice {
        has_callback: true,
        ..WaveDevice::default()
    };
    assert_eq!(dev.end_playback(MCI_NOTIFY, 0), 0);
    assert_eq!(dev.sent, [MCI_NOTIFY_SUCCESSFUL]);

    dev.has_callback = true;
    assert_eq!(
        dev.end_playback(MCI_NOTIFY, MCIERR_DEVICE_OPEN),
        MCIERR_DEVICE_OPEN
    );
    assert_eq!(dev.sent, [MCI_NOTIFY_SUCCESSFUL, MCI_NOTIFY_FAILURE]);
}

#[test]
fn end_playback_without_a_callback_sends_nothing() {
    let mut dev = WaveDevice::default();
    assert_eq!(dev.end_playback(MCI_NOTIFY, 0), 0);
    assert!(dev.sent.is_empty());
}

#[test]
fn riff_skeleton_makes_the_format_non_default_and_empties_the_chunk() {
    let mut dev = WaveDevice {
        cksize: 99,
        format: WaveFormat {
            channels: 2,
            bits_per_sample: 16,
            block_align: 0,
            ..PCM_DEFAULT
        },
        ..WaveDevice::default()
    };
    dev.riff_skeleton();
    assert!(!dev.format_is_default);
    assert_eq!(dev.cksize, 0);
    assert_eq!(dev.format.block_align, 4);
}

#[test]
fn play_on_a_paused_playback_resumes_it() {
    let mut dev = WaveDevice {
        status: MCI_MODE_PAUSE,
        ..play_device(1000)
    };
    let outcome = playback::play(&mut dev, &Env::default(), MCI_WAIT, 0, 0, 0);
    assert_eq!(outcome, Playback::Done { ret: 0, seek: None });
    assert_eq!(dev.status, MCI_MODE_PLAY);
}

#[test]
fn play_on_a_paused_recording_stops_it_first() {
    let mut dev = WaveDevice {
        status: MCI_MODE_PAUSE,
        f_input: true,
        ..play_device(1000)
    };
    let outcome = playback::play(&mut dev, &env(true), MCI_WAIT, 0, 0, 0);
    assert_eq!(
        outcome,
        Playback::Stream {
            seek: 0,
            end: 1000,
            buffer: BUFFER_3675
        }
    );
    assert!(!dev.f_input);
}

#[test]
fn play_from_on_a_paused_playback_stops_it_instead_of_resuming() {
    let mut dev = WaveDevice {
        status: MCI_MODE_PAUSE,
        ..play_device(1000)
    };
    let outcome = playback::play(&mut dev, &env(true), MCI_WAIT | MCI_FROM, 0, 0, 0);
    assert_eq!(
        outcome,
        Playback::Stream {
            seek: 0,
            end: 1000,
            buffer: BUFFER_3675
        }
    );
}

#[test]
fn play_stops_a_playing_device_and_reports_a_stop_failure() {
    let mut dev = WaveDevice {
        status: MCI_MODE_PLAY,
        ..play_device(1000)
    };
    let env = Env {
        control_ret: 5,
        ..Env::default()
    };
    let outcome = playback::play(&mut dev, &env, MCI_WAIT, 0, 0, 0);
    assert_eq!(outcome, Playback::Done { ret: 5, seek: None });
}

#[test]
fn play_to_past_the_end_of_the_data_is_out_of_range() {
    let mut dev = play_device(1000);
    let outcome = playback::play(&mut dev, &env(true), MCI_WAIT | MCI_TO, 0, 2000, 0);
    assert_eq!(
        outcome,
        Playback::Done {
            ret: MCIERR_OUTOFRANGE,
            seek: None
        }
    );
}

#[test]
fn play_to_within_the_data_sets_the_end() {
    let mut dev = play_device(1000);
    let outcome = playback::play(&mut dev, &env(true), MCI_WAIT | MCI_TO, 0, 500, 0);
    assert_eq!(
        outcome,
        Playback::Stream {
            seek: 0,
            end: 500,
            buffer: BUFFER_3675
        }
    );
}

#[test]
fn play_from_a_position_past_the_end_is_out_of_range() {
    let mut dev = WaveDevice {
        position: 2000,
        ..play_device(1000)
    };
    let outcome = playback::play(&mut dev, &env(true), MCI_WAIT, 0, 0, 0);
    assert_eq!(
        outcome,
        Playback::Done {
            ret: MCIERR_OUTOFRANGE,
            seek: None
        }
    );
}

#[test]
fn play_from_with_a_zero_block_align_is_internal() {
    let mut dev = WaveDevice {
        format: no_align(),
        ..play_device(1000)
    };
    let outcome = playback::play(&mut dev, &env(true), MCI_WAIT | MCI_FROM, 0, 0, 0);
    assert_eq!(
        outcome,
        Playback::Done {
            ret: MCIERR_INTERNAL,
            seek: None
        }
    );
}

#[test]
fn play_with_a_zero_block_align_cannot_size_its_buffer() {
    let mut dev = WaveDevice {
        format: no_align(),
        ..play_device(1000)
    };
    let outcome = playback::play(&mut dev, &env(true), MCI_WAIT, 0, 0, 0);
    assert_eq!(
        outcome,
        Playback::Done {
            ret: MCIERR_INTERNAL,
            seek: Some(0)
        }
    );
    assert_eq!(dev.status, MCI_MODE_STOP);
}

#[test]
fn play_at_the_end_of_the_data_does_nothing() {
    let mut dev = WaveDevice {
        position: 1000,
        ..play_device(1000)
    };
    let outcome = playback::play(&mut dev, &env(true), MCI_WAIT, 0, 0, 0);
    assert_eq!(outcome, Playback::Done { ret: 0, seek: None });
    assert_eq!(dev.status, MCI_MODE_STOP);
}

#[test]
fn play_replaces_a_pending_callback_and_reports_success() {
    let mut dev = WaveDevice {
        has_callback: true,
        ..play_device(1000)
    };
    let flags = MCI_WAIT | MCI_NOTIFY;
    let outcome = playback::play(&mut dev, &env(true), flags, 0, 0, 7);
    assert_eq!(
        outcome,
        Playback::Stream {
            seek: 0,
            end: 1000,
            buffer: BUFFER_3675
        }
    );
    assert_eq!(dev.end_playback(flags, 0), 0);
    assert_eq!(dev.sent, [MCI_NOTIFY_ABORTED, MCI_NOTIFY_SUCCESSFUL]);
}

#[test]
fn play_with_a_zero_callback_window_keeps_no_callback() {
    let mut dev = play_device(1000);
    let flags = MCI_WAIT | MCI_NOTIFY;
    let outcome = playback::play(&mut dev, &env(true), flags, 0, 0, 0x0001_0000);
    assert_eq!(
        outcome,
        Playback::Stream {
            seek: 0,
            end: 1000,
            buffer: BUFFER_3675
        }
    );
    assert!(!dev.has_callback);
    assert_eq!(dev.end_playback(flags, 0), 0);
    assert!(dev.sent.is_empty());
}

#[test]
fn play_open_failure_reports_failure_to_the_callback() {
    let mut dev = play_device(1000);
    let outcome = playback::play(&mut dev, &env(false), MCI_WAIT | MCI_NOTIFY, 0, 0, 7);
    assert_eq!(
        outcome,
        Playback::Done {
            ret: MCIERR_DEVICE_OPEN,
            seek: Some(0)
        }
    );
    assert_eq!(dev.sent, [MCI_NOTIFY_FAILURE]);
    assert_eq!(dev.status, MCI_MODE_STOP);
}

#[test]
fn record_on_a_paused_recording_resumes_it() {
    let mut dev = WaveDevice {
        status: MCI_MODE_PAUSE,
        f_input: true,
        ..WaveDevice::default()
    };
    let outcome = playback::record(&mut dev, &Env::default(), MCI_WAIT, 0, 0, 0);
    assert_eq!(outcome, Playback::Done { ret: 0, seek: None });
    assert_eq!(dev.status, MCI_MODE_RECORD);
}

#[test]
fn record_refuses_a_device_that_is_playing() {
    let mut dev = WaveDevice {
        status: MCI_MODE_PLAY,
        ..WaveDevice::default()
    };
    let outcome = playback::record(&mut dev, &Env::default(), MCI_WAIT, 0, 0, 0);
    assert_eq!(
        outcome,
        Playback::Done {
            ret: MCIERR_INTERNAL,
            seek: None
        }
    );
    assert_eq!(dev.status, MCI_MODE_PLAY);
}

#[test]
fn record_from_past_the_empty_chunk_is_out_of_range_and_keeps_record_mode() {
    let mut dev = WaveDevice {
        time_format: MCI_FORMAT_BYTES,
        ..WaveDevice::default()
    };
    let outcome = playback::record(&mut dev, &Env::default(), MCI_WAIT | MCI_FROM, 1, 0, 0);
    assert_eq!(
        outcome,
        Playback::Done {
            ret: MCIERR_OUTOFRANGE,
            seek: None
        }
    );
    assert_eq!(dev.status, MCI_MODE_RECORD);
    assert!(dev.has_file);
    assert_eq!(dev.file_name, None);
}

#[test]
fn record_from_the_start_streams_from_zero() {
    let mut dev = WaveDevice::default();
    let outcome = playback::record(&mut dev, &env(true), MCI_WAIT | MCI_FROM, 0, 0, 0);
    assert_eq!(
        outcome,
        Playback::Stream {
            seek: 0,
            end: u32::MAX,
            buffer: BUFFER_3675
        }
    );
}

#[test]
fn record_to_sets_the_end_and_streams() {
    let mut dev = WaveDevice {
        time_format: MCI_FORMAT_BYTES,
        ..WaveDevice::default()
    };
    let outcome = playback::record(&mut dev, &env(true), MCI_WAIT | MCI_TO, 0, 500, 0);
    assert_eq!(
        outcome,
        Playback::Stream {
            seek: 0,
            end: 500,
            buffer: BUFFER_3675
        }
    );
    assert_eq!(dev.status, MCI_MODE_RECORD);
}

#[test]
fn record_without_a_range_streams_to_the_end_of_the_file() {
    let mut dev = WaveDevice::default();
    let outcome = playback::record(&mut dev, &env(true), MCI_WAIT, 0, 0, 0);
    assert_eq!(
        outcome,
        Playback::Stream {
            seek: 0,
            end: u32::MAX,
            buffer: BUFFER_3675
        }
    );
}

#[test]
fn record_to_the_start_returns_success_and_leaves_record_mode() {
    let mut dev = WaveDevice {
        time_format: MCI_FORMAT_BYTES,
        ..WaveDevice::default()
    };
    let outcome = playback::record(&mut dev, &env(true), MCI_WAIT | MCI_TO, 0, 0, 0);
    assert_eq!(outcome, Playback::Done { ret: 0, seek: None });
    assert_eq!(dev.status, MCI_MODE_RECORD);
}

#[test]
fn record_from_a_zero_block_align_is_internal() {
    let mut dev = WaveDevice {
        time_format: MCI_FORMAT_BYTES,
        format: no_align(),
        ..WaveDevice::default()
    };
    let outcome = playback::record(&mut dev, &env(true), MCI_WAIT | MCI_FROM, 0, 0, 0);
    assert_eq!(
        outcome,
        Playback::Done {
            ret: MCIERR_INTERNAL,
            seek: None
        }
    );
}

#[test]
fn record_with_a_zero_block_align_cannot_align_the_chunk() {
    let mut dev = WaveDevice {
        format: WaveFormat {
            format_tag: 2,
            block_align: 0,
            ..PCM_DEFAULT
        },
        ..WaveDevice::default()
    };
    let outcome = playback::record(&mut dev, &env(true), MCI_WAIT, 0, 0, 0);
    assert_eq!(
        outcome,
        Playback::Done {
            ret: MCIERR_INTERNAL,
            seek: None
        }
    );
}

#[test]
fn stop_on_a_stopped_device_keeps_the_callback() {
    let mut dev = WaveDevice {
        has_callback: true,
        ..WaveDevice::default()
    };
    assert_eq!(control::stop(&mut dev, &Env::default(), 0, false), 0);
    assert!(dev.has_callback);
    assert!(dev.sent.is_empty());
}

#[test]
fn stop_aborts_the_callback_of_a_playing_device() {
    let mut dev = WaveDevice {
        status: MCI_MODE_PLAY,
        has_callback: true,
        ..WaveDevice::default()
    };
    assert_eq!(
        control::stop(&mut dev, &Env::default(), MCI_NOTIFY, false),
        0
    );
    assert_eq!(dev.sent, [MCI_NOTIFY_ABORTED]);
    assert_eq!(dev.status, MCI_MODE_STOP);
}

#[test]
fn pause_and_resume_send_nothing_without_parameters() {
    let mut dev = WaveDevice {
        status: MCI_MODE_PLAY,
        ..WaveDevice::default()
    };
    assert_eq!(
        control::pause(&mut dev, &Env::default(), MCI_NOTIFY, false),
        0
    );
    assert_eq!(
        control::resume(&mut dev, &Env::default(), MCI_NOTIFY, false),
        0
    );
    assert!(dev.sent.is_empty());
}

#[test]
fn seek_to_start_moves_to_the_first_byte() {
    let mut dev = WaveDevice {
        position: 500,
        cksize: 1000,
        ..WaveDevice::default()
    };
    assert_eq!(
        control::seek(&mut dev, &Env::default(), MCI_SEEK_TO_START, 0),
        0
    );
    assert_eq!(dev.position, 0);
}

#[test]
fn seek_with_notify_reports_success() {
    let mut dev = WaveDevice {
        cksize: 1000,
        ..WaveDevice::default()
    };
    assert_eq!(
        control::seek(&mut dev, &Env::default(), MCI_SEEK_TO_END | MCI_NOTIFY, 0),
        0
    );
    assert_eq!(dev.position, 1000);
    assert_eq!(dev.sent, [MCI_NOTIFY_SUCCESSFUL]);
}

#[test]
fn seek_reports_a_stop_failure_and_keeps_the_position() {
    let mut dev = WaveDevice {
        status: MCI_MODE_PLAY,
        position: 500,
        cksize: 1000,
        ..WaveDevice::default()
    };
    let env = Env {
        control_ret: 5,
        ..Env::default()
    };
    assert_eq!(control::seek(&mut dev, &env, MCI_SEEK_TO_START, 0), 5);
    assert_eq!(dev.position, 500);
}

#[test]
fn seek_to_the_end_needs_no_block_align() {
    let mut dev = WaveDevice {
        cksize: 1000,
        format: no_align(),
        ..WaveDevice::default()
    };
    assert_eq!(
        control::seek(&mut dev, &Env::default(), MCI_SEEK_TO_END, 0),
        0
    );
    assert_eq!(dev.position, 1000);
}

#[test]
fn seek_before_the_end_with_a_zero_block_align_is_internal() {
    let mut dev = WaveDevice {
        cksize: 1000,
        format: no_align(),
        ..WaveDevice::default()
    };
    assert_eq!(
        control::seek(&mut dev, &Env::default(), MCI_SEEK_TO_START, 0),
        MCIERR_INTERNAL
    );
}

#[test]
fn set_accepts_each_time_format() {
    let mut dev = WaveDevice::default();
    for time_format in [
        MCI_FORMAT_MILLISECONDS,
        MCI_FORMAT_BYTES,
        MCI_FORMAT_SAMPLES,
    ] {
        let params = SetParams {
            time_format,
            ..SetParams::default()
        };
        assert_eq!(
            settings::set(&mut dev, &Env::default(), MCI_SET_TIME_FORMAT, &params),
            0
        );
        assert_eq!(dev.time_format, time_format);
    }
}

#[test]
fn set_input_to_the_same_device_does_not_stop_it() {
    let mut dev = WaveDevice {
        input: 1,
        status: MCI_MODE_PLAY,
        ..WaveDevice::default()
    };
    let env = Env {
        wave_in_devs: 2,
        ..Env::default()
    };
    let params = SetParams {
        input: 1,
        ..SetParams::default()
    };
    assert_eq!(settings::set(&mut dev, &env, MCI_WAVE_INPUT, &params), 0);
    assert_eq!(dev.status, MCI_MODE_PLAY);
}

#[test]
fn set_output_to_the_same_device_does_not_stop_it() {
    let mut dev = WaveDevice {
        output: 1,
        status: MCI_MODE_PLAY,
        ..WaveDevice::default()
    };
    let env = Env {
        wave_out_devs: 2,
        ..Env::default()
    };
    let params = SetParams {
        output: 1,
        ..SetParams::default()
    };
    assert_eq!(settings::set(&mut dev, &env, MCI_WAVE_OUTPUT, &params), 0);
    assert_eq!(dev.status, MCI_MODE_PLAY);
}

#[test]
fn set_any_output_keeps_a_playing_device_when_the_output_is_unchanged() {
    let mut dev = WaveDevice {
        status: MCI_MODE_PLAY,
        ..WaveDevice::default()
    };
    let params = SetParams {
        output: WAVE_MAPPER,
        ..SetParams::default()
    };
    assert_eq!(
        settings::set(&mut dev, &Env::default(), MCI_WAVE_SET_ANYOUTPUT, &params),
        0
    );
    assert_eq!(dev.output, WAVE_MAPPER);
    assert_eq!(dev.status, MCI_MODE_PLAY);
}

#[test]
fn set_any_input_stops_a_playing_device_when_the_input_changes() {
    let mut dev = WaveDevice {
        input: 1,
        status: MCI_MODE_PLAY,
        ..WaveDevice::default()
    };
    let params = SetParams {
        input: 0,
        ..SetParams::default()
    };
    assert_eq!(
        settings::set(&mut dev, &Env::default(), MCI_WAVE_SET_ANYINPUT, &params),
        0
    );
    assert_eq!(dev.status, MCI_MODE_STOP);
    assert_eq!(dev.input, WAVE_MAPPER);
}

#[test]
fn set_format_fields_are_refused_once_the_format_is_not_default() {
    let mut dev = WaveDevice {
        format_is_default: false,
        ..WaveDevice::default()
    };
    let refused = [
        MCI_WAVE_SET_FORMATTAG,
        MCI_WAVE_SET_CHANNELS,
        MCI_WAVE_SET_SAMPLESPERSEC,
        MCI_WAVE_SET_AVGBYTESPERSEC,
        MCI_WAVE_SET_BLOCKALIGN,
        MCI_WAVE_SET_BITSPERSAMPLE,
    ];
    for flags in refused {
        assert_eq!(
            settings::set(&mut dev, &Env::default(), flags, &SetParams::default()),
            MCIERR_NONAPPLICABLE_FUNCTION
        );
    }
    assert_eq!(dev.format, PCM_DEFAULT);
}

#[test]
fn set_with_notify_reports_success() {
    let mut dev = WaveDevice::default();
    let outcome = settings::set(&mut dev, &Env::default(), MCI_NOTIFY, &SetParams::default());
    assert_eq!(outcome, 0);
    assert_eq!(dev.sent, [MCI_NOTIFY_SUCCESSFUL]);
}

#[test]
fn status_position_needs_a_file() {
    let mut dev = WaveDevice::default();
    assert_eq!(
        query::status(&mut dev, MCI_STATUS_ITEM, MCI_STATUS_POSITION),
        query::Reply {
            ret: MCIERR_UNSUPPORTED_FUNCTION,
            value: 0
        }
    );
}

#[test]
fn status_wave_devices_report_the_id_or_the_mapper_name() {
    let mut dev = WaveDevice {
        input: 3,
        output: WAVE_MAPPER,
        ..WaveDevice::default()
    };
    assert_eq!(
        query::status(&mut dev, MCI_STATUS_ITEM, MCI_WAVE_INPUT),
        query::Reply { ret: 0, value: 3 }
    );
    let mapper = query::Reply {
        ret: MCI_RESOURCE_RETURNED,
        value: make_resource(u32::from(WAVE_MAPPER), WAVE_MAPPER_S),
    };
    assert_eq!(
        query::status(&mut dev, MCI_STATUS_ITEM, MCI_WAVE_OUTPUT),
        mapper
    );
}

#[test]
fn status_with_notify_reports_success_on_an_ok_reply() {
    let mut dev = WaveDevice::default();
    let reply = query::status(
        &mut dev,
        MCI_STATUS_ITEM | MCI_NOTIFY,
        MCI_STATUS_CURRENT_TRACK,
    );
    assert_eq!(reply, query::Reply { ret: 0, value: 1 });
    assert_eq!(dev.sent, [MCI_NOTIFY_SUCCESSFUL]);
}

#[test]
fn status_with_notify_sends_nothing_on_an_error_reply() {
    let mut dev = WaveDevice::default();
    let reply = query::status(&mut dev, MCI_STATUS_ITEM | MCI_NOTIFY, MCI_STATUS_POSITION);
    assert_eq!(reply.ret, MCIERR_UNSUPPORTED_FUNCTION);
    assert!(dev.sent.is_empty());
}

#[test]
fn status_ready_is_true_unless_the_device_is_not_ready() {
    let mut dev = WaveDevice::default();
    assert_eq!(
        query::status(&mut dev, MCI_STATUS_ITEM, MCI_STATUS_READY),
        query::Reply {
            ret: MCI_RESOURCE_RETURNED,
            value: bool_resource(true)
        }
    );
}

#[test]
fn dev_caps_with_notify_reports_success_on_an_ok_reply() {
    let mut dev = WaveDevice::default();
    let reply = query::dev_caps(
        &mut dev,
        &Env::default(),
        MCI_GETDEVCAPS_ITEM | MCI_NOTIFY,
        MCI_GETDEVCAPS_DEVICE_TYPE,
    );
    assert_eq!(
        reply,
        query::Reply {
            ret: MCI_RESOURCE_RETURNED,
            value: make_resource(MCI_DEVTYPE_WAVEFORM_AUDIO, MCI_DEVTYPE_WAVEFORM_AUDIO)
        }
    );
    assert_eq!(dev.sent, [MCI_NOTIFY_SUCCESSFUL]);
}

#[test]
fn dev_caps_with_notify_sends_nothing_on_an_unknown_item() {
    let mut dev = WaveDevice::default();
    let reply = query::dev_caps(
        &mut dev,
        &Env::default(),
        MCI_GETDEVCAPS_ITEM | MCI_NOTIFY,
        99,
    );
    assert_eq!(
        reply,
        query::Reply {
            ret: MCIERR_UNRECOGNIZED_COMMAND,
            value: 0
        }
    );
    assert!(dev.sent.is_empty());
}

#[test]
fn info_with_notify_reports_success() {
    let mut dev = WaveDevice::default();
    assert_eq!(
        query::info(&mut dev, MCI_INFO_PRODUCT | MCI_NOTIFY, 64),
        (0, "Wine's audio player".to_string())
    );
    assert_eq!(dev.sent, [MCI_NOTIFY_SUCCESSFUL]);
}

#[test]
fn info_with_a_one_unit_buffer_returns_an_empty_string() {
    let mut dev = WaveDevice::default();
    assert_eq!(
        query::info(&mut dev, MCI_INFO_PRODUCT, 1),
        (0, String::new())
    );
}

#[test]
fn info_cuts_a_surrogate_pair_to_a_replacement_character() {
    let mut dev = WaveDevice {
        file_name: Some("\u{1F600}".to_string()),
        ..WaveDevice::default()
    };
    assert_eq!(
        query::info(&mut dev, MCI_INFO_FILE, 2),
        (0, "\u{FFFD}".to_string())
    );
}
