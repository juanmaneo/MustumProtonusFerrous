//! Replays of the edge rows E01 to E35 in `target/canon/mciqtz32/oracle.txt`, and Rust-only
//! tests for the branches that no oracle row reaches. The expected values are the `ret=` and
//! `out=` fields of the oracle rows, or the C control flow for the Rust-only tests.

use crate::abi::*;
use crate::args::*;
use crate::audio::*;
use crate::caps::*;
use crate::driver::*;
use crate::oracle_checks::{frame_of, wide};
use crate::put::*;
use crate::seek::*;
use crate::status::*;
use crate::time::*;
use crate::window::*;

const ZERO: Rect = Rect {
    left: 0,
    top: 0,
    right: 0,
    bottom: 0,
};

#[test]
fn e01_drv_open_without_params_is_session_device() {
    assert_eq!(route(DRV_OPEN, 0), Route::DrvOpen);
    assert_eq!(drv_open_reply(false, true, true, 0), SESSION_DEVICE);
}

#[test]
fn e02_drv_open_with_failed_class_replies_0() {
    assert_eq!(drv_open_reply(true, class_ok(false, 5), true, 7), 0);
}

#[test]
fn e03_drv_open_with_class_replies_device_id() {
    assert_eq!(drv_open_reply(true, class_ok(true, 0), true, 7), 7);
}

#[test]
fn e04_drv_close_of_open_device_replies_1() {
    assert_eq!(drv_close_reply(true, 3), 1);
}

#[test]
fn e05_drv_configure_of_open_device_replies_1() {
    assert_eq!(drv_configure_reply(true), 1);
}

#[test]
fn e06_drv_configure_of_missing_device_replies_0() {
    assert_eq!(drv_configure_reply(false), 0);
}

#[test]
fn e07_drv_remove_replies_restart() {
    assert_eq!(route(DRV_REMOVE, 0), Route::Reply(2));
}

#[test]
fn e08_mci_play_on_the_session_replies_1() {
    assert_eq!(route(MCI_PLAY, SESSION_DEVICE), Route::Reply(1));
}

#[test]
fn e09_mci_spin_is_unrecognized() {
    assert_eq!(route(MCI_SPIN, 3), Route::Unrecognized);
}

#[test]
fn e10_unknown_message_goes_to_default() {
    assert_eq!(route(0x9999, 3), Route::Default);
}

#[test]
fn e11_getdevcaps_without_item_flag_is_missing_parameter() {
    assert_eq!(
        devcap(0, MCI_GETDEVCAPS_CAN_RECORD),
        Err(MCIERR_MISSING_PARAMETER)
    );
}

#[test]
fn e12_getdevcaps_unknown_item_is_unsupported() {
    assert_eq!(
        devcap(MCI_GETDEVCAPS_ITEM, 0x999),
        Err(MCIERR_UNSUPPORTED_FUNCTION)
    );
}

#[test]
fn e13_status_mode_of_unknown_state_leaves_dwreturn_alone() {
    assert_eq!(mode(7, NotifyThread::Absent), None);
}

#[test]
fn e14_status_number_of_tracks_is_unrecognized() {
    assert_eq!(
        status_item(MCI_STATUS_ITEM, MCI_STATUS_NUMBER_OF_TRACKS),
        Err(MCIERR_UNRECOGNIZED_COMMAND)
    );
}

#[test]
fn e15_put_without_rect_replies_1() {
    assert_eq!(
        plan_put(0, ZERO),
        PutPlan {
            reply: 1,
            position: None,
            notify: false,
        }
    );
}

#[test]
fn e16_put_test_replies_0_and_changes_nothing() {
    assert_eq!(
        plan_put(MCI_DGV_RECT | MCI_TEST | MCI_DGV_PUT_DESTINATION, ZERO),
        PutPlan {
            reply: 0,
            position: None,
            notify: false,
        }
    );
}

#[test]
fn e17_put_rect_with_notify_asks_for_notify() {
    assert_eq!(
        plan_put(MCI_DGV_RECT | MCI_NOTIFY, ZERO),
        PutPlan {
            reply: 0,
            position: None,
            notify: true,
        }
    );
}

