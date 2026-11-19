//! Edge cases that the oracle rows do not reach: the argument checks, the failure of each device
//! call, the use count and handle state that each command leaves behind, and the constructors.
//! The expected values are those of the verbatim Wine code for the same inputs.

use crate::devcaps::{get_dev_caps, DevCapsParms};
use crate::device::QChannel;
use crate::error::*;
use crate::frame::Msf;
use crate::info::info;
use crate::mci::*;
use crate::open::{close, open, OpenParms};
use crate::oracle_checks::{quoted, Mock};
use crate::seek::{seek, skip_data_tracks, SeekParms};
use crate::set::{set, SetParms};
use crate::status::{status, StatusParms};
use crate::time::{calc_frame, calc_time};
use crate::toc::{Toc, TrackData};
use crate::Driver;

/// The value that a failed call leaves in the `value` field of its parameter block.
const UNWRITTEN: u32 = 0xdead_beef;

fn entry(control: u8, minute: u8, second: u8, frame: u8) -> TrackData {
    TrackData {
        control,
        address: Msf {
            minute,
            second,
            frame,
        },
    }
}

/// `1-2/4:0:2:0,0:20:0:0,0:40:0:0`: track 1 is a data track at 150 frames, track 2 at 90000.
fn toc_t2() -> Toc {
    Toc::new(
        1,
        2,
        &[entry(4, 0, 2, 0), entry(0, 20, 0, 0), entry(0, 40, 0, 0)],
    )
}

/// `1-3/4:0:2:0,0:20:0:0,0:40:0:0,0:50:0:0`
fn toc_t3() -> Toc {
    Toc::new(
        1,
        3,
        &[
            entry(4, 0, 2, 0),
            entry(0, 20, 0, 0),
            entry(0, 40, 0, 0),
            entry(0, 50, 0, 0),
        ],
    )
}

fn mock_with(toc: Toc) -> Mock {
    Mock {
        toc: Some(toc),
        ..Default::default()
    }
}

/// A device whose control calls fail with `last_error`.
fn dev_err(last_error: u32) -> Mock {
    Mock {
        last_error,
        ..Default::default()
    }
}

/// A device whose Q channel reads track 1 at 0:2:0 in `audio_status`.
fn dev_q(audio_status: u8) -> Mock {
    Mock {
        q: Some(QChannel {
            audio_status,
            track: 1,
            absolute: Msf {
                minute: 0,
                second: 2,
                frame: 0,
            },
        }),
        ..Default::default()
    }
}

/// A device with CD-ROM drives at `letters`.
fn cd_mock(letters: &str) -> Mock {
    let mut mock = Mock::default();
    for letter in letters.bytes() {
        mock.cd_drives[usize::from(letter - b'A')] = true;
    }
    mock
}

