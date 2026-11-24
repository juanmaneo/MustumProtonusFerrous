//! Replays of Wine's oracle rows. The oracle compiles the verbatim `info.c` in
//! `target/canon/mciavi32` with `-fsanitize=address,undefined`, with a fake device table
//! (`open_dev` in `main.c`) and no window queries. Each test names the row it reproduces, and the
//! expected values are the printed fields of `oracle.txt`. A field the row does not print is left
//! untouched in the oracle, which is `None` here.

use super::*;

pub(crate) fn units(text: &[u8]) -> Vec<u16> {
    text.iter().map(|&byte| u16::from(byte)).collect()
}

/// `open_dev` of the oracle: a device with the given fields, and `clip.avi` open when `has_file`.
pub(crate) fn device(
    time_format: u32,
    micro_sec_per_frame: u32,
    total_frames: u32,
    current_frame: u32,
    status: u32,
    set: u32,
    has_file: bool,
) -> Device {
    Device {
        time_format,
        micro_sec_per_frame,
        total_frames,
        current_frame,
        status,
        set,
        has_file,
        file_name: if has_file {
            units(b"clip.avi")
        } else {
            Vec::new()
        },
        bit_count: 24,
        audio_stream: 1,
    }
}

fn set_params(time_format: u32, audio: u32) -> Option<SetParams> {
    Some(SetParams { time_format, audio })
}

#[test]
fn o01_frame_to_time_milliseconds() {
    let dev = device(
        MCI_FORMAT_MILLISECONDS,
        33366,
        0,
        0,
        MCI_MODE_STOP,
        0,
        false,
    );
    assert_eq!(frame_to_time_format(&dev, 100), 3336);
}

#[test]
fn o02_frame_to_time_frames() {
    let dev = device(MCI_FORMAT_FRAMES, 33366, 0, 0, MCI_MODE_STOP, 0, false);
    assert_eq!(frame_to_time_format(&dev, 100), 100);
}

#[test]
fn o03_frame_to_time_unknown_format() {
    let dev = device(7, 33366, 0, 0, MCI_MODE_STOP, 0, false);
    assert_eq!(frame_to_time_format(&dev, 100), 0);
}

#[test]
fn o04_frame_to_time_wraps_the_product() {
    let dev = device(MCI_FORMAT_MILLISECONDS, 2, 0, 0, MCI_MODE_STOP, 0, false);
    assert_eq!(frame_to_time_format(&dev, 0xFFFF_FFFF), 4_294_967);
}

#[test]
fn o05_time_to_frame_milliseconds() {
    let dev = device(
        MCI_FORMAT_MILLISECONDS,
        40000,
        0,
        0,
        MCI_MODE_STOP,
        0,
        false,
    );
    assert_eq!(time_format_to_frame(&dev, 1000), Some(25));
}

#[test]
fn o06_time_to_frame_wraps_the_product() {
    let dev = device(
        MCI_FORMAT_MILLISECONDS,
        40000,
        0,
        0,
        MCI_MODE_STOP,
        0,
        false,
    );
    assert_eq!(time_format_to_frame(&dev, 0x8000_0000), Some(0));
}

#[test]
fn o07_dev_caps_null_parameter_block() {
    let dev = device(MCI_FORMAT_MILLISECONDS, 33366, 0, 0, MCI_MODE_STOP, 0, true);
    assert_eq!(
        mci_get_dev_caps(Some(&dev), MCI_GETDEVCAPS_ITEM, None),
        Reply::code(0x129)
    );
}

#[test]
fn o08_dev_caps_closed_device() {
    assert_eq!(
        mci_get_dev_caps(None, MCI_GETDEVCAPS_ITEM, Some(MCI_GETDEVCAPS_DEVICE_TYPE)),
        Reply::code(0x101)
    );
}