#[test]
fn e18_setaudio_test_within_range_changes_nothing() {
    let flags = MCI_DGV_SETAUDIO_ITEM | MCI_DGV_SETAUDIO_VALUE | MCI_TEST;
    assert_eq!(setaudio(flags, MCI_DGV_SETAUDIO_VOLUME, 500), Ok(None));
}

#[test]
fn e19_setaudio_test_out_of_range_still_fails() {
    let flags = MCI_DGV_SETAUDIO_ITEM | MCI_DGV_SETAUDIO_VALUE | MCI_TEST;
    assert_eq!(
        setaudio(flags, MCI_DGV_SETAUDIO_VOLUME, 1001),
        Err(MCIERR_OUTOFRANGE)
    );
}

#[test]
fn e20_setaudio_zero_is_silence() {
    let flags = MCI_DGV_SETAUDIO_ITEM | MCI_DGV_SETAUDIO_VALUE;
    assert_eq!(
        setaudio(flags, MCI_DGV_SETAUDIO_VOLUME, 0),
        Ok(Some(-10000))
    );
}

#[test]
fn e21_setaudio_1000_is_0_decibels() {
    let flags = MCI_DGV_SETAUDIO_ITEM | MCI_DGV_SETAUDIO_VALUE;
    assert_eq!(setaudio(flags, MCI_DGV_SETAUDIO_VOLUME, 1000), Ok(Some(0)));
}

#[test]
fn e22_seek_to_start_is_0() {
    assert_eq!(seek_target(MCI_SEEK_TO_START), Ok(0));
}

#[test]
fn e23_seek_to_a_position_is_internal() {
    assert_eq!(seek_target(MCI_TO), Err(MCIERR_INTERNAL));
}

#[test]
fn e24_play_to_5_in_media_time_stops_at_50000() {
    assert_eq!(
        play_range(MCI_TO, 0, 5, true, 0),
        PlayRange {
            start: 0,
            start_flags: AM_SEEKING_NO_POSITIONING,
            stop: 50000,
        }
    );
}

#[test]
fn e25_play_with_failed_run_is_internal() {
    assert_eq!(play_reply(E_FAIL, true), MCIERR_INTERNAL);
}

#[test]
fn e26_play_without_a_thread_is_internal() {
    assert_eq!(play_reply(S_OK, false), MCIERR_INTERNAL);
}

#[test]
fn e27_audio_only_file_needs_no_window() {
    let video = video_size(E_NOINTERFACE, 0, 0);
    assert_eq!(video, VideoSize::Audio);
    let plan = plan_window(video, MCI_OPEN_ELEMENT, WS_POPUP, 120, frame_of);
    assert_eq!(plan, WindowPlan::Audio);
    assert_eq!(window_status(plan, false), 0);
}

#[test]
fn e28_failed_video_size_is_internal() {
    let video = video_size(E_FAIL, 0, 0);
    assert_eq!(video, VideoSize::Failed);
    let plan = plan_window(video, MCI_OPEN_ELEMENT, WS_POPUP, 120, frame_of);
    assert_eq!(plan, WindowPlan::Failed);
    assert_eq!(window_status(plan, false), MCIERR_INTERNAL);
}

#[test]
fn e29_failed_createwindow_is_internal_with_default_style() {
    let plan = plan_window(
        VideoSize::Size {
            width: 100,
            height: 50,
        },
        MCI_OPEN_ELEMENT,
        WS_POPUP,
        120,
        frame_of,
    );
    assert_eq!(DEFAULT_STYLE, 0x02CE_0000);
    assert_eq!(
        plan,
        WindowPlan::Create {
            style: DEFAULT_STYLE,
            width: 120,
            height: 88,
        }
    );
    assert_eq!(window_status(plan, false), MCIERR_INTERNAL);
}

#[test]
fn e30_com_no_interface_is_internal() {
    assert_eq!(com_status(E_NOINTERFACE), MCIERR_INTERNAL);
}

