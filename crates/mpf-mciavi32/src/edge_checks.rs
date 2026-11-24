//! Edge checks for what the oracle rows do not reach: the order of argument checks, the window
//! items, the zero frame time, the wrapping and rounding of the arithmetic, and the `dwSet` bits.
//! Their expected values follow the C control flow of `info.c`.

use super::*;
use crate::oracle_checks::{device, units};

const MS: u32 = MCI_FORMAT_MILLISECONDS;
const FRAMES: u32 = MCI_FORMAT_FRAMES;

fn set_params(time_format: u32, audio: u32) -> Option<SetParams> {
    Some(SetParams { time_format, audio })
}

#[test]
fn time_to_frame_in_milliseconds_is_none_without_a_frame_time() {
    let dev = device(MS, 0, 0, 0, 0, 0, false);
    assert_eq!(time_format_to_frame(&dev, 5), None);
    assert_eq!(frame_to_time_format(&dev, 5), 0);
}

#[test]
fn time_to_frame_in_milliseconds_truncates() {
    let dev = device(MS, 33366, 0, 0, 0, 0, false);
    assert_eq!(time_format_to_frame(&dev, 10009), Some(299));
}

#[test]
fn time_to_frame_in_milliseconds_wraps_the_product() {
    let dev = device(MS, 1000, 0, 0, 0, 0, false);
    // 0xffffffff * 1000 wraps to 0xfffffc18 before the division by 1000.
    assert_eq!(time_format_to_frame(&dev, u32::MAX), Some(4_294_966));
}

#[test]
fn frames_format_passes_values_through() {
    let dev = device(FRAMES, 33366, 300, 50, 0, 0, true);
    assert_eq!(frame_to_time_format(&dev, 300), 300);
    assert_eq!(time_format_to_frame(&dev, 7), Some(7));
}

#[test]
fn unknown_time_format_gives_zero() {
    let dev = device(9, 33366, 300, 50, 0, 0, true);
    assert_eq!(frame_to_time_format(&dev, 300), 0);
    assert_eq!(time_format_to_frame(&dev, 300), Some(0));
}

#[test]
fn dev_caps_answer_every_known_item() {
    let dev = device(MS, 33366, 0, 0, 0, 0, false);
    let get = |item| mci_get_dev_caps(Some(&dev), MCI_GETDEVCAPS_ITEM, Some(item));
    assert_eq!(
        get(MCI_GETDEVCAPS_DEVICE_TYPE),
        Reply::resource(0x0208_0208)
    );
    for item in [
        MCI_GETDEVCAPS_HAS_AUDIO,
        MCI_GETDEVCAPS_HAS_VIDEO,
        MCI_GETDEVCAPS_USES_FILES,
        MCI_GETDEVCAPS_COMPOUND_DEVICE,
        MCI_GETDEVCAPS_CAN_PLAY,
        MCI_DGV_GETDEVCAPS_CAN_TEST,
    ] {
        assert_eq!(get(item), Reply::resource(0x0214_0001), "item {item:#x}");
    }
    for item in [
        MCI_GETDEVCAPS_CAN_EJECT,
        MCI_GETDEVCAPS_CAN_RECORD,
        MCI_GETDEVCAPS_CAN_SAVE,
        MCI_DGV_GETDEVCAPS_CAN_REVERSE,
        MCI_DGV_GETDEVCAPS_CAN_STRETCH,
        MCI_DGV_GETDEVCAPS_CAN_LOCK,
        MCI_DGV_GETDEVCAPS_CAN_FREEZE,
        MCI_DGV_GETDEVCAPS_CAN_STR_IN,
        MCI_DGV_GETDEVCAPS_HAS_STILL,
        MCI_DGV_GETDEVCAPS_PALETTES,
    ] {
        assert_eq!(get(item), Reply::resource(0x0213_0000), "item {item:#x}");
    }
}