#[test]
fn o09_dev_caps_without_item_flag() {
    let dev = device(MCI_FORMAT_MILLISECONDS, 33366, 0, 0, MCI_MODE_STOP, 0, true);
    assert_eq!(
        mci_get_dev_caps(Some(&dev), 0, Some(MCI_GETDEVCAPS_DEVICE_TYPE)),
        Reply::code(0x111)
    );
}

#[test]
fn o10_dev_caps_device_type() {
    let dev = device(MCI_FORMAT_MILLISECONDS, 33366, 0, 0, MCI_MODE_STOP, 0, true);
    assert_eq!(
        mci_get_dev_caps(
            Some(&dev),
            MCI_GETDEVCAPS_ITEM,
            Some(MCI_GETDEVCAPS_DEVICE_TYPE)
        ),
        Reply::resource(0x0208_0208)
    );
}

#[test]
fn o11_dev_caps_unknown_item() {
    let dev = device(MCI_FORMAT_MILLISECONDS, 33366, 0, 0, MCI_MODE_STOP, 0, true);
    assert_eq!(
        mci_get_dev_caps(Some(&dev), MCI_GETDEVCAPS_ITEM, Some(0x7777)),
        Reply::code(0x112)
    );
}

#[test]
fn o12_info_file_truncates_to_the_buffer() {
    let dev = device(MCI_FORMAT_MILLISECONDS, 33366, 0, 0, MCI_MODE_STOP, 0, true);
    let mut text = units(b"########");
    assert_eq!(mci_info(Some(&dev), MCI_INFO_FILE, Some(&mut text[..4])), 0);
    assert_eq!(text, units(b"cli\0####"));
}

#[test]
fn o13_info_product_takes_priority_over_version() {
    let dev = device(MCI_FORMAT_MILLISECONDS, 33366, 0, 0, MCI_MODE_STOP, 0, true);
    let mut text = units(&[b'#'; 32]);
    assert_eq!(
        mci_info(
            Some(&dev),
            MCI_INFO_PRODUCT | MCI_INFO_VERSION,
            Some(&mut text[..])
        ),
        0
    );
    assert_eq!(text[..24], units(b"Wine's AVI player\0######")[..]);
}

#[test]
fn o14_info_unknown_flags_leave_the_buffer_alone() {
    let dev = device(MCI_FORMAT_MILLISECONDS, 33366, 0, 0, MCI_MODE_STOP, 0, true);
    let mut text = units(b"########");
    assert_eq!(
        mci_info(Some(&dev), MCI_NOTIFY, Some(&mut text[..8])),
        0x105
    );
    assert_eq!(text, units(b"########"));
}

#[test]
fn o15_set_time_format_rejects_unknown_format() {
    let mut dev = device(MCI_FORMAT_MILLISECONDS, 33366, 0, 0, MCI_MODE_STOP, 0, true);
    assert_eq!(
        mci_set(Some(&mut dev), MCI_SET_TIME_FORMAT, set_params(9, 0)),
        0x125
    );
    assert_eq!(dev.time_format, MCI_FORMAT_MILLISECONDS);
    assert_eq!(dev.set, 0);
}

#[test]
fn o16_set_time_format_stays_when_door_is_open() {
    let mut dev = device(MCI_FORMAT_MILLISECONDS, 33366, 0, 0, MCI_MODE_STOP, 0, true);
    assert_eq!(
        mci_set(
            Some(&mut dev),
            MCI_SET_TIME_FORMAT | MCI_SET_DOOR_OPEN,
            set_params(MCI_FORMAT_FRAMES, 0)
        ),
        0x112
    );
    assert_eq!(dev.time_format, MCI_FORMAT_FRAMES);
    assert_eq!(dev.set, 0);
}

#[test]
fn o17_set_on_video_and_all_audio() {
    let mut dev = device(MCI_FORMAT_MILLISECONDS, 33366, 0, 0, MCI_MODE_STOP, 0, true);
    assert_eq!(
        mci_set(
            Some(&mut dev),
            MCI_SET_ON | MCI_SET_VIDEO | MCI_SET_AUDIO,
            set_params(0, MCI_SET_AUDIO_ALL)
        ),
        0
    );
    assert_eq!(dev.time_format, MCI_FORMAT_MILLISECONDS);
    assert_eq!(dev.set, 7);
}