#[test]
fn e31_child_of_parent_uses_client_size() {
    let flags = MCI_OPEN_ELEMENT | MCI_DGV_OPEN_PARENT;
    let plan = plan_window(
        VideoSize::Size {
            width: 100,
            height: 50,
        },
        flags,
        0,
        120,
        frame_of,
    );
    assert_eq!(
        plan,
        WindowPlan::Create {
            style: DEFAULT_STYLE,
            width: 120,
            height: 88,
        }
    );
    assert_eq!(placed_size(DEFAULT_STYLE, (100, 50), (64, 48)), (64, 48));
    assert_eq!(window_status(plan, true), 0);
}

#[test]
fn e32_open_without_a_name_is_internal() {
    assert_eq!(
        check_open_element(MCI_OPEN_ELEMENT, None),
        Err(MCIERR_INTERNAL)
    );
}

#[test]
fn e33_open_with_element_id_is_internal() {
    let movie = wide("movie.avi");
    assert_eq!(
        check_open_element(
            MCI_OPEN_ELEMENT | MCI_OPEN_ELEMENT_ID,
            Some(movie.as_slice())
        ),
        Err(MCIERR_INTERNAL)
    );
}

#[test]
fn e34_missing_parameter_block_is_null_parameter() {
    assert_eq!(check_params(false, true), Err(MCIERR_NULL_PARAMETER_BLOCK));
}

#[test]
fn e35_missing_device_is_invalid_device_id() {
    assert_eq!(check_params(true, false), Err(MCIERR_INVALID_DEVICE_ID));
}

#[test]
fn route_maps_each_mci_command_to_its_variant() {
    let table = [
        (MCI_OPEN_DRIVER, Command::Open),
        (MCI_CLOSE_DRIVER, Command::Close),
        (MCI_PLAY, Command::Play),
        (MCI_SEEK, Command::Seek),
        (MCI_STOP, Command::Stop),
        (MCI_PAUSE, Command::Pause),
        (MCI_RESUME, Command::Resume),
        (MCI_GETDEVCAPS, Command::GetDevCaps),
        (MCI_SET, Command::Set),
        (MCI_STATUS, Command::Status),
        (MCI_WHERE, Command::Where),
        (MCI_SETAUDIO, Command::SetAudio),
        (MCI_UPDATE, Command::Update),
        (MCI_WINDOW, Command::Window),
        (MCI_PUT, Command::Put),
    ];
    for (msg, command) in table {
        assert_eq!(route(msg, 3), Route::Command(command));
    }
}

#[test]
fn route_sends_each_unimplemented_command_to_unrecognized() {
    let names = [
        MCI_RECORD,
        MCI_INFO,
        MCI_LOAD,
        MCI_SAVE,
        MCI_FREEZE,
        MCI_REALIZE,
        MCI_UNFREEZE,
        MCI_STEP,
        MCI_COPY,
        MCI_CUT,
        MCI_DELETE,
        MCI_PASTE,
        MCI_CUE,
        MCI_CAPTURE,
        MCI_MONITOR,
        MCI_RESERVE,
        MCI_SIGNAL,
        MCI_SETVIDEO,
        MCI_QUALITY,
        MCI_LIST,
        MCI_UNDO,
        MCI_CONFIGURE,
        MCI_RESTORE,
        MCI_SPIN,
        MCI_ESCAPE,
        MCI_OPEN,
        MCI_CLOSE,
    ];
    assert_eq!(names.len(), 27);
    for msg in names {
        assert_eq!(route(msg, 3), Route::Unrecognized);
    }
}

#[test]
fn route_answers_driver_messages_before_the_device() {
    for msg in [
        DRV_LOAD,
        DRV_FREE,
        DRV_ENABLE,
        DRV_DISABLE,
        DRV_QUERYCONFIGURE,
    ] {
        assert_eq!(route(msg, 3), Route::Reply(1));
    }
    assert_eq!(route(DRV_CONFIGURE, 3), Route::DrvConfigure);
    assert_eq!(route(DRV_CLOSE, 3), Route::DrvClose);
    assert_eq!(route(DRV_EXITSESSION, 3), Route::Default);
}

#[test]
fn class_ok_accepts_a_new_or_an_existing_class() {
    assert!(class_ok(true, 0));
    assert!(class_ok(false, ERROR_CLASS_ALREADY_EXISTS));
    assert!(!class_ok(false, 5));
}