#[test]
fn dev_caps_check_order() {
    let dev = device(MS, 33366, 0, 0, 0, 0, false);
    let item = Some(MCI_GETDEVCAPS_DEVICE_TYPE);
    assert_eq!(
        mci_get_dev_caps(None, 0, None),
        Reply::code(MCIERR_NULL_PARAMETER_BLOCK)
    );
    assert_eq!(
        mci_get_dev_caps(None, MCI_TEST, item),
        Reply::code(MCIERR_INVALID_DEVICE_ID)
    );
    assert_eq!(
        mci_get_dev_caps(Some(&dev), MCI_TEST, item),
        Reply::code(MCIERR_MISSING_PARAMETER)
    );
    assert_eq!(
        mci_get_dev_caps(Some(&dev), MCI_GETDEVCAPS_ITEM | MCI_TEST, Some(0x7777)),
        Reply::code(0)
    );
}

#[test]
fn make_resource_masks_each_word_to_16_bits() {
    assert_eq!(make_resource(0x1_0005, 0x1_0006), 0x0006_0005);
    assert_eq!(make_resource(MCI_ON, MCI_ON_S), 0x8000_0001);
}

#[test]
fn bool_resource_packs_true_and_false() {
    assert_eq!(bool_resource(true), 0x0214_0001);
    assert_eq!(bool_resource(false), 0x0213_0000);
}

#[test]
fn mul_div_rounds_half_away_from_zero() {
    assert_eq!(mul_div(1, 1, 2), 1);
    assert_eq!(mul_div(8, 1, 3), 3);
    assert_eq!(mul_div(-3, -3, 2), 5);
    assert_eq!(mul_div(-7, 1, 2), -4);
    assert_eq!(mul_div(7, -1, 2), -4);
    assert_eq!(mul_div(10, 3, -4), -8);
    assert_eq!(mul_div(1, 1, -2), -1);
}

#[test]
fn mul_div_returns_minus_one_for_zero_divisor_and_overflow() {
    assert_eq!(mul_div(5, 1, 0), -1);
    assert_eq!(mul_div(i32::MAX, 2, 1), -1);
    assert_eq!(mul_div(-i32::MAX, 2, 1), -1);
    assert_eq!(mul_div(i32::MIN, 1, -1), -1);
    assert_eq!(mul_div(i32::MAX, 1, 1), i32::MAX);
    assert_eq!(mul_div(-i32::MAX, 1, 1), -i32::MAX);
}

#[test]
fn info_copies_at_most_one_unit_less_than_the_buffer() {
    let dev = device(MS, 33366, 0, 0, 0, 0, true);
    let mut empty: [u16; 0] = [];
    assert_eq!(
        mci_info(Some(&dev), MCI_INFO_PRODUCT, Some(&mut empty[..])),
        0
    );

    let mut one = units(b"#");
    assert_eq!(
        mci_info(Some(&dev), MCI_INFO_PRODUCT, Some(&mut one[..])),
        0
    );
    assert_eq!(one, units(b"\0"));

    let mut four = units(b"####");
    assert_eq!(
        mci_info(Some(&dev), MCI_INFO_PRODUCT, Some(&mut four[..3])),
        0
    );
    assert_eq!(four, units(b"Wi\0#"));
}

#[test]
fn info_stops_at_an_embedded_nul() {
    let mut dev = device(MS, 33366, 0, 0, 0, 0, true);
    dev.file_name = units(b"a\0b");
    let mut text = units(b"####");
    assert_eq!(mci_info(Some(&dev), MCI_INFO_FILE, Some(&mut text[..])), 0);
    assert_eq!(text, units(b"a\0##"));
}

#[test]
fn info_file_without_a_name_is_empty() {
    let dev = device(MS, 33366, 0, 0, 0, 0, false);
    let mut text = units(b"####");
    assert_eq!(mci_info(Some(&dev), MCI_INFO_FILE, Some(&mut text[..])), 0);
    assert_eq!(text, units(b"\0###"));
}