#[test]
fn o18_set_off_left_audio_keeps_the_left_bit() {
    let mut dev = device(MCI_FORMAT_MILLISECONDS, 33366, 0, 0, MCI_MODE_STOP, 3, true);
    assert_eq!(
        mci_set(
            Some(&mut dev),
            MCI_SET_OFF | MCI_SET_AUDIO,
            set_params(0, MCI_SET_AUDIO_LEFT)
        ),
        0
    );
    assert_eq!(dev.time_format, MCI_FORMAT_MILLISECONDS);
    assert_eq!(dev.set, 1);
}

#[test]
fn o19_set_on_unknown_audio_channel_changes_nothing() {
    let mut dev = device(MCI_FORMAT_MILLISECONDS, 33366, 0, 0, MCI_MODE_STOP, 2, true);
    assert_eq!(
        mci_set(
            Some(&mut dev),
            MCI_SET_ON | MCI_SET_AUDIO,
            set_params(0, 42)
        ),
        0
    );
    assert_eq!(dev.time_format, MCI_FORMAT_MILLISECONDS);
    assert_eq!(dev.set, 2);
}

#[test]
fn o20_status_length_without_a_file() {
    let dev = device(
        MCI_FORMAT_MILLISECONDS,
        33366,
        300,
        0,
        MCI_MODE_STOP,
        0,
        false,
    );
    assert_eq!(
        mci_status(Some(&dev), MCI_STATUS_ITEM, Some(MCI_STATUS_LENGTH)),
        Some(Reply {
            ret: 0x112,
            dw_return: Some(0)
        })
    );
}

#[test]
fn o21_status_length_with_a_file() {
    let dev = device(
        MCI_FORMAT_MILLISECONDS,
        33366,
        300,
        0,
        MCI_MODE_STOP,
        0,
        true,
    );
    assert_eq!(
        mci_status(Some(&dev), MCI_STATUS_ITEM, Some(MCI_STATUS_LENGTH)),
        Some(Reply::value(0x2719))
    );
}

#[test]
fn o22_status_mode_is_a_resource() {
    let dev = device(MCI_FORMAT_MILLISECONDS, 33366, 0, 0, MCI_MODE_PLAY, 0, true);
    assert_eq!(
        mci_status(Some(&dev), MCI_STATUS_ITEM, Some(MCI_STATUS_MODE)),
        Some(Reply::resource(0x020e_020e))
    );
}

#[test]
fn o23_status_frame_rate() {
    let dev = device(MCI_FORMAT_MILLISECONDS, 33366, 0, 0, MCI_MODE_STOP, 0, true);
    assert_eq!(
        mci_status(Some(&dev), MCI_STATUS_ITEM, Some(MCI_DGV_STATUS_FRAME_RATE)),
        Some(Reply::value(0x7513))
    );
}

#[test]
fn o24_status_frame_rate_with_zero_frame_time() {
    let dev = device(MCI_FORMAT_MILLISECONDS, 0, 0, 0, MCI_MODE_STOP, 0, true);
    assert_eq!(
        mci_status(Some(&dev), MCI_STATUS_ITEM, Some(MCI_DGV_STATUS_FRAME_RATE)),
        Some(Reply::value(0xffff_ffff))
    );
}

#[test]
fn o25_status_pause_mode_while_playing() {
    let dev = device(MCI_FORMAT_MILLISECONDS, 33366, 0, 0, MCI_MODE_PLAY, 0, true);
    assert_eq!(
        mci_status(Some(&dev), MCI_STATUS_ITEM, Some(MCI_DGV_STATUS_PAUSE_MODE)),
        Some(Reply::code(0x12e))
    );
}