#[test]
fn drv_open_with_failed_allocation_replies_0_and_close_of_session_replies_1() {
    assert_eq!(drv_open_reply(true, true, false, 7), 0);
    assert_eq!(drv_close_reply(false, SESSION_DEVICE), 1);
}

#[test]
fn check_params_and_check_device_pass_when_present() {
    assert_eq!(check_params(true, true), Ok(()));
    assert_eq!(check_device(true), Ok(()));
    assert_eq!(check_device(false), Err(MCIERR_INVALID_DEVICE_ID));
}

#[test]
fn check_open_element_rejects_missing_flag_empty_and_nul_names() {
    let movie = wide("movie.avi");
    assert_eq!(
        check_open_element(0, Some(movie.as_slice())),
        Err(MCIERR_INTERNAL)
    );
    let empty: &[u16] = &[];
    assert_eq!(
        check_open_element(MCI_OPEN_ELEMENT, Some(empty)),
        Err(MCIERR_INTERNAL)
    );
    let nul_first = [0u16, 0x6d];
    assert_eq!(
        check_open_element(MCI_OPEN_ELEMENT, Some(nul_first.as_slice())),
        Err(MCIERR_INTERNAL)
    );
}

#[test]
fn set_time_format_without_flag_keeps_the_format() {
    assert_eq!(
        set_time_format(0, MCI_FORMAT_HMS, TimeFormat::Frames),
        Ok(TimeFormat::Frames)
    );
}

#[test]
fn set_time_format_milliseconds_arm_is_stored() {
    assert_eq!(
        set_time_format(
            MCI_SET_TIME_FORMAT,
            MCI_FORMAT_MILLISECONDS,
            TimeFormat::Frames
        ),
        Ok(TimeFormat::Milliseconds)
    );
    assert_eq!(time_format_value(TimeFormat::Milliseconds), 0x0215_0000);
}

#[test]
fn status_item_needs_the_flag_and_maps_each_known_item() {
    assert_eq!(
        status_item(0, MCI_STATUS_LENGTH),
        Err(MCIERR_UNRECOGNIZED_COMMAND)
    );
    assert_eq!(
        status_item(MCI_STATUS_ITEM, MCI_STATUS_POSITION),
        Ok(StatusItem::Position)
    );
    assert_eq!(
        status_item(MCI_STATUS_ITEM, MCI_STATUS_MODE),
        Ok(StatusItem::Mode)
    );
    assert_eq!(
        status_item(MCI_STATUS_ITEM, MCI_STATUS_TIME_FORMAT),
        Ok(StatusItem::Format)
    );
    for item in [
        MCI_STATUS_MEDIA_PRESENT,
        MCI_STATUS_READY,
        MCI_STATUS_CURRENT_TRACK,
    ] {
        assert_eq!(
            status_item(MCI_STATUS_ITEM, item),
            Err(MCIERR_UNRECOGNIZED_COMMAND)
        );
    }
    assert_eq!(StatusItem::Mode.reply(), MCI_RESOURCE_RETURNED);
}

#[test]
fn length_is_zero_for_a_failed_duration_or_a_negative_one() {
    assert_eq!(length(TimeFormat::Milliseconds, S_OK, E_FAIL, 5_000_000), 0);
    assert_eq!(length(TimeFormat::Frames, S_OK, S_OK, -1), 0);
}

#[test]
fn position_of_a_failed_call_is_internal() {
    assert_eq!(position(E_FAIL, 0), Err(MCIERR_INTERNAL));
}

#[test]
fn mode_reads_stop_for_stopped_and_for_running_without_a_live_thread() {
    assert_eq!(mode(STATE_STOPPED, NotifyThread::Absent), Some(0x020d_020d));
    assert_eq!(mode(STATE_RUNNING, NotifyThread::Absent), Some(0x020d_020d));
    assert_eq!(
        mode(STATE_RUNNING, NotifyThread::Finished),
        Some(0x020d_020d)
    );
}