#[test]
fn info_version_takes_priority_over_file() {
    let dev = device(MS, 33366, 0, 0, 0, 0, true);
    let mut text = units(b"########");
    assert_eq!(
        mci_info(
            Some(&dev),
            MCI_INFO_VERSION | MCI_INFO_FILE,
            Some(&mut text[..])
        ),
        0
    );
    assert_eq!(text, units(b"1.1\0####"));
}

#[test]
fn info_check_order() {
    let dev = device(MS, 33366, 0, 0, 0, 0, true);
    let mut text = units(b"####");
    assert_eq!(mci_info(None, 0, None), MCIERR_NULL_PARAMETER_BLOCK);
    assert_eq!(
        mci_info(None, MCI_INFO_PRODUCT, Some(&mut text[..])),
        MCIERR_INVALID_DEVICE_ID
    );
    assert_eq!(
        mci_info(Some(&dev), MCI_TEST, None),
        MCIERR_NULL_PARAMETER_BLOCK
    );
    assert_eq!(
        mci_info(Some(&dev), MCI_TEST | MCI_INFO_PRODUCT, Some(&mut text[..])),
        0
    );
    assert_eq!(text, units(b"####"));
    assert_eq!(
        mci_info(Some(&dev), 0, Some(&mut text[..])),
        MCIERR_UNRECOGNIZED_COMMAND
    );
}

#[test]
fn set_on_then_off_in_one_call() {
    let mut dev = device(MS, 33366, 0, 0, 0, 0, false);
    let flags = MCI_SET_ON | MCI_SET_OFF | MCI_SET_VIDEO | MCI_SET_AUDIO;
    assert_eq!(
        mci_set(Some(&mut dev), flags, set_params(MS, MCI_SET_AUDIO_ALL)),
        0
    );
    assert_eq!(dev.set, 0);
}

#[test]
fn set_on_audio_channels() {
    let cases = [
        (MCI_SET_AUDIO_ALL, 0, 3),
        (MCI_SET_AUDIO_LEFT, 0, 1),
        (MCI_SET_AUDIO_RIGHT, 0, 2),
        (42, 4, 4),
    ];
    for (audio, before, after) in cases {
        let mut dev = device(MS, 33366, 0, 0, 0, before, false);
        assert_eq!(
            mci_set(
                Some(&mut dev),
                MCI_SET_ON | MCI_SET_AUDIO,
                set_params(MS, audio)
            ),
            0
        );
        assert_eq!(dev.set, after, "audio {audio}");
    }
}

#[test]
fn set_on_without_audio_flag_keeps_audio_bits() {
    let mut dev = device(MS, 33366, 0, 0, 0, 2, false);
    assert_eq!(
        mci_set(
            Some(&mut dev),
            MCI_SET_ON | MCI_SET_VIDEO,
            set_params(MS, MCI_SET_AUDIO_ALL)
        ),
        0
    );
    assert_eq!(dev.set, 6);
    assert_eq!(
        mci_set(
            Some(&mut dev),
            MCI_SET_ON,
            set_params(MS, MCI_SET_AUDIO_ALL)
        ),
        0
    );
    assert_eq!(dev.set, 6);
}

#[test]
fn set_off_video_and_audio_channels() {
    let cases = [
        (MCI_SET_OFF | MCI_SET_VIDEO, 0, 5, 1),
        (MCI_SET_OFF | MCI_SET_AUDIO, MCI_SET_AUDIO_ALL, 7, 4),
        // Wine clears the right bit for the left channel too, so the left bit stays set.
        (MCI_SET_OFF | MCI_SET_AUDIO, MCI_SET_AUDIO_LEFT, 7, 5),
        (MCI_SET_OFF | MCI_SET_AUDIO, MCI_SET_AUDIO_RIGHT, 3, 1),
        (MCI_SET_OFF | MCI_SET_AUDIO, 42, 7, 7),
    ];
    for (flags, audio, before, after) in cases {
        let mut dev = device(MS, 33366, 0, 0, 0, before, false);
        assert_eq!(mci_set(Some(&mut dev), flags, set_params(MS, audio)), 0);
        assert_eq!(dev.set, after, "flags {flags:#x} audio {audio}");
    }
}

