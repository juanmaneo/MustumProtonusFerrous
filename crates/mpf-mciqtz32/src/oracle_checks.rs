//! Replays of the oracle rows O01 to O25 in `target/canon/mciqtz32/oracle.txt`. The oracle
//! compiles the verbatim `mciqtz.c` with `-fsanitize=address,undefined`, and mocks the Win32,
//! COM and DirectShow calls. Each test names the row it replays. The expected values are the
//! `ret=` and `out=` fields of that row.

use crate::abi::*;
use crate::args::*;
use crate::audio::*;
use crate::caps::*;
use crate::driver::*;
use crate::put::*;
use crate::seek::*;
use crate::status::*;
use crate::time::*;
use crate::window::*;

/// The oracle's `AdjustWindowRect` moves the left edge out by 8, the top out by 30, and the right
/// and bottom edges out by 8. That grows the frame by 16 across and by 38 down.
pub(crate) fn frame_of(width: i32, height: i32) -> (i32, i32) {
    (width + 16, height + 38)
}

/// The UTF-16 units of a file name, as `lpstrElementName` holds them, without the terminator.
pub(crate) fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

#[test]
fn o01_setaudio_volume_100_is_minus_2000() {
    let flags = MCI_DGV_SETAUDIO_ITEM | MCI_DGV_SETAUDIO_VALUE;
    assert_eq!(
        setaudio(flags, MCI_DGV_SETAUDIO_VOLUME, 100),
        Ok(Some(-2000))
    );
    assert_eq!(com_status(S_OK), 0);
}

#[test]
fn o02_setaudio_volume_3_is_minus_5045() {
    let flags = MCI_DGV_SETAUDIO_ITEM | MCI_DGV_SETAUDIO_VALUE;
    assert_eq!(setaudio(flags, MCI_DGV_SETAUDIO_VOLUME, 3), Ok(Some(-5045)));
    assert_eq!(com_status(S_OK), 0);
}

#[test]
fn o03_setaudio_volume_1001_is_out_of_range() {
    let flags = MCI_DGV_SETAUDIO_ITEM | MCI_DGV_SETAUDIO_VALUE;
    assert_eq!(
        setaudio(flags, MCI_DGV_SETAUDIO_VOLUME, 1001),
        Err(MCIERR_OUTOFRANGE)
    );
}

#[test]
fn o04_set_time_format_frames_stores_frames() {
    let format = set_time_format(
        MCI_SET_TIME_FORMAT,
        MCI_FORMAT_FRAMES,
        TimeFormat::Milliseconds,
    );
    assert_eq!(format, Ok(TimeFormat::Frames));
    assert_eq!(TimeFormat::Frames.code(), 3);
}

#[test]
fn o05_set_time_format_hms_is_bad_and_keeps_milliseconds() {
    let format = set_time_format(
        MCI_SET_TIME_FORMAT,
        MCI_FORMAT_HMS,
        TimeFormat::Milliseconds,
    );
    assert_eq!(format, Err(MCIERR_BAD_TIME_FORMAT));
}

#[test]
fn o06_status_length_in_milliseconds_is_1234() {
    assert_eq!(
        length(TimeFormat::Milliseconds, S_OK, S_OK, 12_345_678),
        1234
    );
    assert_eq!(StatusItem::Length.reply(), MCI_INTEGER_RETURNED);
    assert_eq!(
        status_item(MCI_STATUS_ITEM, MCI_STATUS_LENGTH),
        Ok(StatusItem::Length)
    );
}

#[test]
fn o07_status_length_in_frames_is_777() {
    assert_eq!(length(TimeFormat::Frames, S_OK, S_OK, 777), 777);
}

#[test]
fn o08_status_length_with_failed_set_format_is_0() {
    assert_eq!(
        length(TimeFormat::Milliseconds, E_FAIL, S_OK, 12_345_678),
        0
    );
    assert_eq!(StatusItem::Length.reply(), MCI_INTEGER_RETURNED);
}

#[test]
fn o09_status_position_negative_sign_extends() {
    assert_eq!(position(S_OK, -12_345), Ok(u64::MAX));
    assert_eq!(StatusItem::Position.reply(), MCI_INTEGER_RETURNED);
}

#[test]
fn o10_status_mode_running_with_live_thread_is_play() {
    let thread = NotifyThread::from_wait(WAIT_TIMEOUT);
    assert_eq!(mode(STATE_RUNNING, thread), Some(0x020e_020e));
    assert_eq!(StatusItem::Mode.reply(), MCI_RESOURCE_RETURNED);
}

#[test]
fn o11_status_mode_running_with_finished_thread_is_stop() {
    let thread = NotifyThread::from_wait(WAIT_OBJECT_0);
    assert_eq!(mode(STATE_RUNNING, thread), Some(0x020d_020d));
}