#[test]
fn devcap_answers_every_item_with_its_value() {
    let no = Capability {
        value: 0x0213_0000,
        reply: MCI_RESOURCE_RETURNED,
    };
    for item in [
        MCI_GETDEVCAPS_CAN_RECORD,
        MCI_GETDEVCAPS_CAN_EJECT,
        MCI_GETDEVCAPS_CAN_SAVE,
        MCI_DGV_GETDEVCAPS_CAN_REVERSE,
        MCI_DGV_GETDEVCAPS_CAN_STRETCH,
        MCI_DGV_GETDEVCAPS_CAN_LOCK,
        MCI_DGV_GETDEVCAPS_CAN_FREEZE,
        MCI_DGV_GETDEVCAPS_CAN_STR_IN,
        MCI_DGV_GETDEVCAPS_HAS_STILL,
        MCI_DGV_GETDEVCAPS_CAN_TEST,
    ] {
        assert_eq!(devcap(MCI_GETDEVCAPS_ITEM, item), Ok(no));
    }
    let yes = Capability {
        value: 0x0214_0001,
        reply: MCI_RESOURCE_RETURNED,
    };
    for item in [
        MCI_GETDEVCAPS_HAS_AUDIO,
        MCI_GETDEVCAPS_HAS_VIDEO,
        MCI_GETDEVCAPS_USES_FILES,
        MCI_GETDEVCAPS_COMPOUND_DEVICE,
        MCI_GETDEVCAPS_CAN_PLAY,
    ] {
        assert_eq!(devcap(MCI_GETDEVCAPS_ITEM, item), Ok(yes));
    }
    assert_eq!(
        devcap(MCI_GETDEVCAPS_ITEM, MCI_GETDEVCAPS_DEVICE_TYPE),
        Ok(Capability {
            value: 0x0208_0208,
            reply: MCI_RESOURCE_RETURNED,
        })
    );
    assert_eq!(
        devcap(MCI_GETDEVCAPS_ITEM, MCI_DGV_GETDEVCAPS_MINIMUM_RATE),
        Err(MCIERR_UNSUPPORTED_FUNCTION)
    );
}

#[test]
fn setaudio_ignores_requests_without_the_volume_item_or_value() {
    assert_eq!(setaudio(0, MCI_DGV_SETAUDIO_VOLUME, 100), Ok(None));
    assert_eq!(
        setaudio(MCI_DGV_SETAUDIO_ITEM | MCI_DGV_SETAUDIO_VALUE, 1, 100),
        Ok(None)
    );
    assert_eq!(
        setaudio(MCI_DGV_SETAUDIO_ITEM, MCI_DGV_SETAUDIO_VOLUME, 100),
        Ok(None)
    );
}

#[test]
fn volume_for_maps_the_low_and_high_ends() {
    assert_eq!(volume_for(1), -6000);
    assert_eq!(volume_for(MAX_VOLUME), 0);
}

#[test]
fn play_range_from_and_to_in_frames_keeps_the_values() {
    assert_eq!(
        play_range(MCI_FROM | MCI_TO, 2, 5, false, 0),
        PlayRange {
            start: 2,
            start_flags: AM_SEEKING_ABSOLUTE_POSITIONING,
            stop: 5,
        }
    );
}

#[test]
fn seek_without_a_position_flag_is_missing_parameter() {
    assert_eq!(seek_target(0), Err(MCIERR_MISSING_PARAMETER));
}

#[test]
fn plan_window_for_a_wide_video_keeps_its_width() {
    assert_eq!(
        plan_window(
            VideoSize::Size {
                width: 200,
                height: 100,
            },
            0,
            0,
            120,
            frame_of
        ),
        WindowPlan::Create {
            style: DEFAULT_STYLE,
            width: 216,
            height: 138,
        }
    );
}

#[test]
fn style_uses_the_requested_style_only_with_open_ws() {
    assert_eq!(style(0, WS_POPUP), DEFAULT_STYLE);
    assert_eq!(style(MCI_DGV_OPEN_WS, WS_POPUP), WS_POPUP);
}

#[test]
fn placed_size_of_a_child_uses_the_video_size() {
    assert_eq!(placed_size(WS_CHILD, (100, 50), (64, 48)), (100, 50));
}