#[test]
fn set_audio_without_on_or_off_changes_nothing() {
    let mut dev = device(MS, 33366, 0, 0, 0, 0, false);
    assert_eq!(
        mci_set(
            Some(&mut dev),
            MCI_SET_AUDIO | MCI_SET_VIDEO,
            set_params(MS, MCI_SET_AUDIO_ALL)
        ),
        0
    );
    assert_eq!(dev.set, 0);
}

#[test]
fn set_fixme_flags_change_nothing() {
    let mut dev = device(FRAMES, 33366, 0, 0, 0, 0, false);
    let before = dev.clone();
    let flags = MCI_DGV_SET_FILEFORMAT | MCI_DGV_SET_SPEED | MCI_DGV_SET_SEEK_EXACTLY;
    assert_eq!(mci_set(Some(&mut dev), flags, set_params(MS, 0)), 0);
    assert_eq!(dev, before);
}

#[test]
fn set_door_flags_check_after_the_time_format() {
    let mut dev = device(MS, 33366, 0, 0, 0, 0, false);
    assert_eq!(
        mci_set(Some(&mut dev), MCI_SET_TIME_FORMAT, set_params(9, 0)),
        MCIERR_BAD_TIME_FORMAT
    );
    assert_eq!(
        mci_set(
            Some(&mut dev),
            MCI_SET_TIME_FORMAT | MCI_SET_DOOR_CLOSED,
            set_params(FRAMES, 0)
        ),
        MCIERR_UNSUPPORTED_FUNCTION
    );
    assert_eq!(dev.time_format, FRAMES);

    let before = dev.clone();
    assert_eq!(
        mci_set(
            Some(&mut dev),
            MCI_SET_DOOR_OPEN | MCI_SET_ON | MCI_SET_VIDEO,
            set_params(MS, 0)
        ),
        MCIERR_UNSUPPORTED_FUNCTION
    );
    assert_eq!(dev, before);
}

#[test]
fn set_test_flag_changes_nothing() {
    let mut dev = device(MS, 33366, 0, 0, 0, 0, false);
    let before = dev.clone();
    let flags = MCI_TEST | MCI_SET_TIME_FORMAT | MCI_SET_ON | MCI_SET_VIDEO;
    assert_eq!(mci_set(Some(&mut dev), flags, set_params(9, 0)), 0);
    assert_eq!(dev, before);
}

#[test]
fn set_check_order() {
    let mut dev = device(MS, 33366, 0, 0, 0, 0, false);
    assert_eq!(mci_set(None, 0, None), MCIERR_NULL_PARAMETER_BLOCK);
    assert_eq!(
        mci_set(None, 0, set_params(MS, 0)),
        MCIERR_INVALID_DEVICE_ID
    );
    assert_eq!(
        mci_set(Some(&mut dev), MCI_SET_ON, None),
        MCIERR_NULL_PARAMETER_BLOCK
    );
}

#[test]
fn status_check_order() {
    let dev = device(MS, 33366, 300, 0, 0, 0, true);
    let mode = Some(MCI_STATUS_MODE);
    assert_eq!(
        mci_status(None, 0, None),
        Some(Reply::code(MCIERR_NULL_PARAMETER_BLOCK))
    );
    assert_eq!(
        mci_status(None, MCI_STATUS_ITEM, mode),
        Some(Reply::code(MCIERR_INVALID_DEVICE_ID))
    );
    assert_eq!(
        mci_status(Some(&dev), MCI_TEST, mode),
        Some(Reply::code(MCIERR_MISSING_PARAMETER))
    );
    assert_eq!(
        mci_status(Some(&dev), MCI_STATUS_ITEM | MCI_TEST, Some(0x7777)),
        Some(Reply::code(0))
    );
    assert_eq!(
        mci_status(
            Some(&dev),
            MCI_STATUS_ITEM | MCI_TEST,
            Some(MCI_DGV_STATUS_HWND)
        ),
        Some(Reply::code(0))
    );
}