#[test]
fn o12_status_mode_paused_is_pause() {
    assert_eq!(mode(STATE_PAUSED, NotifyThread::Absent), Some(0x0211_0211));
}

#[test]
fn o13_status_time_format_frames_is_resource_0x02180003() {
    assert_eq!(time_format_value(TimeFormat::Frames), 0x0218_0003);
    assert_eq!(StatusItem::Format.reply(), MCI_RESOURCE_RETURNED);
}

#[test]
fn o14_getdevcaps_can_record_is_false_resource() {
    assert_eq!(
        devcap(MCI_GETDEVCAPS_ITEM, MCI_GETDEVCAPS_CAN_RECORD),
        Ok(Capability {
            value: 0x0213_0000,
            reply: MCI_RESOURCE_RETURNED,
        })
    );
}

#[test]
fn o15_getdevcaps_max_windows_is_1_with_reply_0() {
    assert_eq!(
        devcap(MCI_GETDEVCAPS_ITEM, MCI_DGV_GETDEVCAPS_MAX_WINDOWS),
        Ok(Capability { value: 1, reply: 0 })
    );
}

#[test]
fn o16_getdevcaps_maximum_rate_is_unsupported() {
    assert_eq!(
        devcap(MCI_GETDEVCAPS_ITEM, MCI_DGV_GETDEVCAPS_MAXIMUM_RATE),
        Err(MCIERR_UNSUPPORTED_FUNCTION)
    );
}

#[test]
fn o17_play_from_2_to_5_in_media_time_is_20000_to_50000() {
    let range = play_range(MCI_FROM | MCI_TO, 2, 5, true, 0);
    assert_eq!(
        range,
        PlayRange {
            start: 20000,
            start_flags: AM_SEEKING_ABSOLUTE_POSITIONING,
            stop: 50000,
        }
    );
    assert_eq!(play_reply(S_OK, true), 0);
}

#[test]
fn o18_play_without_range_runs_to_the_duration() {
    let range = play_range(0, 0, 0, false, 777);
    assert_eq!(
        range,
        PlayRange {
            start: 0,
            start_flags: AM_SEEKING_NO_POSITIONING,
            stop: 777,
        }
    );
    assert_eq!(play_reply(S_OK, true), 0);
}

#[test]
fn o19_seek_to_end_is_internal() {
    assert_eq!(seek_target(MCI_SEEK_TO_END), Err(MCIERR_INTERNAL));
}

#[test]
fn o20_put_destination_sets_the_window_position() {
    let rc = Rect {
        left: 1,
        top: 2,
        right: 11,
        bottom: 22,
    };
    assert_eq!(
        plan_put(MCI_DGV_RECT | MCI_DGV_PUT_DESTINATION, rc),
        PutPlan {
            reply: 0,
            position: Some(Position {
                left: 1,
                top: 2,
                width: 10,
                height: 20,
            }),
            notify: false,
        }
    );
}

#[test]
fn o21_drv_install_replies_restart() {
    assert_eq!(route(DRV_INSTALL, 0), Route::Reply(2));
}

#[test]
fn o22_drv_exitsession_on_the_session_replies_1() {
    assert_eq!(route(DRV_EXITSESSION, SESSION_DEVICE), Route::Reply(1));
}

#[test]
fn o23_drv_close_of_a_missing_device_replies_0() {
    assert_eq!(drv_close_reply(false, 9), 0);
}

#[test]
fn o24_open_with_element_id_fails_after_the_com_steps() {
    let movie = wide("movie.avi");
    assert_eq!(
        check_open_element(
            MCI_OPEN_ELEMENT | MCI_OPEN_ELEMENT_ID,
            Some(movie.as_slice())
        ),
        Err(MCIERR_INTERNAL)
    );
    assert_eq!(com_status(S_OK), 0);
}

#[test]
fn o25_open_video_window_is_popup_120_by_88() {
    let movie = wide("movie.avi");
    let flags = MCI_OPEN_ELEMENT | MCI_DGV_OPEN_WS;
    assert_eq!(check_open_element(flags, Some(movie.as_slice())), Ok(()));
    let video = video_size(S_OK, 100, 50);
    assert_eq!(
        video,
        VideoSize::Size {
            width: 100,
            height: 50,
        }
    );
    let plan = plan_window(video, flags, WS_POPUP, 120, frame_of);
    assert_eq!(
        plan,
        WindowPlan::Create {
            style: WS_POPUP,
            width: 120,
            height: 88,
        }
    );
    assert_eq!(window_status(plan, true), 0);
    assert_eq!(placed_size(WS_POPUP, (100, 50), (64, 48)), (100, 50));
}