fn utf16(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

fn fields(at: Msf) -> (u8, u8, u8) {
    (at.minute, at.second, at.frame)
}

fn open_driver(time_format: u32) -> Driver {
    Driver {
        use_count: 1,
        shareable: false,
        time_format,
    }
}

fn closed_driver() -> Driver {
    Driver {
        use_count: 0,
        shareable: false,
        time_format: MCI_FORMAT_MSF,
    }
}

/// Returns the return code and the value that `status` stored.
fn run_status(drv: &Driver, dev: &mut Mock, flags: u32, item: u32, track: u32) -> (u32, u32) {
    let mut parms = StatusParms {
        item,
        track,
        value: UNWRITTEN,
    };
    let ret = status(drv, dev, flags, Some(&mut parms));
    (ret, parms.value)
}

fn run_open(drv: &mut Driver, dev: &mut Mock, flags: u32, element: &[u16]) -> u32 {
    open(drv, dev, flags, Some(&OpenParms { element }))
}

fn run_seek(drv: &Driver, dev: &mut Mock, flags: u32, to: u32) -> u32 {
    seek(drv, dev, flags, Some(&SeekParms { to }))
}

fn run_set(drv: &mut Driver, dev: &mut Mock, flags: u32, time_format: u32) -> u32 {
    set(drv, dev, flags, Some(&SetParms { time_format }))
}

/// Returns the return code and the text that was copied into a buffer of `len` characters.
fn run_info(drv: &Driver, dev: &mut Mock, flags: u32, len: usize) -> (u32, String) {
    let mut buffer = vec![0u16; len];
    let ret = info(drv, dev, flags, Some(&mut buffer[..]));
    (ret, quoted(&buffer))
}

#[test]
fn status_argument_checks() {
    let drv = open_driver(MCI_FORMAT_MSF);
    let mut dev = Mock::default();
    assert_eq!(
        status(&drv, &mut dev, MCI_STATUS_ITEM, None),
        MCIERR_NULL_PARAMETER_BLOCK
    );
    assert_eq!(
        run_status(
            &closed_driver(),
            &mut dev,
            MCI_STATUS_ITEM,
            MCI_STATUS_MODE,
            0
        ),
        (MCIERR_INVALID_DEVICE_ID, UNWRITTEN)
    );
    assert_eq!(
        run_status(&drv, &mut dev, 0, MCI_STATUS_MODE, 0),
        (MCIERR_MISSING_PARAMETER, UNWRITTEN)
    );
    assert_eq!(
        run_status(&drv, &mut dev, MCI_STATUS_ITEM, 99, 0),
        (MCIERR_UNSUPPORTED_FUNCTION, UNWRITTEN)
    );
}

#[test]
fn status_current_track() {
    let drv = open_driver(MCI_FORMAT_MSF);
    assert_eq!(
        run_status(
            &drv,
            &mut dev_q(0x11),
            MCI_STATUS_ITEM,
            MCI_STATUS_CURRENT_TRACK,
            0
        ),
        (0, 1)
    );
    assert_eq!(
        run_status(
            &drv,
            &mut dev_err(21),
            MCI_STATUS_ITEM,
            MCI_STATUS_CURRENT_TRACK,
            0
        ),
        (MCIERR_DEVICE_NOT_READY, UNWRITTEN)
    );
}

#[test]
fn status_length() {
    let msf = open_driver(MCI_FORMAT_MSF);
    let tmsf = open_driver(MCI_FORMAT_TMSF);
    let mut dev = mock_with(toc_t2());
    assert_eq!(
        run_status(
            &msf,
            &mut dev_err(21),
            MCI_STATUS_ITEM,
            MCI_STATUS_LENGTH,
            0
        ),
        (MCIERR_DEVICE_NOT_READY, UNWRITTEN)
    );
    assert_eq!(
        run_status(
            &msf,
            &mut dev,
            MCI_STATUS_ITEM | MCI_TRACK,
            MCI_STATUS_LENGTH,
            3
        ),
        (MCIERR_OUTOFRANGE, UNWRITTEN)
    );
    assert_eq!(
        run_status(
            &msf,
            &mut dev,
            MCI_STATUS_ITEM | MCI_TRACK,
            MCI_STATUS_LENGTH,
            1
        ),
        (MCI_COLONIZED3_RETURN, 0x3a13)
    );
    assert_eq!(
        run_status(
            &msf,
            &mut dev,
            MCI_STATUS_ITEM | MCI_TRACK,
            MCI_STATUS_LENGTH,
            2
        ),
        (MCI_COLONIZED3_RETURN, 0x4a3b13)
    );
    assert_eq!(
        run_status(&tmsf, &mut dev, MCI_STATUS_ITEM, MCI_STATUS_LENGTH, 0),
        (MCI_COLONIZED3_RETURN, 0x4a3927)
    );
}

#[test]
fn status_mode() {
    let drv = open_driver(MCI_FORMAT_MSF);
    let cases = [
        (0x11, 0x020e_020e),
        (0x12, 0x0211_0211),
        (0x13, 0x020d_020d),
        (0x15, 0x020d_020d),
        (0x14, 0x020c_020c),
        (0x00, 0x020c_020c),
    ];
    for (audio_status, value) in cases {
        assert_eq!(
            run_status(
                &drv,
                &mut dev_q(audio_status),
                MCI_STATUS_ITEM,
                MCI_STATUS_MODE,
                0
            ),
            (MCI_RESOURCE_RETURNED, value)
        );
    }
    assert_eq!(
        run_status(&drv, &mut dev_err(21), MCI_STATUS_ITEM, MCI_STATUS_MODE, 0),
        (MCI_RESOURCE_RETURNED, 0x0212_0212)
    );
    assert_eq!(
        run_status(&drv, &mut dev_err(50), MCI_STATUS_ITEM, MCI_STATUS_MODE, 0),
        (MCI_RESOURCE_RETURNED, 0x020c_020c)
    );
}

#[test]
fn status_media_present() {
    let drv = open_driver(MCI_FORMAT_MSF);
    assert_eq!(
        run_status(
            &drv,
            &mut dev_q(0x11),
            MCI_STATUS_ITEM,
            MCI_STATUS_MEDIA_PRESENT,
            0
        ),
        (MCI_RESOURCE_RETURNED, 0x0214_0001)
    );
    assert_eq!(
        run_status(
            &drv,
            &mut dev_err(21),
            MCI_STATUS_ITEM,
            MCI_STATUS_MEDIA_PRESENT,
            0
        ),
        (MCI_RESOURCE_RETURNED, 0x0213_0000)
    );
}

#[test]
fn status_number_of_tracks() {
    let drv = open_driver(MCI_FORMAT_MSF);
    assert_eq!(
        run_status(
            &drv,
            &mut mock_with(toc_t2()),
            MCI_STATUS_ITEM,
            MCI_STATUS_NUMBER_OF_TRACKS,
            0
        ),
        (0, 2)
    );
    assert_eq!(
        run_status(
            &drv,
            &mut dev_err(21),
            MCI_STATUS_ITEM,
            MCI_STATUS_NUMBER_OF_TRACKS,
            0
        ),
        (MCIERR_DEVICE_NOT_READY, UNWRITTEN)
    );
}

#[test]
fn status_position() {
    let drv = open_driver(MCI_FORMAT_MSF);
    let mut dev = mock_with(toc_t2());
    let item = MCI_STATUS_POSITION;
    assert_eq!(
        run_status(
            &drv,
            &mut dev,
            MCI_STATUS_ITEM | MCI_STATUS_START | MCI_TRACK,
            item,
            1
        ),
        (MCIERR_FLAGS_NOT_COMPATIBLE, UNWRITTEN)
    );
    assert_eq!(
        run_status(&drv, &mut dev_err(50), MCI_STATUS_ITEM, item, 0),
        (MCIERR_HARDWARE, UNWRITTEN)
    );
    assert_eq!(
        run_status(
            &drv,
            &mut dev_err(21),
            MCI_STATUS_ITEM | MCI_STATUS_START,
            item,
            0
        ),
        (MCIERR_DEVICE_NOT_READY, UNWRITTEN)
    );
    assert_eq!(
        run_status(&drv, &mut dev, MCI_STATUS_ITEM | MCI_TRACK, item, 9),
        (MCIERR_OUTOFRANGE, UNWRITTEN)
    );
    assert_eq!(
        run_status(&drv, &mut dev_err(21), MCI_STATUS_ITEM | MCI_TRACK, item, 1),
        (MCIERR_DEVICE_NOT_READY, UNWRITTEN)
    );
    assert_eq!(
        run_status(&drv, &mut dev, MCI_STATUS_ITEM | MCI_STATUS_START, item, 0),
        (0x20000, 0x200)
    );
    assert_eq!(
        run_status(&drv, &mut dev, MCI_STATUS_ITEM | MCI_TRACK, item, 2),
        (0x20000, 0x14)
    );
    assert_eq!(
        run_status(&drv, &mut dev_q(0x11), MCI_STATUS_ITEM, item, 0),
        (0x20000, 0x200)
    );
}

#[test]
fn status_ready() {
    let drv = open_driver(MCI_FORMAT_MSF);
    let item = MCI_STATUS_READY;
    assert_eq!(
        run_status(&drv, &mut dev_q(0x11), MCI_STATUS_ITEM, item, 0),
        (MCI_RESOURCE_RETURNED, 0x0214_0001)
    );
    assert_eq!(
        run_status(&drv, &mut dev_q(0x14), MCI_STATUS_ITEM, item, 0),
        (MCI_RESOURCE_RETURNED, 0x0213_0000)
    );
    assert_eq!(
        run_status(&drv, &mut dev_err(21), MCI_STATUS_ITEM, item, 0),
        (MCI_RESOURCE_RETURNED, 0x0213_0000)
    );
}

#[test]
fn status_time_format() {
    let item = MCI_STATUS_TIME_FORMAT;
    assert_eq!(
        run_status(
            &open_driver(MCI_FORMAT_MILLISECONDS),
            &mut Mock::default(),
            MCI_STATUS_ITEM,
            item,
            0
        ),
        (MCI_RESOURCE_RETURNED, 0x0215_0000)
    );
    assert_eq!(
        run_status(
            &open_driver(MCI_FORMAT_MSF),
            &mut Mock::default(),
            MCI_STATUS_ITEM,
            item,
            0
        ),
        (MCI_RESOURCE_RETURNED, 0x0217_0002)
    );
}

#[test]
fn status_track_type() {
    let drv = open_driver(MCI_FORMAT_MSF);
    let mut dev = mock_with(toc_t2());
    let track = MCI_STATUS_ITEM | MCI_TRACK;
    for item in [4001, MCI_CDA_STATUS_TYPE_TRACK] {
        assert_eq!(
            run_status(&drv, &mut dev, MCI_STATUS_ITEM, item, 1),
            (MCIERR_MISSING_PARAMETER, UNWRITTEN)
        );
        assert_eq!(
            run_status(&drv, &mut dev_err(21), track, item, 1),
            (MCIERR_DEVICE_NOT_READY, UNWRITTEN)
        );
        assert_eq!(
            run_status(&drv, &mut dev, track, item, 9),
            (MCIERR_OUTOFRANGE, UNWRITTEN)
        );
        assert_eq!(run_status(&drv, &mut dev, track, item, 2), (0, 0x440));
        assert_eq!(run_status(&drv, &mut dev, track, item, 1), (0, 0x441));
    }
}

#[test]
fn device_error_codes() {
    assert_eq!(device_error(21), MCIERR_DEVICE_NOT_READY);
    assert_eq!(device_error(50), MCIERR_HARDWARE);
    assert_eq!(device_error(1117), MCIERR_HARDWARE);
    assert_eq!(device_error(5), MCIERR_DRIVER_INTERNAL);
}

#[test]
fn dev_caps_items() {
    let values = [
        0x0213_0000,
        0x0214_0001,
        0x0213_0000,
        0x0204_0204,
        0x0213_0000,
        0x0213_0000,
        0x0214_0001,
        0x0214_0001,
        0x0213_0000,
    ];
    for (index, &value) in values.iter().enumerate() {
        let mut parms = DevCapsParms {
            item: index as u32 + 1,
            value: UNWRITTEN,
        };
        assert_eq!(
            get_dev_caps(MCI_GETDEVCAPS_ITEM, Some(&mut parms)),
            MCI_RESOURCE_RETURNED
        );
        assert_eq!(parms.value, value, "item {}", index + 1);
    }
}

#[test]
fn dev_caps_argument_checks() {
    let mut parms = DevCapsParms {
        item: 1,
        value: UNWRITTEN,
    };
    assert_eq!(get_dev_caps(0, Some(&mut parms)), MCIERR_MISSING_PARAMETER);
    assert_eq!(parms.value, UNWRITTEN);
    let mut parms = DevCapsParms {
        item: 10,
        value: UNWRITTEN,
    };
    assert_eq!(
        get_dev_caps(MCI_GETDEVCAPS_ITEM, Some(&mut parms)),
        MCIERR_UNSUPPORTED_FUNCTION
    );
    assert_eq!(parms.value, UNWRITTEN);
    assert_eq!(
        get_dev_caps(MCI_GETDEVCAPS_ITEM, None),
        MCIERR_NULL_PARAMETER_BLOCK
    );
}

#[test]
fn calc_frame_conversions() {
    assert_eq!(
        calc_frame(&mut mock_with(toc_t2()), MCI_FORMAT_TMSF, 2),
        90000
    );
    assert_eq!(
        calc_frame(&mut mock_with(toc_t2()), MCI_FORMAT_TMSF, 0x20001),
        300
    );
    assert_eq!(calc_frame(&mut Mock::default(), MCI_FORMAT_TMSF, 0x0101), 0);
    assert_eq!(calc_frame(&mut mock_with(toc_t2()), MCI_FORMAT_TMSF, 0), 0);
    assert_eq!(calc_frame(&mut mock_with(toc_t2()), MCI_FORMAT_TMSF, 9), 0);
    assert_eq!(
        calc_frame(&mut Mock::default(), MCI_FORMAT_MILLISECONDS, 0),
        0
    );
    assert_eq!(
        calc_frame(&mut Mock::default(), MCI_FORMAT_MILLISECONDS, 1000),
        75
    );
}

#[test]
fn calc_time_conversions() {
    assert_eq!(
        calc_time(&mut Mock::default(), MCI_FORMAT_TMSF, 100),
        (0, 0)
    );
    assert_eq!(
        calc_time(&mut mock_with(toc_t2()), MCI_FORMAT_TMSF, 100),
        (0, 0)
    );
    assert_eq!(
        calc_time(&mut mock_with(toc_t2()), MCI_FORMAT_TMSF, 180001),
        (0, 0)
    );
    assert_eq!(
        calc_time(&mut mock_with(toc_t2()), MCI_FORMAT_TMSF, 180000),
        (0x1402, MCI_COLONIZED4_RETURN)
    );
    assert_eq!(
        calc_time(&mut mock_with(toc_t2()), MCI_FORMAT_TMSF, 150),
        (1, MCI_COLONIZED4_RETURN)
    );
    assert_eq!(
        calc_time(&mut Mock::default(), MCI_FORMAT_MILLISECONDS, 0),
        (1, 0)
    );
    assert_eq!(
        calc_time(&mut Mock::default(), MCI_FORMAT_MSF, 1),
        (0x10000, MCI_COLONIZED3_RETURN)
    );
}

#[test]
fn msf_wraps_minutes_to_a_byte() {
    assert_eq!(fields(Msf::from_frames(256 * 4500)), (0, 0, 0));
    assert_eq!(fields(Msf::from_frames(179999)), (39, 59, 74));
}

#[test]
fn toc_lookups() {
    let toc = toc_t2();
    assert_eq!(toc.track_count(), 2);
    assert_eq!(toc.first_frame(), 150);
    assert_eq!(toc.lead_out_frame(), 180000);
    assert!(toc.contains_track(2));
    assert!(!toc.contains_track(3));
    assert_eq!(Toc::new(0, 99, &[]).frame_of(100), 0);
    assert_eq!(toc_t3().serial(), 3932672);
    assert_eq!(toc_t2().serial(), 1491082);
    assert_eq!(
        Toc::new(1, 1, &[entry(4, 0, 2, 0), entry(0, 40, 0, 0)]).serial(),
        180362
    );
}

#[test]
fn toc_skip_data_tracks() {
    let toc = toc_t2();
    assert_eq!(toc.skip_data_tracks(100), None);
    assert_eq!(toc.skip_data_tracks(180000), None);
    assert_eq!(toc.skip_data_tracks(150), Some(90000));
    assert_eq!(toc.skip_data_tracks(90000), Some(90000));
    assert_eq!(toc.skip_data_tracks(179999), Some(179999));
    let data_tail = Toc::new(
        1,
        2,
        &[entry(4, 0, 2, 0), entry(4, 20, 0, 0), entry(4, 40, 0, 0)],
    );
    assert_eq!(data_tail.skip_data_tracks(100000), Some(179999));
}

#[test]
fn skip_data_tracks_wrapper() {
    let mut frame = 150;
    assert_eq!(skip_data_tracks(&mut mock_with(toc_t2()), &mut frame), 0);
    assert_eq!(frame, 90000);

    let mut frame = 150;
    assert_eq!(
        skip_data_tracks(&mut dev_err(21), &mut frame),
        MCIERR_DEVICE_NOT_READY
    );
    assert_eq!(frame, 150);

    let mut frame = 100;
    assert_eq!(
        skip_data_tracks(&mut mock_with(toc_t2()), &mut frame),
        MCIERR_OUTOFRANGE
    );
    assert_eq!(frame, 100);
}

#[test]
fn open_null_parameter_block() {
    let mut drv = closed_driver();
    assert_eq!(
        open(&mut drv, &mut cd_mock("D"), 0, None),
        MCIERR_NULL_PARAMETER_BLOCK
    );
    assert_eq!(drv.use_count, 0);
}

#[test]
fn open_first_use_selects_first_cd_drive() {
    let mut drv = Driver {
        use_count: 0,
        shareable: false,
        time_format: MCI_FORMAT_TMSF,
    };
    let mut dev = cd_mock("DE");
    assert_eq!(run_open(&mut drv, &mut dev, 0, &[]), 0);
    assert_eq!(drv.use_count, 1);
    assert!(!drv.shareable);
    assert_eq!(drv.time_format, MCI_FORMAT_MSF);
    assert_eq!(dev.opened, Some(b'D'));
}

#[test]
fn open_shareable_use() {
    let mut drv = Driver {
        use_count: 1,
        shareable: true,
        time_format: MCI_FORMAT_MSF,
    };
    assert_eq!(
        run_open(&mut drv, &mut cd_mock("D"), MCI_OPEN_SHAREABLE, &[]),
        0
    );
    assert_eq!(drv.use_count, 2);

    let mut dev = cd_mock("D");
    assert_eq!(
        run_open(
            &mut open_driver(MCI_FORMAT_MSF),
            &mut dev,
            MCI_OPEN_SHAREABLE,
            &[]
        ),
        MCIERR_MUST_USE_SHAREABLE
    );
    assert_eq!(dev.opened, None);

    let mut drv = Driver {
        use_count: 1,
        shareable: true,
        time_format: MCI_FORMAT_MSF,
    };
    assert_eq!(
        run_open(&mut drv, &mut cd_mock("D"), 0, &[]),
        MCIERR_MUST_USE_SHAREABLE
    );
    assert_eq!(drv.use_count, 1);
}

#[test]
fn open_element_checks() {
    let closed = || Driver {
        use_count: 0,
        shareable: false,
        time_format: MCI_FORMAT_MSF,
    };
    let mut drv = closed();
    assert_eq!(
        run_open(
            &mut drv,
            &mut cd_mock("D"),
            MCI_OPEN_ELEMENT | MCI_OPEN_ELEMENT_ID,
            &utf16("D")
        ),
        MCIERR_FLAGS_NOT_COMPATIBLE
    );
    assert_eq!(drv.use_count, 0);

    for element in [utf16("1"), utf16(""), vec![0x100]] {
        let mut drv = closed();
        assert_eq!(
            run_open(&mut drv, &mut cd_mock("D"), MCI_OPEN_ELEMENT, &element),
            MCIERR_INVALID_FILE
        );
        assert_eq!(drv.use_count, 0);
    }

    let mut drv = closed();
    let mut dev = cd_mock("D");
    assert_eq!(
        run_open(&mut drv, &mut dev, MCI_OPEN_ELEMENT, &utf16("d")),
        0
    );
    assert_eq!(dev.opened, Some(b'D'));

    let mut drv = closed();
    assert_eq!(
        run_open(&mut drv, &mut cd_mock("D"), MCI_OPEN_ELEMENT, &utf16("E")),
        MCIERR_INVALID_FILE
    );
    assert_eq!(drv.use_count, 0);

    let mut drv = closed();
    assert_eq!(
        run_open(&mut drv, &mut cd_mock(""), 0, &[]),
        MCIERR_CANNOT_LOAD_DRIVER
    );
    assert_eq!(drv.use_count, 0);

    let mut dev = cd_mock("EF");
    assert_eq!(run_open(&mut closed(), &mut dev, 0, &[]), 0);
    assert_eq!(dev.opened, Some(b'E'));
}

#[test]
fn open_handle_failure_releases_use() {
    let mut drv = Driver {
        use_count: 0,
        shareable: false,
        time_format: MCI_FORMAT_TMSF,
    };
    let mut dev = Mock {
        open_handle_fails: true,
        ..cd_mock("D")
    };
    assert_eq!(
        run_open(&mut drv, &mut dev, 0, &[]),
        MCIERR_MUST_USE_SHAREABLE
    );
    assert_eq!(drv.use_count, 0);
    assert_eq!(drv.time_format, MCI_FORMAT_MSF);
    assert_eq!(dev.opened, Some(b'D'));
}

#[test]
fn close_use_count() {
    let mut drv = open_driver(MCI_FORMAT_MSF);
    let mut dev = Mock::default();
    assert_eq!(close(&mut drv, &mut dev), 0);
    assert_eq!(drv.use_count, 0);
    assert_eq!(dev.closed, 1);

    let mut drv = Driver {
        use_count: 2,
        shareable: true,
        time_format: MCI_FORMAT_MSF,
    };
    let mut dev = Mock::default();
    assert_eq!(close(&mut drv, &mut dev), 0);
    assert_eq!(drv.use_count, 1);
    assert_eq!(dev.closed, 0);

    let mut dev = Mock::default();
    assert_eq!(
        close(&mut closed_driver(), &mut dev),
        MCIERR_INVALID_DEVICE_ID
    );
    assert_eq!(dev.closed, 0);
}

#[test]
fn seek_positions() {
    let drv = open_driver(MCI_FORMAT_MSF);
    let mut dev = mock_with(toc_t2());
    assert_eq!(run_seek(&drv, &mut dev, MCI_SEEK_TO_START, 0), 0);
    assert_eq!(dev.seek.map(fields), Some((20, 0, 0)));

    let mut dev = mock_with(toc_t2());
    assert_eq!(run_seek(&drv, &mut dev, MCI_TO, 0x1e0c0a), 0);
    assert_eq!(dev.seek.map(fields), Some((20, 0, 0)));

    let mut dev = mock_with(toc_t2());
    assert_eq!(run_seek(&drv, &mut dev, MCI_SEEK_TO_END, 0), 0);
    assert_eq!(dev.seek.map(fields), Some((39, 59, 74)));
}

#[test]
fn seek_checks() {
    let drv = open_driver(MCI_FORMAT_MSF);
    let mut dev = Mock::default();
    assert_eq!(
        seek(&drv, &mut dev, MCI_SEEK_TO_START, None),
        MCIERR_NULL_PARAMETER_BLOCK
    );
    assert_eq!(run_seek(&drv, &mut dev, 0, 0), MCIERR_MISSING_PARAMETER);
    assert_eq!(
        run_seek(&drv, &mut dev, MCI_SEEK_TO_START | MCI_SEEK_TO_END, 0),
        MCIERR_FLAGS_NOT_COMPATIBLE
    );
    assert_eq!(
        run_seek(&drv, &mut dev_err(21), MCI_SEEK_TO_START, 0),
        MCIERR_DEVICE_NOT_READY
    );
    assert_eq!(
        run_seek(&closed_driver(), &mut dev, MCI_SEEK_TO_START, 0),
        MCIERR_INVALID_DEVICE_ID
    );
    assert!(dev.seek.is_none());

    let mut dev = mock_with(toc_t2());
    assert_eq!(run_seek(&drv, &mut dev, MCI_TO, 0), MCIERR_OUTOFRANGE);
    assert!(dev.seek.is_none());

    let mut dev = Mock {
        seek_fails: true,
        ..mock_with(toc_t2())
    };
    assert_eq!(
        run_seek(&drv, &mut dev, MCI_SEEK_TO_START, 0),
        MCIERR_HARDWARE
    );
    assert_eq!(dev.seek.map(fields), Some((20, 0, 0)));
}

#[test]
fn set_door_and_time_format() {
    let mut drv = open_driver(MCI_FORMAT_MSF);
    let mut dev = Mock::default();
    assert_eq!(
        run_set(
            &mut drv,
            &mut dev,
            MCI_SET_DOOR_OPEN | MCI_SET_DOOR_CLOSED,
            0
        ),
        0
    );
    assert_eq!(dev.doors, vec![true, false]);
    assert_eq!(drv.time_format, MCI_FORMAT_MSF);

    for format in [MCI_FORMAT_MILLISECONDS, MCI_FORMAT_MSF, MCI_FORMAT_TMSF] {
        assert_eq!(run_set(&mut drv, &mut dev, MCI_SET_TIME_FORMAT, format), 0);
        assert_eq!(drv.time_format, format);
    }
    assert_eq!(
        run_set(&mut drv, &mut dev, MCI_SET_TIME_FORMAT, 7),
        MCIERR_BAD_TIME_FORMAT
    );
    assert_eq!(drv.time_format, MCI_FORMAT_TMSF);
    assert_eq!(run_set(&mut drv, &mut dev, 0, 7), 0);
    assert_eq!(drv.time_format, MCI_FORMAT_TMSF);
}

#[test]
fn set_checks() {
    let mut drv = open_driver(MCI_FORMAT_MSF);
    let mut dev = Mock::default();
    assert_eq!(
        set(&mut drv, &mut dev, MCI_SET_DOOR_OPEN, None),
        MCIERR_NULL_PARAMETER_BLOCK
    );
    assert_eq!(dev.doors, vec![true]);

    let mut dev = Mock::default();
    assert_eq!(
        run_set(
            &mut closed_driver(),
            &mut dev,
            MCI_SET_DOOR_OPEN | MCI_SET_DOOR_CLOSED,
            0
        ),
        MCIERR_INVALID_DEVICE_ID
    );
    assert!(dev.doors.is_empty());
}

#[test]
fn info_text_and_buffer() {
    let drv = open_driver(MCI_FORMAT_MSF);
    let mut dev = mock_with(toc_t2());
    assert_eq!(
        run_info(&drv, &mut dev, MCI_INFO_PRODUCT, 32),
        (0, "Wine's audio CD".to_string())
    );
    assert_eq!(
        run_info(&drv, &mut dev, MCI_INFO_MEDIA_IDENTITY, 32),
        (0, "1491082".to_string())
    );
    assert_eq!(
        run_info(&drv, &mut dev, MCI_INFO_PRODUCT, 2),
        (0, "W".to_string())
    );
    assert_eq!(
        run_info(&drv, &mut dev, MCI_INFO_PRODUCT, 1),
        (0, String::new())
    );
}

#[test]
fn info_checks() {
    let drv = open_driver(MCI_FORMAT_MSF);
    let mut dev = mock_with(toc_t2());
    assert_eq!(
        info(&drv, &mut dev, MCI_INFO_PRODUCT, None),
        MCIERR_NULL_PARAMETER_BLOCK
    );
    assert_eq!(
        run_info(&closed_driver(), &mut dev, MCI_INFO_PRODUCT, 32),
        (MCIERR_INVALID_DEVICE_ID, String::new())
    );
    assert_eq!(
        run_info(&drv, &mut dev, MCI_INFO_MEDIA_UPC, 0),
        (MCIERR_NO_IDENTITY, String::new())
    );
    assert_eq!(
        run_info(&drv, &mut dev_err(21), MCI_INFO_MEDIA_IDENTITY, 32),
        (MCIERR_DEVICE_NOT_READY, String::new())
    );
    assert_eq!(
        run_info(&drv, &mut dev, 0, 4),
        (MCIERR_MISSING_PARAMETER, String::new())
    );
    assert_eq!(
        run_info(&drv, &mut dev, MCI_INFO_PRODUCT, 0),
        (MCIERR_PARAM_OVERFLOW, String::new())
    );
}