#[test]
fn status_window_items_are_left_to_the_caller() {
    let dev = device(MS, 33366, 0, 0, 0, 0, true);
    for item in [
        MCI_DGV_STATUS_HWND,
        MCI_DGV_STATUS_WINDOW_VISIBLE,
        MCI_DGV_STATUS_WINDOW_MINIMIZED,
        MCI_DGV_STATUS_WINDOW_MAXIMIZED,
    ] {
        assert_eq!(mci_status(Some(&dev), MCI_STATUS_ITEM, Some(item)), None);
    }
}

#[test]
fn status_track_counts_are_one() {
    let dev = device(MS, 33366, 0, 0, 0, 0, false);
    for item in [MCI_STATUS_CURRENT_TRACK, MCI_STATUS_NUMBER_OF_TRACKS] {
        assert_eq!(
            mci_status(Some(&dev), MCI_STATUS_ITEM, Some(item)),
            Some(Reply::value(1))
        );
    }
}

#[test]
fn status_mode_ready_time_format_and_media_are_resources() {
    let paused = device(MS, 33366, 0, 0, MCI_MODE_PAUSE, 0, false);
    assert_eq!(
        mci_status(Some(&paused), MCI_STATUS_ITEM, Some(MCI_STATUS_MODE)),
        Some(Reply::resource(0x0211_0211))
    );

    let not_ready = device(MS, 33366, 0, 0, MCI_MODE_NOT_READY, 0, false);
    assert_eq!(
        mci_status(Some(&not_ready), MCI_STATUS_ITEM, Some(MCI_STATUS_READY)),
        Some(Reply::resource(0x0213_0000))
    );
    let stopped = device(MS, 33366, 0, 0, MCI_MODE_STOP, 0, false);
    assert_eq!(
        mci_status(Some(&stopped), MCI_STATUS_ITEM, Some(MCI_STATUS_READY)),
        Some(Reply::resource(0x0214_0001))
    );

    for (format, expected) in [(MS, 0x0215_0000), (FRAMES, 0x0218_0003)] {
        let dev = device(format, 33366, 0, 0, 0, 0, false);
        assert_eq!(
            mci_status(Some(&dev), MCI_STATUS_ITEM, Some(MCI_STATUS_TIME_FORMAT)),
            Some(Reply::resource(expected))
        );
    }

    for item in [MCI_STATUS_MEDIA_PRESENT, MCI_DGV_STATUS_FORWARD] {
        assert_eq!(
            mci_status(Some(&stopped), MCI_STATUS_ITEM, Some(item)),
            Some(Reply::resource(0x0214_0001))
        );
    }
}

#[test]
fn status_audio_and_video_are_driver_resources() {
    let audio = device(MS, 33366, 0, 0, 0, 1, false);
    let video = device(MS, 33366, 0, 0, 0, 4, false);
    let driver = MCI_RESOURCE_RETURNED | MCI_RESOURCE_DRIVER;
    assert_eq!(
        mci_status(Some(&audio), MCI_STATUS_ITEM, Some(MCI_DGV_STATUS_AUDIO)),
        Some(Reply {
            ret: driver,
            dw_return: Some(0x8000_0001)
        })
    );
    assert_eq!(
        mci_status(Some(&audio), MCI_STATUS_ITEM, Some(MCI_DGV_STATUS_VIDEO)),
        Some(Reply {
            ret: driver,
            dw_return: Some(0x8001_0000)
        })
    );
    assert_eq!(
        mci_status(Some(&video), MCI_STATUS_ITEM, Some(MCI_DGV_STATUS_AUDIO)),
        Some(Reply {
            ret: driver,
            dw_return: Some(0x8001_0000)
        })
    );
    assert_eq!(
        mci_status(Some(&video), MCI_STATUS_ITEM, Some(MCI_DGV_STATUS_VIDEO)),
        Some(Reply {
            ret: driver,
            dw_return: Some(0x8000_0001)
        })
    );
}

#[test]
fn status_plain_values() {
    let mut dev = device(MS, 40000, 0, 0, 0, 0, false);
    dev.audio_stream = 3;
    let value = |item| mci_status(Some(&dev), MCI_STATUS_ITEM, Some(item));
    assert_eq!(value(MCI_DGV_STATUS_BITSPERPEL), Some(Reply::value(24)));
    assert_eq!(value(MCI_DGV_STATUS_HPAL), Some(Reply::value(0)));
    assert_eq!(value(MCI_DGV_STATUS_SPEED), Some(Reply::value(1000)));
    assert_eq!(value(MCI_DGV_STATUS_AUDIO_STREAM), Some(Reply::value(3)));
}

#[test]
fn status_frame_rate_for_common_and_extreme_frame_times() {
    let fast = device(MS, 40000, 0, 0, 0, 0, false);
    let frame_rate =
        |dev: &Device| mci_status(Some(dev), MCI_STATUS_ITEM, Some(MCI_DGV_STATUS_FRAME_RATE));
    assert_eq!(frame_rate(&fast), Some(Reply::value(25000)));
    let unit = device(MS, 1, 0, 0, 0, 0, false);
    assert_eq!(frame_rate(&unit), Some(Reply::value(1_000_000_000)));
}

#[test]
fn status_pause_mode_needs_the_paused_state() {
    let paused = device(MS, 33366, 0, 0, MCI_MODE_PAUSE, 0, false);
    assert_eq!(
        mci_status(
            Some(&paused),
            MCI_STATUS_ITEM,
            Some(MCI_DGV_STATUS_PAUSE_MODE)
        ),
        Some(Reply::resource(0x020e_020e))
    );
}

#[test]
fn status_position_without_a_file_fails_like_length() {
    let dev = device(MS, 33366, 300, 50, 0, 0, false);
    assert_eq!(
        mci_status(Some(&dev), MCI_STATUS_ITEM, Some(MCI_STATUS_POSITION)),
        Some(Reply {
            ret: MCIERR_UNSUPPORTED_FUNCTION,
            dw_return: Some(0)
        })
    );
}

#[test]
fn status_position_start_and_current() {
    let dev = device(MS, 33366, 300, 50, 0, 0, true);
    assert_eq!(
        mci_status(Some(&dev), MCI_STATUS_ITEM, Some(MCI_STATUS_POSITION)),
        Some(Reply::value(1668))
    );
    assert_eq!(
        mci_status(
            Some(&dev),
            MCI_STATUS_ITEM | MCI_STATUS_START,
            Some(MCI_STATUS_POSITION)
        ),
        Some(Reply::value(0))
    );
}

#[test]
fn status_frames_format_reports_frames() {
    let dev = device(FRAMES, 33366, 300, 50, 0, 0, true);
    assert_eq!(
        mci_status(Some(&dev), MCI_STATUS_ITEM, Some(MCI_STATUS_LENGTH)),
        Some(Reply::value(300))
    );
    assert_eq!(
        mci_status(Some(&dev), MCI_STATUS_ITEM, Some(MCI_STATUS_POSITION)),
        Some(Reply::value(50))
    );
}

#[test]
fn status_unknown_item_is_unsupported() {
    let dev = device(MS, 33366, 0, 0, 0, 0, false);
    assert_eq!(
        mci_status(Some(&dev), MCI_STATUS_ITEM, Some(0x7777)),
        Some(Reply::code(MCIERR_UNSUPPORTED_FUNCTION))
    );
}

#[test]
fn status_notify_does_not_change_the_reply() {
    let dev = device(MS, 33366, 0, 0, MCI_MODE_PLAY, 0, false);
    assert_eq!(
        mci_status(
            Some(&dev),
            MCI_STATUS_ITEM | MCI_NOTIFY,
            Some(MCI_STATUS_MODE)
        ),
        mci_status(Some(&dev), MCI_STATUS_ITEM, Some(MCI_STATUS_MODE))
    );
}
